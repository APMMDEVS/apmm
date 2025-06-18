// android 模块
mod update_binary;
mod updater_script;

// 重新导出生成函数
pub use update_binary::generate_update_binary;
pub use updater_script::generate_updater_script;
