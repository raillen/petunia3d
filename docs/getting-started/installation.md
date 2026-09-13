# Instalação e Requisitos

O Petunia3D é distribuído como um binário compilado nativo e portátil, sem dependências de frameworks externos ou runtimes adicionais.

## Requisitos de Sistema

| Componente | Requisito Mínimo | Recomendado |
| :--- | :--- | :--- |
| **Sistema Operacional** | Linux (x86_64/aarch64), Windows 10/11, macOS 12+ | Linux com Wayland/X11 ou Windows 11 |
| **Placa de Vídeo (GPU)** | Suporte a Vulkan 1.1, Metal 2, DirectX 12 ou OpenGL 3.3 | GPU dedicada com suporte a WebGPU / Vulkan |
| **Memória RAM** | 1 GB de RAM livre | 4 GB de RAM |
| **Armazenamento** | 50 MB de espaço em disco | SSD |

## Compilando a partir do Código-Fonte

Se você possui a toolchain [Rust](https://rustup.rs/) instalada, pode compilar e executar o Petunia3D diretamente:

### 1. Clonar o repositório
```bash
git clone https://github.com/raillen/petunia3d.git
cd petunia3d
```

### 2. Dependências do Sistema (Linux)
Em distribuições baseadas em Debian/Ubuntu:
```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev libudev-dev pkg-config libx11-dev libxcursor-dev libxi-dev
```

Em distribuições Arch Linux / Manjaro:
```bash
sudo pacman -S alsa-lib systemd pkgconf libx11 libxcursor libxi
```

### 3. Execução em Desenvolvimento
```bash
cargo run
```

### 4. Compilação Otimizada para Produção
```bash
cargo build --release
```
O binário final estará disponível em `target/release/petunia3d`.

## Verificação do Driver Gráfico
Ao iniciar, o Petunia3D inicializa preferencialmente o backend **WebGPU**. Caso a GPU não possua suporte aos drivers Vulkan/DirectX12 mais recentes, o motor ativa automaticamente o fallback para **OpenGL**, garantindo compatibilidade abrangente.
