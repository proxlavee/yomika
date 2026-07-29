---
title: 运行时与模型下载
---

# 运行时与模型下载

Yomika 是本地优先应用，但首次使用并不是完全离线的。在本地流水线开始工作之前，Yomika 可能需要下载原生运行时文件和模型权重。

如果这些下载失败，先检查网络路径。你的机器、ISP、防火墙、代理，或者所在地区如果无法访问某个主机，Yomika 也不可能从那里下载文件。

## Yomika 会下载什么

Yomika 下载的内容大致分成三类：

- 原生运行时包，比如 `llama.cpp` 二进制、受支持 NVIDIA 环境下的 CUDA 支持文件，以及受支持 Windows AMD 环境下的 ZLUDA 文件
- 默认本地页面流水线所需的 bootstrap 模型包
- 按需下载的模型包，例如之后才会选择的 OCR / inpainting 引擎和本地 GGUF 翻译模型

需要明确的是：

- `yomika --download` 会准备 bootstrap 运行时和模型包，然后退出
- 它不会把模型选择器里所有可选引擎和本地 LLM 一次性全部下完

## 文件保存在哪里

Yomika 会把运行时包存到当前配置的 `Data Path` 下。模型库默认使用 `<Data Path>/models`，也可以在 **Settings > Runtime** 中改为其他绝对路径。

常见示例：

- Windows: `%LOCALAPPDATA%\Yomika`
- macOS: `~/Library/Application Support/Yomika`
- Linux: `~/.local/share/Yomika`

当前实现里比较重要的子目录如下：

```text
<Data Path>/
  config.toml
  runtime/
    .downloads/
    cuda/
    llama.cpp/
    zluda/
  models/                    # 默认模型库位置
    huggingface/
```

实际含义：

- `runtime/.downloads` 是原生运行时下载归档的通用缓存目录
- `runtime/*` 存的是 Yomika 真正会加载的已解压库文件
- `<Model Library>/huggingface` 是 vision 模型和本地 GGUF 模型文件使用的 Hugging Face 缓存

并不是所有平台都会出现全部目录。比如 `zluda/` 只在 Windows 上有意义，而且只针对受支持的 AMD 场景。

更改模型库时，可以选择 **Use existing** 直接采用目标文件夹中的文件，也可以选择 **Move current models** 复制并验证 Yomika 管理的缓存，再移除旧缓存。更改成功后应用会重启，确保所有运行时组件使用同一个位置。

关于当前配置路径和 HTTP 设置，可参考 [设置参考](../reference/settings.md)。

## 运行时下载是怎么工作的

当 Yomika 启动，或者你执行 `yomika --download` 时，它会根据当前平台和计算策略准备 bootstrap 包。

大致流程如下：

1. Yomika 在当前数据路径下创建运行时和模型目录。
2. 它检查每个 bootstrap 包是否已经是最新状态。
3. 如果某个原生运行时包缺失或过期，就把归档下载到 `runtime/.downloads`。
4. 它把需要的文件解压到对应运行时安装目录，并写入 install marker。
5. 在本地流水线真正启动前，它会 preload 运行时库。

在当前源码里，原生运行时下载来源会因包而异：

- `llama.cpp` 来自 GitHub Releases
- 受支持平台上的 CUDA 运行时组件来自 PyPI 元数据和 wheel 文件
- 受支持 Windows AMD 环境下的 ZLUDA 来自上游 release asset

所以运行时下载失败，并不一定就是 Hugging Face 的问题。

## 模型下载是怎么工作的

大多数模型下载都使用 `<Model Library>/huggingface` 下的共享 Hugging Face 缓存。

大致流程如下：

1. Yomika 请求某个具体的 `repo/file` 组合。
2. 它先检查本地 Hugging Face 缓存。
3. 如果文件已经缓存，直接复用。
4. 如果没有，就只下载这个文件，并按 Hugging Face 缓存布局存起来。
5. 后续加载会直接复用缓存文件，而不是重新下载。

这既适用于默认视觉模型栈，也适用于之后按需下载的本地 GGUF 翻译模型。

## 管理下载和磁盘空间

本地翻译模型使用彼此独立的 **Download** 和 **Load** 操作。模型缺失时，**Load** 不会自动开始下载。下载期间可以在模型选择器或通知区域选择 **Cancel**；Yomika 会清理不完整文件并报告最终状态。

在 **Settings > Runtime** 中可以：

- 查看模型库和临时下载占用的空间
- 清理已完成的运行时归档和不完整模型文件
- 删除某个已下载的本地翻译模型
- 删除包括默认视觉模型栈在内的全部模型文件
- 删除某个本地翻译模型并立即重新下载

更改模型文件前，请先卸载本地模型，并等待当前任务结束或取消正在进行的下载。已删除的默认模型会在相应流水线阶段下次需要时重新下载。

## Hugging Face 是什么

[Hugging Face](https://huggingface.co/) 是一个模型托管平台。在 Yomika 里，它主要是很多模型文件的存放位置。

真正执行推理的不是 Hugging Face。Yomika 会先从 Hugging Face 下载模型文件并缓存在本地，之后再通过本地运行时栈在你的机器上执行。

如果你的网络环境里 Hugging Face 被屏蔽，那么无论你在应用里点哪个按钮，Yomika 都拿不到这些模型文件。

## 这里说的“有互联网连接”到底是什么意思

对 Yomika 来说，“我能上网”不只是“Wi-Fi 图标连着”或者“浏览器能打开 Google”。

真正重要的是：

- DNS 能解析需要的主机名
- HTTPS 连接可以建立
- 文件下载能开始并顺利完成
- ISP、防火墙、代理、杀毒软件或地区限制没有拦截目标主机

能打开别的网站，并不能证明 `huggingface.co`、`github.com` 或 `pypi.org` 从你当前网络一定可达。

## 先在 Yomika 之外测试连接

如果这些检查在 Yomika 外部都失败了，先把网络路径问题解决掉。这不是 Yomika 的 bug。

### 浏览器检查

用普通浏览器打开：

- `https://huggingface.co`
- `https://huggingface.co/ogkalu/comic-text-and-bubble-detector`
- `https://github.com`
- `https://pypi.org`

如果浏览器都打不开，Yomika 也不可能下载成功。

### macOS / Linux 检查

```bash
nslookup huggingface.co
curl -I --max-time 20 https://huggingface.co
curl -I --max-time 20 https://github.com
curl -I --max-time 20 https://pypi.org
curl -L --max-time 20 -o /dev/null -w '%{http_code}\n' \
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

理想结果是：

- `nslookup` 返回地址，而不是 DNS 解析失败
- `curl -I` 返回正常 HTTPS 响应，例如 `200`
- 直接文件测试输出 `200`

### Windows PowerShell 检查

请使用 `curl.exe`，不要用 PowerShell 的 `curl` 别名：

```powershell
nslookup huggingface.co
curl.exe -I --max-time 20 https://huggingface.co
curl.exe -I --max-time 20 https://github.com
curl.exe -I --max-time 20 https://pypi.org
curl.exe -L --max-time 20 -o NUL -w "%{http_code}\n" `
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

预期同样是：DNS 正常解析、HTTPS 请求成功、直接文件测试返回 `200`。

## 怎么判断 Hugging Face 在你所在地区或网络里被拦了

常见信号有这些：

- `huggingface.co` 超时、连接被重置、或者一直加载不完，但别的网站能正常打开
- 上面的直接文件测试始终拿不到 `200`
- 同样的问题会同时出现在浏览器、`curl` 和 Yomika 中
- 换一条网络，比如手机热点，就能工作；原来的网络却不行
- GitHub 或 PyPI 可达，但 Hugging Face 不可达

如果换网络就恢复正常，那问题就在网络路径上。

如果在同一台机器上，Yomika 之外访问 Hugging Face 也一直失败，那就先解决这个问题，再谈 Yomika bug。

## 外部检查通过后，再用 Yomika 测试

当浏览器和 `curl` 检查都通过之后，再直接测试 Yomika：

```bash
# macOS / Linux
yomika --download --debug
yomika --cpu --download --debug

# Windows
yomika.exe --download --debug
yomika.exe --cpu --download --debug
```

这两条命令分别能帮你确认：

- `--download` 用来测试正常的 bootstrap 下载路径
- `--cpu --download` 会跳过 GPU 优先逻辑，方便把网络问题和 GPU 运行时准备问题分开

如果其中任意一条失败，请保留准确的错误文本。它比“download broken”这种描述有用得多。

## 提交 bug 之前先确认这些

请先检查：

- 同一台机器上的浏览器能不能打开 Hugging Face、GitHub 和 PyPI
- 上面的 `curl` 测试在 Yomika 之外是否成功
- 换一条网络后，问题是否会变化
- `--cpu --download` 和 `--download` 的行为是否不同
- 当前配置的 `Data Path` 是什么
- Yomika 打印出的准确错误文本是什么

如果在 Yomika 之外目标主机本来就不可达，那应该先按网络、防火墙、代理、VPN 或 ISP 问题处理。

如果外部检查都通过了，Yomika 仍然失败，再带上以上信息去提交 Yomika bug。
