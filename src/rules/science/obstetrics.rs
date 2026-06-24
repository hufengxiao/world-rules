//! 妇产科学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ObstetricsRules, name: "妇产科学定律", desc: "妇产科学定律", origin: "国际", tags: ["科学", "医学"] }
impl ObstetricsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["妊娠分娩"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["月经肿瘤"]
    }
}
impl Rule for ObstetricsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("obstetrics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "妇产科学定律",
            &[("产科", &self.section_0()), ("妇科", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ObstetricsRules::new();
        assert!(!r.explain().is_empty());
    }
}
