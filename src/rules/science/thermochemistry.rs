//! 热化学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ThermochemistryRules, name: "热化学定律", desc: "热化学定律", origin: "国际", tags: ["科学", "化学"] }
impl ThermochemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赫斯定律", "基尔霍夫定律"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["燃烧热", "生成热", "键能"]
    }
}
impl Rule for ThermochemistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("thermochemistry")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "热化学定律",
            &[("定律", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ThermochemistryRules::new();
        assert!(!r.explain().is_empty());
    }
}
