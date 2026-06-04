//! 城市地理学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: UrbanGeographyRules, name: "城市地理学定律", desc: "城市地理学定律", origin: "国际", tags: ["科学", "地理"] }
impl UrbanGeographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["中心地理论", "城市化进程"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["功能分区", "交通网络"]
    }
}
impl Rule for UrbanGeographyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("urban_geography")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "城市地理学定律",
            &[("理论", &self.section_0()), ("规划", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = UrbanGeographyRules::new();
        assert!(!r.explain().is_empty());
    }
}
