//! 失业保险
//!
//! 失业保险领取条件、金额与程序要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: UnemploymentBenefitRules,
    name: "失业保险",
    desc: "失业保险领取条件、金额与程序要点",
    origin: "中国",
    tags: ["社保", "失业", "保险", "救济"]
}

impl UnemploymentBenefitRules {
    /// 申领条件
    pub fn condition(&self) -> Vec<&'static str> {
        vec![
            "依法参保一定年限",
            "非自愿失业",
            "已办失业登记",
            "有意愿求职",
        ]
    }

    /// 领取标准
    pub fn amount(&self) -> Vec<&'static str> {
        vec![
            "按当地标准领取",
            "领取期限按月计",
            "不高于缴费年限限",
            "失业补助应急",
        ]
    }

    /// 申领程序
    pub fn process(&self) -> Vec<&'static str> {
        vec![
            "及时办失业登记",
            "带齐身份材料",
            "依规提交申请",
            "按流程申领",
        ]
    }

    /// 享受再就业
    pub fn rejob(&self) -> Vec<&'static str> {
        vec![
            "领取期间积极求职",
            "参加培训可享",
            "重新就业衔接",
            "保障过渡生活",
        ]
    }
}

impl Rule for UnemploymentBenefitRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("unemployment")
    }

    fn explain(&self) -> String {
        format!(
            "【失业保险】\n{}",
            [
                format!(
                    "申领条件：\\n{}",
                    self.condition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "领取标准：\\n{}",
                    self.amount()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "申领程序：\\n{}",
                    self.process()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "享受再就业：\\n{}",
                    self.rejob()
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
    fn test_unemploymentbenefitrules_basic() {
        let rules = UnemploymentBenefitRules::new();
        assert_eq!(rules.metadata().name, "失业保险");
        assert!(!rules.condition().is_empty());
        assert!(!rules.amount().is_empty());
        assert!(!rules.process().is_empty());
        assert!(!rules.rejob().is_empty());
    }

    #[test]
    fn test_unemploymentbenefitrules_validation() {
        let rules = UnemploymentBenefitRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("unemployment"));
    }

    #[test]
    fn test_unemploymentbenefitrules_explain() {
        let rules = UnemploymentBenefitRules::new();
        let e = rules.explain();
        assert!(e.contains("申领条件"));
        assert!(e.contains("领取标准"));
        assert!(e.contains("申领程序"));
    }
}
