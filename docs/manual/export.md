# Exportação & Formatos

O Petunia3D oferece exportação nativa direta para formatos padrões da indústria, garantindo integração perfeita com game engines modernas como Godot, Unity, Unreal Engine e ferramentas de renderização como o Blender.

---

## 1. Formatos Suportados

| Formato | Extensão | Principais Aplicações | Recursos Preservados |
| :--- | :--- | :--- | :--- |
| **glTF 2.0 Binary** | `.glb` | **Recomendado para Godot, Unity, Unreal, Web** | Malhas, cores de vértices, coordenadas UV, materiais e hierarquia de nós. |
| **Wavefront OBJ** | `.obj` + `.mtl` | Compatibilidade universal e impressão 3D | Vértices, normais, coordenadas UV e grupos de faces. |
| **Petunia Nativo** | `.petunia` | Arquivo do projeto para edição contínua | Todos os dados da cena, anotações, medidas, histórico e coleções. |

---

## 2. Como Exportar

1. Acesse o menu superior: **`Arquivo > Exportar`**;
2. Escolha o formato desejado (`Exportar como GLB...` ou `Exportar como OBJ...`);
3. No diálogo nativo do sistema operacional, escolha o diretório e clique em **Salvar**;
4. O arquivo gerado está imediatamente pronto para importação direta no seu motor de jogo favorito sem necessidade de conversão prévia.
