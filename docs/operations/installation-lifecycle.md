# Ciclo de Vida de Instalação e Operações — Petunia3D

Este documento formaliza as garantias operacionais de empacotamento, distribuição, caminhos de execução e tolerância a interrupções.

---

## 1. Installation Paths e Distribuição

- O Petunia3D é distribuído prioritariamente como binário nativo estático pré-compilado ou via Cargo.
- Paths canônicos:
  - Binário: `/usr/local/bin/petunia3d` ou `~/.local/bin/petunia3d` ou `~/.cargo/bin/simple3d-modeling`.
  - Configurações: `~/.config/petunia3d/`.
  - Cache de shaders e buffers: `/tmp/petunia3d/` ou gerenciado dinamicamente em memória pela GPU.

---

## 2. Ownership e Permissões

- Não são necessários privilégios administrativos para executar ou atualizar o Petunia3D.
- Todo o ciclo de vida opera sob as permissões do usuário corrente (UID local).

---

## 3. Rollback Strategy e Atualizações Interrompidas

- **O que acontece após atualização interrompida? (Interrupted update)**:
  - O processo de atualização via scripts de release ou gerenciadores de pacotes realiza download e verificação de integridade (checksum SHA-256) antes de substituir o arquivo executável no disco.
  - A escrita no destino ocorre através de link atômico (`mv temp_binary dest_binary`). Caso a atualização seja interrompida no meio do processo, o binário anterior permanece íntegro e executável.
  - Para rollback imediato: o usuário pode manter o executável anterior renomeado (ex: `petunia3d.old`) ou reinstalar a versão fixa desejada via Git/Cargo.

---

## 4. Uninstall Safety (Desinstalação Limpa e Segura)

- A desinstalação resume-se à remoção do binário único e da pasta opcional de configurações do usuário.
- Nenhuma chave de registro obscura ou serviço de background em execução persistente permanece no sistema operacional.
