//! # Foundation — fundação visual da UI Petunia
//!
//! Tokens e métricas **sem dependência de runtime**: aqui não há cálculo de
//! layout, nem `egui_taffy`, nem conhecimento de painel. É a única camada que os
//! componentes e adapters podem consultar para spacing, raio, tipografia,
//! densidade e motion.
//!
//! Regra de dependência (§22.1 da diretiva Egui Ecosystem Final Push):
//!
//! ```text
//! panels/screens
//!       ↓
//! Petunia Components
//!       ↓
//! Petunia Adapters / Foundation     ← este módulo e `crate::adapters`
//!       ↓
//! egui / helper crates
//! ```
//!
//! Proibido: um painel calcular layout com `available_width()`, definir spacing
//! próprio com `spacing_mut()` ou escolher cor literal. Ver §36 e
//! `cargo xtask ui-guard`.
//!
//! ## Mapa
//!
//! | Módulo | Responsabilidade |
//! | --- | --- |
//! | [`theme`] | leitura de tokens de tema do estado ativo |
//! | [`density`] | métricas por preset de densidade (linha, campo, gap) |
//! | [`spacing`] | gaps estruturais do shell e entre controles |
//! | [`radius`] | raios canônicos de canto |
//! | [`typography`] | papéis tipográficos e escalas de fonte |
//! | [`motion`] | durações e curvas de animação |

pub mod density;
pub mod motion;
pub mod radius;
pub mod spacing;
pub mod theme;
pub mod typography;

pub use density::PetuniaDensity;
pub use radius::{RADIUS_CONTAINER, RADIUS_CONTROL, RADIUS_PILL, RADIUS_SMALL};
pub use spacing::PetuniaSpacing;
pub use typography::TextRole;
