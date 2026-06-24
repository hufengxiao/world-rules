//! 环保法详解3
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EnvironmentalDetailedLaw3Rules, name: "环保法详解3", desc: "环保法详解3", origin: "中国", tags: ["法律", "环境"] }
impl EnvironmentalDetailedLaw3Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["环评排污许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["环境公益诉讼"]
    }
}
impl Rule for EnvironmentalDetailedLaw3Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("environmental_detailed_law3")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "环保法详解3",
            &[("制度", &self.section_0()), ("公益", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnvironmentalDetailedLaw3Rules::new();
        assert!(!r.explain().is_empty());
    }
}
