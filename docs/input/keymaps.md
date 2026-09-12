# Perfis de Teclado e Atalhos — Petunia3D

O **Petunia3D** oferece um sistema abrangente e modular de atalhos de teclado implementado em `crates/config/src/keybinds.rs`. O sistema foi concebido para atender tanto usuários vindos de outros softwares 3D tradicionais quanto usuários de notebooks e iniciantes.

---

## 1. Perfis Canônicos de Teclado

O Petunia3D distribui 8 perfis em formato TOML em `assets/keymaps/`:

| Perfil | Arquivo | Filosofia de Atalhos |
| :--- | :--- | :--- |
| **Petunia Padrão** | `petunia-default.toml` | Atalhos balanceados: G/R/S para transformações, Tab para alternar modo, M para medir, D para anotar. |
| **Petunia Simplificado** | `petunia-simple.toml` | Atalhos intuitivos inspirados em motores de jogos: W para transladar, E para rotacionar, R para escalar. |
| **Petunia Notebook** | `petunia-notebook.toml` | Substitui controles que exigem numpad por combinações de números superiores e teclas padrão. |
| **Blender** | `blender.toml` | Paridade estrita com as convenções do Blender (G=Grab, R=Rotate, S=Scale, Shift+A=Add Menu). |
| **Blender Notebook** | `blender-notebook.toml` | Convenções do Blender com emulação de visualizações de câmera sem teclado numérico. |
| **Autodesk Maya** | `maya.toml` | Q para selecionar, W para mover, E para girar, R para escalar, F para enquadrar. |
| **Autodesk 3ds Max** | `3ds-max.toml` | Q para selecionar, W/E/R para transformações, Z para enquadrar objeto selecionado. |
| **Cinema 4D** | `cinema-4d.toml` | E para mover, R para girar, T para escalar, espaço para alternar última ferramenta. |

---

## 2. Detecção e Alerta de Conflitos

O motor de keybinds possui um analisador que detecta automaticamente:
- Dois comandos distintos mapeados para a mesmíssima combinação de teclas.
- Colisão entre teclas modificadoras (`Ctrl`, `Shift`, `Alt`).

Quando um conflito é detectado em qualquer perfil carregado, o modal de Configurações exibe um alerta visual de advertência (`⚠️ Conflitos Detectados`) listando as ações concorrentes para que o usuário possa remapear ou alternar de perfil.

---

## 3. Extensibilidade

Novos perfis de atalhos podem ser adicionados simplesmente criando um novo arquivo `.toml` no diretório `assets/keymaps/`. A aplicação faz a varredura dinâmica dos perfis ao inicializar e no painel de configurações.
