//! Gerenciador persistente de Projetos Recentes (P3D-001 §Recent Projects).
//!
//! Histórico é preferência da aplicação (não dado do documento).
//! Projetos ausentes no disco não quebram o menu e são filtrados de forma graciosa.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Registro individual de um projeto recentemente acessado.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentProjectEntry {
    pub path: PathBuf,
    pub name: String,
    pub last_opened: u64,
}

/// Coleção ordenada de projetos recentes com limite configurável.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentProjects {
    pub entries: Vec<RecentProjectEntry>,
    pub max_entries: usize,
}

impl Default for RecentProjects {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 10,
        }
    }
}

impl RecentProjects {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries: max_entries.clamp(1, 50),
        }
    }

    /// Adiciona ou move para o topo um projeto recém-aberto ou salvo.
    pub fn add(&mut self, path: &Path, name: &str, timestamp: u64) {
        let norm_path = path.to_path_buf();
        self.entries.retain(|e| e.path != norm_path);

        self.entries.insert(
            0,
            RecentProjectEntry {
                path: norm_path,
                name: name.to_string(),
                last_opened: timestamp,
            },
        );

        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }
    }

    /// Remove um caminho do histórico.
    pub fn remove(&mut self, path: &Path) {
        self.entries.retain(|e| e.path != path);
    }

    /// Remove entradas cujos arquivos não existem mais fisicamente no disco.
    pub fn prune_missing(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|e| e.path.exists());
        before - self.entries.len()
    }

    /// Retorna as entradas válidas atuais.
    pub fn list(&self) -> &[RecentProjectEntry] {
        &self.entries
    }

    /// Limpa todo o histórico.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Carrega histórico a partir de um arquivo JSON.
    pub fn load_from_path(path: &Path) -> Self {
        if !path.exists() {
            return Self::default();
        }
        fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    /// Salva histórico de forma segura no caminho especificado em JSON.
    pub fn save_to_path(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        fs::write(path, json)
    }

    /// Retorna as entradas como fatia (alias ergonômico para list).
    pub fn projects(&self) -> &[RecentProjectEntry] {
        &self.entries
    }

    /// Caminho padrão para persistência de preferências de projetos recentes.
    pub fn default_path() -> PathBuf {
        if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME") {
            PathBuf::from(config_home)
                .join("petunia3d")
                .join("recent_projects.json")
        } else if let Some(home) = std::env::var_os("HOME") {
            PathBuf::from(home)
                .join(".config")
                .join("petunia3d")
                .join("recent_projects.json")
        } else {
            PathBuf::from(".recent_projects.json")
        }
    }

    /// Carrega histórico a partir da localização canônica de configuração.
    pub fn load() -> Self {
        Self::load_from_path(&Self::default_path())
    }

    /// Salva histórico na localização canônica de configuração.
    pub fn save(&self) -> Result<(), std::io::Error> {
        self.save_to_path(&Self::default_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_projects_ordering_and_cap() {
        let mut recents = RecentProjects::new(3);

        recents.add(Path::new("/tmp/proj1.petunia"), "Proj 1", 100);
        recents.add(Path::new("/tmp/proj2.petunia"), "Proj 2", 200);
        recents.add(Path::new("/tmp/proj3.petunia"), "Proj 3", 300);

        assert_eq!(recents.list().len(), 3);
        assert_eq!(recents.list()[0].name, "Proj 3");

        // Adiciona um 4º -> remove o mais antigo (Proj 1)
        recents.add(Path::new("/tmp/proj4.petunia"), "Proj 4", 400);
        assert_eq!(recents.list().len(), 3);
        assert_eq!(recents.list()[0].name, "Proj 4");
        assert!(!recents.list().iter().any(|e| e.name == "Proj 1"));

        // Reabre Proj 2 -> vai para o topo sem duplicar
        recents.add(Path::new("/tmp/proj2.petunia"), "Proj 2", 500);
        assert_eq!(recents.list().len(), 3);
        assert_eq!(recents.list()[0].name, "Proj 2");
    }

    #[test]
    fn test_recent_projects_prune_missing() {
        let dir = std::env::temp_dir().join(format!("petunia_recents_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let file1 = dir.join("existing.petunia");
        fs::write(&file1, b"test").unwrap();
        let file2 = dir.join("non_existing.petunia");

        let mut recents = RecentProjects::new(5);
        recents.add(&file1, "Existing", 100);
        recents.add(&file2, "Missing", 200);
        assert_eq!(recents.list().len(), 2);

        let pruned = recents.prune_missing();
        assert_eq!(pruned, 1);
        assert_eq!(recents.list().len(), 1);
        assert_eq!(recents.list()[0].name, "Existing");

        let _ = fs::remove_dir_all(&dir);
    }
}
