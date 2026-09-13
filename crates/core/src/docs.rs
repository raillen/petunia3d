//! Tópicos canônicos de documentação e resolução de URLs de ajuda contextual (P3D-115).
//! Desacopla links externos da interface gráfica e centraliza a árvore de ajuda.

use serde::{Deserialize, Serialize};

/// Tópicos de documentação contextual do Petunia3D.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocsTopic {
    GettingStarted,
    Interface,
    Navigation,
    Modeling,
    Extrude,
    Bevel,
    LoopCut,
    Knife,
    UvUnwrapping,
    TexturePainting,
    Materials,
    Keymaps,
    Themes,
    ImportExport,
    Assets,
}

impl DocsTopic {
    /// Rótulo legível para menus e tooltips.
    pub fn title(&self) -> &'static str {
        match self {
            Self::GettingStarted => "Getting Started",
            Self::Interface => "Interface Overview",
            Self::Navigation => "3D Viewport Navigation",
            Self::Modeling => "Mesh Modeling Tools",
            Self::Extrude => "Extrude Faces",
            Self::Bevel => "Beveling Edges",
            Self::LoopCut => "Loop Cut & Topology",
            Self::Knife => "Knife Tool",
            Self::UvUnwrapping => "UV Unwrapping",
            Self::TexturePainting => "Texture Painting",
            Self::Materials => "Materials & Shading",
            Self::Keymaps => "Keyboard Shortcuts & Profiles",
            Self::Themes => "Custom Themes & Tokens",
            Self::ImportExport => "Importing & Exporting Files",
            Self::Assets => "Model Library & Asset Management",
        }
    }

    /// Caminho relativo canônico da página no site de documentação oficial.
    pub fn doc_path(&self) -> &'static str {
        match self {
            Self::GettingStarted => "manual/getting-started.html",
            Self::Interface => "manual/interface.html",
            Self::Navigation => "manual/navigation.html",
            Self::Modeling => "manual/modeling.html",
            Self::Extrude => "manual/modeling.html#extrude",
            Self::Bevel => "manual/modeling.html#bevel",
            Self::LoopCut => "manual/modeling.html#loop-cut",
            Self::Knife => "manual/modeling.html#knife",
            Self::UvUnwrapping => "manual/uv.html",
            Self::TexturePainting => "manual/paint.html",
            Self::Materials => "manual/materials.html",
            Self::Keymaps => "manual/shortcuts.html",
            Self::Themes => "manual/themes.html",
            Self::ImportExport => "manual/export.html",
            Self::Assets => "manual/assets.html",
        }
    }

    /// URL absoluta canônica oficial online.
    pub fn canonical_url(&self) -> String {
        format!("https://petunia3d.org/docs/{}", self.doc_path())
    }

    /// Lista todos os tópicos canônicos.
    pub fn all() -> &'static [DocsTopic] {
        &[
            Self::GettingStarted,
            Self::Interface,
            Self::Navigation,
            Self::Modeling,
            Self::Extrude,
            Self::Bevel,
            Self::LoopCut,
            Self::Knife,
            Self::UvUnwrapping,
            Self::TexturePainting,
            Self::Materials,
            Self::Keymaps,
            Self::Themes,
            Self::ImportExport,
            Self::Assets,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_docs_topics_have_valid_urls_and_titles() {
        for topic in DocsTopic::all() {
            let title = topic.title();
            let path = topic.doc_path();
            let url = topic.canonical_url();

            assert!(!title.trim().is_empty(), "Topic title cannot be empty");
            assert!(
                path.ends_with(".html") || path.contains(".html#"),
                "Topic path must end with .html or anchor: {path}"
            );
            assert!(
                url.starts_with("https://petunia3d.org/docs/"),
                "Canonical URL must be secure: {url}"
            );
        }
    }
}
