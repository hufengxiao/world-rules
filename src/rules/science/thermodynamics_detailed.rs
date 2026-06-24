//! 热力学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ThermodynamicsDetailedRules, name: "热力学详细定律", desc: "热力学详细定律", origin: "国际", tags: ["科学", "物理"] }
impl ThermodynamicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["零一二三定律"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["卡诺循环", "熵增原理"]
    }
}
impl Rule for ThermodynamicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("thermodynamics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "热力学详细定律",
            &[("四大定律", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ThermodynamicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
