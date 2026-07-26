---
title: Introdução
---

# Contribuindo com o Yomika

Obrigado pelo seu interesse em contribuir com o Yomika. Estamos construindo um tradutor de mangá local-first, movido a ML, com backend em Rust e interface Tauri/Next.js — e adoraríamos a sua ajuda.

## Início Rápido

A forma mais rápida de começar é pelas nossas [good first issues](https://github.com/proxlavee/yomika/contribute). São tarefas selecionadas para quem está contribuindo pela primeira vez.

Precisa de orientação? Abra uma [GitHub Discussion](https://github.com/proxlavee/yomika/discussions) ou pergunte na issue relevante.

## Formas de Contribuir

Qualquer forma de contribuição é bem-vinda.

### Relatos de Bugs

- Falhas na pipeline de detecção, OCR, inpainting ou tradução
- Crashes, regressões e quedas de performance
- Casos de borda em renderização, exportação PSD ou integração com provedores

### Desenvolvimento de Funcionalidades

- Novos backends de OCR, detecção, inpainting ou LLM
- Melhorias no renderizador de texto, na API HTTP ou no servidor MCP
- Expansão da UI com painéis, atalhos e fluxos novos

### Documentação

- Melhorar guias de primeiros passos e How-Tos
- Adicionar exemplos, screenshots e tutoriais curtos
- Traduzir conteúdo para outras línguas

### Testes

- Testes unitários em Rust para as crates do workspace
- Ampliar os testes Vitest em `ui/tests/` e os testes de integração Rust em `tests/integration-tests/`
- Contribuir com páginas reais de mangá para OCR e detecção

### Infraestrutura

- Melhorias em build e CI
- Ajustes em download de modelos, cache de runtime e paths de aceleração
- Manter o empacotamento saudável em Windows, macOS e Linux

## Entendendo o Código

O Yomika é um workspace Rust com shell Tauri e UI em Next.js:

- **`crates/yomika/`** — shell desktop Tauri
- **`crates/yomika-app/`** — backend da aplicação e orquestração da pipeline
- **`crates/yomika-core/`** — tipos, eventos e utilitários compartilhados
- **`crates/yomika-ml/`** — detecção, OCR, inpainting e análise de fontes
- **`crates/yomika-llm/`** — bindings para llama.cpp e provedores de LLM
- **`crates/yomika-renderer/`** — shaping e renderização de texto
- **`crates/yomika-psd/`** — exportação PSD em camadas
- **`crates/yomika-rpc/`** — API HTTP e servidor MCP
- **`crates/yomika-runtime/`** — gerência de runtime e download de modelos
- **`ui/`** — UI Web em Next.js
- **`tests/integration-tests/`** — testes de integração HTTP e da aplicação em Rust
- **`ui/tests/`** — testes unitários da interface e do frontend com Vitest
- **`docs/`** — site de documentação (English, 日本語, 简体中文, Português)

## Sua Primeira Contribuição

1. **Explore issues.** Procure pela label [`good first issue`](https://github.com/proxlavee/yomika/labels/good%20first%20issue).
2. **Faça perguntas.** Peça esclarecimento na issue ou no GitHub Discussions.
3. **Comece pequeno.** Ajustes em docs e correções pontuais são os mais fáceis de entrar.
4. **Leia o código.** Siga os padrões já presentes no arquivo que você está editando.

## Comunidade

### Canais de Comunicação

- **[GitHub Discussions](https://github.com/proxlavee/yomika/discussions)** — discussões de design e dúvidas
- **[GitHub Issues](https://github.com/proxlavee/yomika/issues)** — relatos de bugs e pedidos de funcionalidades

### Política de Uso de IA

Ao usar ferramentas de IA (LLMs como ChatGPT, Claude, Copilot, etc.) para contribuir com o Yomika:

- **Por favor, informe o uso de IA** para reduzir a fadiga dos mantenedores
- **Você é responsável** por todas as issues ou PRs gerados com IA que enviar
- **Envios de baixa qualidade ou sem revisão podem ser fechados imediatamente.** Cada pessoa é responsável por entender e validar todas as mudanças que envia.

Incentivamos o uso de IA como apoio, mas toda contribuição precisa ser revisada e testada pelo contribuidor antes de ser enviada. Código gerado por IA deve ser compreendido, validado e adaptado ao padrão do Yomika.

## Próximos Passos

Pronto para contribuir? Pontos de partida:

- **Configurar ambiente** — veja [Primeiros Passos](development.md)
- **Encontrar uma issue** — navegue pelas [good first issues](https://github.com/proxlavee/yomika/contribute)
- **Discutir uma ideia** — abra uma [GitHub Discussion](https://github.com/proxlavee/yomika/discussions)
- **Conhecer a pipeline** — leia [Como o Yomika Funciona](../explanation/how-yomika-works.md) e o [Mergulho Técnico](../explanation/technical-deep-dive.md)
