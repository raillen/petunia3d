# P3D-155 — Live Asset Link

## Objetivo

Sincronizar alterações de assets com Map Editor/Game Engine sem export manual repetitivo.

## Escopo futuro

File watching inicialmente; IPC/protocolo versionado quando justificar. Mesh, material, texture, animation, collision, sockets e LOD metadata podem ser sincronizados.

## Regra

Integração é opcional; Petunia funciona totalmente offline e independente da engine. Falha/ausência da engine não pode bloquear o editor.