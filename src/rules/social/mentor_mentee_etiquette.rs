//! 师徒相处礼仪
//!
//! 职场带教关系中导师与新人相处的分寸与礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MentorMenteeEtiquetteRules,
    name: "师徒相处礼仪",
    desc: "职场带教关系中导师与新人相处的分寸与礼节",
    origin: "职场通用",
    tags: ["职场", "带教", "关系"]
}

impl MentorMenteeEtiquetteRules {
    /// 导师之责
    pub fn mentor_duty(&self) -> Vec<&'static str> {
        vec![
            "耐心指导不摆架子",
            "给予试错空间与鼓励",
            "因材施教循序渐进",
            "及时反馈不秋后算账",
        ]
    }

    /// 新人之礼
    pub fn mentee_courtesy(&self) -> Vec<&'static str> {
        vec![
            "主动请教虚心受教",
            "独立思考后再提问",
            "尊重导师时间与安排",
            "受助后真诚致谢",
        ]
    }

    /// 沟通方式
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "约定固定的沟通节奏",
            "提问先自行查证",
            "反馈对事不对人",
            "不传八卦不越级抱怨",
        ]
    }

    /// 边界把握
    pub fn boundary(&self) -> Vec<&'static str> {
        vec![
            "公私分明不过度依附",
            "导师不越权分配私活",
            "新人应独立承担责任",
            "关系平等相互尊重",
        ]
    }
}

impl Rule for MentorMenteeEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("mentor_mentee_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【师徒相处礼仪】\n{}",
            [
                format!(
                    "导师之责：\\n{}",
                    self.mentor_duty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "新人之礼：\\n{}",
                    self.mentee_courtesy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沟通方式：\\n{}",
                    self.communication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "边界把握：\\n{}",
                    self.boundary()
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
    fn test_mentormenteeetiquetterules_basic() {
        let rules = MentorMenteeEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "师徒相处礼仪");
        assert!(!rules.mentor_duty().is_empty());
        assert!(!rules.mentee_courtesy().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.boundary().is_empty());
    }

    #[test]
    fn test_mentormenteeetiquetterules_validation() {
        let rules = MentorMenteeEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("mentor_mentee_etiquette")
        );
    }

    #[test]
    fn test_mentormenteeetiquetterules_explain() {
        let rules = MentorMenteeEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("导师之责"));
        assert!(e.contains("新人之礼"));
        assert!(e.contains("沟通方式"));
    }
}
