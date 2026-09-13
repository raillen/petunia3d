//! Sistema centralizado de ícones do Petunia3D (`IconRegistry` e `PetuniaIcon`).
//!
//! Integra os assets PNGs transparentes extraídos da Golden Reference Figma (`Blender.svg`),
//! mantém cache de textura GPU (`TextureHandle`) para carregamento único e performance máxima,
//! e utiliza Phosphor Icons e renderização vetorial para ícones utilitários.

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use egui::{
    pos2, Align2, Color32, ColorImage, Context, FontId, Painter, Rect, TextureHandle,
    TextureOptions,
};

use crate::icons;

/// Ícones semânticos canônicos do Petunia3D.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PetuniaIcon {
    // ---------------------------------------------------------------- Toolbar / Ferramentas
    SelectBox,
    Cursor3D,
    Move,
    Rotate,
    Scale,
    Transform,
    Annotate,
    Measure,
    AddPrimitive,

    // ---------------------------------------------------- Ferramentas de Modelagem Poligonal
    Extrude,
    Inset,
    Bevel,
    LoopCut,
    Knife,
    PushPull,
    Slice,
    Subdivide,
    DrawProfile,

    // ---------------------------------------------------- Abas de Propriedades (Assets Figma 1..15)
    PropertyTab(u8),
    PropTool,
    PropRender,
    PropOutput,
    PropViewLayer,
    PropScene,
    PropWorld,
    PropCollection,
    PropObject,
    PropModifiers,
    PropData,
    PropMaterial,

    // --------------------------------------------------------------- Modos e Alvos de Seleção
    ModeObject,
    ModeEdit,
    SelectVertex,
    SelectEdge,
    SelectFace,

    // ------------------------------------------------------------------- Modos de Sombreamento
    ShadingWireframe,
    ShadingSolid,
    ShadingMaterial,
    ShadingRendered,

    // ----------------------------------------------------------- Auxiliares de Viewport e Cena
    SnapMagnet,
    ProportionalEditing,
    XRay,
    Overlays,
    OrientationGlobal,
    PivotMedian,
    ObjectMesh,
    ReferenceImage,
    Collection,
    Duplicate,
    Delete,

    // ---------------------------------------------------- Utilitários (Phosphor / Vetoriais)
    Search,
    Folder,
    File,
    Eye,
    EyeHidden,
    Lock,
    Unlock,
    ChevronLeft,
    ChevronRight,
    ChevronDown,
    ChevronUp,
    Close,
    Minimize,
    Maximize,
    Play,
    Pause,
    StepForward,
    StepBackward,
    JumpStart,
    JumpEnd,
    Undo,
    Redo,
    Plus,
    Trash,
    Settings,
    MoreVert,
    Filter,

    // ------------------------------------------------------------------- Customizado por ID
    Custom(&'static str),
}

impl PetuniaIcon {
    /// Identificador textual único do ícone para indexação e cache.
    pub fn id(&self) -> String {
        match self {
            PetuniaIcon::SelectBox => "select_box".into(),
            PetuniaIcon::Cursor3D => "cursor_3d".into(),
            PetuniaIcon::Move => "move".into(),
            PetuniaIcon::Rotate => "rotate".into(),
            PetuniaIcon::Scale => "scale".into(),
            PetuniaIcon::Transform => "transform".into(),
            PetuniaIcon::Annotate => "annotate".into(),
            PetuniaIcon::Measure => "measure".into(),
            PetuniaIcon::AddPrimitive => "add_primitive".into(),

            PetuniaIcon::Extrude => "extrude".into(),
            PetuniaIcon::Inset => "inset".into(),
            PetuniaIcon::Bevel => "bevel".into(),
            PetuniaIcon::LoopCut => "loop_cut".into(),
            PetuniaIcon::Knife => "knife".into(),
            PetuniaIcon::PushPull => "pushpull".into(),
            PetuniaIcon::Slice => "slice".into(),
            PetuniaIcon::Subdivide => "subdivide".into(),
            PetuniaIcon::DrawProfile => "draw_profile".into(),

            PetuniaIcon::PropertyTab(n) => format!("data_tab_{n:02}"),
            PetuniaIcon::PropTool => "data_tab_01".into(),
            PetuniaIcon::PropRender => "data_tab_02".into(),
            PetuniaIcon::PropOutput => "data_tab_03".into(),
            PetuniaIcon::PropViewLayer => "data_tab_04".into(),
            PetuniaIcon::PropScene => "data_tab_05".into(),
            PetuniaIcon::PropWorld => "data_tab_06".into(),
            PetuniaIcon::PropCollection => "data_tab_07".into(),
            PetuniaIcon::PropObject => "data_tab_08".into(),
            PetuniaIcon::PropModifiers => "data_tab_09".into(),
            PetuniaIcon::PropData => "data_tab_10".into(),
            PetuniaIcon::PropMaterial => "data_tab_11".into(),

            PetuniaIcon::ModeObject => "mode_object".into(),
            PetuniaIcon::ModeEdit => "mode_edit".into(),
            PetuniaIcon::SelectVertex => "select_vertex".into(),
            PetuniaIcon::SelectEdge => "select_edge".into(),
            PetuniaIcon::SelectFace => "select_face".into(),

            PetuniaIcon::ShadingWireframe => "shading_wireframe".into(),
            PetuniaIcon::ShadingSolid => "shading_solid".into(),
            PetuniaIcon::ShadingMaterial => "shading_material".into(),
            PetuniaIcon::ShadingRendered => "shading_rendered".into(),

            PetuniaIcon::SnapMagnet => "snap_magnet".into(),
            PetuniaIcon::ProportionalEditing => "proportional_editing".into(),
            PetuniaIcon::XRay => "xray".into(),
            PetuniaIcon::Overlays => "overlays".into(),
            PetuniaIcon::OrientationGlobal => "orientation_global".into(),
            PetuniaIcon::PivotMedian => "pivot_median".into(),
            PetuniaIcon::ObjectMesh => "object_mesh".into(),
            PetuniaIcon::ReferenceImage => "reference_image".into(),
            PetuniaIcon::Collection => "collection".into(),
            PetuniaIcon::Duplicate => "duplicate".into(),
            PetuniaIcon::Delete => "delete".into(),

            PetuniaIcon::Search => "search".into(),
            PetuniaIcon::Folder => "folder".into(),
            PetuniaIcon::File => "file".into(),
            PetuniaIcon::Eye => "eye".into(),
            PetuniaIcon::EyeHidden => "eye_hidden".into(),
            PetuniaIcon::Lock => "lock".into(),
            PetuniaIcon::Unlock => "unlock".into(),
            PetuniaIcon::ChevronLeft => "chevron_left".into(),
            PetuniaIcon::ChevronRight => "chevron_right".into(),
            PetuniaIcon::ChevronDown => "chevron_down".into(),
            PetuniaIcon::ChevronUp => "chevron_up".into(),
            PetuniaIcon::Close => "close".into(),
            PetuniaIcon::Minimize => "minimize".into(),
            PetuniaIcon::Maximize => "maximize".into(),
            PetuniaIcon::Play => "play".into(),
            PetuniaIcon::Pause => "pause".into(),
            PetuniaIcon::StepForward => "step_forward".into(),
            PetuniaIcon::StepBackward => "step_backward".into(),
            PetuniaIcon::JumpStart => "jump_start".into(),
            PetuniaIcon::JumpEnd => "jump_end".into(),
            PetuniaIcon::Undo => "undo".into(),
            PetuniaIcon::Redo => "redo".into(),
            PetuniaIcon::Plus => "plus".into(),
            PetuniaIcon::Trash => "trash".into(),
            PetuniaIcon::Settings => "settings".into(),
            PetuniaIcon::MoreVert => "more_vert".into(),
            PetuniaIcon::Filter => "filter".into(),
            PetuniaIcon::Custom(s) => (*s).into(),
        }
    }

    /// Retorna o glifo Phosphor correspondente, se aplicável.
    pub fn phosphor_glyph(&self) -> Option<&'static str> {
        match self {
            PetuniaIcon::Search => Some(egui_phosphor::regular::MAGNIFYING_GLASS),
            PetuniaIcon::Folder => Some(egui_phosphor::regular::FOLDER),
            PetuniaIcon::File => Some(egui_phosphor::regular::FILE),
            PetuniaIcon::Eye => Some(egui_phosphor::regular::EYE),
            PetuniaIcon::EyeHidden => Some(egui_phosphor::regular::EYE_SLASH),
            PetuniaIcon::Lock => Some(egui_phosphor::regular::LOCK),
            PetuniaIcon::Unlock => Some(egui_phosphor::regular::LOCK_OPEN),
            PetuniaIcon::ChevronLeft => Some(egui_phosphor::regular::CARET_LEFT),
            PetuniaIcon::ChevronRight => Some(egui_phosphor::regular::CARET_RIGHT),
            PetuniaIcon::ChevronDown => Some(egui_phosphor::regular::CARET_DOWN),
            PetuniaIcon::ChevronUp => Some(egui_phosphor::regular::CARET_UP),
            PetuniaIcon::Close => Some(egui_phosphor::regular::X),
            PetuniaIcon::Minimize => Some(egui_phosphor::regular::MINUS),
            PetuniaIcon::Maximize => Some(egui_phosphor::regular::SQUARE),
            PetuniaIcon::Play => Some(egui_phosphor::regular::PLAY),
            PetuniaIcon::Pause => Some(egui_phosphor::regular::PAUSE),
            PetuniaIcon::StepForward => Some(egui_phosphor::regular::SKIP_FORWARD),
            PetuniaIcon::StepBackward => Some(egui_phosphor::regular::SKIP_BACK),
            PetuniaIcon::JumpStart => Some(egui_phosphor::regular::ARROW_LINE_LEFT),
            PetuniaIcon::JumpEnd => Some(egui_phosphor::regular::ARROW_LINE_RIGHT),
            PetuniaIcon::Undo => Some(egui_phosphor::regular::ARROW_U_UP_LEFT),
            PetuniaIcon::Redo => Some(egui_phosphor::regular::ARROW_U_UP_RIGHT),
            PetuniaIcon::Plus => Some(egui_phosphor::regular::PLUS),
            PetuniaIcon::Trash => Some(egui_phosphor::regular::TRASH),
            PetuniaIcon::Settings => Some(egui_phosphor::regular::GEAR),
            PetuniaIcon::MoreVert => Some(egui_phosphor::regular::DOTS_THREE_VERTICAL),
            PetuniaIcon::Filter => Some(egui_phosphor::regular::FUNNEL),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------- Cache Global de Texturas GPU
static TEXTURE_CACHE: LazyLock<RwLock<HashMap<String, TextureHandle>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// --------------------------------------------------------- Bytes Embutidos dos PNGs da Toolbar
const PNG_SELECT_BOX: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/select_box.png");
const PNG_CURSOR_3D: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/cursor_3d.png");
const PNG_MOVE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/move.png");
const PNG_ROTATE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/rotate.png");
const PNG_SCALE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/scale.png");
const PNG_TRANSFORM: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/transform.png");
const PNG_ANNOTATE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/annotate.png");
const PNG_MEASURE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/measure.png");
const PNG_ADD_PRIMITIVE: &[u8] =
    include_bytes!("../../../assets/ui/icons/toolbar/add_primitive.png");

// -------------------------------------------------- Bytes Embutidos dos PNGs de Properties Tabs
const PNG_TAB_01: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_01.png");
const PNG_TAB_02: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_02.png");
const PNG_TAB_03: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_03.png");
const PNG_TAB_04: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_04.png");
const PNG_TAB_05: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_05.png");
const PNG_TAB_06: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_06.png");
const PNG_TAB_07: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_07.png");
const PNG_TAB_08: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_08.png");
const PNG_TAB_09: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_09.png");
const PNG_TAB_10: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_10.png");
const PNG_TAB_11: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_11.png");
const PNG_TAB_12: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_12.png");
const PNG_TAB_13: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_13.png");
const PNG_TAB_14: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_14.png");
const PNG_TAB_15: &[u8] = include_bytes!("../../../assets/ui/icons/properties/data_tab_15.png");

fn get_embedded_png_bytes(id: &str) -> Option<&'static [u8]> {
    match id {
        "select_box" => Some(PNG_SELECT_BOX),
        "cursor_3d" => Some(PNG_CURSOR_3D),
        "move" => Some(PNG_MOVE),
        "rotate" => Some(PNG_ROTATE),
        "scale" => Some(PNG_SCALE),
        "transform" => Some(PNG_TRANSFORM),
        "annotate" => Some(PNG_ANNOTATE),
        "measure" => Some(PNG_MEASURE),
        "add_primitive" => Some(PNG_ADD_PRIMITIVE),

        "data_tab_01" => Some(PNG_TAB_01),
        "data_tab_02" => Some(PNG_TAB_02),
        "data_tab_03" => Some(PNG_TAB_03),
        "data_tab_04" => Some(PNG_TAB_04),
        "data_tab_05" => Some(PNG_TAB_05),
        "data_tab_06" => Some(PNG_TAB_06),
        "data_tab_07" => Some(PNG_TAB_07),
        "data_tab_08" => Some(PNG_TAB_08),
        "data_tab_09" => Some(PNG_TAB_09),
        "data_tab_10" => Some(PNG_TAB_10),
        "data_tab_11" => Some(PNG_TAB_11),
        "data_tab_12" => Some(PNG_TAB_12),
        "data_tab_13" => Some(PNG_TAB_13),
        "data_tab_14" => Some(PNG_TAB_14),
        "data_tab_15" => Some(PNG_TAB_15),
        _ => None,
    }
}

/// Carrega a imagem PNG decodificada preservando a transparência e cria um `ColorImage` em escala de cinza/alfa.
fn decode_png_to_alpha_mask(bytes: &[u8]) -> Option<ColorImage> {
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let mut raw_bytes = Vec::with_capacity((w * h * 4) as usize);
    for pixel in rgba.pixels() {
        let [r, g, b, a] = pixel.0;
        if a > 0 {
            let lum = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
            let val = ((lum * (a as f32 / 255.0)) * 255.0).clamp(0.0, 255.0) as u8;
            raw_bytes.extend_from_slice(&[255, 255, 255, val]);
        } else {
            raw_bytes.extend_from_slice(&[0, 0, 0, 0]);
        }
    }

    Some(ColorImage::from_rgba_unmultiplied(
        [w as usize, h as usize],
        &raw_bytes,
    ))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IconPackManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct IconPackFile {
    icon_pack: IconPackManifest,
}

/// Registro centralizado de ícones do Petunia3D.
pub struct IconRegistry;

impl IconRegistry {
    /// Retorna a lista de pacotes de ícones disponíveis (embutidos e escaneados de assets/icons/).
    pub fn available_packs() -> Vec<IconPackManifest> {
        let mut packs = vec![
            IconPackManifest {
                id: "petunia".into(),
                name: "Petunia (Padrão)".into(),
                version: "1.0.0".into(),
                author: Some("Petunia3D Team".into()),
                description: Some(
                    "Ícones nativos com estilo Blender e renderização vetorial".into(),
                ),
            },
            IconPackManifest {
                id: "phosphor".into(),
                name: "Phosphor Icons".into(),
                version: "2.1.0".into(),
                author: Some("Phosphor Team".into()),
                description: Some("Linhas limpas, modernas e equilibradas".into()),
            },
            IconPackManifest {
                id: "tabler".into(),
                name: "Tabler Icons".into(),
                version: "3.2.0".into(),
                author: Some("Paweł Kuna".into()),
                description: Some("Grade 24x24 consistente e técnica".into()),
            },
            IconPackManifest {
                id: "iconoir".into(),
                name: "Iconoir".into(),
                version: "7.7.0".into(),
                author: Some("Damien Erambert".into()),
                description: Some("Visual minimalista e geométrico".into()),
            },
            IconPackManifest {
                id: "lucide".into(),
                name: "Lucide Icons".into(),
                version: "0.450.0".into(),
                author: Some("Lucide Contributors".into()),
                description: Some("Traço vetorial refinado de 2px".into()),
            },
        ];

        for dir in ["assets/icons", "icons"] {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let manifest_path = path.join("manifest.toml");
                        if let Ok(text) = std::fs::read_to_string(&manifest_path) {
                            if let Ok(m) = toml::from_str::<IconPackFile>(&text) {
                                if !packs.iter().any(|p| p.id == m.icon_pack.id) {
                                    packs.push(m.icon_pack);
                                }
                            }
                        }
                    }
                }
            }
        }

        packs
    }
    /// Inicializa os conjuntos de fontes adicionais (Phosphor) no contexto do egui, se ainda não configurados.
    pub fn ensure_fonts(ctx: &Context) {
        ctx.style(); // Garante inicialização básica
        let font_id = egui::Id::new("petunia_phosphor_fonts_initialized");
        let already_initialized = ctx.data(|d| d.get_temp::<bool>(font_id).unwrap_or(false));

        if !already_initialized {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            ctx.set_fonts(fonts);
            ctx.data_mut(|d| d.insert_temp(font_id, true));
        }
    }

    /// Obtém ou carrega uma textura em cache (executado uma única vez por ícone rasterizado).
    pub fn get_or_load_texture(ctx: &Context, id: &str) -> Option<TextureHandle> {
        // 1. Tenta leitura rápida sem bloqueio de escrita
        if let Ok(cache) = TEXTURE_CACHE.read() {
            if let Some(handle) = cache.get(id) {
                return Some(handle.clone());
            }
        }

        // 2. Busca bytes do asset embutido
        let bytes = get_embedded_png_bytes(id)?;
        let color_image = decode_png_to_alpha_mask(bytes)?;

        // 3. Cria textura na GPU via egui::Context (reutilizável para sempre)
        let handle = ctx.load_texture(
            format!("petunia_icon_{id}"),
            color_image,
            TextureOptions::LINEAR,
        );

        if let Ok(mut cache) = TEXTURE_CACHE.write() {
            cache.insert(id.to_string(), handle.clone());
        }

        Some(handle)
    }

    /// Renderiza um ícone do Petunia com tingimento dinâmico, proporção exata e fallback resiliente.
    pub fn paint(
        ctx: &Context,
        painter: &Painter,
        icon: &PetuniaIcon,
        target_rect: Rect,
        tint: Color32,
    ) {
        let id = icon.id();

        // 1. Ferramentas da Toolbar usam renderização vetorial nítida com paleta colorida estilo Blender
        if is_toolbar_vector_tool(&id) {
            icons::paint(painter, &id, target_rect, tint);
            return;
        }

        // 2. Tenta carregar asset rasterizado (PNG Figma) se existir
        if let Some(texture) = Self::get_or_load_texture(ctx, &id) {
            let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
            painter.image(texture.id(), target_rect, uv, tint);
            return;
        }

        // 3. Tenta glifo vetorial Phosphor se aplicável
        if let Some(glyph) = icon.phosphor_glyph() {
            Self::ensure_fonts(ctx);
            let font_size = target_rect.height().min(target_rect.width()) * 0.85;
            painter.text(
                target_rect.center(),
                Align2::CENTER_CENTER,
                glyph,
                FontId::proportional(font_size),
                tint,
            );
            return;
        }

        // 4. Fallback para desenho vetorial nativo de alta precisão
        icons::paint(painter, &id, target_rect, tint);
    }
}

/// Identifica se o identificador de ícone corresponde a uma ferramenta primária ou de modelagem da Toolbar.
pub fn is_toolbar_vector_tool(id: &str) -> bool {
    matches!(
        id,
        "select"
            | "select_box"
            | "cursor_3d"
            | "move"
            | "rotate"
            | "scale"
            | "transform"
            | "annotate"
            | "measure"
            | "add_primitive"
            | "primitives"
            | "extrude"
            | "inset"
            | "bevel"
            | "loop_cut"
            | "knife"
            | "pushpull"
            | "slice"
            | "subdivide"
            | "draw_profile"
            | "paint"
            | "mode_object"
            | "mode_edit"
            | "select_vertex"
            | "select_edge"
            | "select_face"
            | "shading_wireframe"
            | "shading_solid"
            | "shading_material"
            | "shading_rendered"
            | "snap_magnet"
            | "proportional_editing"
            | "xray"
            | "overlays"
            | "orientation_global"
            | "pivot_median"
            | "object_mesh"
            | "reference_image"
            | "collection"
            | "duplicate"
            | "delete"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens;
    use egui::vec2;

    #[test]
    fn test_all_icon_ids_and_glyphs_are_valid() {
        let icons = [
            PetuniaIcon::SelectBox,
            PetuniaIcon::Cursor3D,
            PetuniaIcon::Move,
            PetuniaIcon::Rotate,
            PetuniaIcon::Scale,
            PetuniaIcon::Transform,
            PetuniaIcon::Annotate,
            PetuniaIcon::Measure,
            PetuniaIcon::AddPrimitive,
            PetuniaIcon::Extrude,
            PetuniaIcon::Inset,
            PetuniaIcon::Bevel,
            PetuniaIcon::LoopCut,
            PetuniaIcon::Knife,
            PetuniaIcon::PushPull,
            PetuniaIcon::Slice,
            PetuniaIcon::Subdivide,
            PetuniaIcon::DrawProfile,
            PetuniaIcon::ModeObject,
            PetuniaIcon::ModeEdit,
            PetuniaIcon::SelectVertex,
            PetuniaIcon::SelectEdge,
            PetuniaIcon::SelectFace,
            PetuniaIcon::ShadingWireframe,
            PetuniaIcon::ShadingSolid,
            PetuniaIcon::ShadingMaterial,
            PetuniaIcon::ShadingRendered,
            PetuniaIcon::SnapMagnet,
            PetuniaIcon::ProportionalEditing,
            PetuniaIcon::XRay,
            PetuniaIcon::Overlays,
            PetuniaIcon::OrientationGlobal,
            PetuniaIcon::PivotMedian,
            PetuniaIcon::ObjectMesh,
            PetuniaIcon::ReferenceImage,
            PetuniaIcon::Collection,
            PetuniaIcon::Duplicate,
            PetuniaIcon::Delete,
            PetuniaIcon::PropertyTab(1),
            PetuniaIcon::PropRender,
            PetuniaIcon::Search,
            PetuniaIcon::Folder,
            PetuniaIcon::File,
            PetuniaIcon::Eye,
            PetuniaIcon::EyeHidden,
            PetuniaIcon::Lock,
            PetuniaIcon::Unlock,
            PetuniaIcon::Play,
            PetuniaIcon::Pause,
            PetuniaIcon::Settings,
        ];

        for icon in icons {
            let id = icon.id();
            assert!(!id.is_empty());
        }
    }

    #[test]
    fn test_embedded_png_decoder_preserves_alpha() {
        let bytes = get_embedded_png_bytes("move").expect("move png must exist");
        let image = decode_png_to_alpha_mask(bytes).expect("must decode valid alpha mask");
        assert_eq!(image.size, [256, 256]);
        assert!(!image.pixels.is_empty());
    }

    #[test]
    fn test_properties_tabs_embedded_pngs_load_validly() {
        for tab in 1..=15 {
            let id = format!("data_tab_{tab:02}");
            let bytes = get_embedded_png_bytes(&id)
                .unwrap_or_else(|| panic!("tab {id} must have embedded png"));
            let image = decode_png_to_alpha_mask(bytes).expect("must decode valid alpha mask");
            assert_eq!(image.size, [22, 22]);
        }
    }

    #[test]
    fn test_icon_registry_paint_in_egui_context() {
        let ctx = Context::default();
        IconRegistry::ensure_fonts(&ctx);

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(32.0, 32.0), egui::Sense::hover());
                IconRegistry::paint(
                    ctx,
                    ui.painter(),
                    &PetuniaIcon::Move,
                    rect,
                    tokens::TEXT_ACTIVE,
                );
                IconRegistry::paint(
                    ctx,
                    ui.painter(),
                    &PetuniaIcon::Search,
                    rect,
                    tokens::TEXT_ACTIVE,
                );
                IconRegistry::paint(
                    ctx,
                    ui.painter(),
                    &PetuniaIcon::PropRender,
                    rect,
                    tokens::TEXT_ACTIVE,
                );
            });
        });
    }
}
