# Temas Visuais

O Petunia3D possui 4 temas nativos calibrados para contraste ótimo (WCAG AA) e conforto visual prolongado:

1. **`petunia-dark` (Padrão)**: Fundo escuro canônico (`#121212`), superfícies de painel em cinza escuro neutro (`#202020`) e azul de acento profissional (`#3169e3`).
2. **`petunia-light`**: Tema claro balanceado com fundos suaves para ambientes bem iluminados.
3. **`petunia-capuccino`**: Tons quentes de café, bege e marrom terra para reduzir a fadiga ocular.
4. **`petunia-tokyo-nights`**: Tema estilo cyberpunk com contrastes em roxo neon, ciano e azul escuro profundo.

## Criando seu Próprio Tema (`.toml`)

Você pode criar um novo tema criando um arquivo no diretório de configuração do usuário:

```toml
[theme]
id = "meu-tema-custom"
name = "Meu Tema Personalizado"
author = "Seu Nome"

[colors]
bg_app = "#181824"
bg_header = "#12121a"
bg_panel = "#1f1f2e"
bg_surface = "#2a2a3d"
bg_surface_hover = "#383852"
accent_blue = "#4a7bf5"
accent_green = "#2ecc71"
text_primary = "#f0f0f5"
text_secondary = "#a0a0b5"
```
Ao reiniciar a aplicação ou selecionar o tema na aba **Temas** das Configurações (`Ctrl+,`), as cores são aplicadas em tempo real sem necessidade de recompilar.
