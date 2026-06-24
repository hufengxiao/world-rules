//! 微分方程定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DifferentialEquationsRules, name: "微分方程定律", desc: "微分方程定律", origin: "国际", tags: ["科学", "数学"] }
impl DifferentialEquationsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一阶二阶"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["热方程波动方程"]
    }
}
impl Rule for DifferentialEquationsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("differential_equations")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "微分方程定律",
            &[("常微分", &self.section_0()), ("偏微分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DifferentialEquationsRules::new();
        assert!(!r.explain().is_empty());
    }
}
