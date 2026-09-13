//! Serviço unificado de consulta, filtro, ordenação e metadados da Biblioteca de Modelos (P3D-003).
//!
//! Tanto o `Asset Browser` (gaveta compacta na viewport) quanto o `Project Model Library`
//! (janela utilitária completa) consomem as mesmas regras sem duplicação de dados.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Asset, Project};

/// Critérios de ordenação suportados pela Biblioteca de Modelos (P3D-003 §Sort).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModelLibrarySort {
    #[default]
    NameAsc,
    NameDesc,
    TrianglesAsc,
    TrianglesDesc,
    VerticesAsc,
    VerticesDesc,
    IndexAsc,
    IndexDesc,
}

/// Parâmetros de busca e filtragem da Biblioteca de Modelos (P3D-003 §Search, §Filtros).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelLibraryQuery {
    pub search: String,
    pub tag: Option<String>,
    pub collection: Option<String>,
    pub only_favorites: bool,
    pub sort: ModelLibrarySort,
}

/// Resumo leve de metadados de um Asset para exibição em grades e tabelas.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetSummary {
    pub id: Uuid,
    pub original_index: usize,
    pub name: String,
    pub triangles: usize,
    pub vertices: usize,
    pub favorite: bool,
    pub tags: Vec<String>,
    pub collection: Option<String>,
    pub visible: bool,
    pub locked: bool,
    pub base_color: [f32; 3],
    pub has_texture: bool,
}

impl AssetSummary {
    pub fn from_asset(asset: &Asset, index: usize) -> Self {
        Self {
            id: asset.id,
            original_index: index,
            name: asset.name.clone(),
            triangles: asset.mesh.tri_count(),
            vertices: asset.mesh.vert_count(),
            favorite: asset.favorite,
            tags: asset.tags.clone(),
            collection: asset.collection.clone(),
            visible: asset.visible,
            locked: asset.locked,
            base_color: asset.base_color,
            has_texture: asset.texture.is_some(),
        }
    }
}

/// Serviço puro de consulta e agregação sobre os modelos do projeto.
pub struct ModelLibraryService;

impl ModelLibraryService {
    /// Filtra e ordena os assets do projeto de acordo com a consulta fornecida.
    pub fn query(project: &Project, query: &ModelLibraryQuery) -> Vec<AssetSummary> {
        let trimmed_search = query.search.trim().to_lowercase();
        let tag_filter = query.tag.as_deref().map(|t| t.trim().to_lowercase());
        let col_filter = query.collection.as_deref().map(|c| c.trim().to_lowercase());

        let mut results: Vec<AssetSummary> = project
            .assets
            .iter()
            .enumerate()
            .filter_map(|(idx, asset)| {
                // 1. Filtro de Favoritos
                if query.only_favorites && !asset.favorite {
                    return None;
                }

                // 2. Filtro de Coleção
                if let Some(ref col) = col_filter {
                    let asset_col = asset.collection.as_deref().unwrap_or("").to_lowercase();
                    if &asset_col != col {
                        return None;
                    }
                }

                // 3. Filtro de Tag
                if let Some(ref t) = tag_filter {
                    if !asset.tags.iter().any(|at| at.to_lowercase() == *t) {
                        return None;
                    }
                }

                // 4. Busca textual (no nome e nas tags)
                if !trimmed_search.is_empty() {
                    let matches_name = asset.name.to_lowercase().contains(&trimmed_search);
                    let matches_tags = asset
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&trimmed_search));
                    if !matches_name && !matches_tags {
                        return None;
                    }
                }

                Some(AssetSummary::from_asset(asset, idx))
            })
            .collect();

        // 5. Ordenação
        match query.sort {
            ModelLibrarySort::NameAsc => {
                results.sort_by_key(|a| a.name.to_lowercase());
            }
            ModelLibrarySort::NameDesc => {
                results.sort_by_key(|a| std::cmp::Reverse(a.name.to_lowercase()));
            }
            ModelLibrarySort::TrianglesAsc => {
                results.sort_by_key(|a| a.triangles);
            }
            ModelLibrarySort::TrianglesDesc => {
                results.sort_by_key(|a| std::cmp::Reverse(a.triangles));
            }
            ModelLibrarySort::VerticesAsc => {
                results.sort_by_key(|a| a.vertices);
            }
            ModelLibrarySort::VerticesDesc => {
                results.sort_by_key(|a| std::cmp::Reverse(a.vertices));
            }
            ModelLibrarySort::IndexAsc => {
                results.sort_by_key(|a| a.original_index);
            }
            ModelLibrarySort::IndexDesc => {
                results.sort_by_key(|a| std::cmp::Reverse(a.original_index));
            }
        }

        results
    }

    /// Retorna todas as tags distintas presentes nos assets do projeto com suas contagens.
    pub fn all_tags_with_counts(project: &Project) -> Vec<(String, usize)> {
        let mut counts = std::collections::BTreeMap::new();
        for asset in &project.assets {
            for tag in &asset.tags {
                let normalized = tag.trim().to_lowercase();
                if !normalized.is_empty() {
                    *counts.entry(normalized).or_insert(0) += 1;
                }
            }
        }
        counts.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_mesh::Mesh;

    #[test]
    fn test_model_library_query_search_and_filter() {
        let mut p = Project::new();
        p.assets.clear();

        let mut a1 = Asset::new("Dragon Boss", Mesh::cube(2.0));
        a1.favorite = true;
        a1.add_tag("character");
        a1.add_tag("boss");
        a1.collection = Some("Enemies".into());

        let mut a2 = Asset::new("Pine Tree", Mesh::cube(1.0));
        a2.favorite = false;
        a2.add_tag("environment");
        a2.add_tag("prop");
        a2.collection = Some("Props".into());

        let mut a3 = Asset::new("Goblin Archer", Mesh::plane(1.0));
        a3.favorite = true;
        a3.add_tag("character");
        a3.add_tag("minion");
        a3.collection = Some("Enemies".into());

        p.assets.push(a1);
        p.assets.push(a2);
        p.assets.push(a3);

        // 1. Busca por nome "goblin"
        let q = ModelLibraryQuery {
            search: "goblin".to_string(),
            ..Default::default()
        };
        let res = ModelLibraryService::query(&p, &q);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "Goblin Archer");

        // 2. Filtro por tag "character"
        let q = ModelLibraryQuery {
            tag: Some("character".into()),
            ..Default::default()
        };
        let res = ModelLibraryService::query(&p, &q);
        assert_eq!(res.len(), 2);

        // 3. Filtro por favorites
        let q = ModelLibraryQuery {
            only_favorites: true,
            ..Default::default()
        };
        let res = ModelLibraryService::query(&p, &q);
        assert_eq!(res.len(), 2);

        // 4. Filtro por coleção "Enemies" + favoritos + ordenação A-Z
        let q = ModelLibraryQuery {
            collection: Some("Enemies".into()),
            only_favorites: true,
            sort: ModelLibrarySort::NameAsc,
            ..Default::default()
        };
        let res = ModelLibraryService::query(&p, &q);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].name, "Dragon Boss");
        assert_eq!(res[1].name, "Goblin Archer");

        // 5. Contagem de tags
        let tags = ModelLibraryService::all_tags_with_counts(&p);
        assert!(tags.iter().any(|(t, c)| t == "character" && *c == 2));
        assert!(tags.iter().any(|(t, c)| t == "boss" && *c == 1));
    }
}
