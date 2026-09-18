//! 吹泡泡
//!
//! 吹泡泡、接泡泡与户外趣味玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BubbleCatchingBlowRules,
    name: "吹泡泡",
    desc: "吹泡泡、接泡泡与户外趣味玩法",
    origin: "国际",
    tags: ["游戏", "泡泡", "儿童"]
}

impl BubbleCatchingBlowRules {
    /// 吹泡方法
    pub fn blow(&self) -> Vec<&'static str> {
        vec!["蘸泡泡水", "轻吹成泡", "大小适度", "飘在空中"]
    }

    /// 接泡玩法
    pub fn catch(&self) -> Vec<&'static str> {
        vec!["手掌接泡", "追泡泡跑", "不落地面破", "比谁接多"]
    }

    /// 吹大泡泡
    pub fn big(&self) -> Vec<&'static str> {
        vec!["用吹泡棒吹大", "缓缓拉出", "风小泡泡圆", "趣味多多"]
    }

    /// 安全注意
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["泡泡水勿入口", "远离眼睛", "幼儿看护", "开心安全"]
    }
}

impl Rule for BubbleCatchingBlowRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bubble")
    }

    fn explain(&self) -> String {
        format!(
            "【吹泡泡】\n{}",
            [
                format!(
                    "吹泡方法：\\n{}",
                    self.blow()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "接泡玩法：\\n{}",
                    self.catch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "吹大泡泡：\\n{}",
                    self.big()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全注意：\\n{}",
                    self.safety()
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
    fn test_bubblecatchingblowrules_basic() {
        let rules = BubbleCatchingBlowRules::new();
        assert_eq!(rules.metadata().name, "吹泡泡");
        assert!(!rules.blow().is_empty());
        assert!(!rules.catch().is_empty());
        assert!(!rules.big().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_bubblecatchingblowrules_validation() {
        let rules = BubbleCatchingBlowRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("bubble"));
    }

    #[test]
    fn test_bubblecatchingblowrules_explain() {
        let rules = BubbleCatchingBlowRules::new();
        let e = rules.explain();
        assert!(e.contains("吹泡方法"));
        assert!(e.contains("接泡玩法"));
        assert!(e.contains("吹大泡泡"));
    }
}
