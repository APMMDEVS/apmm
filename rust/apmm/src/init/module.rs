/// 生成module.prop内容
pub fn generate_module_prop(module_id: &str, author: &str, version_code: i64) -> String {
    format!(r#"id = "{}"
name = "{}"
description = "APMM (Android Patch Module Manager)"
version = "v0.1.0"
versionCode = {}
author = "{}"
license = "MIT"
# updateJson = ""

[script]
# hello = "echo 'world'"

[build]
# 全局build配置
[build.module]
# extra = ["Path/to/extra/file1", "Path/to/extra/file2"]
exclude = ["src", "rust", ".*", "uv.lock", "dist", "build"]

[build.src]
# extra = ["src/extra1", "src/extra2"]
exclude = ["src", "rust", ".*", "uv.lock", "dist", "build"]

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

[build.system]
requires = ["apmm>=0.3.0"]
build-backend = "apmm"

[github]
# repo = ""
# path = "." # 这个很重要，表示模块是在仓库根目录下
# branch = "main"
# proxy-provider = "https://api.akams.cn/github"
"#, module_id, module_id, version_code, author)
}

