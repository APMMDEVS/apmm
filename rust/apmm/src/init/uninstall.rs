

/// 生成 uninstall.sh 脚本内容（卸载时执行）
pub fn generate_uninstall_sh(module_id: &str) -> String {
    format!(r#"#!/system/bin/sh

# APMM Module: {}
# This script will be executed during module uninstallation

# Log file
LOG_FILE="/data/adb/modules/{}/uninstall.log"

# Ensure log directory exists
mkdir -p "$(dirname "$LOG_FILE")"

# Log start
echo "$(date): Uninstalling module {}" > "$LOG_FILE"

# Add your cleanup logic here
# Example:
# rm -rf /data/local/tmp/{}
# echo "$(date): Cleaned up temporary files" >> "$LOG_FILE"

# Log completion
echo "$(date): Module {} uninstalled successfully" >> "$LOG_FILE"

ui_print "Module {} uninstalled successfully!"
"#, module_id, module_id, module_id, module_id, module_id, module_id)
}
