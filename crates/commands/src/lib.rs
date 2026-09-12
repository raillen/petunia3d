//! Petunia3D — Undo/Redo via Command Pattern (snapshots).
//!
//! Regra do spec: operações destrutivas são comandos desfazíveis.
//! Snapshots clonados são suficientes para low-poly (malhas pequenas) e
//! mantêm a implementação simples e correta. Cap de 100 níveis.

/// Pilha genérica de undo/redo sobre estado clonável.
#[derive(Debug, Default)]
pub struct UndoStack<T: Clone> {
    undo: Vec<(String, T)>,
    redo: Vec<(String, T)>,
    cap: usize,
}

impl<T: Clone> UndoStack<T> {
    pub fn new() -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
            cap: 100,
        }
    }

    /// Salva o estado ATUAL antes de uma mutação (chamar antes de mudar).
    pub fn checkpoint(&mut self, label: impl Into<String>, current: &T) {
        self.undo.push((label.into(), current.clone()));
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

    /// Desfaz: guarda estado atual no redo, retorna estado anterior.
    pub fn undo(&mut self, current: T) -> Option<T> {
        let (label, prev) = self.undo.pop()?;
        self.redo.push((label, current));
        Some(prev)
    }

    /// Refaz: guarda estado atual no undo, retorna próximo estado.
    pub fn redo(&mut self, current: T) -> Option<T> {
        let (label, next) = self.redo.pop()?;
        self.undo.push((label, current));
        Some(next)
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
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
}
