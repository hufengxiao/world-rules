//! UNO牌通用规则
//!
//! UNO出牌、功能牌与计分的通用规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: UnoHouseRules,
    name: "UNO牌通用规则",
    desc: "UNO出牌、功能牌与计分的通用规则",
    origin: "国际",
    tags: ["游戏", "UNO", "纸牌"]
}

impl UnoHouseRules {
    /// 出牌原则
    pub fn play(&self) -> Vec<&'static str> {
        vec![
            "轮到自己出牌",
            "匹配颜色或数字",
            "无法出牌抓一张",
            "抽到能出则出",
        ]
    }

    /// 功能牌
    pub fn action(&self) -> Vec<&'static str> {
        vec![
            "反转改变出牌方向",
            "跳过令下家失去回合",
            "罚二抽两张",
            "万能牌任挑颜色",
        ]
    }

    /// 叫UNO
    pub fn call_uno(&self) -> Vec<&'static str> {
        vec![
            "剩最后一张喊UNO",
            "忘喊被罚抽牌",
            "喊出声清晰响亮",
            "规则事先约定",
        ]
    }

    /// 计分结算
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "先出完手牌者胜轮",
            "数字牌记面值",
            "功能牌赋分值",
            "多轮累计定胜负",
        ]
    }
}

impl Rule for UnoHouseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("uno")
    }

    fn explain(&self) -> String {
        format!(
            "【UNO牌通用规则】\n{}",
            [
                format!(
                    "出牌原则：\\n{}",
                    self.play()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "功能牌：\\n{}",
                    self.action()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "叫UNO：\\n{}",
                    self.call_uno()
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
    fn test_unohouserules_basic() {
        let rules = UnoHouseRules::new();
        assert_eq!(rules.metadata().name, "UNO牌通用规则");
        assert!(!rules.play().is_empty());
        assert!(!rules.action().is_empty());
        assert!(!rules.call_uno().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_unohouserules_validation() {
        let rules = UnoHouseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("uno"));
    }

    #[test]
    fn test_unohouserules_explain() {
        let rules = UnoHouseRules::new();
        let e = rules.explain();
        assert!(e.contains("出牌原则"));
        assert!(e.contains("功能牌"));
        assert!(e.contains("叫UNO"));
    }
}
