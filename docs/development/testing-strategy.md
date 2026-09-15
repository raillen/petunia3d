# Estratégia de Testes e Garantia de Qualidade — Petunia3D

Este documento codifica o **ciclo rigoroso de testes, quality gates e conformance** para o Petunia3D. Nenhum commit é mesclado sem passar por todas as barreiras de verificação.

---

## 1. Test Levels (Níveis de Teste)

1. **Testes Unitários de Malha e Operações (`crates/mesh`)**:
   - Geometria computacional determinística: adição de vértices, indexação de faces, subdivisão, extrusão, triangulação e projeção UV.
   - Primitivas padrão: Cubo, Cilindro, Esfera UV, Ico-esfera, Cone, Plano e Toro com topologia válida e orientações normais corretas.
2. **Testes de Persistência e Limites de Confiança (`crates/project`)**:
   - Testes com entradas hostis: arquivos OBJ com índices fora de limites (OOB), vértices com NaN/infinito, arquivos `.petunia` corrompidos e sem assets.
   - Validação da integridade transacional de serialização binária e exportação glTF 2.0 / GLB.
3. **Testes de Undo/Redo e Comandos (`crates/commands`)**:
   - Gravação de snapshots, reversão exata de estado de malha e verificação de integridade pós-desfazer/refazer.
4. **Testes de Conformance Gráfica e Shaders (`crates/render`, `crates/render-gl`, `crates/render-wgpu`)**:
   - Validação de shaders GLSL 330 contra hardware de referência.
   - Testes de render-on-demand: garantia de que estados sem eventos (`idle`) geram zero redesenhos de frame na GPU.

---

## 2. Quality Gates (Portais de Qualidade)

Para aprovação de qualquer alteração no repositório:

| Gate | Comando | Critério de Aceitação |
|------|---------|------------------------|
| **Testes da Workspace** | `cargo test --workspace` | 100% dos testes aprovados (mínimo 23 testes ativos). |
| **Linter Estático** | `cargo clippy --workspace --all-targets` | 0 avisos (zero warnings tolerados). |
| **Formatação Canônica** | `cargo fmt --all --check` | 100% de conformidade com as regras do `rustfmt`. |
| **Trust Boundary Sanitization** | `cargo test -p petunia_project -p petunia_mesh` | Rejeição sem pânico de malhas degeneradas e arquivos malformados. |
| **Prumo Governance** | `prumo validate . && prumo doctor .` | Validação estrutural de governança e diagnósticos sem erros. |

---

## 3. Conformance Strategy (Estratégia de Conformidade)

- **Padrão Gráfico**: Estrita aderência ao OpenGL 3.3 Core Profile e GLSL 330. Nenhuma extensão específica de vendedor é obrigatória.
- **Exportação Game-Ready**: O formato glTF 2.0 (`.glb`) deve ser aceito sem avisos pelo validador oficial Khronos glTF Validator e importável diretamente na Godot Engine 4.x e Unity.
- **Isolamento de Cfg/Plataforma**: Validação contínua do fallback de backend (`PETUNIA_BACKEND=gl` vs `PETUNIA_BACKEND=wgpu`).

---

## 4. Evidence Expectations (Expectativas de Evidências)

- **Relatórios Gauntlet**: Todas as mudanças estruturais relevantes devem atualizar o documento canônico `docs/GAUNTLET.md` com métricas empíricas:
  - Tempo de startup até o primeiro frame exibido (meta: $\le 500$ ms).
  - Pegada de memória RAM residente (VmRSS) em idle (meta: $\le 100$ MB).
  - Taxa de quadros estável em cena teste de modelagem ($\ge 60$ FPS em hardware modesto).
  - Número exato de testes executados e aprovados.
- **Logs Operacionais**: Em caso de falha de renderização, logs estruturados via `RUST_LOG=wgpu_hal=debug,petunia=debug` devem ser anexados à evidência da tarefa.
