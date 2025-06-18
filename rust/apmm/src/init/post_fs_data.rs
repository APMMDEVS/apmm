

/// 生成 post-fs-data.sh 脚本内容（早期启动时执行）
pub fn generate_post_fs_data_sh(module_id: &str) -> String {
    format!(r#"#!/system/bin/sh
# APMM Module: {}
# 这个阶段是阻塞的。在执行完成之前或者 10 秒钟之后，启动过程会暂停。
# 脚本在任何模块被挂载之前运行。这使得模块开发者可以在模块被挂载之前动态地调整它们的模块。
# 这个阶段发生在 Zygote 启动之前。
# 使用 setprop 会导致启动过程死锁！请使用 resetprop -n <prop_name> <prop_value> 代替。
# 只有在必要时才在此模式下运行脚本。

# 模块路径
MODDIR="${{0%/*}}"

# 建议不要使用本脚本，除非你知道你在干什么
"#, module_id)
}