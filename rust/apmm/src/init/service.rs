/// Generate service.sh template for Magisk module
/// This script runs in late_start service mode with full access to system APIs
pub fn generate_service_sh() -> String {
    r#"#!/system/bin/sh
# 则以 late_start 服务模式运行
"#.to_string()
}