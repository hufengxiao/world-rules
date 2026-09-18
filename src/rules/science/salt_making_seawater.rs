//! 海水晒盐
//!
//! 海水晒盐蒸发结晶的原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SaltMakingSeawaterRules,
    name: "海水晒盐",
    desc: "海水晒盐蒸发结晶的原理",
    origin: "化学",
    tags: ["科学", "盐", "海水", "结晶"]
}

impl SaltMakingSeawaterRules {
    /// 晒盐原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec!["海水含盐分", "阳光风蒸发水分", "盐分浓度渐高", "过饱和析盐"]
    }

    /// 蒸发结晶
    pub fn crystallize(&self) -> Vec<&'static str> {
        vec!["盐田浅水蒸发", "水分跑出", "盐固体析出", "收盐晒干"]
    }

    /// 应用生活
    pub fn life(&self) -> Vec<&'static str> {
        vec!["食盐调味", "腌制保鲜", "海水淡化另法", "资源利用"]
    }

    /// 保护资源
    pub fn awareness(&self) -> Vec<&'static str> {
        vec!["海盐取用适量", "不浪费", "环境可持续", "循环利用"]
    }
}

impl Rule for SaltMakingSeawaterRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("salt_making")
    }

    fn explain(&self) -> String {
        format!(
            "【海水晒盐】\n{}",
            [
                format!(
                    "晒盐原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "蒸发结晶：\\n{}",
                    self.crystallize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应用生活：\\n{}",
                    self.life()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保护资源：\\n{}",
                    self.awareness()
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
    fn test_saltmakingseawaterrules_basic() {
        let rules = SaltMakingSeawaterRules::new();
        assert_eq!(rules.metadata().name, "海水晒盐");
        assert!(!rules.principle().is_empty());
        assert!(!rules.crystallize().is_empty());
        assert!(!rules.life().is_empty());
        assert!(!rules.awareness().is_empty());
    }

    #[test]
    fn test_saltmakingseawaterrules_validation() {
        let rules = SaltMakingSeawaterRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("salt_making"));
    }

    #[test]
    fn test_saltmakingseawaterrules_explain() {
        let rules = SaltMakingSeawaterRules::new();
        let e = rules.explain();
        assert!(e.contains("晒盐原理"));
        assert!(e.contains("蒸发结晶"));
        assert!(e.contains("应用生活"));
    }
}
