# Exportação & Formatos

O Petunia3D exporta direto para formatos de pipeline de jogos. A prioridade da V1 é **glTF/GLB** (principal) e **OBJ** (secundário); **FBX fica fora da V1** por complexidade/licenciamento/ecossistema. A triangulação do export é determinística.

---

## 1. Formatos Suportados

| Formato | Extensão | Principais Aplicações | Recursos Preservados |
| :--- | :--- | :--- | :--- |
| **glTF 2.0 Binary** | `.glb` | **Formato principal** para motores de jogo e web | Malhas, cores de vértices, coordenadas UV, materiais, **textura Albedo pintada (PNG embutido)** e hierarquia de nós. |
| **Wavefront OBJ** | `.obj` | Formato secundário; compatibilidade universal | Posições, normais, coordenadas UV e grupos de faces (sem materiais/texturas). |
| **Petunia Nativo** | `.petunia` | Arquivo do projeto para edição contínua | Todos os dados da cena, anotações, medidas, histórico e coleções. |

---

## 2. Como Exportar

1. Acesse o menu superior: **`Arquivo > Exportar`**;
2. Escolha o formato desejado (`Exportar como GLB...` ou `Exportar como OBJ...`);
3. No diálogo nativo do sistema operacional, escolha o diretório e clique em **Salvar**;
4. O arquivo gerado está imediatamente pronto para importação no seu motor de jogo, sem conversão prévia.

## 3. Checagem antes de exportar

Game-readiness faz parte do fluxo `CHECK → EXPORT`: triangulação determinística, orientação de faces, faces duplicadas, faces degeneradas, arestas non-manifold e pontos soltos são detectados antes da gravação (P3D-144, pós-GA). A contagem discreta de pontos/faces/triângulos fica sempre visível na status strip.
