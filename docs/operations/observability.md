# Observabilidade, Diagnósticos e Telemetria — Petunia3D

Diretrizes para rastreamento de execução, diagnósticos de backend gráfico e benchmarks de desempenho no Petunia3D.

---

## 1. Níveis de Log e Rastreamento (`RUST_LOG`)

O Petunia3D utiliza a infraestrutura padrão do ecossistema Rust (`log` / `env_logger` / `tracing`):

- **Logs de Inicialização**:
  - Emite a versão do OpenGL ou adaptador wgpu detectado.
  - Informa extensões disponíveis e limites de hardware (resolução máxima de textura, tamanho de VBOs).
- **Controle via Linha de Comando**:
  ```bash
  # Diagnóstico padrão de inicialização e erros
  RUST_LOG=info cargo run

  # Diagnóstico profundo do pipeline gráfico wgpu/hal
  RUST_LOG=wgpu_hal=debug cargo run

  # Rastreamento completo de eventos e mutações da malha
  RUST_LOG=petunia=trace,wgpu=warn cargo run
  ```

---

## 2. Métricas de Desempenho e Frame Timing

- **Diagnóstico de Taxa de Quadros**:
  - No modo padrão render-on-demand, a aplicação consome 0 frames quando ociosa.
  - Para habilitar medição contínua de renderização e benchmarks de stress:
    ```bash
    SIMPLE3D_SPIN=1 cargo run --release
    ```
- **Fases de Renderização Monitoradas**:
  - **egui layout pass**: tempo de processamento de elementos de UI (~2–5 ms em release).
  - **mesh draw pass**: tempo de envio de geometria e chamada de draw do backend GPU (~1–3 ms para malhas low-poly).
  - **swap buffer**: tempo de espera de V-Sync e sincronização com o compositor de janelas do sistema operacional.

---

## 3. Monitoramento de Memória

- A pegada de memória é auditada periodicamente (conforme registrado em [`docs/GAUNTLET.md`](../GAUNTLET.md)):
  - VmRSS baseline: $\le 80$ MB.
  - Destruição e prune de texturas de assets deletados via verificação de UUIDs no frame draw.
