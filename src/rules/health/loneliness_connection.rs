//! 应对孤独感
//!
//! 认识孤独、主动联结与人际支持的调节方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LonelinessConnectionRules,
    name: "应对孤独感",
    desc: "认识孤独、主动联结与人际支持的调节方法",
    origin: "心理学",
    tags: ["健康", "孤独", "联结", "心理"]
}

impl LonelinessConnectionRules {
    /// 认识孤独
    pub fn understand(&self) -> Vec<&'static str> {
        vec![
            "孤独是常见情绪",
            "关注长期孤立感受",
            "区分独处与孤独",
            "不因孤独自责",
        ]
    }

    /// 主动联结
    pub fn connect(&self) -> Vec<&'static str> {
        vec![
            "主动联系家人朋友",
            "参与社交活动",
            "加入兴趣社群",
            "向他人表达关心",
        ]
    }

    /// 自我充实
    pub fn self_care(&self) -> Vec<&'static str> {
        vec![
            "培养爱好丰富日常",
            "练习与自己相处",
            "保持生活节奏",
            "适度运动户外",
        ]
    }

    /// 寻求支持
    pub fn support(&self) -> Vec<&'static str> {
        vec![
            "需要时倾诉求助",
            "向信任的人敞开心",
            "长期困扰寻求专业",
            "不把自己隔离太久",
        ]
    }
}

impl Rule for LonelinessConnectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("loneliness")
    }

    fn explain(&self) -> String {
        format!(
            "【应对孤独感】\n{}",
            [
                format!(
                    "认识孤独：\\n{}",
                    self.understand()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "主动联结：\\n{}",
                    self.connect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "自我充实：\\n{}",
                    self.self_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "寻求支持：\\n{}",
                    self.support()
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
    fn test_lonelinessconnectionrules_basic() {
        let rules = LonelinessConnectionRules::new();
        assert_eq!(rules.metadata().name, "应对孤独感");
        assert!(!rules.understand().is_empty());
        assert!(!rules.connect().is_empty());
        assert!(!rules.self_care().is_empty());
        assert!(!rules.support().is_empty());
    }

    #[test]
    fn test_lonelinessconnectionrules_validation() {
        let rules = LonelinessConnectionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("loneliness"));
    }

    #[test]
    fn test_lonelinessconnectionrules_explain() {
        let rules = LonelinessConnectionRules::new();
        let e = rules.explain();
        assert!(e.contains("认识孤独"));
        assert!(e.contains("主动联结"));
        assert!(e.contains("自我充实"));
    }
}
