//! 养老院探访礼仪
//!
//! 探访养老院、敬老院长者时的尊重与关怀礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NursingHomeVisitRules,
    name: "养老院探访礼仪",
    desc: "探访养老院、敬老院长者时的尊重与关怀礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "养老院", "探访", "长者"]
}

impl NursingHomeVisitRules {
    /// 探访准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前与机构约定探访时间",
            "了解机构探访与聚集规定",
            "准备合适的关怀礼物",
            "避免在长者休息时段打扰",
        ]
    }

    /// 交流方式
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "耐心聆听长者说话",
            "语速放慢配合对方节奏",
            "不打断并及时回应",
            "称呼得体不过分高高在上",
        ]
    }

    /// 尊重关怀
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "陪伴多于带来礼物",
            "尊重长者拒绝或不愿",
            "协助须先征得同意",
            "不越界翻动物品或隐私",
        ]
    }

    /// 告别致谢
    pub fn farewell(&self) -> Vec<&'static str> {
        vec![
            "临别礼貌道再见",
            "约定下次探访时间",
            "向护理人员致谢",
            "离开不打扰其他长者",
        ]
    }
}

impl Rule for NursingHomeVisitRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("nursing_home")
    }

    fn explain(&self) -> String {
        format!(
            "【养老院探访礼仪】\n{}",
            [
                format!(
                    "探访准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交流方式：\\n{}",
                    self.communication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重关怀：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "告别致谢：\\n{}",
                    self.farewell()
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
    fn test_nursinghomevisitrules_basic() {
        let rules = NursingHomeVisitRules::new();
        assert_eq!(rules.metadata().name, "养老院探访礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.farewell().is_empty());
    }

    #[test]
    fn test_nursinghomevisitrules_validation() {
        let rules = NursingHomeVisitRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("nursing_home"));
    }

    #[test]
    fn test_nursinghomevisitrules_explain() {
        let rules = NursingHomeVisitRules::new();
        let e = rules.explain();
        assert!(e.contains("探访准备"));
        assert!(e.contains("交流方式"));
        assert!(e.contains("尊重关怀"));
    }
}
