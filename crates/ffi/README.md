# petunia_ffi

**Camada C-ABI / FFI para Integração com Frontends Externos (Gauntlet G10).**

O crate `petunia_ffi` fornece uma interface de funções em C (`extern "C"`) compatível com ABI padrão C para permitir a integração do núcleo de modelagem 3D do **Petunia3D** com clientes e interfaces externas, tais como:
- **C++**: Aplicações Qt, ImGui customizado, Unreal Engine, visualizadores proprietários.
- **C#**: Unity, Godot (.NET), Avalonia, WPF, MAUI.
- **Python**: Automações via `ctypes` ou `cffi`, plugins Blender, pipelines de processamento geométrico.
- **Go**: Serviços web e ferramentas CLI utilizando `cgo`.

---

## 🚀 Arquitetura e Características

1. **Memória Segura e Opaque Pointers**: Todas as sessões operam sob um ponteiro opaco `PetuniaContext*` alocado e liberado estritamente pela biblioteca (`petunia_context_create`, `petunia_context_destroy`).
2. **Zero Dependência de UI**: O crate não depende de `egui`, mantendo footprint mínimo e tempo de linkagem ultrarrápido.
3. **Códigos de Erro Padronizados**: Nenhuma chamada entra em pânico através da fronteira FFI. Erros retornam inteiros negativos (`PETUNIA_ERR_*`) e preenchem uma mensagem descritiva recuperável via `petunia_last_error_message`.
4. **Interoperabilidade com DTOs (Gauntlet G9)**: Consultas completas de cena, seleção e ferramentas podem ser serializadas diretamente para JSON para consumo universal por linguagens com parsing dinâmico.
5. **Manipulação por Identificador Estável (`Uuid`)**: Suporte nativo à ativação e remoção de malhas por UUIDs canônicos em formato string.

---

## 📋 Tabela de Códigos de Retorno

| Código | Constante | Descrição |
| :---: | :--- | :--- |
| `0` | `PETUNIA_OK` | Operação executada com sucesso. |
| `-1` | `PETUNIA_ERR_NULL_PTR` | Ponteiro nulo inválido passado como parâmetro. |
| `-2` | `PETUNIA_ERR_INVALID_UTF8` | String fornecida contém bytes que não são UTF-8 válidos. |
| `-3` | `PETUNIA_ERR_OPERATION_FAILED` | Falha interna de execução (ex: I/O, formato inválido, nada para desfazer). |
| `-4` | `PETUNIA_ERR_BUFFER_TOO_SMALL` | Buffer fornecido pelo chamador é insuficiente para armazenar o resultado. |
| `-5` | `PETUNIA_ERR_NOT_FOUND` | Ativo ou entidade solicitada (por UUID ou índice) não existe. |

---

## 🛠️ Exemplos de Uso

### 1. C++
```cpp
#include <iostream>
#include <vector>
#include "include/petunia.h"

int main() {
    PetuniaContext* ctx = petunia_context_create("en");
    if (!ctx) return -1;

    // Adiciona primitivas
    petunia_add_primitive(ctx, "Sphere");
    petunia_add_primitive(ctx, "Cylinder8");

    // Consulta contagem e totais
    uint32_t verts = 0, faces = 0;
    petunia_get_scene_summary(ctx, &verts, &faces);
    std::cout << "Cena: " << verts << " vertices, " << faces << " faces.\n";

    // Exporta cena completa para GLB binário
    petunia_export_glb(ctx, "output_scene.glb");

    petunia_context_destroy(ctx);
    return 0;
}
```

### 2. Python (`ctypes`)
```python
import ctypes

lib = ctypes.CDLL("./target/debug/libpetunia_ffi.so")

# Configura assinaturas
lib.petunia_context_create.restype = ctypes.c_void_p
lib.petunia_context_destroy.argtypes = [ctypes.c_void_p]
lib.petunia_add_primitive.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
lib.petunia_add_primitive.restype = ctypes.c_int32
lib.petunia_get_asset_count.argtypes = [ctypes.c_void_p]
lib.petunia_get_asset_count.restype = ctypes.c_int32

# Execução
ctx = lib.petunia_context_create(None)
lib.petunia_add_primitive(ctx, b"Capsule")
print("Total de assets:", lib.petunia_get_asset_count(ctx))
lib.petunia_context_destroy(ctx)
```

---

## 🧪 Testes de Conformance

Os testes automatizados cobrem o ciclo de vida completo:
```bash
cargo test -p petunia_ffi -j 2
```
