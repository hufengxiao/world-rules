//! 突破棋
//!
//! 兵棋前进突破对方底线的简明策略棋

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BreakthroughBoardRules,
    name: "突破棋",
    desc: "兵棋前进突破对方底线的简明策略棋",
    origin: "美国",
    tags: ["棋类", "策略", "抽象"]
}

impl BreakthroughBoardRules {
    /// 棋盘布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "常见八乘八方格棋盘",
            "双方各占两排棋子",
            "棋子如兵位于前列",
            "中央区域空置",
        ]
    }

    /// 移动规则
    pub fn movement(&self) -> Vec<&'static str> {
        vec![
            "棋子只可向前移动",
            "可斜前方吃子",
            "可直前方走入空位",
            "不可后退或横行",
        ]
    }

    /// 吃子方式
    pub fn capturing(&self) -> Vec<&'static str> {
        vec![
            "斜前一格有敌棋即吃",
            "被吃棋移出棋盘",
            "每次只吃一枚",
            "直行方向不可吃子",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "率先有棋抵达对方底线即胜",
            "被吃光棋子则败",
            "无法移动亦判负",
            "突破即赢简洁明快",
        ]
    }

    /// 策略要点
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "保持兵力相互策应",
            "寻找斜向突破缺口",
            "牺牲棋子换通路",
            "防守反击并重",
        ]
    }
}

impl Rule for BreakthroughBoardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("breakthrough_board")
    }

    fn explain(&self) -> String {
        format!(
            "【突破棋】\n{}",
            [
                format!(
                    "棋盘布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "移动规则：\\n{}",
                    self.movement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "吃子方式：\\n{}",
                    self.capturing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负条件：\\n{}",
                    self.winning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "策略要点：\\n{}",
                    self.strategy()
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
    fn test_breakthroughboardrules_basic() {
        let rules = BreakthroughBoardRules::new();
        assert_eq!(rules.metadata().name, "突破棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.capturing().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_breakthroughboardrules_validation() {
        let rules = BreakthroughBoardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("breakthrough_board"));
    }

    #[test]
    fn test_breakthroughboardrules_explain() {
        let rules = BreakthroughBoardRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘布置"));
        assert!(e.contains("移动规则"));
        assert!(e.contains("吃子方式"));
    }
}
