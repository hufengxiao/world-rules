//! 斯卡特
//!
//! 德国国民牌戏，三人竞叫定牌型的吃墩游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SkatCardRules,
    name: "斯卡特",
    desc: "德国国民牌戏，三人竞叫定牌型的吃墩游戏",
    origin: "德国",
    tags: ["扑克", "吃墩", "德国"]
}

impl SkatCardRules {
    /// 发牌准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "使用三十二张德国牌",
            "三人各发十张牌",
            "两张牌作斯卡特底牌",
            "轮流坐庄发牌",
        ]
    }

    /// 竞叫定约
    pub fn bidding(&self) -> Vec<&'static str> {
        vec![
            "竞叫争取当庄家",
            "庄家可看底牌换牌",
            "依牌型定分数目标",
            "竞叫依牌分值报价",
        ]
    }

    /// 牌型玩法
    pub fn variants(&self) -> Vec<&'static str> {
        vec![
            "分花色牌型与无将",
            "杰克为最高牌",
            "依约定玩大满贯",
            "手牌或底牌玩法",
        ]
    }

    /// 计分胜负
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "庄家达目标分得分",
            "未达则扣分",
            "闲家合力阻止庄家",
            "累计总分定胜负",
        ]
    }
}

impl Rule for SkatCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("skat_card")
    }

    fn explain(&self) -> String {
        format!(
            "【斯卡特】\n{}",
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
                    "竞叫定约：\\n{}",
                    self.bidding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "牌型玩法：\\n{}",
                    self.variants()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分胜负：\\n{}",
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
    fn test_skatcardrules_basic() {
        let rules = SkatCardRules::new();
        assert_eq!(rules.metadata().name, "斯卡特");
        assert!(!rules.setup().is_empty());
        assert!(!rules.bidding().is_empty());
        assert!(!rules.variants().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_skatcardrules_validation() {
        let rules = SkatCardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("skat_card"));
    }

    #[test]
    fn test_skatcardrules_explain() {
        let rules = SkatCardRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌准备"));
        assert!(e.contains("竞叫定约"));
        assert!(e.contains("牌型玩法"));
    }
}
