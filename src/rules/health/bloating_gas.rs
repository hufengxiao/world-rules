//! 腹胀排气护理
//!
//! 腹部胀气与排气过多的饮食调整与缓解方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BloatingGasRules,
    name: "腹胀排气护理",
    desc: "腹部胀气与排气过多的饮食调整与缓解方法",
    origin: "中国",
    tags: ["消化", "胀气", "护理"]
}

impl BloatingGasRules {
    /// 产气食物
    pub fn trigger_food(&self) -> Vec<&'static str> {
        vec![
            "豆类薯类易产气",
            "洋葱韭菜萝卜慎食",
            "碳酸饮料含气饮入",
            "乳制品乳糖不耐者慎用",
        ]
    }

    /// 进食方式
    pub fn eating(&self) -> Vec<&'static str> {
        vec![
            "细嚼慢咽减少吞气",
            "避免用吸管快速饮水",
            "少嚼口香糖少含硬糖",
            "勿边吃边说大口吞咽",
        ]
    }

    /// 缓解方法
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "顺时针按摩腹部助排气",
            "适度走动促进肠蠕动",
            "热敷腹部放松肠道",
            "薄荷茶茴香茶助舒缓",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "持续胀痛须查因",
            "伴呕吐便秘发热就诊",
            "体重下降应排查",
            "反复发作可查食物不耐受",
        ]
    }
}

impl Rule for BloatingGasRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("bloating_gas")
    }

    fn explain(&self) -> String {
        format!(
            "【腹胀排气护理】\n{}",
            [
                format!(
                    "产气食物：\\n{}",
                    self.trigger_food()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "进食方式：\\n{}",
                    self.eating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "缓解方法：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_bloatinggasrules_basic() {
        let rules = BloatingGasRules::new();
        assert_eq!(rules.metadata().name, "腹胀排气护理");
        assert!(!rules.trigger_food().is_empty());
        assert!(!rules.eating().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_bloatinggasrules_validation() {
        let rules = BloatingGasRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("bloating_gas"));
    }

    #[test]
    fn test_bloatinggasrules_explain() {
        let rules = BloatingGasRules::new();
        let e = rules.explain();
        assert!(e.contains("产气食物"));
        assert!(e.contains("进食方式"));
        assert!(e.contains("缓解方法"));
    }
}
