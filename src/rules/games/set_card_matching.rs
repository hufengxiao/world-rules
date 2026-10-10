//! SET 卡片游戏
//!
//! SET 是一种考验观察力的卡片游戏，玩家需从桌面卡牌中找出三张在颜色、形状、数量、填充四个属性上「全同或全不同」的牌组成一组。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SetCardMatchingRules,
    name: "SET 卡片游戏",
    desc: "SET 是一种考验观察力的卡片游戏，玩家需从桌面卡牌中找出三张在颜色、形状、数量、填充四个属性上「全同或全不同」的牌组成一组。",
    origin: "由遗传学家 Marsha Falco 于1974年发明",
    tags: ["卡牌", "观察", "匹配", "桌面游戏"]
}

impl SetCardMatchingRules {
    /// 卡牌属性
    pub fn cards(&self) -> Vec<&'static str> {
        vec![
            "每张牌有四个独立属性",
            "颜色分红绿紫三色",
            "形状分菱形波浪椭圆",
            "数量为一到三个且填充分实心空心条纹",
        ]
    }

    /// 成组条件
    pub fn matching(&self) -> Vec<&'static str> {
        vec![
            "三张牌每一属性须全同或全不同",
            "四个属性须同时满足条件",
            "任意两两组合不得部分相同",
            "符合条件的三张即为一组",
        ]
    }

    /// 游戏流程
    pub fn play(&self) -> Vec<&'static str> {
        vec![
            "桌面摆出十二张明牌",
            "玩家找出并喊出SET组合",
            "确认后取走该组牌",
            "从牌堆补足桌面牌数",
        ]
    }

    /// 计分与结束
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每找出一组得一分",
            "找错组须扣分或暂停",
            "牌堆用尽且桌面无解时结束",
            "得分最高者获胜",
        ]
    }
}

impl Rule for SetCardMatchingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("set_card")
    }

    fn explain(&self) -> String {
        format!(
            "【SET 卡片游戏】\n{}",
            [
                format!(
                    "卡牌属性：\\n{}",
                    self.cards()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "成组条件：\\n{}",
                    self.matching()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "游戏流程：\\n{}",
                    self.play()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分与结束：\\n{}",
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
    fn test_setcardmatchingrules_basic() {
        let rules = SetCardMatchingRules::new();
        assert_eq!(rules.metadata().name, "SET 卡片游戏");
        assert!(!rules.cards().is_empty());
        assert!(!rules.matching().is_empty());
        assert!(!rules.play().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_setcardmatchingrules_validation() {
        let rules = SetCardMatchingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("set_card"));
    }

    #[test]
    fn test_setcardmatchingrules_explain() {
        let rules = SetCardMatchingRules::new();
        let e = rules.explain();
        assert!(e.contains("卡牌属性"));
        assert!(e.contains("成组条件"));
        assert!(e.contains("游戏流程"));
    }
}
