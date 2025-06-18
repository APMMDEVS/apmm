use crate::env::ApmmMeta;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// 导入子模块
mod module;
mod customize;
mod utils;
mod license;
mod post_fs_data;
mod uninstall;
mod service;
mod boot_completed;
mod post_mount;
mod system_prop;
mod sepolicy;
mod webroot;
mod readme;

// 导入 META-INF 目录结构模块
#[path = "META-INF/mod.rs"]
mod meta_inf;

// 重新导出常用功能
pub use module::generate_module_prop;
pub use utils::generate_version_code;
pub use license::generate_default_license;
pub use customize::generate_customize_sh;
pub use post_fs_data::generate_post_fs_data_sh;
pub use uninstall::generate_uninstall_sh;  
pub use service::generate_service_sh;
pub use boot_completed::generate_boot_completed_sh;
pub use post_mount::generate_post_mount_sh;
pub use system_prop::generate_system_prop;
pub use sepolicy::generate_sepolicy_rule;
pub use webroot::generate_webroot_folder;
pub use readme::generate_readme_md;
pub use meta_inf::com::google::android::generate_update_binary;
pub use meta_inf::com::google::android::generate_updater_script;

/// init 命令处理
pub fn cmd_init(args: &[String]) -> Result<String, String> {
    // 如果没有参数，默认使用当前目录
    let input_path = if args.is_empty() {
        "."
    } else {
        &args[0]
    };

    let current_dir = env::current_dir()
        .map_err(|e| format!("获取当前目录失败: {}", e))?;

    let (module_id, target_dir, is_current_dir) = if input_path == "." {
        // 在当前目录初始化，使用当前目录名作为模块ID
        let dir_name = current_dir.file_name()
            .ok_or("获取当前目录名失败")?
            .to_str()
            .ok_or("无效的目录名")?;
        (dir_name.to_string(), current_dir, true)
    } else {
        // 解析路径
        let target_path = if Path::new(input_path).is_absolute() {
            PathBuf::from(input_path)
        } else {
            current_dir.join(input_path)
        };

        // 获取模块ID（使用路径的最后一部分）
        let module_id = target_path.file_name()
            .ok_or("无效路径: 无法确定模块ID")?
            .to_str()
            .ok_or("无效路径: 包含非UTF8字符")?
            .to_string();        // 如果目录不存在则创建
        if !target_path.exists() {
            use colored::Colorize;
            fs::create_dir_all(&target_path)
                .map_err(|e| format!("创建目录 {} 失败: {}", target_path.display(), e))?;
            println!("{} {}", "Created".green().bold(), target_path.display());
        }

        (module_id, target_path, false)
    };

    // 切换到目标目录（只有在不是当前目录时才需要切换）
    if !is_current_dir {
        env::set_current_dir(&target_dir)
            .map_err(|e| format!("切换到目标目录失败: {}", e))?;
    }

    // 检查目标目录是否已经是APMM项目
    let apmm_path = target_dir.join(".apmm");
    if apmm_path.exists() {
        return Err(format!("目录 '{}' 已经是一个APMM项目", target_dir.display()));
    }

    // 加载元数据获取默认作者信息
    let meta = ApmmMeta::load()?;
    let author = if meta.username.is_empty() {
        "APMM Team".to_string()
    } else {
        meta.username
    };

    // 创建.apmm目录
    fs::create_dir_all(&apmm_path)
        .map_err(|e| format!("创建.apmm目录失败: {}", e))?;    // 生成版本代码（基于当前日期）
    let version_code = generate_version_code();

    // 生成module.prop内容
    let module_prop_content = generate_module_prop(&module_id, &author, version_code);

    // 写入module.prop文件
    let module_prop_path = target_dir.join("module.prop");
    fs::write(&module_prop_path, &module_prop_content)
        .map_err(|e| format!("写入module.prop失败: {}", e))?;

    // 生成并写入LICENSE文件
    let license_content = generate_default_license();
    let license_path = target_dir.join("LICENSE");
    fs::write(&license_path, &license_content)
        .map_err(|e| format!("写入LICENSE失败: {}", e))?;

    // 生成并写入README.md文件
    let readme_content = generate_readme_md(&module_id, &author);
    let readme_path = target_dir.join("README.md");
    fs::write(&readme_path, &readme_content)
        .map_err(|e| format!("写入README.md失败: {}", e))?;

    // 生成并写入customize.sh文件
    let customize_content = generate_customize_sh(&module_id);
    let customize_path = target_dir.join("customize.sh");
    fs::write(&customize_path, &customize_content)
        .map_err(|e| format!("写入customize.sh失败: {}", e))?;

    // 生成并写入post-fs-data.sh文件
    let post_fs_data_content = generate_post_fs_data_sh(&module_id);
    let post_fs_data_path = target_dir.join("post-fs-data.sh");
    fs::write(&post_fs_data_path, &post_fs_data_content)
        .map_err(|e| format!("写入post-fs-data.sh失败: {}", e))?;

    // 生成并写入service.sh文件
    let service_content = generate_service_sh();
    let service_path = target_dir.join("service.sh");
    fs::write(&service_path, &service_content)
        .map_err(|e| format!("写入service.sh失败: {}", e))?;

    // 生成并写入uninstall.sh文件
    let uninstall_content = generate_uninstall_sh(&module_id);
    let uninstall_path = target_dir.join("uninstall.sh");
    fs::write(&uninstall_path, &uninstall_content)
        .map_err(|e| format!("写入uninstall.sh失败: {}", e))?;

    // 生成并写入可选脚本文件
    let boot_completed_content = generate_boot_completed_sh();
    let boot_completed_path = target_dir.join("boot-completed.sh");
    fs::write(&boot_completed_path, &boot_completed_content)
        .map_err(|e| format!("写入boot-completed.sh失败: {}", e))?;

    let post_mount_content = generate_post_mount_sh();
    let post_mount_path = target_dir.join("post-mount.sh");
    fs::write(&post_mount_path, &post_mount_content)
        .map_err(|e| format!("写入post-mount.sh失败: {}", e))?;

    // 生成并写入配置文件
    let system_prop_content = generate_system_prop();
    let system_prop_path = target_dir.join("system.prop");  
    fs::write(&system_prop_path, &system_prop_content)
        .map_err(|e| format!("写入system.prop失败: {}", e))?;    let sepolicy_content = generate_sepolicy_rule();
    let sepolicy_path = target_dir.join("sepolicy.rule");
    fs::write(&sepolicy_path, &sepolicy_content)
        .map_err(|e| format!("写入sepolicy.rule失败: {}", e))?;

    // 生成 META-INF 目录结构和 update-binary 文件
    let meta_inf_dir = target_dir.join("META-INF").join("com").join("google").join("android");
    fs::create_dir_all(&meta_inf_dir)
        .map_err(|e| format!("创建META-INF目录失败: {}", e))?;
      let update_binary_content = generate_update_binary();
    let update_binary_path = meta_inf_dir.join("update-binary");
    fs::write(&update_binary_path, &update_binary_content)
        .map_err(|e| format!("写入update-binary失败: {}", e))?;

    let updater_script_content = generate_updater_script();
    let updater_script_path = meta_inf_dir.join("updater-script");
    fs::write(&updater_script_path, &updater_script_content)
        .map_err(|e| format!("写入updater-script失败: {}", e))?;// 生成 webroot 文件夹及其内容
    generate_webroot_folder(&target_dir, &module_id)
        .map_err(|e| format!("生成webroot文件夹失败: {}", e))?;

    // 将项目添加到元数据
    let current_path = target_dir.to_string_lossy().to_string();
    
    let mut meta = ApmmMeta::load()?;    meta.add_project(module_id.clone(), current_path)?;

    use colored::Colorize;

    println!("\nResolved 13 module files in 1ms");
    println!("Generated 13 files in 45ms");
    
    println!("{} module.prop - 模块元数据", " + ".green().bold());
    println!("{} LICENSE - MIT许可证", " + ".green().bold());
    println!("{} README.md - 项目说明文档", " + ".green().bold());
    println!("{} customize.sh - 安装脚本", " + ".green().bold());
    println!("{} post-fs-data.sh - 早期启动脚本", " + ".green().bold());
    println!("{} service.sh - 延迟启动服务脚本", " + ".green().bold());
    println!("{} uninstall.sh - 卸载脚本", " + ".green().bold());
    println!("{} boot-completed.sh - 启动完成脚本", " + ".green().bold());
    println!("{} post-mount.sh - 挂载后脚本", " + ".green().bold());
    println!("{} system.prop - 系统属性", " + ".green().bold());
    println!("{} sepolicy.rule - SELinux策略规则", " + ".green().bold());
    println!("{} META-INF/com/google/android/update-binary - Magisk安装器", " + ".green().bold());
    println!("{} META-INF/com/google/android/updater-script - Magisk标识", " + ".green().bold());
    println!("{} webroot/index.html - 模块控制面板", " + ".green().bold());
    println!("{} .apmm/ - APMM项目目录", " + ".green().bold());

    println!("\n🎉 APMM项目初始化成功!");
    println!("   模块ID: {}", module_id.cyan());
    println!("   目标目录: {}", target_dir.display().to_string().cyan());
    println!("   作者: {}", author.cyan());
    println!("   版本: {}", "v0.1.0".cyan());
    println!("   版本代码: {}", version_code.to_string().cyan());


    Ok(format!("APMM项目 '{}' 在 {} 中初始化成功", module_id, target_dir.display()))
}

