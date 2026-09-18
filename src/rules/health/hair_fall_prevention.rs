//! 防脱发护理
//!
//! 因人认因少掉发、保护头皮与毛发的日常护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HairFallPreventionRules,
    name: "防脱发护理",
    desc: "因人认因少掉发、保护头皮与毛发的日常护理",
    origin: "中国",
    tags: ["健康", "头发", "脱发", "护理"]
}

impl HairFallPreventionRules {
    /// 正常掉发
    pub fn know(&self) -> Vec<&'static str> {
        vec![
            "日掉百根属正常",
            "季节性更替",
            "洗护掉落正常",
            "过多才需留意",
        ]
    }

    /// 头皮护理
    pub fn scalp(&self) -> Vec<&'static str> {
        vec!["温和洗发", "适量按摩头皮", "勿过频过猛", "护养发根环境"]
    }

    /// 习惯调整
    pub fn habit(&self) -> Vec<&'static str> {
        vec!["避网强力拉扯", "少烫染", "均衡营养", "作息规律减压"]
    }

    /// 就医提示
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec![
            "头发明显一片掉落",
            "斑秃成片",
            "伴随头皮疼痛",
            "及早皮肤科查",
        ]
    }
}

impl Rule for HairFallPreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hair_fall")
    }

    fn explain(&self) -> String {
        format!(
            "【防脱发护理】\n{}",
            [
                format!(
                    "正常掉发：\\n{}",
                    self.know()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "头皮护理：\\n{}",
                    self.scalp()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "习惯调整：\\n{}",
                    self.habit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek_help()
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
    fn test_hairfallpreventionrules_basic() {
        let rules = HairFallPreventionRules::new();
        assert_eq!(rules.metadata().name, "防脱发护理");
        assert!(!rules.know().is_empty());
        assert!(!rules.scalp().is_empty());
        assert!(!rules.habit().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_hairfallpreventionrules_validation() {
        let rules = HairFallPreventionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hair_fall"));
    }

    #[test]
    fn test_hairfallpreventionrules_explain() {
        let rules = HairFallPreventionRules::new();
        let e = rules.explain();
        assert!(e.contains("正常掉发"));
        assert!(e.contains("头皮护理"));
        assert!(e.contains("习惯调整"));
    }
}
