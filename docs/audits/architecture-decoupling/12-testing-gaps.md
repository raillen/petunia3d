# 12 — Lacunas da Suíte de Testes (Testing Gap Report)

> **Auditoria dos 198 testes automatizados do workspace, mapeamento da concentração de testes em simulação de UI e plano de testes arquiteturais de fitness.**

---

## 1. Distribuição Atual dos Testes Automatizados

O workspace do Petunia3D possui **198 testes automatizados** aprovados, distribuídos da seguinte forma entre os crates:

```text
DISTRIBUIÇÃO DOS TESTES NO WORKSPACE:
┌───────────────────────────┬──────────────┬──────────────┐
│ Crate                     │ Qtd Testes   │ % do Total   │
├───────────────────────────┼──────────────┼──────────────┤
│ petunia_ui                │ 88 testes    │ 44.4%        │
│ petunia_mesh              │ 53 testes    │ 26.8%        │
│ petunia_core              │ 42 testes    │ 21.2%        │
│ petunia_project           │ 13 testes    │  6.6%        │
│ petunia_commands          │  2 testes    │  1.0%        │
│ petunia_app               │  0 testes    │  0.0%        │
│ petunia_config            │  0 testes    │  0.0%        │
│ petunia_module_model      │  0 testes    │  0.0%        │
│ petunia_module_paint      │  0 testes    │  0.0%        │
│ petunia_module_uv         │  0 testes    │  0.0%        │
│ petunia_module_assets     │  0 testes    │  0.0%        │
│ petunia_render            │  0 testes    │  0.0%        │
│ petunia_render_wgpu       │  0 testes    │  0.0%        │
│ petunia_render_gl         │  0 testes    │  0.0%        │
├───────────────────────────┼──────────────┼──────────────┤
│ TOTAL                     │ 198 testes   │ 100.0%       │
└───────────────────────────┴──────────────┴──────────────┘
```

---

## 2. Diagnóstico das Lacunas Principais

### Lacuna 1: Concentração Desproporcional de Testes de Comportamento na UI
Quase metade dos testes do sistema vive dentro de `crates/ui` (`cutting_tests.rs`, `modal_tests.rs`, `paint_tests.rs`, `outliner.rs`):
* Esses testes validam regras cruciais de modelagem (ex: corte de aresta por faca, cancelamento por Escape de transformação modal, arraste de pincel de pintura).
* **O Problema Arquitetural**: Para validar essas regras, os testes instanciam um `egui::Context`, simulam cliques e coordenadas de mouse virtuais do egui (`ctx.run(...)`) e inspecionam o resultado no `AppState`.
* Se o `egui` for substituído amanhã por outro framework, **88 testes automatizados serão jogados fora ou quebrarão**, não porque a lógica de modelagem mudou, mas porque os testes estavam acoplados ao simulador de eventos do egui.

### Lacuna 2: Zero Testes de Integração de Sessão Headless
Não existe nenhum teste que valide o ciclo completo de sessão de um usuário sem carregar widgets:
* Iniciar sessão -> Criar cubo -> Selecionar face 1 -> Despachar extrusão -> Verificar malha -> Desfazer -> Salvar arquivo -> Recarregar.
* Hoje esse fluxo só é exercitado através dos testes de UI do kittest ou manualmente pela aplicação desktop.

### Lacuna 3: Zero Testes em Módulos, Configuração e App Runner
* Os crates `petunia_module_*`, `petunia_config` e `petunia_app` possuem **zero testes unitários**.
* Erros de carregamento de arquivo TOML de atalhos ou regressões no mapeamento de teclas só são percebidos se o aplicativo for executado interativamente.

---

## 3. Introdução de Funções de Fitness Arquitetural (Architecture Fitness Tests)

Para garantir que a futura refatoração não sofra regressão e que nenhuma nova dependência proibida seja adicionada, o projeto deve introduzir **testes de fitness arquitetural**:

### Exemplo de Teste de Fitness (CI / xtask):
```rust
#[test]
fn core_must_never_depend_on_egui() {
    let output = std::process::Command::new("cargo")
        .args(["tree", "-p", "petunia_core", "--depth", "1", "-e", "normal"])
        .output()
        .expect("failed to run cargo tree");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("egui"),
        "VIOLAÇÃO ARQUITETURAL: petunia_core depende diretamente de egui!\n{}",
        stdout
    );
}
```

Esses testes garantem que as regras de isolamento sejam aplicadas automaticamente pelo compilador e pelo pipeline de CI, sem depender da memória ou disciplina manual dos desenvolvedores.
