//! 材料科学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MaterialsScienceDetailedRules, name: "材料科学详细定律", desc: "材料科学定律", origin: "国际", tags: ["科学", "材料"] }
impl MaterialsScienceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合金相图"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["高分子结构"]
    }
}
impl Rule for MaterialsScienceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("materials_science_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "材料科学详细定律",
            &[("金属", &self.section_0()), ("聚合物", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MaterialsScienceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
