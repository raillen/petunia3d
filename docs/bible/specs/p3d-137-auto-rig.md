# P3D-137 — Auto-Rig

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 10 (Pós-GA)
- **Status Canônico**: `COMPLIANT (Implementado e Verificado)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementado via `auto_fit_humanoid` e `compute_auto_skin_weights` em `crates/project/src/animation.rs`** · Prioridade: P3.

</aside>

## Objetivo

Assistir criação/posicionamento de rig sem virar caixa-preta.

## Regras

- resultado sempre editável;
- indicar falhas/ambiguidade;
- permitir correção manual antes de bind;
- não ocultar limitações de topologia/pose.

## Estratégia

Começar por fitting simples de presets com landmarks/manual hints; técnicas mais automáticas só após benchmark de qualidade.

## Dependências

P3D-135, P3D-136.

## Testes / DoD

Modelos humanos/quadrúpedes simples, proporções diferentes, falha segura, undo e comparação com rig manual.