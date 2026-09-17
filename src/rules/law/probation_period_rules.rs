//! 试用期劳动权益
//!
//! 劳动合同试用期的期限、工资与权利要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProbationPeriodRules,
    name: "试用期劳动权益",
    desc: "劳动合同试用期的期限、工资与权利要点",
    origin: "中国",
    tags: ["法律", "试用期", "劳动合同"]
}

impl ProbationPeriodRules {
    /// 期限合规
    pub fn duration(&self) -> Vec<&'static str> {
        vec![
            "试用期有法定期限限制",
            "期限与合同相配",
            "不得随意延长",
            "试用期须签订在合同内",
        ]
    }

    /// 工资待遇
    pub fn wage(&self) -> Vec<&'static str> {
        vec![
            "试用期工资不得低于法定",
            "工资按其约定发放",
            "知悉社保缴纳义务",
            "不得无故压低待遇",
        ]
    }

    /// 权利保障
    pub fn rights(&self) -> Vec<&'static str> {
        vec![
            "依法享受休息休假",
            "有权获得劳动保护",
            "试用可依规解除",
            "被解除可依法核实",
        ]
    }

    /// 纠纷处理
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "试用期满依约定转正",
            "有争议先协商",
            "可依法申请仲裁",
            "保留劳动合同证据",
        ]
    }
}

impl Rule for ProbationPeriodRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("probation")
    }

    fn explain(&self) -> String {
        format!(
            "【试用期劳动权益】\n{}",
            [
                format!(
                    "期限合规：\\n{}",
                    self.duration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "工资待遇：\\n{}",
                    self.wage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "权利保障：\\n{}",
                    self.rights()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "纠纷处理：\\n{}",
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
    fn test_probationperiodrules_basic() {
        let rules = ProbationPeriodRules::new();
        assert_eq!(rules.metadata().name, "试用期劳动权益");
        assert!(!rules.duration().is_empty());
        assert!(!rules.wage().is_empty());
        assert!(!rules.rights().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_probationperiodrules_validation() {
        let rules = ProbationPeriodRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("probation"));
    }

    #[test]
    fn test_probationperiodrules_explain() {
        let rules = ProbationPeriodRules::new();
        let e = rules.explain();
        assert!(e.contains("期限合规"));
        assert!(e.contains("工资待遇"));
        assert!(e.contains("权利保障"));
    }
}
