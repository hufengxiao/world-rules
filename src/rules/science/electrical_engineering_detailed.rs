//! 电气工程详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ElectricalEngineeringDetailedRules, name: "电气工程详细定律", desc: "电气工程定律", origin: "国际", tags: ["科学", "工程"] }
impl ElectricalEngineeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["欧姆定律基尔霍夫"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["电机原理"]
    }
}
impl Rule for ElectricalEngineeringDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("electrical_engineering_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电气工程详细定律",
            &[("电路", &self.section_0()), ("电机", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ElectricalEngineeringDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
