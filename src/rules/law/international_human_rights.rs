//! 国际人权法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: InternationalHumanRightsRules, name: "国际人权法", desc: "国际人权法律规则", origin: "国际", tags: ["法律", "人权"] }
impl InternationalHumanRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["公民权利", "经济社会文化权利"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人权理事会"]
    }
}
impl Rule for InternationalHumanRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_human_rights")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "国际人权法",
            &[("公约", &self.section_0()), ("机制", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalHumanRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
