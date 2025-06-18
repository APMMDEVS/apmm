use std::fs;
use std::path::Path;

// 从 index.rs 导入 HTML 生成函数
use super::index::generate_index_html;

/// 生成 webroot 目录及 index.html 文件
pub fn generate_webroot_folder(target_dir: &Path, module_id: &str) -> Result<(), String> {
    let webroot_dir = target_dir.join("webroot");
    
    // 创建 webroot 目录
    fs::create_dir_all(&webroot_dir)
        .map_err(|e| format!("创建webroot目录失败: {}", e))?;
    
    // 生成 index.html 文件
    let index_html_content = generate_index_html(module_id);
    let index_html_path = webroot_dir.join("index.html");
    fs::write(&index_html_path, &index_html_content)
        .map_err(|e| format!("写入index.html失败: {}", e))?;
    
    Ok(())
}

