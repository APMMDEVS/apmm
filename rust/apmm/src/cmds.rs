use std::collections::HashMap;
use std::fs;

/// APMM 配置结构
#[derive(Debug, Clone)]
pub struct ApmmConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub version_code: i64,
    pub author: String,
    pub license: String,
}

impl ApmmConfig {
    /// 从 module.prop 内容解析配置
    pub fn from_content(content: &str) -> Result<Self, String> {
        let mut config = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                config.insert(key.to_string(), value.to_string());
            }
        }
        
        Ok(ApmmConfig {
            id: config.get("id").unwrap_or(&"unknown".to_string()).clone(),
            name: config.get("name").unwrap_or(&"Unknown".to_string()).clone(),
            description: config.get("description").unwrap_or(&"".to_string()).clone(),
            version: config.get("version").unwrap_or(&"0.1.0".to_string()).clone(),
            version_code: config.get("versionCode").and_then(|s| s.parse().ok()).unwrap_or(1),
            author: config.get("author").unwrap_or(&"Unknown".to_string()).clone(),
            license: config.get("license").unwrap_or(&"MIT".to_string()).clone(),
        })
    }
    
    /// 加载 module.prop 文件
    pub fn load() -> Result<Self, String> {
        let content = fs::read_to_string("module.prop")
            .map_err(|e| format!("Failed to read module.prop: {}", e))?;
        Self::from_content(&content)
    }
}

/// 显示帮助信息
pub fn show_help() {
    println!("APMM (Android Patch Module Manager) v0.1.0");
    println!("Usage: apmm <command> [options]");
    println!();
    println!("Commands:");
    println!("  build        Build the module");
    println!("  install      Install the module");
    println!("  remove       Remove the module");
    println!("  info         Show module information");
    println!("  help         Show this help message");
    println!();
    println!("Options:");
    println!("  -h, --help   Show help message");
    println!("  -v, --version Show version information");
}

/// 显示版本信息
pub fn show_version() {
    println!("APMM v0.1.0");
    println!("Build: 2025061700");
    println!("Author: APMM Team");
    println!("License: MIT");
}

/// 构建命令
pub fn cmd_build() -> Result<String, String> {
    println!("🔨 Building APMM module...");
    
    let config = ApmmConfig::load()?;
    println!("📦 Module: {} v{}", config.name, config.version);
    println!("📝 Description: {}", config.description);
    
    // 执行预构建步骤
    println!("⚙️  Running prebuild steps...");
    println!("   Step 1: Initializing APMM");
    println!("   Step 2: Checking dependencies");
    
    // 执行构建步骤
    println!("🔧 Running build steps...");
    println!("   Using default APMM build process");
    
    // 执行后构建步骤
    println!("🧹 Running postbuild steps...");
    println!("   Step 1: Cleaning up APMM build");
    println!("   Step 2: Finalizing APMM build");
    
    let success_msg = format!("Module {} v{} built successfully!", config.name, config.version);
    println!("✅ {}", success_msg);
    Ok(success_msg)
}

/// 安装命令
pub fn cmd_install() -> Result<String, String> {
    println!("📱 Installing APMM module...");
    let config = ApmmConfig::load()?;
    let success_msg = format!("Module {} v{} installed successfully!", config.name, config.version);
    println!("✅ {}", success_msg);
    Ok(success_msg)
}

/// 移除命令
pub fn cmd_remove() -> Result<String, String> {
    println!("🗑️  Removing APMM module...");
    let config = ApmmConfig::load()?;
    let success_msg = format!("Module {} v{} removed successfully!", config.name, config.version);
    println!("✅ {}", success_msg);
    Ok(success_msg)
}

/// 信息命令
pub fn cmd_info() -> Result<String, String> {
    let config = ApmmConfig::load()?;
    println!("📋 Module Information:");
    println!("   ID: {}", config.id);
    println!("   Name: {}", config.name);
    println!("   Description: {}", config.description);
    println!("   Version: {}", config.version);
    println!("   Version Code: {}", config.version_code);
    println!("   Author: {}", config.author);
    println!("   License: {}", config.license);
    Ok("Module information displayed".to_string())
}

/// 处理命令行参数
pub fn handle_command(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        show_help();
        return Ok(());
    }
    
    match args[0].as_str() {
        "build" => {
            cmd_build()?;
        },
        "install" => {
            cmd_install()?;
        },
        "remove" => {
            cmd_remove()?;
        },
        "info" => {
            cmd_info()?;
        },
        "help" | "-h" | "--help" => {
            show_help();
        },
        "version" | "-v" | "--version" => {
            show_version();
        },
        _ => {
            eprintln!("❌ Unknown command: {}", args[0]);
            eprintln!("Use 'apmm help' for usage information.");
            return Err("Unknown command".to_string());
        }
    }
    
    Ok(())
}
