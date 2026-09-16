//! 领奖致辞礼仪
//!
//! 颁奖典礼中获奖、领奖与致感谢辞的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AwardsCeremonyEtiquetteRules,
    name: "领奖致辞礼仪",
    desc: "颁奖典礼中获奖、领奖与致感谢辞的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "颁奖", "获奖", "致辞"]
}

impl AwardsCeremonyEtiquetteRules {
    /// 领奖准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "按时到场了解流程",
            "着装得体尊重场合",
            "提前准备简短感谢辞",
            "受托奖项代领者事先说明",
        ]
    }

    /// 上台领奖
    pub fn accepting(&self) -> Vec<&'static str> {
        vec![
            "有序上台礼貌致意",
            "向颁奖人致谢并握手",
            "双手接奖杯以示尊重",
            "不遗漏在场重要致谢人",
        ]
    }

    /// 致辞表达
    pub fn speech(&self) -> Vec<&'static str> {
        vec![
            "感谢辞简短诚挚",
            "致谢团队与支持者",
            "不拖长或离题",
            "语气谦逊不过分自夸",
        ]
    }

    /// 下台与后续
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "掌声中从容下台",
            "与祝贺者友好互动",
            "不忘向落选者表示尊重",
            "尊重时间与流程安排",
        ]
    }
}

impl Rule for AwardsCeremonyEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("awards_ceremony")
    }

    fn explain(&self) -> String {
        format!(
            "【领奖致辞礼仪】\n{}",
            [
                format!(
                    "领奖准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "上台领奖：\\n{}",
                    self.accepting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "致辞表达：\\n{}",
                    self.speech()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "下台与后续：\\n{}",
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
    fn test_awardsceremonyetiquetterules_basic() {
        let rules = AwardsCeremonyEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "领奖致辞礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.accepting().is_empty());
        assert!(!rules.speech().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_awardsceremonyetiquetterules_validation() {
        let rules = AwardsCeremonyEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("awards_ceremony"));
    }

    #[test]
    fn test_awardsceremonyetiquetterules_explain() {
        let rules = AwardsCeremonyEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("领奖准备"));
        assert!(e.contains("上台领奖"));
        assert!(e.contains("致辞表达"));
    }
}
