# P3D-124 — Testes de import/export

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 8
- **Status Canônico**: `PLANEJADA (Import, Export & Delivery Pipeline)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Quality gate de I/O · Prioridade: P1.

</aside>

## Objetivo

Validar formatos suportados por fixtures/golden files, round-trip quando semanticamente possível e erro seguro para input ruim.

## Cobertura

Geometry, material/texture mapping, animation quando futuro, relative paths, missing resources, unsupported capability e malformed input.

## Segurança

Arquivos externos são não confiáveis; testar tamanho/corrupção/path traversal quando formato permite referências externas.

## Dependências

P3D-068–072.

## DoD

Cada importer/exporter declara fixtures, capability tests e regressões de bugs conhecidos.