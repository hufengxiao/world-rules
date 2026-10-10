//! 总统牌
//!
//! 按牌力出牌抢当总统的爬梯式扑克游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PresidentCardRules,
    name: "总统牌",
    desc: "按牌力出牌抢当总统的爬梯式扑克游戏",
    origin: "中国",
    tags: ["扑克", "出牌", "排名"]
}

impl PresidentCardRules {
    /// 发牌开局
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "整副牌均分给各玩家",
            "先出完牌者获高名次",
            "名次分总统副总统平民",
            "末位者称平民或乞丐",
        ]
    }

    /// 出牌规则
    pub fn playing(&self) -> Vec<&'static str> {
        vec![
            "可出单张对子三条或顺子",
            "张数与牌型须与上家一致",
            "牌力更大方能压过",
            "炸弹可压制任意牌型",
        ]
    }

    /// 过牌与接管
    pub fn passing(&self) -> Vec<&'static str> {
        vec![
            "无法或不愿压牌可过",
            "其余人都过则由最后出牌者续出",
            "可出任意牌型重新开始",
            "出完手牌者退出本轮",
        ]
    }

    /// 名次特权
    pub fn privilege(&self) -> Vec<&'static str> {
        vec![
            "下局总统先出牌",
            "平民须向总统进贡最大牌",
            "总统回赠任意牌",
            "连续当平民者另定规则",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "全部出完牌确定名次",
            "首位出完者为总统",
            "末位者为平民",
            "可多局累计排名",
        ]
    }
}

impl Rule for PresidentCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("president_card")
    }

    fn explain(&self) -> String {
        format!(
            "【总统牌】\n{}",
            [
                format!(
                    "发牌开局：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "出牌规则：\\n{}",
                    self.playing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "过牌与接管：\\n{}",
                    self.passing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "名次特权：\\n{}",
                    self.privilege()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_presidentcardrules_basic() {
        let rules = PresidentCardRules::new();
        assert_eq!(rules.metadata().name, "总统牌");
        assert!(!rules.setup().is_empty());
        assert!(!rules.playing().is_empty());
        assert!(!rules.passing().is_empty());
        assert!(!rules.privilege().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_presidentcardrules_validation() {
        let rules = PresidentCardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("president_card"));
    }

    #[test]
    fn test_presidentcardrules_explain() {
        let rules = PresidentCardRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌开局"));
        assert!(e.contains("出牌规则"));
        assert!(e.contains("过牌与接管"));
    }
}
