//! 光化学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: PhotochemistryRules,
    name: "光化学定律",
    desc: "光化学定律",
    origin: "国际",
    tags: ["科学", "化学"]
}

impl PhotochemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光化学第一定律", "光化学第二定律", "量子产率"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["光合作用", "光刻技术", "光催化降解"]
    }
}

impl Rule for PhotochemistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("photochemistry")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "光化学定律",
            &[("基本定律", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_photochemistry_rules() {
        let r = PhotochemistryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
