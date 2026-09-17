//! 出租车网约车乘车礼仪
//!
//! 乘出租或网约车时的礼仪、沟通与评价

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TaxiRideEtiquetteRules,
    name: "出租车网约车乘车礼仪",
    desc: "乘出租或网约车时的礼仪、沟通与评价",
    origin: "国际",
    tags: ["社交", "礼仪", "出租车", "网约车", "乘车"]
}

impl TaxiRideEtiquetteRules {
    /// 叫车认知
    pub fn booking(&self) -> Vec<&'static str> {
        vec![
            "在允许招手的地段叫车",
            "核对车牌与订单",
            "确认目的地与费用",
            "礼貌道谢协调",
        ]
    }

    /// 乘车礼仪
    pub fn riding(&self) -> Vec<&'static str> {
        vec![
            "系好安全带",
            "不要求司机危险驾驶",
            "车内不乱丢弃",
            "行李放妥不挡视线",
        ]
    }

    /// 沟通边界
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "尊重司机劳动与隐私",
            "不无端催促或投诉",
            "有意见平和表达",
            "不强迫司机违规",
        ]
    }

    /// 评价反馈
    pub fn rating(&self) -> Vec<&'static str> {
        vec![
            "按实际体验客观评价",
            "好服务可给好评",
            "问题如实反映",
            "不恶意差评恐吓",
        ]
    }
}

impl Rule for TaxiRideEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("taxi_ride")
    }

    fn explain(&self) -> String {
        format!(
            "【出租车网约车乘车礼仪】\n{}",
            [
                format!(
                    "叫车认知：\\n{}",
                    self.booking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "乘车礼仪：\\n{}",
                    self.riding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沟通边界：\\n{}",
                    self.communication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "评价反馈：\\n{}",
                    self.rating()
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
    fn test_taxirideetiquetterules_basic() {
        let rules = TaxiRideEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "出租车网约车乘车礼仪");
        assert!(!rules.booking().is_empty());
        assert!(!rules.riding().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.rating().is_empty());
    }

    #[test]
    fn test_taxirideetiquetterules_validation() {
        let rules = TaxiRideEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("taxi_ride"));
    }

    #[test]
    fn test_taxirideetiquetterules_explain() {
        let rules = TaxiRideEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("叫车认知"));
        assert!(e.contains("乘车礼仪"));
        assert!(e.contains("沟通边界"));
    }
}
