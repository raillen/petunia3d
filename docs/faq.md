# Perguntas Frequentes (FAQ)

**O Petunia3D é um clone do Blender?**
Não. É um modelador low-poly shape-first (estética PS1/N64/indie): desenhe a silhueta, gere a malha, detalhe — sem a complexidade de um DCC completo.

**É grátis?**
Sim, MIT. Se ajudar, [apoie no ko-fi](https://ko-fi.com/raillen).

**Roda em GPU antiga?**
Sim: OpenGL-first (GL 3.3 Core) com fallback automático a partir do wgpu. Veja [Diagnóstico & GPU](./reference/diagnostics.md).

**Como desfaço?**
`Ctrl+Z` / `Ctrl+Shift+Z`. Cada gesto = 1 nível de undo.

**Onde ficam meus arquivos?**
Projetos `.petunia` (binário versionado) onde você salvar. Autosave permite recuperar após queda.

**Posso usar comercialmente?**
Sim, a licença MIT permite, inclusive em jogos vendidos.

**Como importar/exportar?**
OBJ importa e exporta; glTF importa; GLB exporta ([matriz](./reference/formats.md)). Lote via diálogo de exportação ou `petunia-cli convert`.

**Tem em português?**
Sim, interface completa em English e Português (Settings → Language).
