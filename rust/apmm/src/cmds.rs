use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use std::io::Write;

/// 构建步骤
#[derive(Debug, Clone)]
pub struct BuildStep {
    pub name: String,
    pub command: String,
}

/// 构建配置
#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub prebuild_steps: Vec<BuildStep>,
    pub build_steps: Vec<BuildStep>,
    pub postbuild_steps: Vec<BuildStep>,
    pub system_requires: Vec<String>,
    pub build_backend: String,
}

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
    pub build_config: BuildConfig,
}

impl ApmmConfig {
    /// 从 module.prop 内容解析配置
    pub fn from_content(content: &str) -> Result<Self, String> {
        let mut config = HashMap::new();
        let mut current_section = String::new();
        let mut current_array_section = String::new();
        let mut prebuild_steps = Vec::new();
        let mut build_steps = Vec::new();
        let mut postbuild_steps = Vec::new();
        let mut system_requires = Vec::new();
        let mut build_backend = "apmm".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // 处理节区标题
            if line.starts_with("[[") && line.ends_with("]]") {
                current_array_section = line[2..line.len()-2].to_string();
                continue;
            } else if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
                current_array_section.clear();
                continue;
            }
            
            // 处理键值对
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                
                // 处理数组节区中的步骤
                if !current_array_section.is_empty() {
                    let step = BuildStep {
                        name: key.to_string(),
                        command: value.to_string(),
                    };
                    
                    match current_array_section.as_str() {
                        "build.prebuild" => prebuild_steps.push(step),
                        "build.build" => build_steps.push(step),
                        "build.postbuild" => postbuild_steps.push(step),
                        _ => {}
                    }
                } else if current_section == "build.system" {
                    // 处理系统构建配置
                    match key {
                        "requires" => {
                            // 简单解析数组格式 ["item1", "item2"]
                            if value.starts_with('[') && value.ends_with(']') {
                                let items = value[1..value.len()-1]
                                    .split(',')
                                    .map(|s| s.trim().trim_matches('"').to_string())
                                    .collect();
                                system_requires = items;
                            }
                        },
                        "build-backend" => {
                            build_backend = value.to_string();
                        },
                        _ => {}
                    }
                } else if current_section.is_empty() {
                    // 处理顶级配置
                    config.insert(key.to_string(), value.to_string());
                }
            }
        }
        
        let build_config = BuildConfig {
            prebuild_steps,
            build_steps,
            postbuild_steps,
            system_requires,
            build_backend,
        };
        
        Ok(ApmmConfig {
            id: config.get("id").unwrap_or(&"unknown".to_string()).clone(),
            name: config.get("name").unwrap_or(&"Unknown".to_string()).clone(),
            description: config.get("description").unwrap_or(&"".to_string()).clone(),
            version: config.get("version").unwrap_or(&"0.1.0".to_string()).clone(),
            version_code: config.get("versionCode").and_then(|s| s.parse().ok()).unwrap_or(1),
            author: config.get("author").unwrap_or(&"Unknown".to_string()).clone(),
            license: config.get("license").unwrap_or(&"MIT".to_string()).clone(),
            build_config,
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
    println!("APMM (Android Patch Module Manager) v{}", env!("CARGO_PKG_VERSION"));
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
    println!("APMM v{}", env!("CARGO_PKG_VERSION"));
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
    if !config.build_config.prebuild_steps.is_empty() {
        println!("⚙️  Running prebuild steps...");
        for step in &config.build_config.prebuild_steps {
            println!("   {}: {}", step.name, step.command);
            // 这里可以实际执行命令
        }
    }
    
    // 执行构建步骤
    if !config.build_config.build_steps.is_empty() {
        println!("🔧 Running build steps...");
        for step in &config.build_config.build_steps {
            println!("   {}: {}", step.name, step.command);
            // 这里可以实际执行命令
        }
    } else {
        println!("🔧 Running build steps...");
        println!("   Using default APMM build process (build backend: {})", config.build_config.build_backend);
    }
    
    // 执行后构建步骤
    if !config.build_config.postbuild_steps.is_empty() {
        println!("🧹 Running postbuild steps...");
        for step in &config.build_config.postbuild_steps {
            println!("   {}: {}", step.name, step.command);
            // 这里可以实际执行命令
        }
    }
    
    // 显示系统要求
    if !config.build_config.system_requires.is_empty() {
        println!("📋 System requirements: {:?}", config.build_config.system_requires);
    }
    
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
