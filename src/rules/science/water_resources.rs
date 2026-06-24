//! 水资源定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WaterResourcesRules, name: "水资源定律", desc: "水资源定律", origin: "国际", tags: ["科学", "环境"] }
impl WaterResourcesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["水资源评价"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["水质标准"]
    }
}
impl Rule for WaterResourcesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("water_resources")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "水资源定律",
            &[("管理", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WaterResourcesRules::new();
        assert!(!r.explain().is_empty());
    }
}
