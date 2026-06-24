//! BMX奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BmxOlympicRules, name: "BMX奥运规则", desc: "BMX奥运会规则", origin: "国际", tags: ["体育", "极限"] }
impl BmxOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["出发门弯道"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["公园U池"]
    }
}
impl Rule for BmxOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bmx_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "BMX奥运规则",
            &[("竞速", &self.section_0()), ("自由式", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BmxOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
