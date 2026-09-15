import { defineConfig } from 'vitepress';
import { withMermaid } from 'vitepress-plugin-mermaid';
import bibleSidebar from './bibleSidebar';
import modernizationSidebar from './modernizationSidebar';

export default withMermaid(
  defineConfig({
    title: 'Petunia3D',
    description: 'Documentação Oficial e Manual do Usuário — Modelador 3D Low-Poly',
    base: process.env.GITHUB_PAGES ? '/simple3d-modeling/' : '/',
    cleanUrls: true,
    lastUpdated: true,
    ignoreDeadLinks: true,
    srcExclude: [
      '**/image-references/**',
      '**/contracts/**',
      '**/GAUNTLET*.md',
      '**/README.md',
    ],
    head: [
      ['link', { rel: 'icon', type: 'image/svg+xml', href: '/assets/logo.svg' }],
      ['meta', { name: 'theme-color', content: '#4772b3' }],
    ],
    themeConfig: {
      logo: '/assets/logo.svg',
      siteTitle: 'Petunia3D',
      nav: [
        { text: 'Início', link: '/' },
        { text: 'Começando', link: '/getting-started/' },
        { text: 'Manual', link: '/manual/' },
        { text: 'Ferramentas', link: '/tools/' },
        { text: 'Bíblia (SSOT)', link: '/bible/' },
        { text: 'Modernização', link: '/modernization/' },
        { text: 'Personalização', link: '/customization/' },
        { text: 'Atalhos', link: '/shortcuts/' },
        { text: 'Desenvolvedores', link: '/developers/' },
        { text: 'Changelog', link: '/changelog/' },
      ],
      sidebar: {
        '/bible/': bibleSidebar,
        '/modernization/': modernizationSidebar,
        '/getting-started/': [
          {
            text: 'Primeiros Passos',
            items: [
              { text: 'Visão Geral', link: '/getting-started/' },
              { text: 'O que é o Petunia3D?', link: '/getting-started/what-is-petunia3d' },
              { text: 'Instalação e Requisitos', link: '/getting-started/installation' },
              { text: 'Seu Primeiro Projeto', link: '/getting-started/first-project' },
              { text: 'Tour da Interface', link: '/getting-started/interface-overview' },
              { text: 'Seu Primeiro Modelo (15 min)', link: '/getting-started/your-first-model' },
            ],
          },
        ],
        '/manual/': [
          {
            text: 'Manual do Usuário',
            items: [
              { text: 'Introdução ao Manual', link: '/manual/' },
              { text: 'Interface & Docking', link: '/manual/interface' },
              { text: 'Viewport 3D & Câmera', link: '/manual/viewport' },
              { text: 'Modos de Seleção', link: '/manual/selection' },
              { text: 'Fluxo de Modelagem', link: '/manual/modeling' },
              { text: 'Primitivas & Criação', link: '/manual/primitives' },
              { text: 'Pintura & Cores', link: '/manual/paint' },
              { text: 'Mapeamento UV', link: '/manual/uv' },
              { text: 'Linha do Tempo', link: '/manual/animation' },
              { text: 'Biblioteca de Assets', link: '/manual/asset-library' },
              { text: 'Projetos (.petunia)', link: '/manual/projects' },
              { text: 'Exportação & Formatos', link: '/manual/export' },
            ],
          },
          {
            text: 'Workspaces',
            items: [
              { text: 'Visão Geral dos Espaços', link: '/workspaces/' },
              { text: 'Modeling', link: '/workspaces/modeling' },
              { text: 'Paint', link: '/workspaces/paint' },
              { text: 'UV', link: '/workspaces/uv' },
              { text: 'Animation', link: '/workspaces/animation' },
            ],
          },
        ],
        '/workspaces/': [
          {
            text: 'Workspaces',
            items: [
              { text: 'Visão Geral dos Espaços', link: '/workspaces/' },
              { text: 'Modeling', link: '/workspaces/modeling' },
              { text: 'Paint', link: '/workspaces/paint' },
              { text: 'UV', link: '/workspaces/uv' },
              { text: 'Animation', link: '/workspaces/animation' },
            ],
          },
        ],
        '/tools/': [
          {
            text: 'Catálogo de Ferramentas',
            items: [
              { text: 'Todas as Ferramentas', link: '/tools/' },
              { text: 'Seleção (Select)', link: '/tools/select' },
              { text: 'Translação (Move)', link: '/tools/move' },
              { text: 'Rotação (Rotate)', link: '/tools/rotate' },
              { text: 'Escala (Scale)', link: '/tools/scale' },
              { text: 'Gizmo Combinado (Transform)', link: '/tools/transform' },
              { text: 'Extrusão (Extrude)', link: '/tools/extrude' },
              { text: 'Inserção (Inset)', link: '/tools/inset' },
              { text: 'Chanfro (Bevel)', link: '/tools/bevel' },
              { text: 'Corte em Anel (Loop Cut)', link: '/tools/loop-cut' },
              { text: 'Faca Topológica (Knife)', link: '/tools/knife' },
              { text: 'Régua & Medição 3D (Measure)', link: '/tools/measure' },
              { text: 'Rascunho 3D (Annotate)', link: '/tools/annotate' },
              { text: 'Adicionar Primitivas', link: '/tools/primitives' },
              { text: 'Operações Avançadas', link: '/tools/advanced' },
            ],
          },
        ],
        '/customization/': [
          {
            text: 'Central de Personalização',
            items: [
              { text: 'Visão Geral', link: '/customization/' },
              { text: 'Sistema de Temas TOML', link: '/customization/themes' },
              { text: 'Pacotes de Ícones', link: '/customization/icons' },
              { text: 'Traduções & i18n', link: '/customization/translations' },
              { text: 'Mapeamento de Teclado', link: '/customization/keymaps' },
            ],
          },
        ],
        '/shortcuts/': [
          {
            text: 'Atalhos de Teclado',
            items: [
              { text: 'Tabela de Perfis Canônicos', link: '/shortcuts/' },
            ],
          },
        ],
        '/tutorials/': [
          {
            text: 'Tutoriais Práticos',
            items: [
              { text: 'Índice de Tutoriais', link: '/tutorials/' },
              { text: 'Criando um Prop Low-Poly', link: '/tutorials/low-poly-prop' },
              { text: 'Construindo um Caixote', link: '/tutorials/create-a-crate' },
              { text: 'Criando um Tema Customizado', link: '/tutorials/custom-theme' },
              { text: 'Criando um Pacote de Ícones', link: '/tutorials/custom-icon-pack' },
            ],
          },
        ],
        '/reference/': [
          {
            text: 'Referência Técnica',
            items: [
              { text: 'Índice de Referência', link: '/reference/' },
              { text: 'Matriz de Formatos 3D', link: '/reference/formats' },
              { text: 'Linha de Comando (CLI)', link: '/reference/cli' },
              { text: 'Glossário Técnico', link: '/reference/glossary' },
              { text: 'Diagnóstico & GPU', link: '/reference/diagnostics' },
              { text: 'Perguntas Frequentes (FAQ)', link: '/faq' },
              { text: 'Solução de Problemas', link: '/troubleshooting/' },
            ],
          },
          {
            text: 'Dados Gerados do Código',
            items: [
              { text: 'Comandos & Ferramentas', link: '/generated/COMMANDS' },
              { text: 'Perfis de Teclado', link: '/generated/KEYBINDS' },
              { text: 'Tokens de Ícones', link: '/generated/ICON_TOKENS' },
              { text: 'Tokens de Tradução', link: '/generated/TEXT_TOKENS' },
              { text: 'Tokens de Temas', link: '/generated/THEME_TOKENS' },
              { text: 'Formatos Suportados', link: '/generated/SUPPORTED_FORMATS' },
            ],
          },
        ],
        '/troubleshooting/': [
          {
            text: 'Solução de Problemas',
            items: [
              { text: 'Guia de Diagnóstico', link: '/troubleshooting/' },
              { text: 'Perguntas Frequentes (FAQ)', link: '/faq' },
            ],
          },
        ],
        '/developers/': [
          {
            text: 'Desenvolvedores',
            items: [
              { text: 'Portal do Desenvolvedor', link: '/developers/' },
              { text: 'Arquitetura de 17 Crates', link: '/developers/architecture' },
              { text: 'Compilação & Build', link: '/developers/building' },
              { text: 'Arquitetura de UI egui', link: '/developers/ui-architecture' },
              { text: 'Pipeline de Renderização', link: '/developers/rendering' },
              { text: 'Sistema de Comandos (Undo/Redo)', link: '/developers/command-system' },
              { text: 'Estratégia de Testes', link: '/developers/testing' },
              { text: 'Registros de Decisões (ADRs)', link: '/developers/adr/' },
              { text: 'Bíblia de Implementação (P3D)', link: '/bible/' },
            ],
          },
          {
            text: 'Contribuição',
            items: [
              { text: 'Diretrizes de Contribuição', link: '/contributing/guidelines' },
              { text: 'Como Adicionar uma Ferramenta', link: '/contributing/adding-a-tool' },
              { text: 'Política de Documentação', link: '/contributing/documentation-policy' },
            ],
          },
        ],
        '/contributing/': [
          {
            text: 'Contribuição',
            items: [
              { text: 'Diretrizes de Contribuição', link: '/contributing/guidelines' },
              { text: 'Como Adicionar uma Ferramenta', link: '/contributing/adding-a-tool' },
              { text: 'Política de Documentação', link: '/contributing/documentation-policy' },
            ],
          },
        ],
        '/changelog/': [
          {
            text: 'Histórico & Releases',
            items: [
              { text: 'Changelog Completo', link: '/changelog/' },
              { text: 'Releases & Destaques', link: '/releases/' },
              { text: 'v0.6.0 (Atual)', link: '/releases/v0.6.0' },
              { text: 'v0.5.0', link: '/releases/v0.5.0' },
              { text: 'v0.4.0', link: '/releases/v0.4.0' },
            ],
          },
        ],
        '/releases/': [
          {
            text: 'Histórico & Releases',
            items: [
              { text: 'Changelog Completo', link: '/changelog/' },
              { text: 'Releases & Destaques', link: '/releases/' },
              { text: 'v0.6.0 (Atual)', link: '/releases/v0.6.0' },
              { text: 'v0.5.0', link: '/releases/v0.5.0' },
              { text: 'v0.4.0', link: '/releases/v0.4.0' },
            ],
          },
        ],
        '/generated/': [
          {
            text: 'Dados Gerados do Código',
            items: [
              { text: 'Comandos & Ferramentas', link: '/generated/COMMANDS' },
              { text: 'Perfis de Teclado', link: '/generated/KEYBINDS' },
              { text: 'Tokens de Ícones', link: '/generated/ICON_TOKENS' },
              { text: 'Tokens de Tradução', link: '/generated/TEXT_TOKENS' },
              { text: 'Tokens de Temas', link: '/generated/THEME_TOKENS' },
              { text: 'Formatos Suportados', link: '/generated/SUPPORTED_FORMATS' },
            ],
          },
        ],
      },
      search: {
        provider: 'local',
        options: {
          locales: {
            root: {
              translations: {
                button: {
                  buttonText: 'Pesquisar documentação...',
                  buttonAriaLabel: 'Pesquisar documentação',
                },
                modal: {
                  noResultsText: 'Nenhum resultado encontrado para',
                  resetButtonTitle: 'Limpar pesquisa',
                  footer: {
                    selectText: 'para selecionar',
                    navigateText: 'para navegar',
                    closeText: 'para fechar',
                  },
                },
              },
            },
          },
        },
      },
      socialLinks: [
        { icon: 'github', link: 'https://github.com/raillen/simple3d-modeling' },
      ],
      editLink: {
        pattern: 'https://github.com/raillen/simple3d-modeling/edit/main/docs/:path',
        text: 'Editar esta página no GitHub',
      },
      footer: {
        message: 'Distribuído sob licença MIT.',
        copyright: 'Copyright © 2026 Petunia3D Team',
      },
      lastUpdated: {
        text: 'Última atualização',
      },
      docFooter: {
        prev: 'Página anterior',
        next: 'Próxima página',
      },
    },
  })
);
