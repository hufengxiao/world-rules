//! 汽车维修纠纷
//!
//! 汽车维修保养纠纷的法律规则，包括维修合同、配件质量、费用争议与索赔途径，维护车主合法权益。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CarRepairDisputeRules,
    name: "汽车维修纠纷",
    desc: "汽车维修保养纠纷的法律规则，包括维修合同、配件质量、费用争议与索赔途径，维护车主合法权益。",
    origin: "消费者权益保护与汽车维修行业管理规定",
    tags: ["法律", "消费", "汽车", "维修"]
}

impl CarRepairDisputeRules {
    /// 维修合同
    pub fn contract(&self) -> Vec<&'static str> {
        vec![
            "维修前确认项目与报价",
            "要求出具书面维修单",
            "明确配件来源与质保期",
            "变更项目须经同意",
        ]
    }

    /// 配件质量
    pub fn parts(&self) -> Vec<&'static str> {
        vec![
            "原厂件与副厂件须明示",
            "不得以次充好以旧充新",
            "保留更换下的旧件",
            "配件质量问题可追责",
        ]
    }

    /// 费用争议
    pub fn fee(&self) -> Vec<&'static str> {
        vec![
            "维修费须事先告知",
            "不得擅自增加收费项目",
            "索要正规发票",
            "对收费有异议先协商",
        ]
    }

    /// 索赔途径
    pub fn claim(&self) -> Vec<&'static str> {
        vec![
            "维修不当致损可要求赔偿",
            "向消协或运管部门投诉",
            "鉴定损失作为证据",
            "协商不成可诉至法院",
        ]
    }
}

impl Rule for CarRepairDisputeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("car_repair")
    }

    fn explain(&self) -> String {
        format!(
            "【汽车维修纠纷】\n{}",
            [
                format!(
                    "维修合同：\\n{}",
                    self.contract()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "配件质量：\\n{}",
                    self.parts()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "费用争议：\\n{}",
                    self.fee()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "索赔途径：\\n{}",
                    self.claim()
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
    fn test_carrepairdisputerules_basic() {
        let rules = CarRepairDisputeRules::new();
        assert_eq!(rules.metadata().name, "汽车维修纠纷");
        assert!(!rules.contract().is_empty());
        assert!(!rules.parts().is_empty());
        assert!(!rules.fee().is_empty());
        assert!(!rules.claim().is_empty());
    }

    #[test]
    fn test_carrepairdisputerules_validation() {
        let rules = CarRepairDisputeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("car_repair"));
    }

    #[test]
    fn test_carrepairdisputerules_explain() {
        let rules = CarRepairDisputeRules::new();
        let e = rules.explain();
        assert!(e.contains("维修合同"));
        assert!(e.contains("配件质量"));
        assert!(e.contains("费用争议"));
    }
}
