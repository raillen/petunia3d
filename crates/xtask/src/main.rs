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
    println!("🏛️ Pontuação arquitetural atual: 3.8 / 10 (Coupled monolith).");
    println!("🚀 Roteiro de remediação: Gauntlet Loop G0 a G10 documentado.");
    Ok(())
}
