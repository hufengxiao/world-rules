//! 气道梗阻急救
//!
//! 异物卡喉的海姆立克、轻拍与急救要领

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChokingReliefRules,
    name: "气道梗阻急救",
    desc: "异物卡喉的海姆立克、轻拍与急救要领",
    origin: "医学",
    tags: ["健康", "窒息", "卡喉", "急救"]
}

impl ChokingReliefRules {
    /// 识别窒息
    pub fn identify(&self) -> Vec<&'static str> {
        vec!["突然不能说话", "捂颈手型示意", "呼吸费力发青", "无法咳嗽"]
    }

    /// 成人急救
    pub fn adult(&self) -> Vec<&'static str> {
        vec!["站其身后抱腰", "握拳内上冲压", "海姆立克法", "反复至排出"]
    }

    /// 儿童婴儿
    pub fn child(&self) -> Vec<&'static str> {
        vec!["婴儿俯卧拍背", "轻压胸部", "儿童用较小力", "勿用力过猛"]
    }

    /// 后续就医
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "立即呼救拨打急救",
            "昏迷做胸外按压",
            "气道排出仍就医",
            "防二次窒息",
        ]
    }
}

impl Rule for ChokingReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("choking")
    }

    fn explain(&self) -> String {
        format!(
            "【气道梗阻急救】\n{}",
            [
                format!(
                    "识别窒息：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "成人急救：\\n{}",
                    self.adult()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "儿童婴儿：\\n{}",
                    self.child()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "后续就医：\\n{}",
                    self.after()
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
    fn test_chokingreliefrules_basic() {
        let rules = ChokingReliefRules::new();
        assert_eq!(rules.metadata().name, "气道梗阻急救");
        assert!(!rules.identify().is_empty());
        assert!(!rules.adult().is_empty());
        assert!(!rules.child().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_chokingreliefrules_validation() {
        let rules = ChokingReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("choking"));
    }

    #[test]
    fn test_chokingreliefrules_explain() {
        let rules = ChokingReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("识别窒息"));
        assert!(e.contains("成人急救"));
        assert!(e.contains("儿童婴儿"));
    }
}
