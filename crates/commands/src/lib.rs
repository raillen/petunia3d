//! Petunia3D — Undo/Redo via Command Pattern (snapshots).
//!
//! Regra do spec: operações destrutivas são comandos desfazíveis.
//! Snapshots clonados são suficientes para low-poly (malhas pequenas) e
//! mantêm a implementação simples e correta. Cap de 100 níveis.

/// Pilha genérica de undo/redo sobre estado clonável com rastreamento determinístico de estado salvo/dirty.
#[derive(Debug, Default)]
pub struct UndoStack<T: Clone> {
    undo: Vec<(String, T)>,
    redo: Vec<(String, T)>,
    cap: usize,
    clean_version: Option<usize>,
    current_version: usize,
}

impl<T: Clone> UndoStack<T> {
    pub fn new() -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
            cap: 100,
            clean_version: Some(0),
            current_version: 0,
        }
    }

    /// Salva o estado ATUAL antes de uma mutação (chamar antes de mudar).
    pub fn checkpoint(&mut self, label: impl Into<String>, current: &T) {
        self.undo.push((label.into(), current.clone()));
        self.current_version = self.current_version.saturating_add(1);
        if self.undo.len() > self.cap {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
    pub fn undo_label(&self) -> Option<&str> {
        self.undo.last().map(|(l, _)| l.as_str())
    }
    pub fn redo_label(&self) -> Option<&str> {
        self.redo.last().map(|(l, _)| l.as_str())
    }
    pub fn depth(&self) -> (usize, usize) {
        (self.undo.len(), self.redo.len())
    }

    /// Marca o estado atual do histórico como sincronizado/salvo em disco.
    pub fn mark_clean(&mut self) {
        self.clean_version = Some(self.current_version);
    }

    /// Força o estado a ser marcado como não salvo (dirty).
    pub fn mark_dirty(&mut self) {
        self.clean_version = None;
    }

    /// Verifica se o estado atual coincide exatamente com o ponto salvo.
    pub fn is_clean(&self) -> bool {
        self.clean_version == Some(self.current_version)
    }

    /// Verifica se há alterações não salvas no histórico.
    pub fn is_dirty(&self) -> bool {
        !self.is_clean()
    }

    /// Desfaz: guarda estado atual no redo, retorna estado anterior.
    pub fn undo(&mut self, current: T) -> Option<T> {
        let (label, prev) = self.undo.pop()?;
        self.current_version = self.current_version.saturating_sub(1);
        self.redo.push((label, current));
        Some(prev)
    }

    /// Refaz: guarda estado atual no undo, retorna próximo estado.
    pub fn redo(&mut self, current: T) -> Option<T> {
        let (label, next) = self.redo.pop()?;
        self.current_version = self.current_version.saturating_add(1);
        self.undo.push((label, current));
        Some(next)
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
        self.current_version = 0;
        self.clean_version = Some(0);
    }
}

/// Trait genérica para comandos executáveis e transacionais.
pub trait Command<Context, Res = (), Err = String>: Send + Sync {
    /// Rótulo legível para telemetria e pilha de desfazer/refazer (Undo/Redo).
    fn label(&self) -> &'static str;

    /// Executa a operação contra o contexto mutável.
    fn execute(&self, ctx: &mut Context) -> Result<Res, Err>;

    /// Indica se o comando altera o estado persistente e exige captura prévia de checkpoint de Undo.
    fn is_destructive(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_redo_roundtrip() {
        let mut st: UndoStack<Vec<i32>> = UndoStack::new();
        let mut cur = vec![1];
        st.checkpoint("add", &cur);
        cur.push(2);
        assert_eq!(st.undo(cur.clone()), Some(vec![1]));
        let cur = vec![1];
        assert_eq!(st.redo(cur), Some(vec![1, 2]));
        assert!(!st.can_redo());
    }

    #[test]
    fn checkpoint_clears_redo() {
        let mut st: UndoStack<i32> = UndoStack::new();
        st.checkpoint("a", &1);
        let _ = st.undo(2);
        assert!(st.can_redo());
        st.checkpoint("b", &1);
        assert!(!st.can_redo());
    }

    struct IncrementCmd(i32);
    impl Command<i32> for IncrementCmd {
        fn label(&self) -> &'static str {
            "increment"
        }
        fn execute(&self, ctx: &mut i32) -> Result<(), String> {
            *ctx += self.0;
            Ok(())
        }
    }

    #[test]
    fn test_undo_stack_dirty_state_tracking() {
        let mut st: UndoStack<i32> = UndoStack::new();
        // Initial state is clean (at saved/start version 0)
        assert!(st.is_clean());
        assert!(!st.is_dirty());

        // Mutation 1 -> dirty
        let mut cur = 0;
        st.checkpoint("step 1", &cur);
        cur = 1;
        assert!(st.is_dirty());

        // Mark saved (clean)
        st.mark_clean();
        assert!(st.is_clean());
        assert!(!st.is_dirty());

        // Mutation 2 -> dirty
        st.checkpoint("step 2", &cur);
        cur = 2;
        assert!(st.is_dirty());

        // Undo -> back to step 1 which was marked clean!
        let prev = st.undo(cur).unwrap();
        assert_eq!(prev, 1);
        assert!(
            st.is_clean(),
            "Undoing back to the saved state must be clean"
        );

        // Redo -> forward to step 2 which is dirty!
        let next = st.redo(prev).unwrap();
        assert_eq!(next, 2);
        assert!(st.is_dirty(), "Redoing to an unsaved state must be dirty");

        // Force dirty
        st.mark_clean();
        assert!(st.is_clean());
        st.mark_dirty();
        assert!(st.is_dirty());
    }

    #[test]
    fn command_trait_executes_and_identifies() {
        let cmd = IncrementCmd(5);
        assert_eq!(cmd.label(), "increment");
        assert!(cmd.is_destructive());
        let mut val = 10;
        assert!(cmd.execute(&mut val).is_ok());
        assert_eq!(val, 15);
    }
}
