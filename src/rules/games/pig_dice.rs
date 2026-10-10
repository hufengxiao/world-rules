//! 猪骰风险游戏
//!
//! 反复掷骰累积点数，掷出一点则清零的风险抉择游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PigDiceRules,
    name: "猪骰风险游戏",
    desc: "反复掷骰累积点数，掷出一点则清零的风险抉择游戏",
    origin: "美国",
    tags: ["骰子", "风险", "概率"]
}

impl PigDiceRules {
    /// 开局准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "只需一枚六面骰",
            "备纸笔记录分数",
            "约定目标总分为一百",
            "轮流进行各自回合",
        ]
    }

    /// 回合掷骰
    pub fn rolling(&self) -> Vec<&'static str> {
        vec![
            "回合内可反复掷骰",
            "掷出二至六累加点数",
            "掷出一则本回合归零",
            "归零后回合即结束",
        ]
    }

    /// 停手收分
    pub fn banking(&self) -> Vec<&'static str> {
        vec![
            "随时可主动停手",
            "本回合累计点计入总分",
            "冒险多掷或保守收分",
            "一念之间决定得失",
        ]
    }

    /// 特殊变体
    pub fn variants(&self) -> Vec<&'static str> {
        vec![
            "连掷两个一则整局清零",
            "掷出六可额外再掷一次",
            "变体须开局前商定",
            "规则一经确定不可改",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "先达目标总分者获胜",
            "到分即停不再冒险",
            "可约定多局累计",
            "落后者亦可反超",
        ]
    }
}

impl Rule for PigDiceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("pig_dice")
    }

    fn explain(&self) -> String {
        format!(
            "【猪骰风险游戏】\n{}",
            [
                format!(
                    "开局准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合掷骰：\\n{}",
                    self.rolling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "停手收分：\\n{}",
                    self.banking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊变体：\\n{}",
                    self.variants()
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
    fn test_pigdicerules_basic() {
        let rules = PigDiceRules::new();
        assert_eq!(rules.metadata().name, "猪骰风险游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.rolling().is_empty());
        assert!(!rules.banking().is_empty());
        assert!(!rules.variants().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_pigdicerules_validation() {
        let rules = PigDiceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("pig_dice"));
    }

    #[test]
    fn test_pigdicerules_explain() {
        let rules = PigDiceRules::new();
        let e = rules.explain();
        assert!(e.contains("开局准备"));
        assert!(e.contains("回合掷骰"));
        assert!(e.contains("停手收分"));
    }
}
