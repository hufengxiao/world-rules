//! 海洋法公约

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MaritimeLawIntlRules,
    name: "海洋法公约",
    desc: "联合国海洋法公约规则",
    origin: "国际",
    tags: ["法律", "国际"]
}

impl MaritimeLawIntlRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["领海12海里", "专属经济区200海里", "大陆架", "公海"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["无害通过权", "过境通行", "群岛海道通过"]
    }
}

impl Rule for MaritimeLawIntlRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("maritime_law_intl")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "海洋法公约",
            &[
                ("海域划分", &self.section_0()),
                ("航行权", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maritime_law_intl_rules() {
        let r = MaritimeLawIntlRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
