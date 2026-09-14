# 10 — Convenções Espaciais, Unidades e Coordenadas

<aside>
📐

Página normativa transversal. Nenhuma tool de transform, picking, import/export, rig, collision ou renderer deve inventar convenções locais. As decisões concretas devem ser confirmadas pela auditoria do código atual antes de congelar o contrato.

</aside>

# Objetivo

Eliminar ambiguidades de unidade, orientação, espaço de coordenadas e conversão entre Core, Renderer, UI e formatos externos.

# Decisões que precisam ser congeladas no Gauntlet 0

- unidade canônica interna e regra de conversão para import/export;
- handedness do espaço 3D;
- eixo vertical/up e convenção forward;
- ordem/composição de transforms;
- convenção angular e quaternion;
- matriz row/column-major apenas na fronteira onde isso importar;
- origem e orientação de UV;
- convenção de normal/tangent space;
- screen-space origin e transformação logical pixels ↔ physical pixels;
- profundidade/NDC e clip-space na boundary do renderer.

Não alterar convenções existentes sem relatório de impacto/migração.

# Espaços semânticos

Nomear explicitamente `Local`, `Object`, `World`, `View`, `Clip`, `ScreenLogical`, `ScreenPhysical`, `UV` e demais espaços realmente necessários. APIs não devem receber `Vec2`/`Vec3` ambíguos quando o espaço puder causar erro.

# Unidades e escala

Definir uma unidade canônica interna e metadata/conversão por importer/exporter. UI pode apresentar unidades amigáveis, mas o Core recebe valores normalizados. Scaling de import não pode ser escondido em widgets.

# Transforms

Definir contrato de Position/Rotation/Scale, pivot, orientation, parent transform, negative scale, non-uniform scale e transformação multi-selection. Operações devem produzir resultados determinísticos e testáveis headless.

# Picking e viewport

Input do frontend deve virar um `ScreenPoint` neutro + viewport descriptor; câmera produz Ray; picking consome Ray/scene data. DPI e resize não podem alterar semântica de picking.

# Import/Export

Cada adapter declara axis/unit conversion, winding, normals/tangents e UV conversion. Round-trip tests documentam perdas inevitáveis.

# Testes

Golden tests de transforms, camera rays, picking, unit conversion, import/export axis conversion, negative/non-uniform scale e DPI. Qualquer mudança de convenção exige migration note e testes de regressão.

# Definition of Done

Existe uma tabela canônica preenchida com as decisões reais do código; nenhum subsistema crítico mantém convenção paralela; import/export e renderer fazem conversões apenas nas boundaries apropriadas.