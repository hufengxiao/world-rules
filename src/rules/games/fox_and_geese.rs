//! 狐鹅棋
//!
//! 一狐对多鹅的非对称追捕棋，狐狸吃鹅或鹅困狐

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FoxAndGeeseRules,
    name: "狐鹅棋",
    desc: "一狐对多鹅的非对称追捕棋，狐狸吃鹅或鹅困狐",
    origin: "欧洲",
    tags: ["棋类", "非对称", "追捕"]
}

impl FoxAndGeeseRules {
    /// 棋盘摆子
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "十字形棋盘布点连线",
            "一只狐狸居中",
            "多只鹅占一侧底线",
            "鹅方棋子数远超狐方",
        ]
    }

    /// 移动规则
    pub fn movement(&self) -> Vec<&'static str> {
        vec![
            "双方沿连线走一格",
            "可走任意方向",
            "目标点须为空",
            "鹅方不得后退",
        ]
    }

    /// 狐狸吃鹅
    pub fn capturing(&self) -> Vec<&'static str> {
        vec![
            "狐狸跳过相邻一鹅",
            "落点须为空位",
            "被跳过的鹅被吃掉",
            "可连跳连续吃子",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "狐狸吃掉足够多鹅即胜",
            "鹅方围死狐狸即胜",
            "狐狸无路可走则败",
            "鹅以数量取胜靠围困",
        ]
    }

    /// 策略要点
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "鹅方保持密集不露空档",
            "狐狸寻找连跳机会",
            "逐步收紧包围圈",
            "耐心布局避免被吃",
        ]
    }
}

impl Rule for FoxAndGeeseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("fox_and_geese")
    }

    fn explain(&self) -> String {
        format!(
            "【狐鹅棋】\n{}",
            [
                format!(
                    "棋盘摆子：\\n{}",
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
                    "狐狸吃鹅：\\n{}",
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
    fn test_foxandgeeserules_basic() {
        let rules = FoxAndGeeseRules::new();
        assert_eq!(rules.metadata().name, "狐鹅棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.capturing().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_foxandgeeserules_validation() {
        let rules = FoxAndGeeseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("fox_and_geese"));
    }

    #[test]
    fn test_foxandgeeserules_explain() {
        let rules = FoxAndGeeseRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘摆子"));
        assert!(e.contains("移动规则"));
        assert!(e.contains("狐狸吃鹅"));
    }
}
