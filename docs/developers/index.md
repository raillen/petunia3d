# Portal do Desenvolvedor

Bem-vindo à documentação de engenharia do **Petunia3D**! Se você deseja contribuir com código, criar extensões, auditar a segurança ou integrar o Petunia3D com agentes autônomos, você está no lugar certo.

## Sumário Técnico

- **[Macroarquitetura](./architecture)**: Visão geral dos 14 crates modulares, fluxo de dependências e Clean Architecture.
- **[Camada de Renderização](./rendering)**: Pipeline híbrido WebGPU (`wgpu`) com fallback automático para OpenGL (`glow`).
- **[Comandos & Transações](./commands)**: Modelo transacional de Undo/Redo com isolamento de checkpoints.
- **[Sistema de Plugins](./plugins)**: Arquitetura de extensões dinâmicas e módulos desacoplados.
- **[Protocolo MCP (Model Context Protocol)](./mcp)**: Ferramentas e automação para agentes de Inteligência Artificial.
- **[Estratégia de Testes](./testing)**: Testes unitários, conformance, testes de interface headless com `egui_kittest` e gates de CI.
- **[Guia de Contribuição](./contributing)**: Padrões de código Rust, formatação, linter estrito e governança do repositório.
