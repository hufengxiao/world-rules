//! 高尔夫扑克牌
//!
//! 以低分为目标翻牌配对的简化扑克游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GolfCardGameRules,
    name: "高尔夫扑克牌",
    desc: "以低分为目标翻牌配对的简化扑克游戏",
    origin: "美国",
    tags: ["扑克", "配对", "记分"]
}

impl GolfCardGameRules {
    /// 发牌布局
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "每人发六张牌摆两行三列",
            "牌面朝下初始翻开两张",
            "牌库剩余作摸牌堆",
            "翻开一张作弃牌堆起牌",
        ]
    }

    /// 回合行动
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "轮到时可选摸牌或取弃牌",
            "取牌后与手上一张交换",
            "换下的牌放入弃牌堆",
            "亦可翻摸牌堆顶牌",
        ]
    }

    /// 配对消牌
    pub fn matching(&self) -> Vec<&'static str> {
        vec![
            "上下相邻同点数可配对",
            "配对后两张移出不再计分",
            "配对牌放至牌堆旁",
            "空位补入新牌",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "未配对牌面值计分",
            "J与Q各计十分",
            "K计零分最为有利",
            "A计一分二至十按面值",
        ]
    }

    /// 结束胜负
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "约定局数后累计总分",
            "总分最低者获胜",
            "同名高尔夫取自低分意",
            "四轮九轮为常见赛制",
        ]
    }
}

impl Rule for GolfCardGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("golf_card_game")
    }

    fn explain(&self) -> String {
        format!(
            "【高尔夫扑克牌】\n{}",
            [
                format!(
                    "发牌布局：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合行动：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "配对消牌：\\n{}",
                    self.matching()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分规则：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结束胜负：\\n{}",
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
    fn test_golfcardgamerules_basic() {
        let rules = GolfCardGameRules::new();
        assert_eq!(rules.metadata().name, "高尔夫扑克牌");
        assert!(!rules.setup().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.matching().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_golfcardgamerules_validation() {
        let rules = GolfCardGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("golf_card_game"));
    }

    #[test]
    fn test_golfcardgamerules_explain() {
        let rules = GolfCardGameRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌布局"));
        assert!(e.contains("回合行动"));
        assert!(e.contains("配对消牌"));
    }
}
