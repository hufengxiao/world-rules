//! 装修纠纷法律
//!
//! 家庭装修合同履行中的常见纠纷与维权

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HomeRenovationLawRules,
    name: "装修纠纷法律",
    desc: "家庭装修合同履行中的常见纠纷与维权",
    origin: "中国",
    tags: ["法律", "消费", "装修"]
}

impl HomeRenovationLawRules {
    /// 合同签订
    pub fn contract(&self) -> Vec<&'static str> {
        vec![
            "签订书面装修合同",
            "明确工期与价款",
            "约定材料品牌规格",
            "写明付款节点",
        ]
    }

    /// 常见纠纷
    pub fn disputes(&self) -> Vec<&'static str> {
        vec![
            "拖延工期",
            "材料以次充好",
            "增项加价不合理",
            "施工质量不合格",
        ]
    }

    /// 维权依据
    pub fn basis(&self) -> Vec<&'static str> {
        vec![
            "依合同约定追究责任",
            "保留施工照片与记录",
            "验收不合格可拒付尾款",
            "质量问题可要求返工",
        ]
    }

    /// 解决途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "先与装修方协商",
            "向消协或住建部门投诉",
            "申请质量鉴定",
            "协商不成可起诉",
        ]
    }
}

impl Rule for HomeRenovationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("home_renovation_law")
    }

    fn explain(&self) -> String {
        format!(
            "【装修纠纷法律】\n{}",
            [
                format!(
                    "合同签订：\\n{}",
                    self.contract()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常见纠纷：\\n{}",
                    self.disputes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权依据：\\n{}",
                    self.basis()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解决途径：\\n{}",
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
    fn test_homerenovationlawrules_basic() {
        let rules = HomeRenovationLawRules::new();
        assert_eq!(rules.metadata().name, "装修纠纷法律");
        assert!(!rules.contract().is_empty());
        assert!(!rules.disputes().is_empty());
        assert!(!rules.basis().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_homerenovationlawrules_validation() {
        let rules = HomeRenovationLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("home_renovation_law"));
    }

    #[test]
    fn test_homerenovationlawrules_explain() {
        let rules = HomeRenovationLawRules::new();
        let e = rules.explain();
        assert!(e.contains("合同签订"));
        assert!(e.contains("常见纠纷"));
        assert!(e.contains("维权依据"));
    }
}
