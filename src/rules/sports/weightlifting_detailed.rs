//! 举重详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: WeightliftingDetailedRules, name: "举重详细规则", desc: "举重详细比赛规则", origin: "IWF", tags: ["体育", "力量"] }
impl WeightliftingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["单次过头", "3次试举"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["翻站挺"]
    }
}
impl Rule for WeightliftingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("weightlifting_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "举重详细规则",
            &[("抓举", &self.section_0()), ("挺举", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WeightliftingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
