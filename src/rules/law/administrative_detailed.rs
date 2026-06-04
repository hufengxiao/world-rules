//! 行政法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: AdministrativeDetailedRules, name: "行政法详解", desc: "行政法详解", origin: "中国", tags: ["法律", "行政"] }
impl AdministrativeDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["行政处罚", "行政许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["行政复议", "国家赔偿"]
    }
}
impl Rule for AdministrativeDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("administrative_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "行政法详解",
            &[("行为", &self.section_0()), ("救济", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AdministrativeDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
