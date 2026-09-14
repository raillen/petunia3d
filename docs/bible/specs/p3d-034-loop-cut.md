# P3D-034 — Loop Cut

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado reportado: **implementado e funcional** · Prioridade: P1.

</aside>

## Objetivo

Inserir edge loops em topologia compatível sem prometer resultado onde não há caminho de loop válido.

## Auditoria

Validar detecção do loop, preview, posição/slide se existir, seleção resultante e undo.

## Regras

Em topologia incompatível, ação fica disabled ou retorna erro/hint claro; não “chuta” um corte.

## Dependências

P3D-018, P3D-041, P3D-123.

## Testes / DoD

Quads regulares, boundaries, poles/ngons, cancel e regressão.