---
title: Referência das ferramentas MCP
---

# Referência das ferramentas MCP

O Yomika expõe ferramentas MCP em:

```text
http://127.0.0.1:<PORT>/mcp
```

O servidor MCP usa o transporte HTTP streamable do `rmcp 1.5` e opera sobre o mesmo estado de projeto, cena e pipeline que a GUI e a API HTTP.

## O que o servidor MCP expõe hoje

A implementação atual expõe deliberadamente uma superfície pequena e de baixo nível, centrada no ciclo de vida do projeto, na camada de histórico e nos jobs de pipeline. Edições granulares passam por `yomika.apply` com um payload `Op`, em vez de ferramentas dedicadas por campo.

Se você precisa de inspeção mais rica (thumbnails de página, camadas de imagem, listas de fontes, snapshots de cena), use a [API HTTP](http-api.md) diretamente. As duas rodam lado a lado na mesma porta e compartilham um único estado in-process.

## Ferramentas

| Ferramenta              | Finalidade                                              | Parâmetros                                                                       |
| ----------------------- | ------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `yomika.apply`          | aplica um `Op` à cena ativa                             | `op` — valor `Op` com tag JSON                                                   |
| `yomika.undo`           | reverte a op mais recente                               | nenhum                                                                           |
| `yomika.redo`           | reaplica a op mais recente desfeita                     | nenhum                                                                           |
| `yomika.open_project`   | abre ou cria um diretório de projeto Yomika             | `path`, `createName` opcional                                                    |
| `yomika.close_project`  | fecha o projeto ativo                                   | nenhum                                                                           |
| `yomika.start_pipeline` | inicia uma execução de pipeline; retorna um `jobId`     | `steps[]`, `pages[]` opcional, `targetLanguage`, `systemPrompt`, `defaultFont`   |

### `yomika.apply`

Aplica uma única mutação à cena, passando pela camada de histórico. O valor de `op` é o mesmo enum `Op` com tag JSON que a API HTTP aceita em `POST /history/apply` — variantes comuns incluem `AddPage`, `RemovePage`, `AddNode`, `UpdateNode`, `RemoveNode` e `Batch`.

Retorna `{ epoch }` — o novo epoch da cena após a op ser aplicada.

### `yomika.undo` / `yomika.redo`

Andam um passo na pilha de histórico em qualquer direção. Ambas retornam `{ epoch }`, onde `epoch` é `null` num limite da pilha (nada mais para desfazer ou refazer).

### `yomika.open_project`

Abre um diretório de projeto existente ou cria um no caminho informado. Passe `createName` para criar um novo projeto sob o caminho; omita para abrir o que já estiver lá.

Retorna `{ name, path }` para a sessão agora ativa.

### `yomika.close_project`

Fecha a sessão atual. Chamadas seguintes que exigem um projeto retornam um erro `invalid request` até que outro projeto seja aberto.

### `yomika.start_pipeline`

Dispara uma execução de pipeline em background. `steps` é uma lista ordenada de ids de engines registradas no `Registry` do pipeline (validados contra `GET /api/v1/engines`). Omita `pages` para rodar em todas as páginas do projeto; passe uma lista de `PageId`s para limitar a execução a um subconjunto.

Retorna `{ jobId }` imediatamente. O progresso e a conclusão são publicados no stream HTTP `/events` como `JobStarted`, `JobProgress`, `JobWarning` e `JobFinished`. O próprio transporte MCP não faz streaming do progresso do job — para isso, você acompanha o SSE.

## Fluxo de agente sugerido

A maioria das sessões de agente segue mais ou menos isto:

1. `yomika.open_project` — aponta para um diretório de projeto gerenciado
2. lê `GET /api/v1/scene.json` via HTTP para inspecionar a cena
3. seja:
    - aplique edições pontuais via `yomika.apply` com payloads `Op` explícitos, ou
    - rode um pipeline ponta a ponta via `yomika.start_pipeline` e acompanhe `GET /api/v1/events`
4. exporte via `POST /api/v1/projects/current/export` por HTTP
5. `yomika.close_project`

`yomika.undo` e `yomika.redo` são úteis quando uma op se mostra errada e você quer voltar atrás em vez de calcular o inverso na mão.

## Páginas relacionadas

- [Configurar clientes MCP](../how-to/configure-mcp-clients.md)
- [Executar modos GUI, Headless e MCP](../how-to/run-gui-headless-and-mcp.md)
- [Referência da API HTTP](http-api.md)
