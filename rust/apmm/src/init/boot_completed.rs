/// Generate boot-completed.sh template for Magisk module
/// This script runs when the system has finished booting
pub fn generate_boot_completed_sh() -> String {
    r#"#!/system/bin/sh
# 在 Android 系统启动完毕后以服务模式运行。
"#.to_string()
}
