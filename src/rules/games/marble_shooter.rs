//! 弹珠游戏
//!
//! 弹珠瞄准、翻弹入圈玩法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MarbleShooterRules,
    name: "弹珠游戏",
    desc: "弹珠瞄准、翻弹入圈玩法规则",
    origin: "国际",
    tags: ["游戏", "弹珠", "户外"]
}

impl MarbleShooterRules {
    /// 摆局
    pub fn setup(&self) -> Vec<&'static str> {
        vec!["地上画圆圈", "圈内放弹珠", "轮流击打", "定好回合"]
    }

    /// 弹法
    pub fn flick(&self) -> Vec<&'static str> {
        vec!["拇指弹珠", "目标弹出圈", "打在码上", "击中得分"]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec!["弹出圈得珠", "未中换人", "先得珠胜", "计分清楚"]
    }

    /// 公平有趣
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["不赖账偷珠", "公平轮流", "输了重来", "尽兴玩"]
    }
}

impl Rule for MarbleShooterRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("marble")
    }

    fn explain(&self) -> String {
        format!(
            "【弹珠游戏】\n{}",
            [
                format!(
                    "摆局：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "弹法：\\n{}",
                    self.flick()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "得分规则：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公平有趣：\\n{}",
                    self.fun()
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
    fn test_marbleshooterrules_basic() {
        let rules = MarbleShooterRules::new();
        assert_eq!(rules.metadata().name, "弹珠游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.flick().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_marbleshooterrules_validation() {
        let rules = MarbleShooterRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("marble"));
    }

    #[test]
    fn test_marbleshooterrules_explain() {
        let rules = MarbleShooterRules::new();
        let e = rules.explain();
        assert!(e.contains("摆局"));
        assert!(e.contains("弹法"));
        assert!(e.contains("得分规则"));
    }
}
