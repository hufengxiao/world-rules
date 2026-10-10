//! 惠斯特
//!
//! 桥牌前身的经典吃墩牌戏，以将牌与配合取胜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WhistCardRules,
    name: "惠斯特",
    desc: "桥牌前身的经典吃墩牌戏，以将牌与配合取胜",
    origin: "英国",
    tags: ["扑克", "吃墩", "传统"]
}

impl WhistCardRules {
    /// 发牌准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "四人两两结对成两队",
            "每人发十三张牌",
            "最后一张翻定将牌",
            "庄家发牌从左起",
        ]
    }

    /// 出牌吃墩
    pub fn trick(&self) -> Vec<&'static str> {
        vec![
            "领出者可出任意花色",
            "其余人须跟同花色",
            "无该花色可出任意牌",
            "将牌可吃非将牌",
        ]
    }

    /// 赢得墩数
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "同花色点数大者得墩",
            "将牌压非将牌",
            "得墩者领出下一轮",
            "共十三墩分胜负",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每超六墩计一分",
            "双方各自计数",
            "先达约定分数者胜",
            "分多者为本局赢家",
        ]
    }
}

impl Rule for WhistCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("whist_card")
    }

    fn explain(&self) -> String {
        format!(
            "【惠斯特】\n{}",
            [
                format!(
                    "发牌准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "出牌吃墩：\\n{}",
                    self.trick()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赢得墩数：\\n{}",
                    self.winning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分方式：\\n{}",
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
    fn test_whistcardrules_basic() {
        let rules = WhistCardRules::new();
        assert_eq!(rules.metadata().name, "惠斯特");
        assert!(!rules.setup().is_empty());
        assert!(!rules.trick().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_whistcardrules_validation() {
        let rules = WhistCardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("whist_card"));
    }

    #[test]
    fn test_whistcardrules_explain() {
        let rules = WhistCardRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌准备"));
        assert!(e.contains("出牌吃墩"));
        assert!(e.contains("赢得墩数"));
    }
}
