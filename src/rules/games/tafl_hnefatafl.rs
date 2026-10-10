//! 北欧塔夫棋
//!
//! 维京时代的非对称棋盘游戏，护卫国王逃脱围捕

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TaflHnefataflRules,
    name: "北欧塔夫棋",
    desc: "维京时代的非对称棋盘游戏，护卫国王逃脱围捕",
    origin: "北欧",
    tags: ["棋类", "非对称", "传统"]
}

impl TaflHnefataflRules {
    /// 棋盘布阵
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "奇数方阵棋盘中央为王座",
            "国王居中央四面护卫",
            "攻击方棋子布于四边",
            "双方棋子数量不等",
        ]
    }

    /// 行棋方式
    pub fn movement(&self) -> Vec<&'static str> {
        vec![
            "棋子沿直线任意格移动",
            "不可跳过其他棋子",
            "不可走对角线",
            "仅国王可停于王座",
        ]
    }

    /// 吃子规则
    pub fn capturing(&self) -> Vec<&'static str> {
        vec![
            "夹击方式吃子",
            "移动后与另一棋夹住敌棋",
            "夹在边角亦可吃子",
            "国王被四面夹击方被擒",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "护卫方送国王至边角即胜",
            "攻击方擒获国王即胜",
            "国王可被夹击吃子",
            "双方目标截然不同",
        ]
    }

    /// 策略要点
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "护卫方集中保护国王",
            "攻击方设网封锁退路",
            "占据要道限制移动",
            "非对称平衡各有妙招",
        ]
    }
}

impl Rule for TaflHnefataflRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("tafl_hnefatafl")
    }

    fn explain(&self) -> String {
        format!(
            "【北欧塔夫棋】\n{}",
            [
                format!(
                    "棋盘布阵：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行棋方式：\\n{}",
                    self.movement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "吃子规则：\\n{}",
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
    fn test_taflhnefataflrules_basic() {
        let rules = TaflHnefataflRules::new();
        assert_eq!(rules.metadata().name, "北欧塔夫棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.capturing().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_taflhnefataflrules_validation() {
        let rules = TaflHnefataflRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("tafl_hnefatafl"));
    }

    #[test]
    fn test_taflhnefataflrules_explain() {
        let rules = TaflHnefataflRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘布阵"));
        assert!(e.contains("行棋方式"));
        assert!(e.contains("吃子规则"));
    }
}
