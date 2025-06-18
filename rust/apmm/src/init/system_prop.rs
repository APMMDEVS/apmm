/// Generate system.prop template for Magisk module
/// This file contains system properties to be set by the module
pub fn generate_system_prop() -> String {
    r#"# System properties for this Magisk module
# More info at https://topjohnwu.github.io/Magisk/guides.html

# Example properties (uncomment and modify as needed):
# ro.my.module.version=1.0
# persist.my.module.enabled=true
# debug.my.module.log=false

# Common system properties:
# ro.build.fingerprint=custom
# ro.product.model=Modified Device
# persist.vendor.camera.privapp.list=*

# Your custom properties here
"#.to_string()
}
