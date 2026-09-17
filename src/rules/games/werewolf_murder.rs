//! 狼人杀桌游规范
//!
//! 狼人杀角色、发言与胜负流程规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WerewolfMurderRules,
    name: "狼人杀桌游规范",
    desc: "狼人杀角色、发言与胜负流程规则",
    origin: "中国",
    tags: ["游戏", "狼人杀", "桌游", "角色"]
}

impl WerewolfMurderRules {
    /// 角色配置
    pub fn roles(&self) -> Vec<&'static str> {
        vec!["村民狼人先知", "按人数配置", "闭眼睁眼轮流", "身份保密"]
    }

    /// 轮流发言
    pub fn speaking(&self) -> Vec<&'static str> {
        vec!["按顺序发言", "不插话抢辩", "合理推理", "听清他人"]
    }

    /// 投票出局
    pub fn voting(&self) -> Vec<&'static str> {
        vec!["发言后投票", "票多者出局", "首夜被杀等", "公平记录"]
    }

    /// 胜负裁定
    pub fn outcome(&self) -> Vec<&'static str> {
        vec!["狼全出局好人赢", "好人灭狼或神判", "法官公正", "尊重结果"]
    }
}

impl Rule for WerewolfMurderRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("werewolf")
    }

    fn explain(&self) -> String {
        format!(
            "【狼人杀桌游规范】\n{}",
            [
                format!(
                    "角色配置：\\n{}",
                    self.roles()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "轮流发言：\\n{}",
                    self.speaking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "投票出局：\\n{}",
                    self.voting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负裁定：\\n{}",
                    self.outcome()
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
    fn test_werewolfmurderrules_basic() {
        let rules = WerewolfMurderRules::new();
        assert_eq!(rules.metadata().name, "狼人杀桌游规范");
        assert!(!rules.roles().is_empty());
        assert!(!rules.speaking().is_empty());
        assert!(!rules.voting().is_empty());
        assert!(!rules.outcome().is_empty());
    }

    #[test]
    fn test_werewolfmurderrules_validation() {
        let rules = WerewolfMurderRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("werewolf"));
    }

    #[test]
    fn test_werewolfmurderrules_explain() {
        let rules = WerewolfMurderRules::new();
        let e = rules.explain();
        assert!(e.contains("角色配置"));
        assert!(e.contains("轮流发言"));
        assert!(e.contains("投票出局"));
    }
}
