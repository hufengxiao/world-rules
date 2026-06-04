//! 国际公法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: InternationalPublicLawRules,
    name: "国际公法",
    desc: "国际公法基本规则",
    origin: "国际",
    tags: ["法律", "国际"]
}

impl InternationalPublicLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["主权平等", "不干涉内政", "和平解决争端"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["国家", "国际组织", "个人"]
    }
}

impl Rule for InternationalPublicLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_public_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "国际公法",
            &[("基本原则", &self.section_0()), ("主体", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_international_public_law_rules() {
        let r = InternationalPublicLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
