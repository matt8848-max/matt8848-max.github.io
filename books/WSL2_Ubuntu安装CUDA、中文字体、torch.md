# WSL2-Ubuntu安装CUDA、中文字体、torch

准备工作：
Windows11 正式版，C盘约20-40G空间（本文不讲如何将WSL如何安装到其它逻辑硬盘上）

## 启用WSL2

以 管理员身份 打开Powershell，键入
```powershell
wsl --install
```

此命令会启用WSL2并安装一个Ubuntu虚拟机。
更多虚拟机可见微软商店和wsl2的文档。

## 安装CUDA
注册源
```bash
wget https://developer.download.nvidia.cn/compute/cuda/repos/wsl-ubuntu/x86_64/cuda-keyring_1.0-1_all.deb
sudo dpkg -i cuda-keyring_1.0-1_all.deb
```

[下载CUDA Toolkit](https://developer.nvidia.com/cuda-13-2-1-download-archive)

## 安装中文字体

挂载win的字体文件
```bash
sudo nano /etc/fonts/local.conf
```
粘贴下面的内容
```xml
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
    <dir>/mnt/c/Windows/Fonts</dir>
</fontconfig>
```

按 Ctrl + O 保存，Enter 确认，然后 Ctrl + X 退出。

然后使用如下命令刷新字体缓存
```bash
sudo fc-cache -fv
```

## torch

推荐使用[uv](https://docs.astral.sh/uv/getting-started/installation/)。


在使用uv创建了项目后，下面为配置了pypi镜像源和torch镜像源的示例

```toml
[project]
name = "a"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
    "jupyter",
    "matplotlib",
    "torch",
    "torchaudio",
    "torchvision",
]


[tool.uv.sources]
torch = [{ index = "pytorch-cu128" }]
torchvision = [{ index = "pytorch-cu128" }]
torchaudio = [{ index = "pytorch-cu128" }]

[[tool.uv.index]]
url = "https://pypi.tuna.tsinghua.edu.cn/simple/"
default = true


[[tool.uv.index]]
name = "pytorch-cpu"
url = "https://download.pytorch.org/whl/cpu"
explicit = true

[[tool.uv.index]]
name = "pytorch-cu128"
url = "https://mirror.sjtu.edu.cn/pytorch-wheels/cu129"
explicit = true
```