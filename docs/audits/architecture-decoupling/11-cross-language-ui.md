# 11 — Integração com Frontends em Outras Linguagens (Cross-Language UI Report)

> **Avaliação técnica de viabilidade para interfaces gráficas em C++, C#, Go, Python e Web, análise de cenários de fronteira e integração de viewport 3D.**

---

## 1. O que Bloqueia Frontends em Outras Linguagens Hoje?

Atualmente, seria **impossível** construir uma interface para o Petunia3D em C++ (Qt), C# (WPF/Avalonia), Go ou TypeScript/Web sem reescrever quase todo o projeto:

1. **Ausência de C-ABI / FFI**:
   * O projeto não expõe nenhuma função `extern "C"`;
   * As estruturas centrais (`AppState`, `Project`, `Mesh`, `Selection`) possuem layouts de memória Rust complexos, com vetores alocados no heap, `Option`, enums com discriminantes dinâmicos e referências com lifetimes, incompatíveis com chamadas binárias diretas de C/C++.
2. **Dependência de Índices `usize` Efêmeros**:
   * A seleção de objetos e navegação de assets usa `usize` como índice em `Vec<Asset>`.
   * Se um objeto é deletado no núcleo, todos os índices subsequentes são invalidados, o que quebra bindings externos que não compartilhem a mesma memória em Rust.
3. **Ausência de DTOs e Protocolo de Serialização de Mensagens**:
   * Não existe um formato neutro de mensagens (JSON-RPC, Protobuf ou FlatBuffers) para representar comandos e eventos entre processos distintos.
4. **Acoplamento do Loop de Janela**:
   * O loop principal do aplicativo é posse do `winit` dentro de `petunia_app`. Em frameworks como Qt ou WPF, o loop de eventos pertence ao framework gráfico (ex: `QApplication::exec()`), exigindo que o core do Petunia3D seja uma biblioteca subordinada (callee), e não o dono do processo (caller).

---

## 2. Comparativo de Cenários de Fronteira Futura

Para orientar o planejamento estratégico, foram avaliados quatro cenários de arquitetura para frontends externos:

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        FRONTENDS ALTERNATIVOS                          │
│                                                                        │
│   [C++ / Qt]       [C# / Avalonia]      [Go / Fyne]     [Web / Electron]│
└───────┬───────────────────┬──────────────────┬──────────────────┬──────┘
        │                   │                  │                  │
        ▼                   ▼                  ▼                  ▼
┌────────────────────────────────────────────────────────────────────────┐
│                CAMADA FFI / C-ABI ESTÁVEL (extern "C")                 │
│                                                                        │
│  petunia_session_create()         petunia_dispatch_command()           │
│  petunia_session_destroy()        petunia_query_scene()                │
│  petunia_viewport_attach()        petunia_subscribe_events()           │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   APPLICATION API (Rust Soberana)                      │
│                                                                        │
│   CommandDispatcher │ Queries │ EventBus │ EditorSession │ DTOs        │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     PETUNIA CORE & PETUNIA MESH                        │
└────────────────────────────────────────────────────────────────────────┘
```

| Critério de Engenharia | Cenário A: Rust Nativo (Slint/Iced) | Cenário B: C-ABI / Same-Process (C++/C#/Go) | Cenário C: IPC Multi-Processo (JSON-RPC/Protobuf) | Cenário D: Híbrido Recomendado |
| :--- | :--- | :--- | :--- | :--- |
| **Mecanismo de Comunicação** | Chamadas diretas em Rust via traits | Ponte `extern "C"` com ponteiros opacos e callbacks | Sockets de domínio ou stdin/stdout com IPC | Direct API para Rust; C-ABI para C++/C# |
| **Latência de Comandos** | **Zero** (< 1 µs) | **Desprezível** (< 2 µs) | **Média** (0.5 a 3 ms por serialização) | **Zero / Desprezível** |
| **Desempenho da Viewport 3D**| Nativo a 60+ FPS via RawWindowHandle | Nativo a 60+ FPS via RawWindowHandle | Requer memória compartilhada de GPU (complexo) | Nativo a 60+ FPS |
| **Complexidade de Debugging** | Muito baixa (StackTrace unificado) | Baixa a Média (GDB/LLDB inter-linguagem) | Alta (dois processos sincronizados) | Equilibrada |
| **Facilidade de Distribuição** | Binário estático único | Biblioteca compartilhada (.so / .dll) + Executável | Dois binários em execução orquestrada | Flexível |

---

## 3. O Desafio Técnico do Viewport 3D em Frontends Externos

Enquanto sincronizar uma árvore de Outliner ou sliders de propriedades via FFI é trivial, a **renderização do Viewport 3D a 60 FPS** exige uma solução de alto desempenho:

* **Abordagem Recomendada: Ligação por Handle de Janela Nativo (`RawWindowHandle`)**:
  1. O framework externo (ex: Qt via `QWindow::winId()` ou C# via `HwndHost`) aloca uma área nativa na tela;
  2. O frontend passa esse identificador de janela do SO (`HWND` no Windows, `NSView` no macOS, `X11 Window / Wayland Surface` no Linux) para a função C-ABI:
     ```c
     petunia_viewport_attach_surface(session, native_window_handle, width, height);
     ```
  3. O `petunia_render_wgpu` cria uma superfície gráfica nativa vinculada a esse handle e renderiza diretamente via GPU no mesmo processo, sem nenhuma cópia de pixels de memória.

---

## 4. Regra de Ouro: Primeiro Desacoplar, Depois Estabilizar a FFI

> [!WARNING]
> **NÃO SE DEVE COMEÇAR A REFATORAÇÃO ESCREVENDO UMA C-ABI/FFI.**  
> Tentar escrever bindings C hoje sobre o código atual apenas exporia o God Object `AppState` e as mutações ad-hoc.
>
> A ordem correta e segura é:
> 1. Desacoplar o núcleo do `egui` internamente em Rust;
> 2. Criar a camada de aplicação semântica (`CommandDispatcher`, `EditorSession`, DTOs);
> 3. Validar a arquitetura com testes headless em Rust;
> 4. Somente então empacotar a API de aplicação em uma C-ABI estável (`extern "C"`).
