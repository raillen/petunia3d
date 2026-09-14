# P3D-140 — Material Shader Profiles & Effects

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Material Shader Profiles)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item · V1.x · Prioridade: P2.

</aside>

## Objetivo

Perfis/parâmetros de material para efeitos como vidro/transparência, emissive/lava, obsidian, unlit e toon sem depender inicialmente de node editor completo.

## Direção

Começar por descriptors simples: `Opaque`, `Unlit`, `Emissive`, `Transparent/Glass`, `Toon`. Parâmetros adicionais podem incluir emission, opacity/transmission e metallic/specular **somente onde renderer/export pipeline suportarem**.

## Arquitetura

Profiles estendem o Material System; não criam material paralelo. Renderer/exporter validam capabilities e fazem fallback/erro claro quando um formato não suporta o efeito.

## Dependências

P3D-050–054, P3D-068–072, P3D-113.

## Testes / DoD

Preview, save/load, fallback, export compatibility e performance em máquinas modestas.