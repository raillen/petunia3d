# P3D-048 — Inspector / Properties

<aside>
🧩

Estado: **parcial / visualização ruim sob Outliner** · Prioridade: P0.

</aside>

## Objetivo

Inspector contextual profissional sem ficar permanentemente espremido em uma sidebar estreita.

## Decisões

- permitir resize;
- permitir detach/floating;
- docking controlado ao lado do Outliner quando viável;
- manter layout inicial simples e previsível;
- não introduzir docking irrestrito em toda a aplicação apenas por esta necessidade.

## Arquitetura

Inspector é frontend de properties/commands. Layout/posição são UI settings; valores do objeto pertencem ao editor/project state. Tool Properties ficam em P3D-083, não aqui.

## Dependências

P3D-049, P3D-078, P3D-083, P3D-084.

## Testes / DoD

Docked/floating/resize/close/restore, objetos de tipos diferentes, nenhuma mutação direta de mesh em callback gigante.