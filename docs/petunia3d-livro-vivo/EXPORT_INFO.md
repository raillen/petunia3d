# Informações da exportação

Snapshot gerado em **2026-09-11** a partir do caderno canônico **Petunia3D — Livro Vivo** no Notion.

## Estrutura

- `README.md`: conteúdo da página raiz + índice navegável.
- `01-...md` a `36-...md`: um arquivo por capítulo.
- `assets/`: diretório reservado para assets locais.

## Conversão

O objetivo deste pacote é funcionar bem em Git/GitHub e com code agents. Por isso:

- texto, headings, listas, Mermaid e code blocks foram preservados;
- tabelas complexas do Notion foram mantidas como HTML compatível com Markdown/GitHub quando apropriado;
- a apresentação visual específica de callouts do Notion foi achatada para Markdown normal/blockquote, preservando o conteúdo;
- links internos do Notion foram reescritos para links relativos entre os arquivos quando o destino pertence a este Livro Vivo;
- links externos permanecem URLs externas;
- assets privados/binários do Notion/Figma não são incorporados automaticamente neste snapshot; referências externas permanecem no texto.

## Fonte canônica

Este pacote é um snapshot portátil. Enquanto o projeto não declarar uma especificação versionada posterior como substituta, o caderno no Notion continua sendo a fonte canônica conforme o contrato documental do próprio Livro Vivo.
