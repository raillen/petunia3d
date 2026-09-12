# Contratos de Documentação (`docs/contracts/`)

## O que é este diretório?
Contém as amarrações formais (bindings) entre a documentação canônica deste repositório e os contratos padronizados do framework Prumo (M5 Documentation Architecture).

## Para que serve?
Permite que ferramentas automatizadas (`prumo docs audit`, `prumo docs readiness`, `prumo docs delta`) validem a integridade semântica da documentação contra regras contratuais, assegurando que áreas vitais como arquitetura, testes, visão de produto, segurança, instalação, CLI e UI possuam documentação canônica verificada e sem pontas soltas.

## Inventário
- `bindings.json`: Mapeamento canônico dos 8 contratos de documentação aplicáveis aos seus respectivos arquivos de origem, propriedade e autoridade.
