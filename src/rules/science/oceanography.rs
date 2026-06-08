//! 海洋学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: OceanographyRules, name: "海洋学定律", desc: "海洋学定律", origin: "国际", tags: ["科学", "地球"] }
impl OceanographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["洋流", "潮汐", "波浪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["盐度", "溶解氧", "碳循环"]
    }
}
impl Rule for OceanographyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("oceanography")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "海洋学定律",
            &[("环流", &self.section_0()), ("化学", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OceanographyRules::new();
        assert!(!r.explain().is_empty());
    }
}
