/// Generate post-mount.sh template for Magisk module
/// This script runs after mount namespace is created
pub fn generate_post_mount_sh() -> String {
    r#"#!/system/bin/sh
# 以 post-mount 模式运行
"#.to_string()
}
