# apmm

AndroidPatchModuleManager

Apmm是 APM的管理器 本身也是一个特殊的APM

支持Android + PC双端

Android端不支持上传至github 但可以借助github actin间接实现

---

# Android使用

```termux
uv tool install apmm
或者：
pip install apmm
```

```apmm.zip
magisk --install-module apmm.zip 
ksud module install apmm.zip
apd module install apmm.zip
```

# 开发

```uv
uv sync # 确保环境一致
uv pip install -e .[pc] 
```

> 在项目根目录执行

`uv tool install -e .[pc]`

> 配置mcp服务器：
> 仅需命令即可，无需args . 默认stdio 可以额外指定为sse 使用 apmcp -h / --help 查看帮助

```apmcp
"apmcp ": {
    "type": "stdio",
    "command": "apmcp"
},


```

> apmcp 开发
``` mcp
uv add "mcp[cli]" httpx
cd ...\apmm\src\apmm
mcp dev apmcp.py
# 注意，你可能需要填写Proxy Session Token才行，这个在终端有
```

> build

```build
maturin develop # 用的最多
uv build # 这个也行
uv tool install -e .[pc] --force # 碰到问题试试这个

```



# apmm拓展配置

```module.prop
id = apmm
name = APMM
description = APMM (Android Patch Module Manager) 
version = 0.1.0
versionCode = 2025061700
author = APMM Team
license = MIT
# updateJson

[script]
# hello = "echo 'world'"

[build]
# 全局build配置

[[build.prebuild]]
step1 = "echo 'Prebuild step 1: Initializing APMM'"
[[build.prebuild]]
step2 = "echo 'Prebuild step 2: Checking dependencies'"

[[build.build]]
# 留空以使用apmm默认打包 如果允许完全自定义

[[build.postbuild]]
step1 = "echo 'Postbuild step 1: cleaning up APMM build'"
[[build.postbuild]]
step2 = "echo 'Postbuild step 2: Finalizing APMM build'"

[github]
# repo = ""
# path = "." 这个很重要，表示模块是在仓库根目录下
# branch = "main"
# proxy-provider = "https://api.akams.cn/github"
# ...


```
