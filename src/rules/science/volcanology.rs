//! 火山学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: VolcanologyRules, name: "火山学定律", desc: "火山学定律", origin: "国际", tags: ["科学", "地球"] }
impl VolcanologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["盾状火山", "层状火山", "复式火山"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["喷发指数", "熔岩流", "火山灰"]
    }
}
impl Rule for VolcanologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("volcanology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "火山学定律",
            &[("类型", &self.section_0()), ("喷发", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VolcanologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
