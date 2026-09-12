# Crate `petunia_project` (`crates/project/`)

Responsável pela persistência e ciclo de vida de projetos do Petunia3D:
- Serialização e desserialização do formato binário proprietário versionado `.petunia`.
- Gestão de múltiplos assets por projeto com identificadores universais persistentes (UUIDs).
- Validadores de integridade contra corrupção de arquivos (`Project::validate`).
- Exportadores de malhas para formatos de intercâmbio de mercado (Wavefront OBJ e glTF 2.0 / GLB).
