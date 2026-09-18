//! 跳房子
//!
//! 跳房子画格、抛石与单双跳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HopscotchGroundGamesRules,
    name: "跳房子",
    desc: "跳房子画格、抛石与单双跳规则",
    origin: "国际",
    tags: ["游戏", "跳房子", "户外"]
}

impl HopscotchGroundGamesRules {
    /// 画格
    pub fn grid(&self) -> Vec<&'static str> {
        vec!["地面画数字格", "单双格排列", "顶端回程", "按次序跳"]
    }

    /// 抛石落格
    pub fn toss(&self) -> Vec<&'static str> {
        vec!["抛小石入格", "落哪格跳哪", "越线违规则", "依序前进"]
    }

    /// 单双跳
    pub fn jump(&self) -> Vec<&'static str> {
        vec!["单格单脚跳", "双格双脚落", "不踩线", "转弯回跳"]
    }

    /// 轮流趣味
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["轮流跳不挤", "失误换人", "公平进行", "乐在其中"]
    }
}

impl Rule for HopscotchGroundGamesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("hopscotch")
    }

    fn explain(&self) -> String {
        format!(
            "【跳房子】\n{}",
            [
                format!(
                    "画格：\\n{}",
                    self.grid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抛石落格：\\n{}",
                    self.toss()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "单双跳：\\n{}",
                    self.jump()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "轮流趣味：\\n{}",
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
    fn test_hopscotchgroundgamesrules_basic() {
        let rules = HopscotchGroundGamesRules::new();
        assert_eq!(rules.metadata().name, "跳房子");
        assert!(!rules.grid().is_empty());
        assert!(!rules.toss().is_empty());
        assert!(!rules.jump().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_hopscotchgroundgamesrules_validation() {
        let rules = HopscotchGroundGamesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("hopscotch"));
    }

    #[test]
    fn test_hopscotchgroundgamesrules_explain() {
        let rules = HopscotchGroundGamesRules::new();
        let e = rules.explain();
        assert!(e.contains("画格"));
        assert!(e.contains("抛石落格"));
        assert!(e.contains("单双跳"));
    }
}
