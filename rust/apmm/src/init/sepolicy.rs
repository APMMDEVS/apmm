/// Generate sepolicy.rule template for Magisk module
/// This file contains SELinux policy rules for the module
pub fn generate_sepolicy_rule() -> String {
    r#"# SELinux policy rules for this Magisk module
# More info at https://topjohnwu.github.io/Magisk/guides.html

# Example rules (uncomment and modify as needed):
# allow untrusted_app system_file:file { read open };
# allow system_app system_file:file { write };
# permissive my_domain;

# Common rules for Magisk modules:
# allow { domain } { domain }:{ class } { permissions };

# Your custom SELinux rules here
"#.to_string()
}
