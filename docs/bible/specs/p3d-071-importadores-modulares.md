# P3D-071 — Importadores modulares

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 8
- **Status Canônico**: `COMPLIANT (Import, Export & Delivery Pipeline)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria arquitetural** · Prioridade: P0.

</aside>

## Objetivo

Arquitetura de importers adicionáveis sem acoplar formato, file picker ou UI ao core.

## Interface esperada

Um importer declara formatos/capabilities, recebe bytes/path/contexto controlado, valida e produz representação Petunia + warnings/errors estruturados.

## Regras

- file dialog pertence ao frontend;
- parsing não depende de egui;
- Asset/Object/Material IDs são criados por serviços apropriados;
- entradas externas são não confiáveis.

## Dependências

P3D-102, P3D-103, P3D-108, P3D-124.

## Testes / DoD

Fixtures válidas/corrompidas, formato não suportado, missing resources e importer registrável sem alterar módulos não relacionados.