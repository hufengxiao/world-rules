//! 正当防卫要点
//!
//! 遭遇不法侵害时的正当防卫限度与法律要件

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SelfDefenseLawRules,
    name: "正当防卫要点",
    desc: "遭遇不法侵害时的正当防卫限度与法律要件",
    origin: "中国",
    tags: ["法律", "正当防卫", "防卫"]
}

impl SelfDefenseLawRules {
    /// 防卫前提
    pub fn premise(&self) -> Vec<&'static str> {
        vec![
            "正在发生不法侵害",
            "防卫针对侵害人",
            "为保护合法权益",
            "非挑拨或互殴",
        ]
    }

    /// 限度把握
    pub fn scope(&self) -> Vec<&'static str> {
        vec![
            "防卫制止侵害即可",
            "避免明显超过必要限度",
            "人身安全优先",
            "不事后报复",
        ]
    }

    /// 事后处置
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "侵害停止即停手",
            "及时报警求助",
            "保留受伤证据",
            "如实陈述经过",
        ]
    }

    /// 法律认识
    pub fn law(&self) -> Vec<&'static str> {
        vec![
            "正当防卫是免责事由",
            "明显过当负相应责任",
            "有疑问依法认定",
            "情况复杂求助法律",
        ]
    }
}

impl Rule for SelfDefenseLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("self_defense")
    }

    fn explain(&self) -> String {
        format!(
            "【正当防卫要点】\n{}",
            [
                format!(
                    "防卫前提：\\n{}",
                    self.premise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "限度把握：\\n{}",
                    self.scope()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "事后处置：\\n{}",
                    self.after()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "法律认识：\\n{}",
                    self.law()
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
    fn test_selfdefenselawrules_basic() {
        let rules = SelfDefenseLawRules::new();
        assert_eq!(rules.metadata().name, "正当防卫要点");
        assert!(!rules.premise().is_empty());
        assert!(!rules.scope().is_empty());
        assert!(!rules.after().is_empty());
        assert!(!rules.law().is_empty());
    }

    #[test]
    fn test_selfdefenselawrules_validation() {
        let rules = SelfDefenseLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("self_defense"));
    }

    #[test]
    fn test_selfdefenselawrules_explain() {
        let rules = SelfDefenseLawRules::new();
        let e = rules.explain();
        assert!(e.contains("防卫前提"));
        assert!(e.contains("限度把握"));
        assert!(e.contains("事后处置"));
    }
}
