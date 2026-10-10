//! 航班延误赔偿
//!
//! 航班延误取消时的旅客权益与索赔依据

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FlightDelayCompensationLawRules,
    name: "航班延误赔偿",
    desc: "航班延误取消时的旅客权益与索赔依据",
    origin: "中国",
    tags: ["法律", "航空", "消费"]
}

impl FlightDelayCompensationLawRules {
    /// 承运人义务
    pub fn duty(&self) -> Vec<&'static str> {
        vec![
            "延误须及时告知旅客",
            "提供退改签选择",
            "安排食宿与交通",
            "做好信息通报服务",
        ]
    }

    /// 赔偿情形
    pub fn compensation(&self) -> Vec<&'static str> {
        vec![
            "承运人原因延误须补偿",
            "天气等不可抗力免责",
            "取消航班应退票或改签",
            "超售拒载须赔偿",
        ]
    }

    /// 旅客权利
    pub fn rights(&self) -> Vec<&'static str> {
        vec![
            "可选择退票全额退款",
            "可要求安排后续航班",
            "延误过夜要求住宿",
            "索取延误证明",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "先与航空公司协商",
            "向民航管理部门投诉",
            "保留登机牌与票据",
            "必要时诉讼索赔",
        ]
    }
}

impl Rule for FlightDelayCompensationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("flight_delay_compensation_law")
    }

    fn explain(&self) -> String {
        format!(
            "【航班延误赔偿】\n{}",
            [
                format!(
                    "承运人义务：\\n{}",
                    self.duty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赔偿情形：\\n{}",
                    self.compensation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "旅客权利：\\n{}",
                    self.rights()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权途径：\\n{}",
                    self.remedy()
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
    fn test_flightdelaycompensationlawrules_basic() {
        let rules = FlightDelayCompensationLawRules::new();
        assert_eq!(rules.metadata().name, "航班延误赔偿");
        assert!(!rules.duty().is_empty());
        assert!(!rules.compensation().is_empty());
        assert!(!rules.rights().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_flightdelaycompensationlawrules_validation() {
        let rules = FlightDelayCompensationLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::law("flight_delay_compensation_law")
        );
    }

    #[test]
    fn test_flightdelaycompensationlawrules_explain() {
        let rules = FlightDelayCompensationLawRules::new();
        let e = rules.explain();
        assert!(e.contains("承运人义务"));
        assert!(e.contains("赔偿情形"));
        assert!(e.contains("旅客权利"));
    }
}
