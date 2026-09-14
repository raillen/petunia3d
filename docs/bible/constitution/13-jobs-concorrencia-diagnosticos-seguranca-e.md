# 13 — Jobs, Concorrência, Diagnósticos, Segurança e Trust Boundaries

<aside>
🛡️

Política transversal para autosave, thumbnails, import/export, caches, plugins, custom packs, MCP e futuras integrações. Dados externos são não confiáveis por padrão.

</aside>

# Thread ownership

Mapear main/UI thread, renderer/GPU ownership e worker jobs. Nenhum worker pode mutar editor state arbitrariamente. Resultados retornam por mensagens/result objects/queues controladas.

# Jobs

Thumbnail generation, filesystem scan, import/export pesado, autosave serialization e processamento de assets devem declarar: start, progress quando útil, cancellation, completion, failure e cleanup.

# Cancellation

Cancelar não pode deixar arquivo parcial, cache inconsistente ou transaction incompleta. Jobs não canceláveis precisam ser explicitamente pequenos ou isolados.

# Determinism e races

Operações concorrentes sobre o mesmo projeto/asset precisam de policy clara: snapshot imutável, version/check generation ou serialização de mutations. Evitar `Arc<Mutex<Everything>>` como arquitetura padrão.

# Diagnostics

Definir categorias e severidade: Info, Warning, Recoverable Error, Fatal/Crash. Erros estruturados possuem código/categoria/contexto técnico; `TextId` produz mensagem user-facing. Logs não substituem UX.

# Crash diagnostics

Planejar log local, build/version, operação ativa e recovery marker sem coletar conteúdo pessoal desnecessário. Telemetria remota não é requisito e não deve ser adicionada silenciosamente.

# Trust boundaries

Validar paths, canonicalization/containment, symlinks quando relevantes, tamanhos máximos razoáveis, arquivos corrompidos e formatos maliciosos. Custom theme/icon/translation/keymap packs não executam código.

# Plugins/MCP

Capabilities explícitas, path/file permissions, operações destrutivas identificáveis, API versioning e logs auditáveis quando apropriado. Nenhum plugin recebe `egui::Context` ou `wgpu::Device` diretamente.

# Resource limits

Importers/image decoders/SVG/parsers devem ter limites e falhar de forma recuperável contra input gigantesco ou inválido. Evitar zip bombs/archive traversal se packages forem introduzidos.

# Tests

Failure injection, cancellation, malformed files, path traversal, permission denied, concurrent job completion, stale result/version mismatch e cleanup após crash/restart.

# DoD

Asynchronous work melhora responsividade sem criar uma segunda ownership model ou comprometer integridade do projeto.