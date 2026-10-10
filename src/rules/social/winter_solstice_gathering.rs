//! 冬至团聚礼仪
//!
//! 冬至节家人团聚吃饺子汤圆的习俗与礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WinterSolsticeGatheringRules,
    name: "冬至团聚礼仪",
    desc: "冬至节家人团聚吃饺子汤圆的习俗与礼节",
    origin: "中国",
    tags: ["节庆", "传统", "团聚"]
}

impl WinterSolsticeGatheringRules {
    /// 节令习俗
    pub fn customs(&self) -> Vec<&'static str> {
        vec![
            "冬至大如年之说",
            "北方吃饺子防冻耳",
            "南方吃汤圆庆团圆",
            "祭祖祈福感念先人",
        ]
    }

    /// 家人团聚
    pub fn reunion(&self) -> Vec<&'static str> {
        vec![
            "尽量回家与家人团聚",
            "长辈张罗节令饮食",
            "晚辈帮忙备餐",
            "共话家常叙亲情",
        ]
    }

    /// 餐桌礼仪
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "长者先动筷",
            "为长辈盛饭添菜",
            "不挑食不浪费",
            "饭后主动收拾",
        ]
    }

    /// 人情往来
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "问候远方亲友",
            "送上冬至祝福",
            "互赠节令食品",
            "以温情御寒冬",
        ]
    }
}

impl Rule for WinterSolsticeGatheringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("winter_solstice_gathering")
    }

    fn explain(&self) -> String {
        format!(
            "【冬至团聚礼仪】\n{}",
            [
                format!(
                    "节令习俗：\\n{}",
                    self.customs()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家人团聚：\\n{}",
                    self.reunion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "餐桌礼仪：\\n{}",
                    self.dining()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "人情往来：\\n{}",
                    self.greetings()
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
    fn test_wintersolsticegatheringrules_basic() {
        let rules = WinterSolsticeGatheringRules::new();
        assert_eq!(rules.metadata().name, "冬至团聚礼仪");
        assert!(!rules.customs().is_empty());
        assert!(!rules.reunion().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.greetings().is_empty());
    }

    #[test]
    fn test_wintersolsticegatheringrules_validation() {
        let rules = WinterSolsticeGatheringRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("winter_solstice_gathering")
        );
    }

    #[test]
    fn test_wintersolsticegatheringrules_explain() {
        let rules = WinterSolsticeGatheringRules::new();
        let e = rules.explain();
        assert!(e.contains("节令习俗"));
        assert!(e.contains("家人团聚"));
        assert!(e.contains("餐桌礼仪"));
    }
}
