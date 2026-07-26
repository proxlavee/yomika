---
title: Downloads de Runtime e Modelos
---

# Downloads de Runtime e Modelos

O Yomika é local-first, mas o primeiro uso não é totalmente offline. Antes que o pipeline local possa rodar, o Yomika pode precisar baixar arquivos de runtime nativo e pesos de modelos.

Se esses downloads falharem, verifique o caminho de rede primeiro. O Yomika não consegue baixar arquivos de hosts que sua máquina, ISP, firewall, proxy ou região não consigam alcançar.

## O que o Yomika baixa

O Yomika baixa três tipos gerais de artefatos:

- pacotes de runtime nativo, como binários do `llama.cpp`, arquivos de suporte a CUDA em sistemas NVIDIA compatíveis e arquivos do ZLUDA em sistemas AMD compatíveis no Windows
- pacotes de modelos de bootstrap necessários para o pipeline local padrão de páginas
- pacotes de modelos sob demanda, como engines opcionais de OCR ou inpainting e modelos locais de tradução em GGUF que você selecionar depois

O detalhe importante de comportamento é:

- `yomika --download` prepara o runtime de bootstrap e os pacotes de modelos, e então encerra
- ele não baixa todas as engines opcionais nem todas as LLMs locais mostradas no seletor

## Onde os arquivos ficam

O Yomika armazena os pacotes de runtime e os caches de modelos dentro do `Data Path` configurado. Por padrão, esse caminho é o diretório local de dados da aplicação na plataforma, acrescido de `Yomika`.

Exemplos típicos:

- Windows: `%LOCALAPPDATA%\Yomika`
- macOS: `~/Library/Application Support/Yomika`
- Linux: `~/.local/share/Yomika`

Na implementação atual, os subdiretórios importantes são:

```text
<Data Path>/
  config.toml
  runtime/
    .downloads/
    cuda/
    llama.cpp/
    zluda/
  models/
    huggingface/
```

Significado prático:

- `runtime/.downloads` é o cache genérico de arquivos para downloads de runtime nativo
- `runtime/*` contém as bibliotecas extraídas que o Yomika realmente carrega
- `models/huggingface` é o cache do Hugging Face usado para modelos de visão e arquivos locais de modelos GGUF

Nem todos os diretórios existem em todas as plataformas. Por exemplo, `zluda/` é exclusivo do Windows e só importa em configurações AMD compatíveis.

Para o caminho configurado e as configurações de HTTP, veja a [Referência de Settings](../reference/settings.md).

## Como o download de runtime funciona

Quando o Yomika inicia, ou quando você executa `yomika --download`, ele prepara os pacotes de bootstrap para a plataforma atual e a política de computação.

Em alto nível:

1. O Yomika cria os diretórios de runtime e modelos dentro do data path atual.
2. Ele verifica se cada pacote de bootstrap já está atualizado.
3. Se um pacote de runtime nativo estiver ausente ou desatualizado, o Yomika baixa o arquivo para `runtime/.downloads`.
4. Ele extrai os arquivos necessários para um diretório de instalação específico do runtime e grava um marker de instalação.
5. Ele pré-carrega as bibliotecas de runtime antes que o pipeline local comece.

Na árvore de código atual, os downloads de runtime nativo vêm de alguns lugares diferentes dependendo do pacote:

- GitHub releases para `llama.cpp`
- metadados do PyPI e arquivos wheel para partes do runtime CUDA em plataformas compatíveis
- assets de releases upstream para ZLUDA em sistemas AMD compatíveis no Windows

Então, uma falha de download de runtime nem sempre é um problema com o Hugging Face.

## Como o download de modelos funciona

A maioria dos downloads de modelos usa o cache compartilhado do Hugging Face em `models/huggingface`.

Em alto nível:

1. O Yomika solicita um par específico `repo/file`.
2. Ele verifica primeiro o cache local do Hugging Face.
3. Se o arquivo já estiver em cache, o Yomika o reutiliza imediatamente.
4. Caso contrário, o Yomika baixa exatamente aquele arquivo e o armazena no layout do cache do Hugging Face.
5. Carregamentos posteriores reutilizam o arquivo em cache em vez de baixá-lo novamente.

Isso vale tanto para a stack padrão de visão quanto para os modelos locais de tradução em GGUF que o Yomika baixa sob demanda.

## O que é o Hugging Face

O [Hugging Face](https://huggingface.co/) é uma plataforma de hospedagem de modelos. No Yomika, ele é basicamente o lugar onde muitos arquivos de modelos ficam.

O Hugging Face não é quem executa a inferência para o Yomika. O Yomika baixa arquivos de modelos do Hugging Face, mantém em cache local e os executa na sua máquina através da stack local de runtime.

Se o Hugging Face estiver bloqueado na sua rede, o Yomika não conseguirá buscar esses arquivos de modelo, não importa em qual botão você clique no app.

## O que significa "conexão com a internet" aqui

Para o Yomika, "eu tenho internet" significa mais do que "meu ícone de Wi-Fi está conectado" ou "o Google abre no navegador".

O que realmente importa é:

- o DNS consegue resolver os hostnames exigidos
- conexões HTTPS podem ser estabelecidas
- downloads de arquivos conseguem iniciar e terminar
- seu ISP, firewall, proxy, antivírus ou região não está bloqueando o host

Conseguir abrir sites não relacionados não comprova que `huggingface.co`, `github.com` ou `pypi.org` estejam acessíveis a partir da sua rede atual.

## Teste a conexão fora do Yomika primeiro

Se essas verificações falharem fora do Yomika, conserte o caminho de rede primeiro. Isso não é um bug do Yomika.

### Verificações no navegador

Abra estes no navegador normal:

- `https://huggingface.co`
- `https://huggingface.co/ogkalu/comic-text-and-bubble-detector`
- `https://github.com`
- `https://pypi.org`

Se eles não carregarem no navegador, o Yomika também não vai conseguir carregá-los.

### Verificações no macOS e Linux

```bash
nslookup huggingface.co
curl -I --max-time 20 https://huggingface.co
curl -I --max-time 20 https://github.com
curl -I --max-time 20 https://pypi.org
curl -L --max-time 20 -o /dev/null -w '%{http_code}\n' \
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

O que você quer ver:

- `nslookup` retorna um endereço em vez de uma falha de DNS
- os comandos `curl -I` retornam uma resposta HTTPS normal como `200`
- o teste de arquivo direto imprime `200`

### Verificações no Windows PowerShell

Use `curl.exe`, não o alias `curl` do PowerShell:

```powershell
nslookup huggingface.co
curl.exe -I --max-time 20 https://huggingface.co
curl.exe -I --max-time 20 https://github.com
curl.exe -I --max-time 20 https://pypi.org
curl.exe -L --max-time 20 -o NUL -w "%{http_code}\n" `
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

As mesmas expectativas se aplicam: resolução normal de DNS, respostas HTTPS bem-sucedidas e `200` no teste de arquivo direto.

## Como saber se o Hugging Face está bloqueado na sua área ou rede

Estes são os sinais comuns:

- `huggingface.co` dá timeout, reseta ou nunca termina de carregar enquanto sites não relacionados continuam funcionando
- o teste de arquivo direto acima nunca retorna `200`
- a mesma falha acontece no navegador, no `curl` e no Yomika
- o comando funciona em outra rede, como um hotspot de celular, mas falha na sua rede normal
- GitHub ou PyPI funciona, mas o Hugging Face não

Se trocar de rede resolver, o problema é o caminho de rede.

Se o Hugging Face falhar em todo lugar fora do Yomika na mesma máquina, resolva isso primeiro antes de abrir um bug no Yomika.

## Teste com o Yomika depois que as verificações externas passarem

Assim que os testes no navegador e no `curl` funcionarem, teste o Yomika diretamente:

```bash
# macOS / Linux
yomika --download --debug
yomika --cpu --download --debug

# Windows
yomika.exe --download --debug
yomika.exe --cpu --download --debug
```

Por que os dois comandos ajudam:

- `--download` testa o caminho normal de download de bootstrap
- `--cpu --download` ignora a preferência de GPU para que você possa separar problemas de rede de problemas de preparação do runtime de GPU

Se um desses comandos falhar, guarde o texto exato do erro. Ele é muito mais útil do que "download quebrado".

## Antes de abrir um bug

Verifique estes pontos primeiro:

- um navegador consegue abrir Hugging Face, GitHub e PyPI a partir da mesma máquina
- os testes de `curl` acima têm sucesso fora do Yomika
- o problema muda em outra rede
- `--cpu --download` se comporta diferente de `--download`
- qual é o seu `Data Path` configurado
- qual foi o texto exato do erro impresso pelo Yomika

Se o host estiver inalcançável fora do Yomika, abra primeiro um chamado de rede, firewall, proxy, VPN ou ISP.

Se as verificações externas passarem e o Yomika ainda falhar, abra um bug no Yomika e inclua os detalhes acima.
