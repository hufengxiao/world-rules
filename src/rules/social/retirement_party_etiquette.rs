//! 退休与送别礼仪
//!
//! 同事退休、离职欢送会的致谢、致辞与情谊礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RetirementPartyEtiquetteRules,
    name: "退休与送别礼仪",
    desc: "同事退休、离职欢送会的致谢、致辞与情谊礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "退休", "欢送", "职场"]
}

impl RetirementPartyEtiquetteRules {
    /// 组织准备
    pub fn organize(&self) -> Vec<&'static str> {
        vec![
            "以当事人意愿为主安排",
            "约时间地点通知参与者",
            "准备回忆与致谢环节",
            "尊重退休者的低调选择",
        ]
    }

    /// 致辞祝福
    pub fn toast(&self) -> Vec<&'static str> {
        vec![
            "真诚回顾共事点滴",
            "致谢多年付出协作",
            "祝福退休后生活充实",
            "言辞恳切不夸大",
        ]
    }

    /// 送礼纪念
    pub fn gifts(&self) -> Vec<&'static str> {
        vec![
            "集体送得体纪念品",
            "有实用或纪念意义",
            "不强求贵重",
            "把功劳客气归给本人",
        ]
    }

    /// 收尾体面
    pub fn closing(&self) -> Vec<&'static str> {
        vec![
            "让当事人充分表达",
            "活动时间张弛有度",
            "拍照留念其乐融融",
            "礼貌道别不再过度挽留",
        ]
    }
}

impl Rule for RetirementPartyEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("retirement")
    }

    fn explain(&self) -> String {
        format!(
            "【退休与送别礼仪】\n{}",
            [
                format!(
                    "组织准备：\\n{}",
                    self.organize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "致辞祝福：\\n{}",
                    self.toast()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "送礼纪念：\\n{}",
                    self.gifts()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收尾体面：\\n{}",
                    self.closing()
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
    fn test_retirementpartyetiquetterules_basic() {
        let rules = RetirementPartyEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "退休与送别礼仪");
        assert!(!rules.organize().is_empty());
        assert!(!rules.toast().is_empty());
        assert!(!rules.gifts().is_empty());
        assert!(!rules.closing().is_empty());
    }

    #[test]
    fn test_retirementpartyetiquetterules_validation() {
        let rules = RetirementPartyEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("retirement"));
    }

    #[test]
    fn test_retirementpartyetiquetterules_explain() {
        let rules = RetirementPartyEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("组织准备"));
        assert!(e.contains("致辞祝福"));
        assert!(e.contains("送礼纪念"));
    }
}
