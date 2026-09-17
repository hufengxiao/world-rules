//! 劳动合同与劳动权益
//!
//! 签订劳动合同、工时工资与离职应知晓的权益要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LaborContractBasicsRules,
    name: "劳动合同与劳动权益",
    desc: "签订劳动合同、工时工资与离职应知晓的权益要点",
    origin: "中国",
    tags: ["法律", "劳动", "合同", "权益", "工资"]
}

impl LaborContractBasicsRules {
    /// 签约审阅
    pub fn contract(&self) -> Vec<&'static str> {
        vec![
            "入职应签订书面劳动合同",
            "明确岗位、报酬与期限",
            "留意试用期约定",
            "保留合同与工资凭证",
        ]
    }

    /// 工时与工资
    pub fn wages(&self) -> Vec<&'static str> {
        vec![
            "关注工作日与加班规定",
            "加班依法获得报酬",
            "社保由单位按时足额缴纳",
            "工资按月足额及时发放",
        ]
    }

    /// 休息与离职
    pub fn leave(&self) -> Vec<&'static str> {
        vec![
            "依法享有休息休假",
            "离职按规定办理交接",
            "依法主张经济补偿",
            "不轻易被迫放弃合同权利",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "缺证据先收集书证",
            "权益受损可申请劳动仲裁",
            "依法咨询劳动监察",
            "严重情况可诉诸法院",
        ]
    }
}

impl Rule for LaborContractBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_contract")
    }

    fn explain(&self) -> String {
        format!(
            "【劳动合同与劳动权益】\n{}",
            [
                format!(
                    "签约审阅：\\n{}",
                    self.contract()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "工时与工资：\\n{}",
                    self.wages()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "休息与离职：\\n{}",
                    self.leave()
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
    fn test_laborcontractbasicsrules_basic() {
        let rules = LaborContractBasicsRules::new();
        assert_eq!(rules.metadata().name, "劳动合同与劳动权益");
        assert!(!rules.contract().is_empty());
        assert!(!rules.wages().is_empty());
        assert!(!rules.leave().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_laborcontractbasicsrules_validation() {
        let rules = LaborContractBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("labor_contract"));
    }

    #[test]
    fn test_laborcontractbasicsrules_explain() {
        let rules = LaborContractBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("签约审阅"));
        assert!(e.contains("工时与工资"));
        assert!(e.contains("休息与离职"));
    }
}
