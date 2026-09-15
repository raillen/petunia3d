# Matriz de Formatos 3D

Resumo amigável. A fonte canônica (gerada do código) está em
[Formatos Suportados](../generated/SUPPORTED_FORMATS.md).

| Formato | Importa | Exporta | Notas |
|---|---|---|---|
| `.petunia` | ✅ | ✅ | Projeto nativo: UUIDs, undo, materiais, tudo |
| `.obj` | ✅ | ✅ | Intercâmbio universal (sem PBR) |
| `.gltf` | ✅ | ❌ | Leitura com PBR/texturas |
| `.glb` | ❌ | ✅ | Entrega game-ready com PBR |
| `.pkg` | ✅ | ✅ | Pacote binário completo |

Opções do pipeline: triangulação na emissão, materiais PBR, escala uniforme
e tolerância a falhas em lote.
