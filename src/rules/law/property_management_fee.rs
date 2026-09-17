//! 物业费与物业纠纷
//!
//! 缴纳物业费、物业服务与业主纠纷处理要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PropertyManagementFeeRules,
    name: "物业费与物业纠纷",
    desc: "缴纳物业费、物业服务与业主纠纷处理要点",
    origin: "中国",
    tags: ["法律", "物业", "物业费", "业主"]
}

impl PropertyManagementFeeRules {
    /// 交费义务
    pub fn obligation(&self) -> Vec<&'static str> {
        vec![
            "按时缴纳物业服务费",
            "了解收费项目与标准",
            "业委会议商定代收费",
            "不按时缴费有责任",
        ]
    }

    /// 服务内容
    pub fn service(&self) -> Vec<&'static str> {
        vec![
            "物业服务有约定范围",
            "保洁安保维修等依约",
            "了解服务标准与投诉",
            "服务不达标积极反映",
        ]
    }

    /// 纠纷处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec![
            "先与物业沟通协商",
            "向业委会或街道反映",
            "有依据可投诉监管",
            "依法定缓交或抗辩",
        ]
    }

    /// 理性维权
    pub fn rights(&self) -> Vec<&'static str> {
        vec![
            "不拖欠无据拒绝缴费",
            "保留服务凭证证据",
            "合理维护业主权益",
            "通过正当途径解决",
        ]
    }
}

impl Rule for PropertyManagementFeeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("property_fee")
    }

    fn explain(&self) -> String {
        format!(
            "【物业费与物业纠纷】\n{}",
            [
                format!(
                    "交费义务：\\n{}",
                    self.obligation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "服务内容：\\n{}",
                    self.service()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "纠纷处理：\\n{}",
                    self.dispute()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "理性维权：\\n{}",
                    self.rights()
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
    fn test_propertymanagementfeerules_basic() {
        let rules = PropertyManagementFeeRules::new();
        assert_eq!(rules.metadata().name, "物业费与物业纠纷");
        assert!(!rules.obligation().is_empty());
        assert!(!rules.service().is_empty());
        assert!(!rules.dispute().is_empty());
        assert!(!rules.rights().is_empty());
    }

    #[test]
    fn test_propertymanagementfeerules_validation() {
        let rules = PropertyManagementFeeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("property_fee"));
    }

    #[test]
    fn test_propertymanagementfeerules_explain() {
        let rules = PropertyManagementFeeRules::new();
        let e = rules.explain();
        assert!(e.contains("交费义务"));
        assert!(e.contains("服务内容"));
        assert!(e.contains("纠纷处理"));
    }
}
