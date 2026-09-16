# P3D-124 — Testes de import/export

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