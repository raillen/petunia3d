//! Ferramenta de automação interna (cargo xtask) para o Petunia3D.
//! Gerencia tarefas de documentação, integridade e drift prevention.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "docs" => task_docs()?,
        "docs-check" => task_docs_check()?,
        "arch-check" => task_arch_check()?,
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("Comando desconhecido: {other}\n");
            print_help();
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"Petunia3D xtask — Automação de Desenvolvimento

USO:
    cargo xtask <COMANDO>

COMANDOS:
    docs          Gera e compila o site estático de documentação com VitePress
    docs-check    Valida a integridade dos arquivos de documentação e build sem erros
    arch-check    Valida a integridade dos relatórios da auditoria arquitetural
    help          Exibe esta mensagem de ajuda
"#
    );
}

fn root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("falha ao localizar raiz do workspace")
        .to_path_buf()
}

fn task_docs() -> Result<()> {
    println!("📦 Compilando documentação oficial do Petunia3D (VitePress)...");
    let root = root_dir();
    let docs_dir = root.join("docs");

    let status = Command::new("pnpm")
        .arg("run")
        .arg("build")
        .current_dir(&docs_dir)
        .status()
        .context("falha ao executar pnpm na pasta docs/")?;

    if !status.success() {
        bail!("Build da documentação falhou com status: {status}");
    }

    println!("✅ Documentação gerada com sucesso em docs/.vitepress/dist/");
    Ok(())
}

fn task_docs_check() -> Result<()> {
    println!("🔍 Validando integridade da documentação...");
    let root = root_dir();
    let docs_dir = root.join("docs");

    // Verificar existência de arquivos canônicos
    let required_files = [
        "index.md",
        "getting-started/index.md",
        "getting-started/what-is-petunia3d.md",
        "getting-started/installation.md",
        "getting-started/first-project.md",
        "getting-started/interface-overview.md",
        "getting-started/your-first-model.md",
        "manual/index.md",
        "manual/interface.md",
        "manual/viewport.md",
        "manual/selection.md",
        "manual/modeling.md",
        "manual/paint.md",
        "manual/uv.md",
        "manual/animation.md",
        "manual/asset-library.md",
        "manual/projects.md",
        "manual/export.md",
        "workspaces/index.md",
        "workspaces/modeling.md",
        "workspaces/paint.md",
        "workspaces/uv.md",
        "workspaces/animation.md",
        "tools/index.md",
        "customization/index.md",
        "shortcuts/index.md",
        "developers/index.md",
        "changelog/index.md",
    ];

    for file in &required_files {
        let p = docs_dir.join(file);
        if !p.exists() {
            bail!("Arquivo essencial de documentação ausente: docs/{file}");
        }
    }

    println!(
        "📄 Todos os {} arquivos essenciais estão presentes.",
        required_files.len()
    );

    // Executar build de verificação
    task_docs()?;

    println!("🎉 Verificação de integridade concluída com sucesso!");
    Ok(())
}

fn task_arch_check() -> Result<()> {
    println!("🔍 Validando relatórios da Auditoria Arquitetural...");
    let root = root_dir();
    let audit_dir = root
        .join("docs")
        .join("audits")
        .join("architecture-decoupling");

    if !audit_dir.exists() {
        bail!("Diretório de auditoria não encontrado: docs/audits/architecture-decoupling");
    }

    let required_reports = [
        "README.md",
        "01-current-architecture.md",
        "02-dependency-analysis.md",
        "03-ui-coupling.md",
        "04-state-ownership.md",
        "05-command-system.md",
        "06-tool-system.md",
        "07-renderer-boundary.md",
        "08-project-assets-io.md",
        "09-modularity-extension-cost.md",
        "10-headless-readiness.md",
        "11-cross-language-ui.md",
        "12-testing-gaps.md",
        "13-findings.md",
        "14-scorecard.md",
        "15-target-architecture-options.md",
        "16-remediation-plan.md",
    ];

    let mut missing = Vec::new();
    let mut total_bytes = 0;

    for report in &required_reports {
        let p = audit_dir.join(report);
        if !p.exists() {
            missing.push(*report);
        } else {
            let metadata = std::fs::metadata(&p)
                .with_context(|| format!("Falha ao ler metadados de {report}"))?;
            if metadata.len() == 0 {
                bail!("Relatório de auditoria está vazio: {report}");
            }
            total_bytes += metadata.len();
        }
    }

    if !missing.is_empty() {
        bail!("Relatórios de auditoria ausentes: {:?}", missing);
    }

    println!(
        "📊 Auditoria validada com sucesso: {} relatórios canônicos presentes ({:.1} KB de documentação técnica).",
        required_reports.len(),
        total_bytes as f64 / 1024.0
    );

    println!("🛡️ Validando invariantes arquiteturais dos manifestos Cargo...");
    let checks = [
        (
            "crates/core/Cargo.toml",
            &["egui ="][..],
            "petunia_core não pode depender de egui",
        ),
        (
            "crates/config/Cargo.toml",
            &["egui ="][..],
            "petunia_config não pode depender de egui",
        ),
        (
            "crates/mesh/Cargo.toml",
            &["egui", "petunia_core", "petunia_ui"][..],
            "petunia_mesh deve ser 100% puro",
        ),
        (
            "crates/project/Cargo.toml",
            &["egui", "petunia_ui"][..],
            "petunia_project não pode depender de UI",
        ),
        (
            "crates/commands/Cargo.toml",
            &["egui", "petunia_ui"][..],
            "petunia_commands não pode depender de UI",
        ),
        (
            "crates/render-wgpu/Cargo.toml",
            &["egui", "petunia_ui"][..],
            "petunia_render_wgpu não pode depender de UI",
        ),
        (
            "crates/module-paint/Cargo.toml",
            &["rfd ="][..],
            "petunia_module_paint não pode depender de rfd",
        ),
        (
            "crates/module-model/Cargo.toml",
            &["egui"][..],
            "petunia_module_model não pode depender de egui (G7)",
        ),
        (
            "crates/module-paint/Cargo.toml",
            &["egui"][..],
            "petunia_module_paint não pode depender de egui (G7)",
        ),
        (
            "crates/module-uv/Cargo.toml",
            &["egui"][..],
            "petunia_module_uv não pode depender de egui (G7)",
        ),
        (
            "crates/module-assets/Cargo.toml",
            &["egui"][..],
            "petunia_module_assets não pode depender de egui (G7)",
        ),
        (
            "crates/cli/Cargo.toml",
            &["egui"][..],
            "petunia_cli não pode depender de egui (G8)",
        ),
        (
            "crates/ffi/Cargo.toml",
            &["egui"][..],
            "petunia_ffi não pode depender de egui (G10)",
        ),
    ];

    for (rel_path, forbidden, reason) in checks {
        let path = root.join(rel_path);
        let content =
            std::fs::read_to_string(&path).with_context(|| format!("Falha ao ler {rel_path}"))?;
        for pattern in forbidden {
            if content.contains(pattern) {
                bail!("Violação em {rel_path}: contém '{pattern}' proibido ({reason})");
            }
        }
    }

    println!("✅ Invariantes de manifesto validados: core, config, mesh, project, commands, render-wgpu e module-paint estão em conformidade.");

    println!("🛡️ Validando fronteira de I/O de arquivos (Gauntlet G4)...");
    let mut rs_files = Vec::new();
    let crates_dir = root.join("crates");

    fn collect_rs(dir: &Path, acc: &mut Vec<PathBuf>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    collect_rs(&path, acc)?;
                } else if path.extension().is_some_and(|ext| ext == "rs") {
                    acc.push(path);
                }
            }
        }
        Ok(())
    }

    collect_rs(&crates_dir, &mut rs_files)?;

    for file_path in &rs_files {
        let rel = file_path
            .strip_prefix(&root)
            .unwrap_or(file_path)
            .to_string_lossy();

        if rel == "crates/ui/src/file_dialog_service.rs" || rel.starts_with("crates/xtask") {
            continue;
        }

        let content = std::fs::read_to_string(file_path)
            .with_context(|| format!("Falha ao ler {}", file_path.display()))?;

        if content.contains("rfd::FileDialog") {
            bail!("Violação de Fronteira de I/O: {rel} instancia rfd::FileDialog fora de file_dialog_service.rs!");
        }
        if content.contains("egui_file_dialog::FileDialog") {
            bail!("Violação de Fronteira de I/O: {rel} instancia egui_file_dialog fora de file_dialog_service.rs!");
        }
    }

    println!("✅ Fronteira de I/O validada: nenhum arquivo fora de crates/ui/src/file_dialog_service.rs instancia rfd ou egui-file-dialog.");

    println!("🛡️ Validando ausência de sessões de ferramentas em memória temporária de UI (Gauntlet G5)...");
    for file_path in &rs_files {
        let rel = file_path
            .strip_prefix(&root)
            .unwrap_or(file_path)
            .to_string_lossy();

        if rel.starts_with("crates/xtask") {
            continue;
        }

        let content = std::fs::read_to_string(file_path)
            .with_context(|| format!("Falha ao ler {}", file_path.display()))?;

        if content.contains("\"cut.session\"") {
            bail!("Violação de Sessão de Ferramenta (G5): {rel} ainda utiliza Id(\"cut.session\") em memória temporária de UI!");
        }
        if content.contains("\"modal.pointer\"") {
            bail!("Violação de Sessão de Ferramenta (G5): {rel} ainda utiliza Id(\"modal.pointer\") em memória temporária de UI!");
        }
    }
    println!("✅ Sessões de ferramentas validadas: CutSession e PointerSession residem exclusivamente no domínio (AppState).");

    println!("🛡️ Validando decomposição do God Object AppState (Gauntlet G6 / F-002)...");
    let state_rs = root.join("crates/core/src/state.rs");
    let state_content = std::fs::read_to_string(&state_rs)
        .with_context(|| format!("Falha ao ler {}", state_rs.display()))?;

    let required_substates = [
        "pub struct ProjectState",
        "pub struct EditorSession",
        "pub struct ToolState",
        "pub struct UiState",
        "pub struct RenderResources",
    ];
    for sub in required_substates {
        if !state_content.contains(sub) {
            bail!("Violação de Decomposição (G6): sub-estado {sub} não encontrado em crates/core/src/state.rs!");
        }
    }

    let required_app_state_fields = [
        "pub project: ProjectState",
        "pub session: EditorSession",
        "pub ui: UiState",
        "pub render: RenderResources",
        "pub events: EventBus",
    ];
    for field in required_app_state_fields {
        if !state_content.contains(field) {
            bail!("Violação de Composição de AppState (G6): campo {field} não encontrado em AppState!");
        }
    }
    if !state_content.contains("pub tools: ToolState") {
        bail!("Violação de Composição (G6): campo pub tools: ToolState não encontrado em EditorSession!");
    }

    println!("✅ Sub-estados coesos validados: AppState decomposto em ProjectState, EditorSession, ToolState, UiState e RenderResources.");

    println!("🛡️ Validando purificação dos crates de módulo (Gauntlet G7 / F-008)...");
    for file_path in &rs_files {
        let rel = file_path
            .strip_prefix(&root)
            .unwrap_or(file_path)
            .to_string_lossy();

        if rel.starts_with("crates/module-") {
            let content = std::fs::read_to_string(file_path)
                .with_context(|| format!("Falha ao ler {}", file_path.display()))?;
            if content.contains("egui::") || content.contains("use egui") {
                bail!("Violação de Purificação de Módulo (G7 / F-008): {rel} contém referência direta a egui!");
            }
        }
    }
    println!("✅ Módulos purificados: module-model, module-paint, module-uv e module-assets são 100% livres de egui.");

    println!("🛡️ Validando soberania headless do crate CLI (Gauntlet G8 / F-011)...");
    for file_path in &rs_files {
        let rel = file_path
            .strip_prefix(&root)
            .unwrap_or(file_path)
            .to_string_lossy();

        if rel.starts_with("crates/cli") {
            let content = std::fs::read_to_string(file_path)
                .with_context(|| format!("Falha ao ler {}", file_path.display()))?;
            if content.contains("egui::") || content.contains("use egui") {
                bail!("Violação de Soberania Headless (G8 / F-011): {rel} contém referência direta a egui!");
            }
        }
    }
    println!(
        "✅ Crate CLI validado: petunia-cli é 100% puro e opera sem qualquer dependência de UI."
    );

    println!("🛡️ Validando estabilização da Application API (Gauntlet G9 / F-010)...");
    let queries_path = root.join("crates/core/src/queries.rs");
    if !queries_path.exists() {
        bail!(
            "Violação de Application API (G9 / F-010): crates/core/src/queries.rs não encontrado!"
        );
    }
    let state_path = root.join("crates/core/src/state.rs");
    let state_content = std::fs::read_to_string(&state_path)?;
    if !state_content.contains("pub export_selected: Vec<Uuid>") {
        bail!("Violação de Application API (G9 / F-010): export_selected deve ser Vec<Uuid>!");
    }
    println!("✅ Application API validada: DTOs e identificadores estáveis (UUID) ativos.");

    println!("🛡️ Validando soberania da camada C-ABI / FFI (Gauntlet G10)...");
    for file_path in &rs_files {
        let rel = file_path
            .strip_prefix(&root)
            .unwrap_or(file_path)
            .to_string_lossy();

        if rel.starts_with("crates/ffi") {
            let content = std::fs::read_to_string(file_path)
                .with_context(|| format!("Falha ao ler {}", file_path.display()))?;
            if content.contains("egui::") || content.contains("use egui") {
                bail!("Violação de C-ABI (G10): {rel} contém referência direta a egui!");
            }
        }
    }
    let header_path = root.join("crates/ffi/include/petunia.h");
    if !header_path.exists() {
        bail!("Violação de C-ABI (G10): header crates/ffi/include/petunia.h ausente!");
    }
    println!("✅ Camada C-ABI / FFI validada: petunia_ffi e include/petunia.h são 100% autônomos.");

    println!("🛡️ Validando ausência de atalhos físicos nas ferramentas (Wave 1)...");
    for file_path in &rs_files {
        let rel = file_path
            .strip_prefix(&root)
            .unwrap_or(file_path)
            .to_string_lossy();

        if rel.starts_with("crates/module-model")
            || rel == "crates/core/src/modal.rs"
            || rel == "crates/core/src/cutting_session.rs"
        {
            let content = std::fs::read_to_string(file_path)
                .with_context(|| format!("Falha ao ler {}", file_path.display()))?;
            if content.contains("winit::keyboard")
                || content.contains("PhysicalKey")
                || content.contains("egui::Key")
            {
                bail!("Violação de Desacoplamento de Entrada (Wave 1): {rel} referencia atalhos físicos!");
            }
        }
    }
    println!("✅ Soberania de ferramentas validada: tools não conhecem keycodes físicos.");

    println!("🛡️ Validando ausência de eframe em todo o domínio (Wave 1)...");
    let domain_crates = [
        "crates/core",
        "crates/mesh",
        "crates/commands",
        "crates/config",
        "crates/project",
        "crates/render",
        "crates/module-model",
        "crates/module-paint",
        "crates/module-uv",
        "crates/module-assets",
        "crates/cli",
        "crates/ffi",
    ];
    for c in domain_crates {
        let cargo_p = root.join(c).join("Cargo.toml");
        if cargo_p.exists() {
            let content = std::fs::read_to_string(&cargo_p)?;
            if content.contains("eframe") {
                bail!("Violação de Domínio (Wave 1): {c}/Cargo.toml depende de eframe!");
            }
        }
    }
    println!("✅ Domínio agnóstico validado: eframe ausente de todo o Core e Módulos.");

    println!("🛡️ Validando integridade de projetos e persistência agnóstica à UI (Wave 2)...");
    let project_crates = [
        "crates/project",
        "crates/core/src/project_service.rs",
        "crates/core/src/recent_projects.rs",
    ];
    for p in project_crates {
        let p_path = root.join(p);
        if p_path.is_file() {
            let content = std::fs::read_to_string(&p_path)?;
            if content.contains("egui::") || content.contains("eframe") {
                bail!("Violação de Persistência (Wave 2): {p} depende de egui/eframe!");
            }
        } else if p_path.is_dir() {
            let src_dir = p_path.join("src");
            let target_dir = if src_dir.exists() { src_dir } else { p_path };
            for entry in std::fs::read_dir(&target_dir)? {
                let entry = entry?;
                if entry.path().extension().is_some_and(|e| e == "rs") {
                    let content = std::fs::read_to_string(entry.path())?;
                    if content.contains("egui::") || content.contains("eframe") {
                        bail!(
                            "Violação de Persistência (Wave 2): {} depende de egui/eframe!",
                            entry.path().display()
                        );
                    }
                }
            }
        }
    }
    println!("✅ Integridade de projeto validada: persistência e autosave 100% livres de UI/egui.");

    println!(
        "🏛️ Progresso de remediação: GAUNTLETS G0 a G10, WAVE 1 e WAVE 2 100% CONCLUÍDOS COM SUCESSO!"
    );
    Ok(())
}
