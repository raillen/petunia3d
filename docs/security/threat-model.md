# Modelo de Ameaças (STRIDE) — Petunia3D

Análise de riscos e matriz de mitigações de segurança para o modelador desktop 3D Petunia3D.

---

## Matriz STRIDE

| Categoria | Vetor Potencial | Mitigação Mandatória no Petunia3D |
|-----------|-----------------|-----------------------------------|
| **Spoofing** (Falsificação) | Substituição maliciosa do binário ou injeção de bibliotecas dinâmicas falsificadas no PATH. | Binário estático assinado em releases; verificação de checksums SHA-256 no pipeline de empacotamento. |
| **Tampering** (Adulteração) | Arquivos `.obj` ou `.petunia` modificados com índices de vértice fora de limite (OOB), NaN, infinitos ou loops de faces. | Sanitização rigorosa no trust boundary (`petunia_mesh::obj`, `Mesh::validate`, `Project::validate`). Rejeição imediata sem crash (`panic!`). |
| **Repudiation** (Não-repúdio) | Modificações destrutivas acidentais na malha sem possibilidade de auditoria ou reversão. | Histórico granular de snapshots via `commands::CommandHistory` que permite auditoria de passos e reversão completa (`undo`/`redo`). |
| **Information Disclosure** (Exposição) | Vazamento de caminhos absolutos locais de arquivos ou dados confidenciais do usuário em logs de renderização. | Sanitização de paths em logs; ausência completa de telemetria externa ou envio de dados para servidores remotos. |
| **Denial of Service** (Negação de Serviço) | Arquivos OBJ com bilhões de polígonos ou imagens com bilhões de pixels (decompression bombs) visando exaurir a RAM. | Limites máximos de polígonos e dimensões de textura verificados antes da alocação de buffers; descarte com `ExportError` ou erro de carregamento. |
| **Elevation of Privilege** (Elevação de Privilégio) | Execução arbitrária de código remoto (RCE) através de metadados embutidos em arquivos 3D. | Parsers escritos 100% em Rust seguro (`#![forbid(unsafe_code)]`). Sem chamada de shell dinâmico ou execução de comandos via scripts externos. |
