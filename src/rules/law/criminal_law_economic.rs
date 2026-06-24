//! 刑法经济犯罪详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CriminalLawEconomicRules, name: "刑法经济犯罪详解", desc: "经济犯罪刑法详解", origin: "中国", tags: ["法律", "刑法"] }
impl CriminalLawEconomicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["非法集资"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["逃税"]
    }
}
impl Rule for CriminalLawEconomicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_law_economic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑法经济犯罪详解",
            &[("金融", &self.section_0()), ("税务", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalLawEconomicRules::new();
        assert!(!r.explain().is_empty());
    }
}
