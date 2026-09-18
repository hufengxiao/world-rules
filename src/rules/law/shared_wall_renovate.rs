//! 共有墙体使用
//!
//! 装修不拆承重墙与共有部分的使用

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SharedWallRenovateRules,
    name: "共有墙体使用",
    desc: "装修不拆承重墙与共有部分的使用",
    origin: "中国",
    tags: ["法律", "装修", "承重墙", "共墙"]
}

impl SharedWallRenovateRules {
    /// 不拆结构
    pub fn structure(&self) -> Vec<&'static str> {
        vec!["不拆承重墙", "不改承重结构", "动墙先报批", "安全为先"]
    }

    /// 装修报批
    pub fn approve(&self) -> Vec<&'static str> {
        vec!["装修向物业报", "办备案", "请专业施工", "依规作业"]
    }

    /// 共用互让
    pub fn shared(&self) -> Vec<&'static str> {
        vec!["外墙不擅自改造", "公共区域不乱占", "共用协商", "邻里和谐"]
    }

    /// 损害担责
    pub fn liability(&self) -> Vec<&'static str> {
        vec!["损坏他墙要赔", "结构受损担责", "及时修复", "依法处理"]
    }
}

impl Rule for SharedWallRenovateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("shared_wall")
    }

    fn explain(&self) -> String {
        format!(
            "【共有墙体使用】\n{}",
            [
                format!(
                    "不拆结构：\\n{}",
                    self.structure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "装修报批：\\n{}",
                    self.approve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共用互让：\\n{}",
                    self.shared()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "损害担责：\\n{}",
                    self.liability()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_sharedwallrenovaterules_basic() {
        let rules = SharedWallRenovateRules::new();
        assert_eq!(rules.metadata().name, "共有墙体使用");
        assert!(!rules.structure().is_empty());
        assert!(!rules.approve().is_empty());
        assert!(!rules.shared().is_empty());
        assert!(!rules.liability().is_empty());
    }

    #[test]
    fn test_sharedwallrenovaterules_validation() {
        let rules = SharedWallRenovateRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("shared_wall"));
    }

    #[test]
    fn test_sharedwallrenovaterules_explain() {
        let rules = SharedWallRenovateRules::new();
        let e = rules.explain();
        assert!(e.contains("不拆结构"));
        assert!(e.contains("装修报批"));
        assert!(e.contains("共用互让"));
    }
}
