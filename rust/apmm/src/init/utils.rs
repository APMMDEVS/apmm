use chrono::{Datelike, Timelike, Utc};

/// 生成版本代码（基于当前时间）
pub fn generate_version_code() -> i64 {
    let now = Utc::now();
    // 格式: YYYYMMDDHHMM
    let version_code = format!("{:04}{:02}{:02}{:02}{:02}", 
        now.year(), now.month(), now.day(), now.hour(), now.minute());
    version_code.parse().unwrap_or(2025061700)
}

/// 生成版本代码（基于指定时间）
pub fn generate_version_code_from_timestamp(timestamp: i64) -> i64 {
    use chrono::{DateTime, Utc};
    
    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
        let version_code = format!("{:04}{:02}{:02}{:02}{:02}", 
            dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute());
        version_code.parse().unwrap_or(2025061700)
    } else {
        generate_version_code()
    }
}

/// 验证模块ID是否有效
pub fn validate_module_id(module_id: &str) -> Result<(), String> {
    if module_id.is_empty() {
        return Err("Module ID cannot be empty".to_string());
    }
    
    if module_id.len() > 50 {
        return Err("Module ID cannot be longer than 50 characters".to_string());
    }
    
    // 检查是否只包含有效字符（字母、数字、下划线、连字符）
    if !module_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err("Module ID can only contain letters, numbers, underscores, and hyphens".to_string());
    }
    
    // 不能以数字开头
    if module_id.chars().next().unwrap().is_ascii_digit() {
        return Err("Module ID cannot start with a number".to_string());
    }
    
    Ok(())
}

/// 清理和规范化模块ID
pub fn sanitize_module_id(input: &str) -> String {
    let mut result = String::new();
    let mut first_char = true;
    
    for c in input.chars() {
        if c.is_ascii_alphanumeric() || (!first_char && (c == '_' || c == '-')) {
            if first_char && c.is_ascii_digit() {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            first_char = false;
        } else if !first_char && (c.is_whitespace() || c == '.') {
            result.push('_');
        }
    }
    
    // 如果结果为空，使用默认值
    if result.is_empty() {
        result = "my_module".to_string();
    }
    
    // 限制长度
    if result.len() > 50 {
        result.truncate(50);
    }
    
    result
}

/// 验证作者名称
pub fn validate_author_name(author: &str) -> Result<(), String> {
    if author.is_empty() {
        return Err("Author name cannot be empty".to_string());
    }
    
    if author.len() > 100 {
        return Err("Author name cannot be longer than 100 characters".to_string());
    }
    
    Ok(())
}

/// 验证版本字符串格式
pub fn validate_version(version: &str) -> Result<(), String> {
    if version.is_empty() {
        return Err("Version cannot be empty".to_string());
    }
    
    // 支持 vX.Y.Z 或 X.Y.Z 格式
    let version_part = version.strip_prefix('v').unwrap_or(version);
    
    let parts: Vec<&str> = version_part.split('.').collect();
    if parts.len() < 2 || parts.len() > 4 {
        return Err("Version must be in format X.Y or X.Y.Z or X.Y.Z.W".to_string());
    }
    
    for part in parts {
        if part.parse::<u32>().is_err() {
            return Err("Version parts must be numbers".to_string());
        }
    }
    
    Ok(())
}

/// 增加版本号
pub fn increment_version(version: &str, part: VersionPart) -> Result<String, String> {
    validate_version(version)?;
    
    let has_v_prefix = version.starts_with('v');
    let version_part = version.strip_prefix('v').unwrap_or(version);
    
    let mut parts: Vec<u32> = version_part.split('.')
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();
    
    // 确保至少有3个部分（major.minor.patch）
    while parts.len() < 3 {
        parts.push(0);
    }
    
    match part {
        VersionPart::Major => {
            parts[0] += 1;
            parts[1] = 0;
            parts[2] = 0;
        },
        VersionPart::Minor => {
            parts[1] += 1;
            parts[2] = 0;
        },
        VersionPart::Patch => {
            parts[2] += 1;
        },
    }
    
    let new_version = parts.iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join(".");
    
    if has_v_prefix {
        Ok(format!("v{}", new_version))
    } else {
        Ok(new_version)
    }
}

/// 版本部分枚举
#[derive(Debug, Clone)]
pub enum VersionPart {
    Major,
    Minor,
    Patch,
}

/// 获取当前年份
pub fn current_year() -> i32 {
    Utc::now().year()
}

/// 格式化时间戳为可读格式
pub fn format_timestamp(timestamp: i64) -> String {
    use chrono::{DateTime, Utc};
    
    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    } else {
        "Invalid timestamp".to_string()
    }
}

/// 检查模块ID是否符合KernelSU规范
/// 
/// KernelSU模块ID规范：
/// - 必须匹配正则表达式：^[a-zA-Z][a-zA-Z0-9._-]+$
/// - 必须以字母开头
/// - 只能包含字母、数字、点(.)、下划线(_)、连字符(-)
/// - 长度限制：1-64个字符（合理的模块ID长度）
/// 
/// 示例：
/// ✓ a_module
/// ✓ a.module  
/// ✓ module-101
/// ✓ MyModule
/// ✓ test_module_v2
/// ✗ a module (包含空格)
/// ✗ 1_module (以数字开头)
/// ✗ -a-module (以连字符开头)
/// ✗ .hidden (以点开头)
/// ✗ _private (以下划线开头)
/// ✗ module@ (包含特殊字符)
pub fn is_valid_module_id(module_id: &str) -> bool {
    // 检查基本长度限制
    if module_id.is_empty() || module_id.len() > 64 {
        return false;
    }
    
    // 检查是否以字母开头
    let first_char = match module_id.chars().next() {
        Some(c) => c,
        None => return false,
    };
    
    if !first_char.is_ascii_alphabetic() {
        return false;
    }
    
    // 检查所有字符是否符合规范
    for c in module_id.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' => {
                // 允许的字符
                continue;
            }
            _ => {
                // 不允许的字符
                return false;
            }
        }
    }
    
    // 检查是否包含连续的特殊字符（避免如 "a..b" 或 "a--b" 等）
    let mut prev_char = '\0';
    for c in module_id.chars() {
        if matches!(c, '.' | '_' | '-') && matches!(prev_char, '.' | '_' | '-') {
            return false;
        }
        prev_char = c;
    }
    
    // 不能以特殊字符结尾
    let last_char = module_id.chars().last().unwrap();
    if matches!(last_char, '.' | '_' | '-') {
        return false;
    }
    
    // 检查是否为保留名称（Android系统保留）
    let reserved_names = [
        "android", "system", "framework", "root", "shell", "bin", "etc",
        "proc", "sys", "dev", "data", "cache", "config", "META-INF"
    ];
    
    let name_lower = module_id.to_lowercase();
    if reserved_names.contains(&name_lower.as_str()) {
        return false;
    }
    
    true
}

/// 检查版本代码是否有效
/// 
/// KernelSU规范：
/// - versionCode 必须是一个正整数
/// - 用于版本比较，数值越大表示版本越新
/// - 范围：1 到 2,147,483,647 (i32::MAX)
pub fn is_valid_version_code(version_code: i32) -> bool {
    version_code > 0
}

/// 检查版本字符串是否有效
/// 
/// 建议格式：
/// - v1.0.0
/// - 1.0.0
/// - v1.0
/// - 1.0
/// 不允许多行或包含控制字符
pub fn is_valid_version_string(version: &str) -> bool {
    if version.is_empty() || version.len() > 32 {
        return false;
    }
    
    // 不能包含换行符或控制字符
    if version.contains('\n') || version.contains('\r') || 
       version.chars().any(|c| c.is_control()) {
        return false;
    }
    
    // 不能只包含空格
    if version.trim().is_empty() {
        return false;
    }
    
    true
}

/// 检查模块名称是否有效
/// 
/// 规范：
/// - 不能为空
/// - 不能包含换行符
/// - 长度限制：1-100个字符
/// - 不能只包含空格
pub fn is_valid_module_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 100 {
        return false;
    }
    
    // 不能包含换行符
    if name.contains('\n') || name.contains('\r') {
        return false;
    }
    
    // 不能只包含空格
    if name.trim().is_empty() {
        return false;
    }
    
    true
}

/// 检查模块描述是否有效
/// 
/// 规范：
/// - 可以为空
/// - 不能包含换行符
/// - 长度限制：0-500个字符
pub fn is_valid_module_description(description: &str) -> bool {
    if description.len() > 500 {
        return false;
    }
    
    // 不能包含换行符
    if description.contains('\n') || description.contains('\r') {
        return false;
    }
    
    true
}

/// 检查作者名称是否有效
/// 
/// 规范：
/// - 不能为空
/// - 不能包含换行符
/// - 长度限制：1-100个字符
/// - 不能只包含空格
pub fn is_valid_author_name(author: &str) -> bool {
    if author.is_empty() || author.len() > 100 {
        return false;
    }
    
    // 不能包含换行符
    if author.contains('\n') || author.contains('\r') {
        return false;
    }
    
    // 不能只包含空格
    if author.trim().is_empty() {
        return false;
    }
    
    true
}

/// 验证完整的模块属性
/// 
/// 返回验证结果和错误信息
pub fn validate_module_props(
    id: &str,
    name: &str,
    version: &str,
    version_code: i32,
    author: &str,
    description: &str,
) -> Result<(), String> {
    if !is_valid_module_id(id) {
        return Err(format!(
            "无效的模块ID '{}': 必须以字母开头，只能包含字母、数字、点(.)、下划线(_)、连字符(-)",
            id
        ));
    }
    
    if !is_valid_module_name(name) {
        return Err(format!(
            "无效的模块名称 '{}': 不能为空，不能包含换行符，长度不能超过100个字符",
            name
        ));
    }
    
    if !is_valid_version_string(version) {
        return Err(format!(
            "无效的版本字符串 '{}': 不能为空，不能包含换行符，长度不能超过32个字符",
            version
        ));
    }
    
    if !is_valid_version_code(version_code) {
        return Err(format!(
            "无效的版本代码 '{}': 必须是正整数",
            version_code
        ));
    }
    
    if !is_valid_author_name(author) {
        return Err(format!(
            "无效的作者名称 '{}': 不能为空，不能包含换行符，长度不能超过100个字符",
            author
        ));
    }
    
    if !is_valid_module_description(description) {
        return Err(format!(
            "无效的模块描述: 不能包含换行符，长度不能超过500个字符"
        ));
    }
    
    Ok(())
}

/// 为了向后兼容，保留原函数名但使用新的验证逻辑
#[deprecated(note = "请使用 is_valid_module_id 替代")]
pub fn is_safe_filename(filename: &str) -> bool {
    is_valid_module_id(filename)
}
