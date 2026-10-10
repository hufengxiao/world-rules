//! 苏拉卡尔塔棋
//!
//! 印尼传统棋戏，借环线绕行吃子的独特棋类

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SurakartaRules,
    name: "苏拉卡尔塔棋",
    desc: "印尼传统棋戏，借环线绕行吃子的独特棋类",
    origin: "印度尼西亚",
    tags: ["棋类", "传统", "环线"]
}

impl SurakartaRules {
    /// 棋盘布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "六乘六格棋盘",
            "四周有环形弧线相连",
            "双方各十二枚棋子",
            "棋子摆于底侧两排",
        ]
    }

    /// 移动规则
    pub fn movement(&self) -> Vec<&'static str> {
        vec![
            "棋子沿格线走一格",
            "可直行亦可斜行",
            "落点须为空位",
            "不可越子移动",
        ]
    }

    /// 环线吃子
    pub fn capturing(&self) -> Vec<&'static str> {
        vec![
            "沿环形弧线绕行吃子",
            "绕行路径上须无阻挡",
            "到达敌方棋子处将其吃",
            "路线可借外环大幅迂回",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "吃光对方棋子者胜",
            "对方无法行动者胜",
            "亦可见实吃净棋子数",
            "和棋可能极少",
        ]
    }

    /// 策略要点
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "善用外环长距离吃子",
            "保持路线畅通",
            "封锁对手环线通路",
            "攻守转换需预判",
        ]
    }
}

impl Rule for SurakartaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("surakarta")
    }

    fn explain(&self) -> String {
        format!(
            "【苏拉卡尔塔棋】\n{}",
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
                    "环线吃子：\\n{}",
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
    fn test_surakartarules_basic() {
        let rules = SurakartaRules::new();
        assert_eq!(rules.metadata().name, "苏拉卡尔塔棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.capturing().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_surakartarules_validation() {
        let rules = SurakartaRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("surakarta"));
    }

    #[test]
    fn test_surakartarules_explain() {
        let rules = SurakartaRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘布置"));
        assert!(e.contains("移动规则"));
        assert!(e.contains("环线吃子"));
    }
}
