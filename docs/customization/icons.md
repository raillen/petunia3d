# Pacotes de Ícones

Troque em **Settings → Icons** sem reiniciar. O pacote muda o chrome
(utilitários, menus, viewport); ferramentas de domínio mantêm arte própria
para não perder significado.

## Pacotes

- **Petunia** (padrão): arte própria + utilitários.
- **Lucide**, **Iconoir**: genéricos, sempre compilados.
- **Tabler**, **Phosphor**: via feature `extended-icon-packs` (exige mais RAM).

## Fallbacks

Todo ícone resolve por cadeia: glifo do pacote → Phosphor (sempre instalado)
→ losango vetorial. Nenhum controle fica invisível ou vira tofu silencioso.

Pacotes extras podem ser descobertos via manifests (o cache atualiza pelo
file watcher); detalhes de criação em [Criando um Pacote de Ícones](../tutorials/custom-icon-pack.md).
