# Protocolo MCP (Model Context Protocol)

O Petunia3D possui suporte integrado ao padrão **Model Context Protocol (MCP)**, permitindo que agentes autônomos de Inteligência Artificial inspecionem e manipulem cenas 3D via automação segura.

---

## Ferramentas MCP Disponíveis

| Ferramenta MCP | Descrição |
| :--- | :--- |
| `petunia_get_scene_summary` | Retorna contagem de vértices, faces, malhas, anotações e medidas ativas. |
| `petunia_add_primitive` | Cria uma nova primitiva (`cube`, `cylinder`, `sphere`, `plane`) nas coordenadas especificadas. |
| `petunia_transform_object` | Translada, rotaciona ou escala um objeto identificado pelo seu UUID. |
| `petunia_export_model` | Exporta a cena ativa para `.glb` ou `.obj`. |
| `petunia_take_viewport_snapshot` | Captura uma imagem estática da cena para inspeção visual por agentes multimodais. |

## Segurança & Sandboxing
Todas as chamadas MCP passam por validação estrita de esquemas JSON, limites de timeout de execução e isolamento de caminhos de arquivos para garantir a proteção total dos dados do usuário.
