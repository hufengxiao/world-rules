//! 舞蹈与舞会礼仪
//!
//! 舞蹈课程与社交舞会中的邀舞、体态与合作礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DanceClassEtiquetteRules,
    name: "舞蹈与舞会礼仪",
    desc: "舞蹈课程与社交舞会中的邀舞、体态与合作礼仪",
    origin: "国际",
    tags: ["体育", "舞蹈", "舞会", "礼仪", "社交"]
}

impl DanceClassEtiquetteRules {
    /// 课堂秩序
    pub fn class(&self) -> Vec<&'static str> {
        vec![
            "准时到课着舞服舞鞋",
            "观察老师示范再跟上",
            "不擅自中断课堂",
            "课后整理舞区",
        ]
    }

    /// 邀舞礼仪
    pub fn invite(&self) -> Vec<&'static str> {
        vec![
            "向舞伴礼貌邀请",
            "尊重受邀者拒绝",
            "舞好后致谢",
            "不勉强不熟识者",
        ]
    }

    /// 舞动配合
    pub fn dancing(&self) -> Vec<&'static str> {
        vec![
            "保持适当间距体态",
            "跟随音乐与节奏",
            "失误不过多埋怨",
            "频率适时轮流",
        ]
    }

    /// 风度修身
    pub fn grace(&self) -> Vec<&'static str> {
        vec![
            "学会领舞与跟舞兼顾",
            "穿着得体恰当",
            "眼神与礼貌交流",
            "散场致谢彼此",
        ]
    }
}

impl Rule for DanceClassEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("dance_class")
    }

    fn explain(&self) -> String {
        format!(
            "【舞蹈与舞会礼仪】\n{}",
            [
                format!(
                    "课堂秩序：\\n{}",
                    self.class()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "邀舞礼仪：\\n{}",
                    self.invite()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "舞动配合：\\n{}",
                    self.dancing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风度修身：\\n{}",
                    self.grace()
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
    fn test_danceclassetiquetterules_basic() {
        let rules = DanceClassEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "舞蹈与舞会礼仪");
        assert!(!rules.class().is_empty());
        assert!(!rules.invite().is_empty());
        assert!(!rules.dancing().is_empty());
        assert!(!rules.grace().is_empty());
    }

    #[test]
    fn test_danceclassetiquetterules_validation() {
        let rules = DanceClassEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("dance_class"));
    }

    #[test]
    fn test_danceclassetiquetterules_explain() {
        let rules = DanceClassEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("课堂秩序"));
        assert!(e.contains("邀舞礼仪"));
        assert!(e.contains("舞动配合"));
    }
}
