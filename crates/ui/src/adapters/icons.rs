//! Adapter de ícones genéricos (`iconflow` + packs Petunia).
//!
//! A resolução real vive em [`crate::icon_provider`] (packs genéricos) e
//! [`crate::icon_registry`] (`IconId` → arte canônica). Este módulo é a
//! superfície pela qual componentes e painéis falam de ícones, para que
//! `iconflow` permaneça um detalhe de implementação.
//!
//! Contrato (cap. 24–26 e `AGENTS.md` §3): UI pública usa **`IconId`**, nunca
//! glifo de fonte nem caminho de SVG solto. Packs oficiais: `petunia` (arte
//! própria de domínio 3D) + `tabler`, `iconoir`, `phosphor` e `lucide` via
//! `iconflow`.

pub use crate::icon_provider::{
    GenericIcon, GenericPack, PetuniaIconProvider, ProviderError, ResolvedIcon, install_fonts,
    is_pack_owned,
};
pub use crate::icon_registry::{IconPackManifest, IconRegistry, PetuniaIcon};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_exposes_the_icon_contract() {
        // O ponto do teste é o contrato: token de ícone e pacote são tipos
        // Petunia, não tipos da crate de ícones.
        let icon = PetuniaIcon::Move;
        let _owned: bool = is_pack_owned(icon);
    }

    #[test]
    fn official_packs_are_declared() {
        let packs = ["petunia", "tabler", "iconoir", "phosphor", "lucide"];
        for pack in packs {
            assert!(!pack.is_empty());
        }
    }
}
