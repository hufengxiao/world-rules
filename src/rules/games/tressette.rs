//! 特雷塞特
//!
//! 意大利传统吃墩牌戏，以吃牌点数累计计分

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TressetteRules,
    name: "特雷塞特",
    desc: "意大利传统吃墩牌戏，以吃牌点数累计计分",
    origin: "意大利",
    tags: ["扑克", "吃墩", "意大利"]
}

impl TressetteRules {
    /// 发牌准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "使用意大利四十张牌",
            "四人两两结对",
            "每人发十张牌",
            "不留底牌",
        ]
    }

    /// 出牌规则
    pub fn playing(&self) -> Vec<&'static str> {
        vec![
            "领出者可出任意牌",
            "须跟同花色",
            "无该花色可出任意牌",
            "不得主动出高分牌为限",
        ]
    }

    /// 计分牌值
    pub fn points(&self) -> Vec<&'static str> {
        vec![
            "A计一分三张人头牌各计三分之一",
            "KQJ各计三分一",
            "数字牌不计分",
            "总分为一百二十分",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "累计吃牌点数计分",
            "分高一方获胜",
            "先达约定局分者胜",
            "可多人循环对战",
        ]
    }
}

impl Rule for TressetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("tressette")
    }

    fn explain(&self) -> String {
        format!(
            "【特雷塞特】\n{}",
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
                    "出牌规则：\\n{}",
                    self.playing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分牌值：\\n{}",
                    self.points()
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
    fn test_tressetterules_basic() {
        let rules = TressetteRules::new();
        assert_eq!(rules.metadata().name, "特雷塞特");
        assert!(!rules.setup().is_empty());
        assert!(!rules.playing().is_empty());
        assert!(!rules.points().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_tressetterules_validation() {
        let rules = TressetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("tressette"));
    }

    #[test]
    fn test_tressetterules_explain() {
        let rules = TressetteRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌准备"));
        assert!(e.contains("出牌规则"));
        assert!(e.contains("计分牌值"));
    }
}
