# Projetos & Arquivos (.petunia)

O formato de arquivo `.petunia` foi desenvolvido com foco em durabilidade, legibilidade e portabilidade de dados em qualquer sistema operacional.

---

## 1. Estrutura do Arquivo

O arquivo `.petunia` utiliza um envelope JSON estruturado e altamente otimizado contendo:
- `version`: Inteiro semântico que determina a versão do schema de dados (ex: `3`);
- `assets`: Lista de malhas (`Mesh`), contendo listas contíguas de vértices `[x, y, z]`, normais, coordenadas UV, cores RGBA e índices poligonais de faces;
- `collections`: Estrutura em árvore de pastas e coleções definidas no Outliner, associando os IDs dos objetos;
- `annotations`: Coleção de traços 3D com coordenadas espaciais, cores, espessuras e matrizes de transformação de cada item;
- `measurements`: Lista de réguas de medição métrica com pontos de início, fim e deltas calculados;
- `settings`: Configurações de iluminação, cor de fundo da cena e posição de câmera.

---

## 2. Compatibilidade e Migração

O Petunia3D garante compatibilidade retroativa com arquivos gerados em versões anteriores. Ao abrir um arquivo de versão anterior, a camada de carregamento (`crates/project/src/io.rs`) aplica migrações automáticas de esquema de forma transparente.
