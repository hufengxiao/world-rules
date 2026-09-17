//! 自我慈悲与接纳
//!
//! 接纳自我、善待自己的成长心态与情绪调节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SelfCompassionRules,
    name: "自我慈悲与接纳",
    desc: "接纳自我、善待自己的成长心态与情绪调节",
    origin: "心理学",
    tags: ["健康", "自我慈悲", "接纳", "心理"]
}

impl SelfCompassionRules {
    /// 善待自己
    pub fn kindness(&self) -> Vec<&'static str> {
        vec![
            "犯错时对自己温和",
            "以朋友口吻鼓励",
            "不过度自我苛责",
            "肯定自己的努力",
        ]
    }

    /// 接纳不足
    pub fn accept(&self) -> Vec<&'static str> {
        vec![
            "承认人无完人",
            "接纳情绪与失误",
            "把挫折当学习",
            "不因瑕疵否定自我",
        ]
    }

    /// 共通人性
    pub fn common_humanity(&self) -> Vec<&'static str> {
        vec![
            "认识到人人会犯错",
            "不独自背负羞耻",
            "把困难视为普遍",
            "减少孤独与自责",
        ]
    }

    /// 平衡心态
    pub fn balance(&self) -> Vec<&'static str> {
        vec![
            "理性看待赞扬批评",
            "设定现实目标",
            "适度自我照顾",
            "成长离不开接纳",
        ]
    }
}

impl Rule for SelfCompassionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("self_compassion")
    }

    fn explain(&self) -> String {
        format!(
            "【自我慈悲与接纳】\n{}",
            [
                format!(
                    "善待自己：\\n{}",
                    self.kindness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "接纳不足：\\n{}",
                    self.accept()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共通人性：\\n{}",
                    self.common_humanity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平衡心态：\\n{}",
                    self.balance()
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
    fn test_selfcompassionrules_basic() {
        let rules = SelfCompassionRules::new();
        assert_eq!(rules.metadata().name, "自我慈悲与接纳");
        assert!(!rules.kindness().is_empty());
        assert!(!rules.accept().is_empty());
        assert!(!rules.common_humanity().is_empty());
        assert!(!rules.balance().is_empty());
    }

    #[test]
    fn test_selfcompassionrules_validation() {
        let rules = SelfCompassionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("self_compassion"));
    }

    #[test]
    fn test_selfcompassionrules_explain() {
        let rules = SelfCompassionRules::new();
        let e = rules.explain();
        assert!(e.contains("善待自己"));
        assert!(e.contains("接纳不足"));
        assert!(e.contains("共通人性"));
    }
}
