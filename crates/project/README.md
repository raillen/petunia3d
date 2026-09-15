# Crate `petunia_project` (`crates/project/`)

Responsável pela persistência e ciclo de vida de projetos do Petunia3D:
- Serialização e desserialização do formato binário proprietário versionado `.petunia`.
- Gestão de múltiplos assets por projeto com identificadores universais persistentes (UUIDs).
- Validadores de integridade contra corrupção de arquivos (`Project::validate`).
- Exportadores e importadores de malhas para formatos de intercâmbio de mercado (Wavefront OBJ, glTF 2.0 / GLB, Petunia Package .pkg).
- Pipeline unificado e modular de entrega e importação (`DeliveryPipeline` / `pipeline.rs`) com declaração de matriz de capacidades (`FormatCapabilities`), exportação individual, múltipla e em lote (Batch Export) determinística com tolerância a falhas parciais.
