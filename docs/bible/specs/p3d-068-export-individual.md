# P3D-068 — Export individual

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 8
- **Status Canônico**: `PLANEJADA (Import, Export & Delivery Pipeline)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **pipeline precisa ser auditado/detalhado** · Prioridade: P1.

</aside>

## Objetivo

Exportar um asset isolado usando um pipeline único, validável e reutilizável.

## Contrato

A UI escolhe asset, profile, formato e destino; `ExportPipeline` valida e executa. Nenhum exporter abre file dialog diretamente.

## UX

Mostrar formato/profile, destino, warnings de compatibilidade e relatório final. Não expor opções que o exporter não suporta.

## Dependências

P3D-071, P3D-072, P3D-124.

## Testes / DoD

Asset válido/inválido, overwrite policy, path failure, material/texture references, saída reproduzível e erros estruturados.