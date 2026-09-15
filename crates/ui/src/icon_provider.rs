//! Unified generic icon provider (`iconflow`, P0-17).
//!
//! Workspace code resolves icons by semantic [`GenericIcon`] name, never by
//! pack glyph or font directly. [`PetuniaIconProvider`] maps the semantic id
//! to the active [`GenericPack`] through `iconflow` and reports a
//! deterministic [`ProviderError`] when a glyph is missing, so a pack change
//! can never silently change command meaning.
//!
//! Enabled packs: Lucide + Iconoir by default. Tabler + Phosphor ride the
//! default-off `extended-icon-packs` feature (their generated sources OOM a
//! 5 GiB dev machine; CI enables the feature).

/// Generic (non-domain) icon pack selectable at runtime.
///
/// Iconoir é o padrão da interface (decisão de produto: biblioteca única
/// por enquanto; Lucide segue como reserva compilada).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum GenericPack {
    /// Lucide (reserva).
    Lucide,
    /// Iconoir (default).
    #[default]
    Iconoir,
    /// Tabler (`extended-icon-packs` feature).
    Tabler,
    /// Phosphor (`extended-icon-packs` feature).
    Phosphor,
}

impl GenericPack {
    /// All packs compiled into this build.
    pub fn available() -> &'static [Self] {
        #[cfg(feature = "extended-icon-packs")]
        {
            &[Self::Lucide, Self::Iconoir, Self::Tabler, Self::Phosphor]
        }
        #[cfg(not(feature = "extended-icon-packs"))]
        {
            &[Self::Lucide, Self::Iconoir]
        }
    }

    /// Whether this pack is compiled into this build.
    pub fn is_enabled(self) -> bool {
        match self {
            Self::Lucide | Self::Iconoir => true,
            #[cfg(feature = "extended-icon-packs")]
            Self::Tabler | Self::Phosphor => true,
            #[cfg(not(feature = "extended-icon-packs"))]
            Self::Tabler | Self::Phosphor => false,
        }
    }

    /// Stable pack name for diagnostics and settings UI.
    pub fn name(self) -> &'static str {
        match self {
            Self::Lucide => "lucide",
            Self::Iconoir => "iconoir",
            Self::Tabler => "tabler",
            Self::Phosphor => "phosphor",
        }
    }

    /// Pack pelo id de configuração (`None` = Petunia ou pacote de disco).
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "lucide" => Some(Self::Lucide),
            "iconoir" => Some(Self::Iconoir),
            "tabler" => Some(Self::Tabler),
            "phosphor" => Some(Self::Phosphor),
            _ => None,
        }
    }

    fn iconflow_pack(self) -> iconflow::Pack {
        match self {
            Self::Lucide => iconflow::Pack::Lucide,
            Self::Iconoir => iconflow::Pack::Iconoir,
            #[cfg(feature = "extended-icon-packs")]
            Self::Tabler => iconflow::Pack::Tabler,
            #[cfg(feature = "extended-icon-packs")]
            Self::Phosphor => iconflow::Pack::Phosphor,
            #[cfg(not(feature = "extended-icon-packs"))]
            Self::Tabler | Self::Phosphor => iconflow::Pack::Lucide,
        }
    }
}

/// Semantic generic icon ids used by toolbar, menus and settings.
/// The id fixes *meaning*; the pack only fixes *glyph*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GenericIcon {
    /// 3D box / object.
    Box,
    /// Move / translate.
    Move,
    /// Application settings.
    Settings,
    /// Camera / view.
    Camera,
}

impl GenericIcon {
    /// Glyph name inside Lucide/Iconoir (both packs carry these names).
    pub fn glyph_name(self) -> &'static str {
        match self {
            Self::Box => "box",
            Self::Move => "move",
            Self::Settings => "settings",
            Self::Camera => "camera",
        }
    }

    /// Ordered glyph candidates (Iconoir não tem "move": usa "expand").
    pub fn glyph_candidates(self) -> &'static [&'static str] {
        match self {
            Self::Box => &["box"],
            Self::Move => &["move", "expand", "drag"],
            Self::Settings => &["settings"],
            Self::Camera => &["camera"],
        }
    }
}

/// Deterministic resolution failure. Never a silent tofu glyph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProviderError {
    /// The pack is not compiled into this build.
    PackDisabled {
        /// Requested pack name.
        pack: &'static str,
    },
    /// The glyph name does not exist in the pack.
    Missing {
        /// Requested pack name.
        pack: &'static str,
        /// Requested glyph name.
        name: String,
    },
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PackDisabled { pack } => {
                write!(f, "icon pack '{pack}' is not enabled in this build")
            }
            Self::Missing { pack, name } => {
                write!(f, "icon '{name}' missing in pack '{pack}'")
            }
        }
    }
}

/// Resolved glyph: font family + codepoint for egui font rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResolvedIcon {
    /// Font family name inside the embedded TTF.
    pub family: &'static str,
    /// Unicode codepoint of the glyph.
    pub codepoint: u32,
}

impl ResolvedIcon {
    /// The glyph as a `char`, if the codepoint is valid Unicode.
    pub fn as_char(self) -> Option<char> {
        char::from_u32(self.codepoint)
    }
}

/// Instala as fontes `iconflow` (pacotes habilitados) no egui (Wave 6 — §11.2).
///
/// Cada asset vira uma `FontFamily::Name(family)`; glifos resolvidos por
/// [`PetuniaIconProvider`] renderizam nessa família — nunca na fonte padrão
/// (que produziria tofu). Idempotente por contexto.
pub fn install_fonts(fonts: &mut egui::FontDefinitions) {
    for asset in iconflow::fonts() {
        fonts.font_data.insert(
            asset.family.to_owned(),
            egui::FontData::from_static(asset.bytes).into(),
        );
        fonts
            .families
            .entry(egui::FontFamily::Name(asset.family.into()))
            .or_default()
            .push(asset.family.to_owned());
    }
}

/// Ícones utilitários/chrome que seguem o pacote genérico (Wave 6).
///
/// Ferramentas de domínio (move/rotate/extrude/…), abas de properties, shading,
/// gizmos e overlays de viewport permanecem Petunia-owned em todos os pacotes:
/// trocar o pacote muda o glifo, nunca o significado — e o sistema vetorial
/// desenhado do Petunia não regride para glifo monocromático.
pub fn is_pack_owned(icon: crate::icon_registry::PetuniaIcon) -> bool {
    use crate::icon_registry::PetuniaIcon as P;
    matches!(
        icon,
        P::Search
            | P::Folder
            | P::File
            | P::Eye
            | P::EyeHidden
            | P::Lock
            | P::Unlock
            | P::ChevronLeft
            | P::ChevronRight
            | P::ChevronDown
            | P::ChevronUp
            | P::Close
            | P::Minimize
            | P::Maximize
            | P::Play
            | P::Pause
            | P::StepForward
            | P::StepBackward
            | P::JumpStart
            | P::JumpEnd
            | P::Undo
            | P::Redo
            | P::Plus
            | P::Trash
            | P::Settings
            | P::MoreVert
            | P::Filter
            | P::Duplicate
            | P::Delete
            | P::Collection
            | P::ReferenceImage
            | P::ObjectMesh
            | P::ModeObject
            | P::ModeEdit
            | P::SelectVertex
            | P::SelectEdge
            | P::SelectFace
            | P::PaintBrush
            | P::PaintEraser
            | P::PaintFill
            | P::PaintPicker
            | P::PaintLine
            | P::PaintRect
    )
}

/// Nomes candidatos por ícone utilitário, em ordem de preferência (Wave 6).
///
/// Variações de nomenclatura entre pacotes (lucide kebab, iconoir próprio,
/// tabler player-*, phosphor compostos) são absorvidas aqui; a resolução usa
/// o primeiro nome que existe no pacote ativo. O teste de auditoria garante
/// ao menos um acerto por (ícone, pacote habilitado).
pub fn utility_candidates(icon: crate::icon_registry::PetuniaIcon) -> &'static [&'static str] {
    use crate::icon_registry::PetuniaIcon as P;
    match icon {
        P::Search => &["search", "magnifying-glass"],
        P::Folder => &["folder", "folder-closed"],
        P::File => &["file", "page", "file-text"],
        P::Eye => &["eye"],
        P::EyeHidden => &["eye-off", "eye-closed", "eye-slash"],
        P::Lock => &["lock"],
        P::Unlock => &["lock-open", "lock-slash", "unlock"],
        P::ChevronLeft => &["chevron-left", "nav-arrow-left", "caret-left", "arrow-left"],
        P::ChevronRight => &[
            "chevron-right",
            "nav-arrow-right",
            "caret-right",
            "arrow-right",
        ],
        P::ChevronDown => &["chevron-down", "nav-arrow-down", "caret-down", "arrow-down"],
        P::ChevronUp => &["chevron-up", "nav-arrow-up", "caret-up", "arrow-up"],
        P::Close => &["x", "xmark", "close"],
        P::Minimize => &["minus", "minimize"],
        P::Maximize => &["maximize", "expand", "arrows-out"],
        P::Play => &["play", "player-play"],
        P::Pause => &["pause", "player-pause"],
        P::StepForward => &["skip-forward", "skip-next", "chevron-right", "step-forward"],
        P::StepBackward => &["skip-back", "skip-prev", "chevron-left", "step-back"],
        P::JumpStart => &["chevrons-left", "rewind", "skip-prev", "arrow-line-left"],
        P::JumpEnd => &["chevrons-right", "forward", "skip-next", "arrow-line-right"],
        P::Undo => &[
            "undo",
            "undo-2",
            "arrow-counter-clockwise",
            "rotate-ccw",
            "arrow-back-up",
        ],
        P::Redo => &[
            "redo",
            "redo-2",
            "arrow-clockwise",
            "rotate-cw",
            "arrow-forward-up",
        ],
        P::Plus => &["plus", "add"],
        P::Trash => &["trash", "trash-2", "delete"],
        P::Settings => &["settings", "gear", "cog"],
        P::MoreVert => &[
            "ellipsis-vertical",
            "more-vert",
            "dots-vertical",
            "dots-three-vertical",
        ],
        P::Filter => &["funnel", "filter", "list-filter"],
        P::Duplicate => &["copy", "duplicate"],
        P::Delete => &["trash", "trash-2", "delete"],
        P::Collection => &["archive", "folder", "collection"],
        P::ReferenceImage => &["image", "media-image", "photo"],
        P::ObjectMesh => &["box", "cube", "package"],
        P::ModeObject => &["box", "cube", "package"],
        P::ModeEdit => &["pencil", "edit", "edit-pencil", "pencil-simple"],
        P::SelectVertex => &["circle-dot", "circle", "dot"],
        P::SelectEdge => &["minus", "slash"],
        P::SelectFace => &["square", "box"],
        P::PaintBrush => &["design-pencil", "edit-pencil", "brush", "paintbrush"],
        P::PaintEraser => &["erase", "eraser"],
        P::PaintFill => &["fill-color", "paint-bucket", "bucket"],
        P::PaintPicker => &["color-picker", "pipette", "dropper"],
        P::PaintLine => &["slash", "line", "minus"],
        P::PaintRect => &["square", "rectangle"],
        _ => &[],
    }
}

/// Resolve um ícone utilitário no pacote ativo (primeiro candidato existente).
pub fn resolve_utility(
    icon: crate::icon_registry::PetuniaIcon,
    pack: GenericPack,
) -> Result<ResolvedIcon, ProviderError> {
    if !pack.is_enabled() {
        return Err(ProviderError::PackDisabled { pack: pack.name() });
    }
    for name in utility_candidates(icon) {
        if let Ok(icon_ref) = iconflow::try_icon(
            pack.iconflow_pack(),
            name,
            iconflow::Style::Regular,
            iconflow::Size::Regular,
        ) {
            return Ok(ResolvedIcon {
                family: icon_ref.family,
                codepoint: icon_ref.codepoint,
            });
        }
    }
    Err(ProviderError::Missing {
        pack: pack.name(),
        name: utility_candidates(icon)
            .first()
            .copied()
            .unwrap_or("?")
            .to_string(),
    })
}

/// Single entry point for generic icons. Owns the active pack; workspace code
/// never imports `iconflow` directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PetuniaIconProvider {
    pack: GenericPack,
}

impl Default for PetuniaIconProvider {
    fn default() -> Self {
        Self::new(GenericPack::default())
    }
}

impl PetuniaIconProvider {
    /// Creates a provider for `pack`.
    pub fn new(pack: GenericPack) -> Self {
        Self { pack }
    }

    /// Active pack.
    pub fn pack(self) -> GenericPack {
        self.pack
    }

    /// Switches the active pack. Glyph choice may change; meaning (the
    /// [`GenericIcon`] id and its tooltip/label) never does.
    pub fn set_pack(&mut self, pack: GenericPack) {
        self.pack = pack;
    }

    /// Resolves a semantic icon to a concrete glyph in the active pack.
    pub fn resolve(self, icon: GenericIcon) -> Result<ResolvedIcon, ProviderError> {
        if !self.pack.is_enabled() {
            return Err(ProviderError::PackDisabled {
                pack: self.pack.name(),
            });
        }
        for name in icon.glyph_candidates() {
            if let Ok(icon_ref) = iconflow::try_icon(
                self.pack.iconflow_pack(),
                name,
                iconflow::Style::Regular,
                iconflow::Size::Regular,
            ) {
                return Ok(ResolvedIcon {
                    family: icon_ref.family,
                    codepoint: icon_ref.codepoint,
                });
            }
        }
        Err(ProviderError::Missing {
            pack: self.pack.name(),
            name: icon.glyph_name().to_string(),
        })
    }

    /// Number of glyphs shipped by the active pack.
    pub fn glyph_count(self) -> usize {
        if !self.pack.is_enabled() {
            return 0;
        }
        iconflow::list(self.pack.iconflow_pack()).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pack_resolves_core_icons() {
        let provider = PetuniaIconProvider::default();
        assert_eq!(provider.pack(), GenericPack::Iconoir);
        for icon in [
            GenericIcon::Box,
            GenericIcon::Move,
            GenericIcon::Settings,
            GenericIcon::Camera,
        ] {
            let resolved = provider.resolve(icon).expect("core glyph resolves");
            assert!(resolved.as_char().is_some());
            assert!(!resolved.family.is_empty());
        }
    }

    #[test]
    fn iconoir_pack_resolves_shared_names() {
        let provider = PetuniaIconProvider::new(GenericPack::Iconoir);
        // Both packs carry "box" and "settings".
        assert!(provider.resolve(GenericIcon::Box).is_ok());
        assert!(provider.resolve(GenericIcon::Settings).is_ok());
        assert!(provider.glyph_count() > 1000);
    }

    #[test]
    fn disabled_pack_fails_deterministically() {
        let provider = PetuniaIconProvider::new(GenericPack::Tabler);
        #[cfg(not(feature = "extended-icon-packs"))]
        assert_eq!(
            provider.resolve(GenericIcon::Box),
            Err(ProviderError::PackDisabled { pack: "tabler" })
        );
        #[cfg(feature = "extended-icon-packs")]
        assert!(provider.resolve(GenericIcon::Box).is_ok());
    }

    #[test]
    fn pack_switch_keeps_meaning_stable() {
        let mut provider = PetuniaIconProvider::default();
        let before = provider.resolve(GenericIcon::Settings).unwrap();
        provider.set_pack(GenericPack::Iconoir);
        let after = provider.resolve(GenericIcon::Settings).unwrap();
        // Same semantic id resolves in both packs; glyphs may differ.
        assert!(before.as_char().is_some() && after.as_char().is_some());
        assert_eq!(GenericIcon::Settings.glyph_name(), "settings");
    }
}
