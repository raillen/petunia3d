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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum GenericPack {
    /// Lucide (default).
    #[default]
    Lucide,
    /// Iconoir.
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
        let name = icon.glyph_name();
        iconflow::try_icon(
            self.pack.iconflow_pack(),
            name,
            iconflow::Style::Regular,
            iconflow::Size::Regular,
        )
        .map(|icon_ref| ResolvedIcon {
            family: icon_ref.family,
            codepoint: icon_ref.codepoint,
        })
        .map_err(|_| ProviderError::Missing {
            pack: self.pack.name(),
            name: name.to_string(),
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
        assert_eq!(provider.pack(), GenericPack::Lucide);
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
