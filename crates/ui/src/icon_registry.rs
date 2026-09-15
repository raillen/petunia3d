//! Sistema centralizado de ícones do Petunia3D (`IconRegistry` e `PetuniaIcon`).
//!
//! Integra os assets PNGs transparentes extraídos da Golden Reference Figma (`Blender.svg`),
//! mantém cache de textura GPU (`TextureHandle`) para carregamento único e performance máxima,
//! e utiliza Phosphor Icons e renderização vetorial para ícones utilitários.

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use egui::{
    Align2, Color32, ColorImage, Context, FontId, Painter, Rect, TextureHandle, TextureOptions,
    pos2,
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

    // ------------------------------------------------- Ferramentas de Pintura (Paint)
    PaintBrush,
    PaintEraser,
    PaintFill,
    PaintPicker,
    PaintLine,
    PaintRect,

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
    Pin,

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
            PetuniaIcon::PaintBrush => "paint_brush".into(),
            PetuniaIcon::PaintEraser => "paint_eraser".into(),
            PetuniaIcon::PaintFill => "paint_fill".into(),
            PetuniaIcon::PaintPicker => "paint_picker".into(),
            PetuniaIcon::PaintLine => "paint_line".into(),
            PetuniaIcon::PaintRect => "paint_rect".into(),

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
            PetuniaIcon::Pin => "pin".into(),
            PetuniaIcon::Custom(s) => (*s).into(),
        }
    }

    /// Todos os ícones nomeados + customs usados no código (Wave 6: auditoria).
    ///
    /// Fonte única para testes de cobertura, smoke de render e prévia de pacotes.
    /// `PropertyTab` entra com amostra paramétrica; customs cobrem os ids reais
    /// em toolbar/shelf/viewport (`merge`, `paint`, `delete`, `flip_diagonal`,
    /// `revolve`).
    pub fn all() -> Vec<PetuniaIcon> {
        vec![
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
            PetuniaIcon::PaintBrush,
            PetuniaIcon::PaintEraser,
            PetuniaIcon::PaintFill,
            PetuniaIcon::PaintPicker,
            PetuniaIcon::PaintLine,
            PetuniaIcon::PaintRect,
            PetuniaIcon::PropertyTab(1),
            PetuniaIcon::PropTool,
            PetuniaIcon::PropRender,
            PetuniaIcon::PropOutput,
            PetuniaIcon::PropViewLayer,
            PetuniaIcon::PropScene,
            PetuniaIcon::PropWorld,
            PetuniaIcon::PropCollection,
            PetuniaIcon::PropObject,
            PetuniaIcon::PropModifiers,
            PetuniaIcon::PropData,
            PetuniaIcon::PropMaterial,
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
            PetuniaIcon::Search,
            PetuniaIcon::Folder,
            PetuniaIcon::File,
            PetuniaIcon::Eye,
            PetuniaIcon::EyeHidden,
            PetuniaIcon::Lock,
            PetuniaIcon::Unlock,
            PetuniaIcon::ChevronLeft,
            PetuniaIcon::ChevronRight,
            PetuniaIcon::ChevronDown,
            PetuniaIcon::ChevronUp,
            PetuniaIcon::Close,
            PetuniaIcon::Minimize,
            PetuniaIcon::Maximize,
            PetuniaIcon::Play,
            PetuniaIcon::Pause,
            PetuniaIcon::StepForward,
            PetuniaIcon::StepBackward,
            PetuniaIcon::JumpStart,
            PetuniaIcon::JumpEnd,
            PetuniaIcon::Undo,
            PetuniaIcon::Redo,
            PetuniaIcon::Plus,
            PetuniaIcon::Trash,
            PetuniaIcon::Settings,
            PetuniaIcon::MoreVert,
            PetuniaIcon::Filter,
            PetuniaIcon::Pin,
            PetuniaIcon::Custom("merge"),
            PetuniaIcon::Custom("paint"),
            PetuniaIcon::Custom("delete"),
            PetuniaIcon::Custom("flip_diagonal"),
            PetuniaIcon::Custom("revolve"),
        ]
    }

    /// Retorna o glifo Phosphor correspondente, se aplicável.
    pub fn phosphor_glyph(&self) -> Option<&'static str> {
        match self {
            PetuniaIcon::SelectBox => Some(egui_phosphor::regular::SELECTION_PLUS),
            PetuniaIcon::Cursor3D => Some(egui_phosphor::regular::CROSSHAIR),
            PetuniaIcon::Move => Some(egui_phosphor::regular::ARROWS_OUT_CARDINAL),
            PetuniaIcon::Rotate => Some(egui_phosphor::regular::ARROWS_CLOCKWISE),
            PetuniaIcon::Scale => Some(egui_phosphor::regular::ARROWS_OUT),
            PetuniaIcon::Transform => Some(egui_phosphor::regular::BOUNDING_BOX),
            PetuniaIcon::Annotate => Some(egui_phosphor::regular::PENCIL_SIMPLE),
            PetuniaIcon::Measure => Some(egui_phosphor::regular::RULER),
            PetuniaIcon::AddPrimitive => Some(egui_phosphor::regular::CUBE),

            PetuniaIcon::Extrude => Some(egui_phosphor::regular::ARROW_UP),
            PetuniaIcon::Inset => Some(egui_phosphor::regular::SQUARES_FOUR),
            PetuniaIcon::Bevel => Some(egui_phosphor::regular::BEZIER_CURVE),
            PetuniaIcon::LoopCut => Some(egui_phosphor::regular::SPLIT_HORIZONTAL),
            PetuniaIcon::Knife => Some(egui_phosphor::regular::SCISSORS),
            PetuniaIcon::PushPull => Some(egui_phosphor::regular::ARROWS_LEFT_RIGHT),
            PetuniaIcon::Slice => Some(egui_phosphor::regular::KNIFE),
            PetuniaIcon::Subdivide => Some(egui_phosphor::regular::GRID_FOUR),
            PetuniaIcon::DrawProfile => Some(egui_phosphor::regular::PEN_NIB),
            PetuniaIcon::PaintBrush => Some(egui_phosphor::regular::PAINT_BRUSH),
            PetuniaIcon::PaintEraser => Some(egui_phosphor::regular::ERASER),
            PetuniaIcon::PaintFill => Some(egui_phosphor::regular::PAINT_BUCKET),
            PetuniaIcon::PaintPicker => Some(egui_phosphor::regular::EYEDROPPER),
            PetuniaIcon::PaintLine => Some(egui_phosphor::regular::LINE_SEGMENT),
            PetuniaIcon::PaintRect => Some(egui_phosphor::regular::SQUARE),

            PetuniaIcon::ModeObject => Some(egui_phosphor::regular::CUBE),
            PetuniaIcon::ModeEdit => Some(egui_phosphor::regular::PENCIL_SIMPLE),
            PetuniaIcon::SelectVertex => Some(egui_phosphor::regular::DOT),
            PetuniaIcon::SelectEdge => Some(egui_phosphor::regular::LINE_SEGMENT),
            PetuniaIcon::SelectFace => Some(egui_phosphor::regular::SQUARE),

            PetuniaIcon::ShadingWireframe => Some(egui_phosphor::regular::CIRCLE),
            PetuniaIcon::ShadingSolid => Some(egui_phosphor::regular::CIRCLE),
            PetuniaIcon::ShadingMaterial => Some(egui_phosphor::regular::CIRCLE_HALF),
            PetuniaIcon::ShadingRendered => Some(egui_phosphor::regular::SUN),

            PetuniaIcon::SnapMagnet => Some(egui_phosphor::regular::MAGNET),
            PetuniaIcon::ProportionalEditing => Some(egui_phosphor::regular::TARGET),
            PetuniaIcon::XRay => Some(egui_phosphor::regular::SCAN),
            PetuniaIcon::Overlays => Some(egui_phosphor::regular::STACK),
            PetuniaIcon::OrientationGlobal => Some(egui_phosphor::regular::COMPASS),
            PetuniaIcon::PivotMedian => Some(egui_phosphor::regular::DOTS_NINE),
            PetuniaIcon::ObjectMesh => Some(egui_phosphor::regular::CUBE),
            PetuniaIcon::ReferenceImage => Some(egui_phosphor::regular::IMAGE),
            PetuniaIcon::Collection => Some(egui_phosphor::regular::FOLDER_NOTCH_OPEN),
            PetuniaIcon::Duplicate => Some(egui_phosphor::regular::COPY),
            PetuniaIcon::Delete => Some(egui_phosphor::regular::TRASH),

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
            PetuniaIcon::Pin => Some(egui_phosphor::regular::PUSH_PIN),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------- Cache Global de Texturas GPU
static TEXTURE_CACHE: LazyLock<RwLock<HashMap<String, TextureHandle>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// -------------------------------------------------- Golden Reference SVG toolbar assets
const SVG_TOOL_SELECT_BOX: &str = include_str!("../../../assets/ui/icons/toolbar/select_box.svg");
const SVG_TOOL_CURSOR_3D: &str = include_str!("../../../assets/ui/icons/toolbar/cursor_3d.svg");
const SVG_TOOL_MOVE: &str = include_str!("../../../assets/ui/icons/toolbar/move.svg");
const SVG_TOOL_ROTATE: &str = include_str!("../../../assets/ui/icons/toolbar/rotate.svg");
const SVG_TOOL_SCALE: &str = include_str!("../../../assets/ui/icons/toolbar/scale.svg");
const SVG_TOOL_TRANSFORM: &str = include_str!("../../../assets/ui/icons/toolbar/transform.svg");
const SVG_TOOL_ANNOTATE: &str = include_str!("../../../assets/ui/icons/toolbar/annotate.svg");
const SVG_TOOL_MEASURE: &str = include_str!("../../../assets/ui/icons/toolbar/measure.svg");
const SVG_TOOL_ADD_PRIMITIVE: &str =
    include_str!("../../../assets/ui/icons/toolbar/add_primitive.svg");

fn embedded_toolbar_svg(id: &str) -> Option<&'static str> {
    match id {
        "select_box" => Some(SVG_TOOL_SELECT_BOX),
        "cursor_3d" => Some(SVG_TOOL_CURSOR_3D),
        "move" => Some(SVG_TOOL_MOVE),
        "rotate" => Some(SVG_TOOL_ROTATE),
        "scale" => Some(SVG_TOOL_SCALE),
        "transform" => Some(SVG_TOOL_TRANSFORM),
        "annotate" => Some(SVG_TOOL_ANNOTATE),
        "measure" => Some(SVG_TOOL_MEASURE),
        "add_primitive" => Some(SVG_TOOL_ADD_PRIMITIVE),
        _ => None,
    }
}

fn rasterize_svg(svg: &str, size: u32) -> Option<ColorImage> {
    let tree = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()).ok()?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size)?;
    let source = tree.size();
    let scale_x = size as f32 / source.width();
    let scale_y = size as f32 / source.height();
    let scale = scale_x.min(scale_y);
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Some(ColorImage::from_rgba_unmultiplied(
        [size as usize, size as usize],
        pixmap.data(),
    ))
}

fn get_or_load_toolbar_svg(ctx: &Context, id: &str) -> Option<TextureHandle> {
    let cache_key = format!("svg-toolbar:{id}");
    if let Ok(cache) = TEXTURE_CACHE.read()
        && let Some(handle) = cache.get(&cache_key)
    {
        return Some(handle.clone());
    }
    let image = rasterize_svg(embedded_toolbar_svg(id)?, 64)?;
    let handle = ctx.load_texture(cache_key.clone(), image, TextureOptions::LINEAR);
    if let Ok(mut cache) = TEXTURE_CACHE.write() {
        cache.insert(cache_key, handle.clone());
    }
    Some(handle)
}

// -------------------------------------------------- Bytes Embutidos dos PNGs de Properties Tabs
// NOTA (Wave 9): os PNGs da toolbar foram removidos — inalcançáveis, pois toda
// ferramenta tem arte vetorial (`is_toolbar_vector_tool`). Restam os PNGs das
// abas de properties (arte raster sem fonte SVG).
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

static ICON_PACK_CACHE: LazyLock<RwLock<Vec<IconPackManifest>>> =
    LazyLock::new(|| RwLock::new(discover_icon_packs()));

fn discover_icon_packs() -> Vec<IconPackManifest> {
    let mut packs = vec![
        IconPackManifest {
            id: "petunia".into(),
            name: "Petunia (Arte própria)".into(),
            version: "1.0.0".into(),
            author: Some("Petunia3D Team".into()),
            description: Some("Ícones nativos com estilo Blender e renderização vetorial".into()),
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
            name: "Iconoir (Padrão)".into(),
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
                    if let Ok(text) = std::fs::read_to_string(&manifest_path)
                        && let Ok(m) = toml::from_str::<IconPackFile>(&text)
                        && !packs.iter().any(|p| p.id == m.icon_pack.id)
                    {
                        packs.push(m.icon_pack);
                    }
                }
            }
        }
    }
    packs
}

/// Registro centralizado de ícones do Petunia3D.
pub struct IconRegistry;

impl IconRegistry {
    /// Retorna a lista de pacotes em cache. O filesystem é consultado somente
    /// na primeira chamada ou em [`Self::refresh_available_packs`].
    pub fn available_packs() -> Vec<IconPackManifest> {
        match ICON_PACK_CACHE.read() {
            Ok(packs) => packs.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }

    /// Reescaneia manifests de pacotes fora do hot path da UI.
    pub fn refresh_available_packs() {
        let packs = discover_icon_packs();
        match ICON_PACK_CACHE.write() {
            Ok(mut cached) => *cached = packs,
            Err(poisoned) => *poisoned.into_inner() = packs,
        }
    }

    /// Inicializa os conjuntos de fontes de ícones no contexto do egui, se ainda
    /// não configurados (Wave 6): Phosphor (fallback) + fontes `iconflow` dos
    /// pacotes habilitados. Glifos de pacote renderizam na família nomeada —
    /// nunca na fonte padrão (que produziria tofu).
    ///
    /// O egui aplica `set_fonts` no fim do pass: [`Self::icon_fonts_ready`]
    /// indica quando a pintura nomeada é segura (sem pânico em ctx fresco).
    pub fn ensure_fonts(ctx: &Context) {
        let _ = ctx.style_of(ctx.theme()); // Garante inicialização básica
        let font_id = egui::Id::new("petunia_icon_fonts_pass");
        let installed = ctx.data(|d| d.get_temp::<u64>(font_id));
        if installed.is_none() {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            crate::icon_provider::install_fonts(&mut fonts);
            ctx.set_fonts(fonts);
            // Fora do closure de dados: `cumulative_pass_nr` trava outro lock.
            let pass = ctx.cumulative_pass_nr();
            ctx.data_mut(|d| d.insert_temp(font_id, pass));
        }
    }

    /// Fontes de ícones prontas neste contexto (fronteira de pass cruzada).
    fn icon_fonts_ready(ctx: &Context) -> bool {
        let font_id = egui::Id::new("petunia_icon_fonts_pass");
        let current = ctx.cumulative_pass_nr();
        ctx.data(|d| {
            d.get_temp::<u64>(font_id)
                .is_some_and(|pass| pass < current)
        })
    }

    /// Pacote genérico correspondente ao id ativo (`None` = Petunia/desconhecido).
    fn generic_pack_for_id(id: &str) -> Option<crate::icon_provider::GenericPack> {
        use crate::icon_provider::GenericPack as P;
        match id {
            "lucide" => Some(P::Lucide),
            "iconoir" => Some(P::Iconoir),
            "tabler" => Some(P::Tabler),
            "phosphor" => Some(P::Phosphor),
            _ => None,
        }
    }

    /// Renderiza um glifo `iconflow` resolvido na sua família nomeada.
    fn paint_resolved(
        painter: &Painter,
        resolved: crate::icon_provider::ResolvedIcon,
        target_rect: Rect,
        tint: Color32,
    ) {
        let Some(ch) = resolved.as_char() else {
            return;
        };
        let size = target_rect.height().min(target_rect.width()).max(1.0);
        let mut text = String::with_capacity(4);
        text.push(ch);
        painter.text(
            target_rect.center(),
            Align2::CENTER_CENTER,
            text,
            egui::FontId::new(size, egui::FontFamily::Name(resolved.family.into())),
            tint,
        );
    }

    /// Obtém ou carrega uma textura em cache (executado uma única vez por ícone rasterizado).
    pub fn get_or_load_texture(ctx: &Context, id: &str) -> Option<TextureHandle> {
        // 1. Tenta leitura rápida sem bloqueio de escrita
        if let Ok(cache) = TEXTURE_CACHE.read()
            && let Some(handle) = cache.get(id)
        {
            return Some(handle.clone());
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

    /// Renderiza um ícone do Petunia usando o pacote ativo no contexto.
    pub fn paint(
        ctx: &Context,
        painter: &Painter,
        icon: &PetuniaIcon,
        target_rect: Rect,
        tint: Color32,
    ) {
        let active_pack = ctx.data(|d| {
            d.get_temp::<String>(egui::Id::new("petunia_active_icon_pack"))
                .unwrap_or_else(|| "petunia".to_string())
        });
        Self::paint_pack(ctx, painter, icon, target_rect, tint, &active_pack);
    }

    /// Renderiza um ícone do Petunia com tingimento dinâmico e suporte explícito ao pacote selecionado.
    ///
    /// Cadeia canônica (Wave 6 — §11.1), sem renderização fictícia:
    /// ```text
    /// PetuniaIcon
    ///   ├── pacote genérico + ícone utilitário → glifo real do pacote (iconflow)
    ///   ├── ferramenta/domínio Petunia → arte vetorial/PNG própria (todos os pacotes)
    ///   ├── fallback → glifo Phosphor (fonte sempre instalada)
    ///   └── último recurso → losango vetorial (nunca tofu silencioso)
    /// ```
    pub fn paint_pack(
        ctx: &Context,
        painter: &Painter,
        icon: &PetuniaIcon,
        target_rect: Rect,
        tint: Color32,
        pack: &str,
    ) {
        use crate::icon_provider::{GenericPack, is_pack_owned, resolve_utility};
        let id = icon.id();

        // 1. Pacote genérico + ícone utilitário/chrome: glifo REAL do pacote.
        // Exige fontes prontas (pós-fronteira de pass); senão cai para o
        // caminho de domínio neste frame e se autocorrige no próximo.
        if let Some(active) = Self::generic_pack_for_id(pack)
            && is_pack_owned(*icon)
        {
            // Pacote ativo primeiro; Lucide (sempre compilado) como reserva.
            let mut candidates = vec![active];
            if active != GenericPack::Lucide {
                candidates.push(GenericPack::Lucide);
            }
            Self::ensure_fonts(ctx);
            if Self::icon_fonts_ready(ctx) {
                for candidate in candidates {
                    if let Ok(resolved) = resolve_utility(*icon, candidate) {
                        Self::paint_resolved(painter, resolved, target_rect, tint);
                        return;
                    }
                }
            }
            // Sem glifo no pacote: registra em dev e cai para o caminho Petunia
            // abaixo (mesmo significado, arte própria) — nunca tofu.
            #[cfg(debug_assertions)]
            {
                static LOGGED: LazyLock<std::sync::Mutex<std::collections::HashSet<String>>> =
                    LazyLock::new(|| std::sync::Mutex::new(std::collections::HashSet::new()));
                let key = format!("{pack}:{}", icon.id());
                if LOGGED
                    .lock()
                    .map(|mut s| s.insert(key.clone()))
                    .unwrap_or(false)
                {
                    eprintln!("petunia icons: '{key}' sem glifo no pacote — fallback Petunia");
                }
            }
        }

        // 2. Arte de domínio Petunia (ferramentas, vetores, PNGs Figma): vale para
        // TODOS os pacotes, por decisão — pacote muda o chrome, não a ferramenta.
        if is_toolbar_vector_tool(&id) {
            if let Some(texture) = get_or_load_toolbar_svg(ctx, &id) {
                let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
                painter.image(texture.id(), target_rect, uv, tint);
            } else {
                icons::paint(painter, &id, target_rect, tint);
            }
            return;
        }

        // 3. Tenta carregar asset rasterizado (PNG Figma) se existir
        if let Some(texture) = Self::get_or_load_texture(ctx, &id) {
            let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
            painter.image(texture.id(), target_rect, uv, tint);
            return;
        }

        // 4. Fallback para glifo Phosphor (fonte sempre instalada em ensure_fonts)
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

        // 5. Último recurso: losango vetorial (nunca tofu silencioso)
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
            | "merge"
            | "flip_diagonal"
            | "revolve"
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
        // PNGs da toolbar foram removidos (Wave 9: inalcançáveis); restam os
        // das abas de properties (22x22).
        let bytes = get_embedded_png_bytes("data_tab_01").expect("data_tab_01 png must exist");
        let image = decode_png_to_alpha_mask(bytes).expect("must decode valid alpha mask");
        assert_eq!(image.size, [22, 22]);
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
    fn test_pack_owned_icons_resolve_in_available_packs() {
        use crate::icon_provider::{GenericPack, is_pack_owned, resolve_utility};
        // Auditoria tofu (Wave 6 — §11.5): todo ícone de pacote tem ao menos um
        // glifo válido em todo pacote habilitado neste build.
        for icon in PetuniaIcon::all() {
            if !is_pack_owned(icon) {
                continue;
            }
            for pack in GenericPack::available() {
                let resolved = resolve_utility(icon, *pack);
                assert!(resolved.is_ok(), "{icon:?} sem glifo em '{}'", pack.name());
                let resolved = resolved.unwrap();
                assert!(resolved.as_char().is_some(), "{icon:?} codepoint inválido");
                assert!(!resolved.family.is_empty(), "{icon:?} sem família");
            }
        }
    }

    #[test]
    fn test_all_icons_paint_without_panic_in_all_packs() {
        use crate::icon_provider::GenericPack;
        // Sem aquecimento de propósito: ctx fresco deve cair no fallback sem pânico.
        let ctx = egui::Context::default();
        let mut packs = vec!["petunia".to_string()];
        packs.extend(
            GenericPack::available()
                .iter()
                .map(|p| p.name().to_string()),
        );

        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                for pack in &packs {
                    for icon in PetuniaIcon::all() {
                        let (rect, _) =
                            ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
                        IconRegistry::paint_pack(
                            &ctx,
                            ui.painter(),
                            &icon,
                            rect,
                            tokens::TEXT_ACTIVE,
                            pack,
                        );
                    }
                }
            });
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn test_icon_registry_paint_in_egui_context() {
        let ctx = egui::Context::default();
        IconRegistry::ensure_fonts(&ctx);

        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(32.0, 32.0), egui::Sense::hover());
                IconRegistry::paint(
                    &ctx,
                    ui.painter(),
                    &PetuniaIcon::Move,
                    rect,
                    tokens::TEXT_ACTIVE,
                );
                IconRegistry::paint(
                    &ctx,
                    ui.painter(),
                    &PetuniaIcon::Search,
                    rect,
                    tokens::TEXT_ACTIVE,
                );
                IconRegistry::paint(
                    &ctx,
                    ui.painter(),
                    &PetuniaIcon::PropRender,
                    rect,
                    tokens::TEXT_ACTIVE,
                );
            });
        })
        .textures_delta
        .clear();
    }
}
