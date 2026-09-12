# Crate `petunia_app` (`crates/app/`)

Crate coordenador do ciclo de vida da aplicação:
- Criação e inicialização de instâncias dos módulos registrados.
- Detecção e inicialização de backends gráficos (`render-gl` vs `render-wgpu`) com lógica de fallback resiliente.
- Implementação do loop de eventos reativo com suporte a render-on-demand (`0 frames` em repouso).
