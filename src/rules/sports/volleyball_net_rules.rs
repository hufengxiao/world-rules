//! 排球网前规则
//!
//! 排球触网、过网与网前球员限制规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VolleyballNetRules,
    name: "排球网前规则",
    desc: "排球触网、过网与网前球员限制规则",
    origin: "国际",
    tags: ["体育", "排球", "网", "规则"]
}

impl VolleyballNetRules {
    /// 触网限制
    pub fn net(&self) -> Vec<&'static str> {
        vec![
            "击球时触网犯规",
            "网下不得穿过",
            "碰网致球落地",
            "判对方得分",
        ]
    }

    /// 过网击球
    pub fn across(&self) -> Vec<&'static str> {
        vec![
            "手臂不越过中线",
            "手掌过网判罚",
            "拦网可越高度",
            "落地不越线",
        ]
    }

    /// 网前队员
    pub fn front(&self) -> Vec<&'static str> {
        vec!["前排拦网职责", "后排不得进攻", "脚不越中线", "保持位置"]
    }

    /// 判罚原则
    pub fn call(&self) -> Vec<&'static str> {
        vec!["听裁判判断", "争议申请挑战", "保持比赛流畅", "尊重规则"]
    }
}

impl Rule for VolleyballNetRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("volleyball_net")
    }

    fn explain(&self) -> String {
        format!(
            "【排球网前规则】\n{}",
            [
                format!(
                    "触网限制：\\n{}",
                    self.net()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "过网击球：\\n{}",
                    self.across()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "网前队员：\\n{}",
                    self.front()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "判罚原则：\\n{}",
                    self.call()
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
    fn test_volleyballnetrules_basic() {
        let rules = VolleyballNetRules::new();
        assert_eq!(rules.metadata().name, "排球网前规则");
        assert!(!rules.net().is_empty());
        assert!(!rules.across().is_empty());
        assert!(!rules.front().is_empty());
        assert!(!rules.call().is_empty());
    }

    #[test]
    fn test_volleyballnetrules_validation() {
        let rules = VolleyballNetRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("volleyball_net"));
    }

    #[test]
    fn test_volleyballnetrules_explain() {
        let rules = VolleyballNetRules::new();
        let e = rules.explain();
        assert!(e.contains("触网限制"));
        assert!(e.contains("过网击球"));
        assert!(e.contains("网前队员"));
    }
}
