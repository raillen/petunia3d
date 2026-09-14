# P3D-155 — Live Asset Link

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 13 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Shared 3D Foundation & Engine)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Sincronizar alterações de assets com Map Editor/Game Engine sem export manual repetitivo.

## Escopo futuro

File watching inicialmente; IPC/protocolo versionado quando justificar. Mesh, material, texture, animation, collision, sockets e LOD metadata podem ser sincronizados.

## Regra

Integração é opcional; Petunia funciona totalmente offline e independente da engine. Falha/ausência da engine não pode bloquear o editor.