//! 美学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: AestheticsRules, name: "美学定律", desc: "美学定律", origin: "国际", tags: ["科学", "艺术"] }
impl AestheticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["审美经验", "形式美法则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["设计美学", "建筑美学"]
    }
}
impl Rule for AestheticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("aesthetics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "美学定律",
            &[("理论", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AestheticsRules::new();
        assert!(!r.explain().is_empty());
    }
}
