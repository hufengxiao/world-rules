//! 遗传学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GeneticsDetailedRules, name: "遗传学详细定律", desc: "遗传学定律", origin: "国际", tags: ["科学", "生物"] }
impl GeneticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["分离自由组合"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因突变重组"]
    }
}
impl Rule for GeneticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("genetics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "遗传学详细定律",
            &[("孟德尔", &self.section_0()), ("分子", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GeneticsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
