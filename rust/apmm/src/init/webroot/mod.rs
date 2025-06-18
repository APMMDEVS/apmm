// webroot 模块
// 用于生成 Web 界面相关文件

pub mod webroot;
pub mod index;

// 重新导出主要功能
pub use webroot::generate_webroot_folder;
