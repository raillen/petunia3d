# Contrato de Segurança e Modelo de Confiança — Petunia3D

Este documento estabelece as **obrigações de segurança, limites de confiança (trust boundaries), modelo de permissões e estratégia de recuperação (recovery)** do Petunia3D.

---

## 1. Trust Boundaries (Fronteiras de Confiança)

O Petunia3D é uma aplicação desktop nativa que consome arquivos do sistema local. Todas as fontes externas são consideradas **não-confiáveis**:

- **Arquivos OBJ / MTL / glTF**: Arquivos 3D externos podem conter índices fora de limites (out-of-bounds), polígonos não planares, coordenadas com NaN ou infinitos, ou vetores maliciosos para exploração de buffer overflow. O parser em `petunia_mesh::obj` higieniza e valida todas as faces antes da criação de structs de malha, descartando primitivas degeneradas sem acionar pânico (`panic!`).
- **Arquivos de Projeto `.petunia`**: Arquivos binários salvos passam por validação estrita via `Project::validate` e `Mesh::validate` ao carregar, verificando correspondência entre índices UV, contagem de vértices e integridade de UUIDs.
- **Imagens e Texturas (PNG/JPEG)**: Decodificadores de imagem operam com dimensões limitadas para evitar ataques de esgotamento de memória (decompression bombs). O canvas albedo aplica clamp estrito de coordenadas em `Canvas::validate`.

---

## 2. Permission Model (Modelo de Permissões e Menor Privilégio)

- **Acesso ao Sistema de Arquivos**: O aplicativo apenas lê e escreve em arquivos expressamente indicados pelo usuário através de diálogos de abertura/salvamento ou argumentos CLI.
- **Sem Conexões de Rede Silenciosas**: Petunia3D opera 100% offline. Nenhuma telemetria, rastreador analítico ou conexão de rede externa é estabelecida pelo núcleo do modelador.
- **Subprocessos**: Não há execução arbitrária de comandos de shell (`std::process::Command` não interpolado).

---

## 3. Secrets Model (Modelo de Segredos)

- O projeto não armazena tokens de API, credenciais de banco ou segredos em texto puro.
- Scripts de desenvolvimento e verificação executam varredura automatizada contra inclusão acidental de chaves em commits.

---

## 4. Recovery Strategy (Estratégia de Recuperação e Resiliência)

- **Recuperação de Falhas de Backend Gráfico (GPU Driver Crash / EGL Failure)**: Se a inicialização do driver `wgpu` falhar (comum em GPUs antigas ou ambientes headless/WSL), o sistema executa fallback limpo para o backend `glow` OpenGL 3.3 Core sem crash da aplicação.
- **Transações Atômicas de Salvamento**: Ao salvar arquivos `.petunia` ou exportar `.glb`, a escrita é realizada em arquivo temporário com substituição atômica (`rename`), prevenindo corrupção de projetos em caso de falta de energia ou interrupção repentina.
- **Snapshots de Desfazer/Refazer (Undo/Redo)**: O gerenciador de comandos registra checkpoints antes de cada mutação estrutural da malha. Em caso de operação inválida, o estado anterior é restaurado de forma determinística.
