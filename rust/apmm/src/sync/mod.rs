use crate::env::ApmmMeta;
use chrono::{Utc, Datelike, Timelike};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// sync 命令处理
pub fn cmd_sync(args: &[String]) -> Result<String, String> {
    let upgrade_version = args.get(0).map(|s| s.as_str()) == Some("-U");
    
    if upgrade_version {
        // 如果是升级版本，需要在当前目录执行
        return sync_current_project_with_upgrade();
    }

    // 全面同步所有项目
    sync_all_projects()
}

/// 同步所有项目
fn sync_all_projects() -> Result<String, String> {
    println!("🔄 Starting full project synchronization...");
    
    let mut meta = ApmmMeta::load()?;
    let mut removed_count = 0;
    let mut added_count = 0;
    let mut valid_count = 0;
    
    // 1. 检查已注册项目的有效性
    let mut projects_to_remove = Vec::new();
    let mut projects_to_add = Vec::new();
    
    // 收集要处理的项目信息，避免借用冲突
    let projects_snapshot: Vec<(String, String)> = meta.projects.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    for (id, path) in projects_snapshot {
        let path_buf = PathBuf::from(&path);
        
        if !path_buf.exists() {
            println!("❌ Project '{}' path no longer exists: {}", id, path);
            projects_to_remove.push(id);
            continue;
        }
        
        // 检查是否是有效的APMM项目
        let apmm_dir = path_buf.join(".apmm");
        let module_prop = path_buf.join("module.prop");
        
        if !apmm_dir.exists() || !module_prop.exists() {
            println!("❌ Project '{}' is not a valid APMM project: {}", id, path);
            projects_to_remove.push(id);
            continue;
        }
        
        // 验证module.prop中的ID是否匹配
        if let Ok(content) = fs::read_to_string(&module_prop) {
            if let Ok(prop_id) = extract_module_id(&content) {
                if prop_id != id {
                    println!("⚠️  Project '{}' has mismatched ID in module.prop: '{}'", id, prop_id);
                    println!("   Updating registration to use correct ID");
                    projects_to_remove.push(id);
                    
                    // 记录要重新注册的项目
                    projects_to_add.push((prop_id, path));
                    continue;
                }
            }
        }
        
        println!("✅ Project '{}' is valid: {}", id, path);
        valid_count += 1;
    }
    
    // 移除失效项目
    for id in projects_to_remove {
        meta.projects.remove(&id);
        removed_count += 1;
    }
    
    // 添加修正的项目
    for (id, path) in projects_to_add {
        meta.projects.insert(id, path);
        added_count += 1;
    }
    
    // 2. 扫描当前目录及其子目录寻找新的APMM项目
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    scan_for_new_projects(&current_dir, &mut meta, &mut added_count)?;
    
    // 3. 保存更新后的元数据
    meta.save()?;
    
    // 4. 显示统计信息
    println!("\n📊 Synchronization completed:");
    println!("   ✅ Valid projects: {}", valid_count);
    println!("   ➕ Added projects: {}", added_count);
    println!("   ❌ Removed projects: {}", removed_count);
    println!("   📋 Total projects: {}", meta.projects.len());
    
    Ok(format!("Synchronized {} projects (+{} -{} ={})", 
               valid_count + added_count + removed_count, 
               added_count, 
               removed_count, 
               meta.projects.len()))
}

/// 扫描目录寻找新的APMM项目
fn scan_for_new_projects(dir: &Path, meta: &mut ApmmMeta, added_count: &mut i32) -> Result<(), String> {
    // 检查当前目录是否是APMM项目
    let apmm_dir = dir.join(".apmm");
    let module_prop = dir.join("module.prop");
    
    if apmm_dir.exists() && module_prop.exists() {
        // 读取模块ID
        let content = fs::read_to_string(&module_prop)
            .map_err(|e| format!("Failed to read module.prop in {}: {}", dir.display(), e))?;
        
        if let Ok(module_id) = extract_module_id(&content) {
            let dir_path = dir.to_string_lossy().to_string();
            
            // 检查是否已经注册
            if let Some(existing_path) = meta.get_project_path(&module_id) {
                if existing_path != &dir_path {
                    println!("⚠️  Found duplicate module ID '{}' in different paths:", module_id);
                    println!("   Existing: {}", existing_path);
                    println!("   Found: {}", dir_path);
                    println!("   Keeping existing registration");
                }
            } else {
                // 新项目，添加到meta
                meta.projects.insert(module_id.clone(), dir_path.clone());
                println!("➕ Added new project '{}': {}", module_id, dir_path);
                *added_count += 1;
            }
        }
    }
    
    // 递归扫描子目录（但跳过.apmm, .git等隐藏目录）
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    
                    // 跳过隐藏目录和一些特殊目录
                    if name.starts_with('.') || name == "node_modules" || name == "target" || name == "build" {
                        continue;
                    }
                    
                    // 递归扫描，但限制深度避免无限递归
                    if let Some(current_dir) = env::current_dir().ok() {
                        if let Ok(relative) = path.strip_prefix(&current_dir) {
                            if relative.components().count() <= 3 { // 限制扫描深度
                                scan_for_new_projects(&path, meta, added_count)?;
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// 同步当前项目并升级版本
fn sync_current_project_with_upgrade() -> Result<String, String> {
    // 检查当前目录是否为APMM项目
    if !Path::new(".apmm").exists() {
        return Err("Current directory is not an APMM project (no .apmm folder found)".to_string());
    }

    if !Path::new("module.prop").exists() {
        return Err("module.prop not found in current directory".to_string());
    }

    // 读取module.prop
    let module_prop_content = fs::read_to_string("module.prop")
        .map_err(|e| format!("Failed to read module.prop: {}", e))?;

    // 解析模块ID
    let module_id = extract_module_id(&module_prop_content)?;

    // 获取当前路径
    let current_path = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?
        .to_string_lossy()
        .to_string();

    // 加载元数据并同步项目信息
    let mut meta = ApmmMeta::load()?;
    
    // 检查项目是否在元数据中
    let needs_sync = match meta.get_project_path(&module_id) {
        Some(registered_path) => registered_path != &current_path,
        None => true,
    };

    if needs_sync {
        meta.add_project(module_id.clone(), current_path.clone())?;
        println!("🔄 Synced project '{}' to meta.toml", module_id);
    }

    // 升级版本
    upgrade_project_version(&module_id)?;

    // 验证项目有效性
    validate_project(&module_id)?;

    println!("✅ Project '{}' synchronized and version upgraded successfully", module_id);
    
    Ok(format!("Project '{}' synchronized and version upgraded", module_id))
}

/// 从module.prop内容中提取模块ID
fn extract_module_id(content: &str) -> Result<String, String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("id = ") {
            let id = line.strip_prefix("id = ")
                .ok_or("Invalid id format")?
                .trim_matches('"')
                .trim();
            return Ok(id.to_string());
        }
    }
    Err("Module ID not found in module.prop".to_string())
}

/// 升级项目版本
fn upgrade_project_version(module_id: &str) -> Result<(), String> {
    println!("🔄 Upgrading version for project '{}'...", module_id);

    // 读取当前module.prop
    let content = fs::read_to_string("module.prop")
        .map_err(|e| format!("Failed to read module.prop: {}", e))?;

    let mut new_content = String::new();
    let mut version_upgraded = false;
    let mut version_code_upgraded = false;

    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with("version = ") && !version_upgraded {
            // 提取当前版本
            if let Some(version_part) = line.strip_prefix("version = ") {
                let current_version = version_part.trim_matches('"').trim();
                let new_version = upgrade_version_string(current_version)?;
                new_content.push_str(&format!("version = \"{}\"\n", new_version));
                version_upgraded = true;
                println!("   Version: {} -> {}", current_version, new_version);
            } else {
                new_content.push_str(line);
                new_content.push('\n');
            }
        } else if line.starts_with("versionCode = ") && !version_code_upgraded {
            // 生成新的版本代码
            let new_version_code = generate_version_code();
            new_content.push_str(&format!("versionCode = {}\n", new_version_code));
            version_code_upgraded = true;
            println!("   Version Code: {}", new_version_code);
        } else {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    // 写回文件
    fs::write("module.prop", new_content)
        .map_err(|e| format!("Failed to write updated module.prop: {}", e))?;

    println!("✅ Version upgraded successfully");
    Ok(())
}

/// 升级版本字符串
fn upgrade_version_string(current: &str) -> Result<String, String> {
    // 移除 'v' 前缀如果存在
    let version_str = current.strip_prefix('v').unwrap_or(current);
    
    // 解析版本号
    let parts: Vec<&str> = version_str.split('.').collect();
    if parts.len() < 3 {
        return Err("Invalid version format, expected X.Y.Z".to_string());
    }

    let major: u32 = parts[0].parse()
        .map_err(|_| "Invalid major version number")?;
    let minor: u32 = parts[1].parse()
        .map_err(|_| "Invalid minor version number")?;
    let mut patch: u32 = parts[2].parse()
        .map_err(|_| "Invalid patch version number")?;

    // 检查是否为Git仓库并获取patch信息
    let git_patch = get_git_patch_info().unwrap_or(0);
    if git_patch > 0 {
        patch = git_patch;
    } else {
        patch += 1;
    }

    Ok(format!("v{}.{}.{}", major, minor, patch))
}

/// 获取Git patch信息
fn get_git_patch_info() -> Option<u32> {
    if !Path::new(".git").exists() {
        return None;
    }

    // 尝试获取commit数量作为patch号
    let output = Command::new("git")
        .args(&["rev-list", "--count", "HEAD"])
        .output()
        .ok()?;

    if output.status.success() {
        let count_str = String::from_utf8_lossy(&output.stdout);
        count_str.trim().parse().ok()
    } else {
        None
    }
}

/// 生成版本代码
fn generate_version_code() -> i64 {
    use chrono::Utc;
    let now = Utc::now();
    let version_code = format!("{:04}{:02}{:02}{:02}{:02}", 
        now.year(), now.month(), now.day(), now.hour(), now.minute());
    version_code.parse().unwrap_or(2025061700)
}

/// 验证项目有效性
fn validate_project(module_id: &str) -> Result<(), String> {
    println!("🔍 Validating project '{}'...", module_id);

    // 检查必要文件
    let required_files = vec!["module.prop"];
    for file in required_files {
        if !Path::new(file).exists() {
            return Err(format!("Required file missing: {}", file));
        }
    }

    // 检查.apmm目录
    if !Path::new(".apmm").exists() {
        return Err(".apmm directory not found".to_string());
    }

    println!("✅ Project validation passed");
    Ok(())
}
