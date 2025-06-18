/// 生成 customize.sh 脚本内容
pub fn generate_customize_sh(module_id: &str) -> String {
    format!(r#"#!/system/bin/sh

ui_print "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
ui_print "   Module: {}"
ui_print "   Author: APMM Team"
ui_print "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 验证安卓版本
if [ "$API" -lt 21 ]; then
    ui_print "! Unsupported Android version: $API"
    ui_print "! Minimum supported version: Android 5.0 (API 21)"
    abort "! Installation aborted"
fi

# 检查架构
case "$ARCH" in
    arm)
        ui_print "- Device architecture: $ARCH"
        ;;
    arm64)
        ui_print "- Device architecture: $ARCH"
        ;;
    x86)
        ui_print "- Device architecture: $ARCH"
        ;;
    x64)
        ui_print "- Device architecture: $ARCH"
        ;;
    *)
        ui_print "! Unsupported architecture: $ARCH"
        abort "! Installation aborted"
        ;;
esac

# 安装逻辑
ui_print "- Installing module files..."

# 设置权限
ui_print "- Setting permissions..."
set_perm_recursive $MODPATH 0 0 0755 0644

# 安装完成
ui_print "- Installation completed!"
ui_print "- Module will be activated after reboot"
ui_print ""
"#, module_id)
}

