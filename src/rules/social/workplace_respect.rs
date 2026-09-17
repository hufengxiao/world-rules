//! 职场尊敬人际
//!
//! 职场中长幼、上下级与同事相处的尊重

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WorkplaceRespectRules,
    name: "职场尊敬人际",
    desc: "职场中长幼、上下级与同事相处的尊重",
    origin: "国际",
    tags: ["职场", "尊敬", "人际"]
}

impl WorkplaceRespectRules {
    /// 称呼得体
    pub fn addressing(&self) -> Vec<&'static str> {
        vec![
            "按职位称呼",
            "初次见面客客气",
            "不用轻浮称呼",
            "见人主动问好",
        ]
    }

    /// 守时守信
    pub fn reliability(&self) -> Vec<&'static str> {
        vec!["上班约谈守时", "答应的事做到", "做不到需早说", "讲信用立身"]
    }

    /// 界限礼貌
    pub fn boundary(&self) -> Vec<&'static str> {
        vec![
            "不打听私人隐私",
            "不背后说闲话",
            "不打断人工作",
            "尊重他人空间",
        ]
    }

    /// 团队氛围
    pub fn team(&self) -> Vec<&'static str> {
        vec!["多赞美少指责", "有帮助先伸", "不抢功邀功", "和谐共处"]
    }
}

impl Rule for WorkplaceRespectRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("workplace_respect")
    }

    fn explain(&self) -> String {
        format!(
            "【职场尊敬人际】\n{}",
            [
                format!(
                    "称呼得体：\\n{}",
                    self.addressing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "守时守信：\\n{}",
                    self.reliability()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "界限礼貌：\\n{}",
                    self.boundary()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "团队氛围：\\n{}",
                    self.team()
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
    fn test_workplacerespectrules_basic() {
        let rules = WorkplaceRespectRules::new();
        assert_eq!(rules.metadata().name, "职场尊敬人际");
        assert!(!rules.addressing().is_empty());
        assert!(!rules.reliability().is_empty());
        assert!(!rules.boundary().is_empty());
        assert!(!rules.team().is_empty());
    }

    #[test]
    fn test_workplacerespectrules_validation() {
        let rules = WorkplaceRespectRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("workplace_respect"));
    }

    #[test]
    fn test_workplacerespectrules_explain() {
        let rules = WorkplaceRespectRules::new();
        let e = rules.explain();
        assert!(e.contains("称呼得体"));
        assert!(e.contains("守时守信"));
        assert!(e.contains("界限礼貌"));
    }
}
