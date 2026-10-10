//! 湖北麻将
//!
//! 湖北地区麻将玩法，含赖子与红中等特色规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MahjongHubeiRules,
    name: "湖北麻将",
    desc: "湖北地区麻将玩法，含赖子与红中等特色规则",
    origin: "中国",
    tags: ["麻将", "湖北", "变体"]
}

impl MahjongHubeiRules {
    /// 牌张特色
    pub fn tiles(&self) -> Vec<&'static str> {
        vec![
            "使用万筒索风箭",
            "红中常作特殊牌",
            "可设赖子代牌",
            "赖子翻牌而定",
        ]
    }

    /// 赖子规则
    pub fn laizi(&self) -> Vec<&'static str> {
        vec![
            "翻一张定赖子牌",
            "赖子可代任意牌",
            "赖子本身不可打出",
            "用作赖子时计特殊番",
        ]
    }

    /// 胡牌牌型
    pub fn patterns(&self) -> Vec<&'static str> {
        vec![
            "基本四组一对",
            "碰碰胡清一色",
            "杠上开花海底捞月",
            "须达起胡标准",
        ]
    }

    /// 计分结算
    pub fn scoring(&self) -> Vec<&'static str> {
        vec!["按番数翻倍计", "红中赖子另计", "自摸加番", "杠牌单独计分"]
    }
}

impl Rule for MahjongHubeiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hubei")
    }

    fn explain(&self) -> String {
        format!(
            "【湖北麻将】\n{}",
            [
                format!(
                    "牌张特色：\\n{}",
                    self.tiles()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赖子规则：\\n{}",
                    self.laizi()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胡牌牌型：\\n{}",
                    self.patterns()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分结算：\\n{}",
                    self.scoring()
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
    fn test_mahjonghubeirules_basic() {
        let rules = MahjongHubeiRules::new();
        assert_eq!(rules.metadata().name, "湖北麻将");
        assert!(!rules.tiles().is_empty());
        assert!(!rules.laizi().is_empty());
        assert!(!rules.patterns().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_mahjonghubeirules_validation() {
        let rules = MahjongHubeiRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("mahjong_hubei"));
    }

    #[test]
    fn test_mahjonghubeirules_explain() {
        let rules = MahjongHubeiRules::new();
        let e = rules.explain();
        assert!(e.contains("牌张特色"));
        assert!(e.contains("赖子规则"));
        assert!(e.contains("胡牌牌型"));
    }
}
