//! 社会心理学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: SocialPsychologyRules,
    name: "社会心理学定律",
    desc: "社会心理学定律",
    origin: "国际",
    tags: ["科学", "心理学"]
}

impl SocialPsychologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["归因理论", "认知失调", "刻板印象"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["从众效应", "服从权威", "社会促进"]
    }
}

impl Rule for SocialPsychologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("social_psychology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "社会心理学定律",
            &[("认知", &self.section_0()), ("影响", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_social_psychology_rules() {
        let r = SocialPsychologyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
