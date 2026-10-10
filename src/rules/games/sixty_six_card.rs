//! 六十六
//!
//! 欧洲流行的两人吃墩牌戏，先得六十六分者胜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SixtySixCardRules,
    name: "六十六",
    desc: "欧洲流行的两人吃墩牌戏，先得六十六分者胜",
    origin: "德国",
    tags: ["扑克", "吃墩", "两人"]
}

impl SixtySixCardRules {
    /// 发牌准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "使用二十四张牌",
            "每人发六张手牌",
            "翻一张定将牌",
            "剩余作牌堆",
        ]
    }

    /// 出牌规则
    pub fn playing(&self) -> Vec<&'static str> {
        vec![
            "每人每次出两张",
            "可出对子或单张",
            "须跟同花色",
            "无法跟可出他牌",
        ]
    }

    /// 得分牌
    pub fn points(&self) -> Vec<&'static str> {
        vec![
            "A计十一分十计十分",
            "K计四分Q三分J两分",
            "将牌QK成对为婚姻",
            "婚姻出牌得四十分",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "先达六十六分者胜",
            "出完末墩额外加分",
            "先赢七局者获全局",
            "未达分者不得局分",
        ]
    }
}

impl Rule for SixtySixCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("sixty_six_card")
    }

    fn explain(&self) -> String {
        format!(
            "【六十六】\n{}",
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
                    "得分牌：\\n{}",
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
    fn test_sixtysixcardrules_basic() {
        let rules = SixtySixCardRules::new();
        assert_eq!(rules.metadata().name, "六十六");
        assert!(!rules.setup().is_empty());
        assert!(!rules.playing().is_empty());
        assert!(!rules.points().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_sixtysixcardrules_validation() {
        let rules = SixtySixCardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("sixty_six_card"));
    }

    #[test]
    fn test_sixtysixcardrules_explain() {
        let rules = SixtySixCardRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌准备"));
        assert!(e.contains("出牌规则"));
        assert!(e.contains("得分牌"));
    }
}
