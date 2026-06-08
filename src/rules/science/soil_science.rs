//! 土壤科学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: SoilScienceRules, name: "土壤科学定律", desc: "土壤科学定律", origin: "国际", tags: ["科学", "地球"] }
impl SoilScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["矿物质", "有机质", "水分", "空气"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["风化", "淋溶", "淀积"]
    }
}
impl Rule for SoilScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("soil_science")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "土壤科学定律",
            &[("组成", &self.section_0()), ("过程", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SoilScienceRules::new();
        assert!(!r.explain().is_empty());
    }
}
