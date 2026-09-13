# Tradução e Idiomas (i18n)

Todas as strings textuais, rótulos de botões, títulos de janelas e mensagens de ajuda do Petunia3D são externalizadas em arquivos TOML de internacionalização.

## Idiomas Oficiais
- **`pt-BR`**: Português do Brasil (nativo completo);
- **`en`**: Inglês (English).

## Adicionando um Novo Idioma

Para adicionar suporte ao seu idioma (ex: Espanhol `es-ES` ou Francês `fr-FR`):
1. Crie o arquivo `i18n/es-ES.toml`;
2. Traduza as chaves mantendo a estrutura de sessões:

```toml
[general]
app_title = "Petunia3D"
unsaved = "Sin guardar"
saved = "Guardado"

[menu]
file = "Archivo"
edit = "Editar"
help = "Ayuda"

[tools]
select = "Seleccionar"
move = "Mover"
rotate = "Rotar"
scale = "Escalar"
extrude = "Extrusión"
```
3. O Petunia3D carregará o arquivo automaticamente ao selecioná-lo nas preferências de Idioma.
