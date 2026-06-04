//! 电化学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ElectrochemistryRules,
    name: "电化学定律",
    desc: "电化学定律",
    origin: "国际",
    tags: ["科学", "化学"]
}

impl ElectrochemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["能斯特方程", "电极电位", "超电势"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["原电池", "电解池", "燃料电池", "电镀"]
    }
}

impl Rule for ElectrochemistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("electrochemistry")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "电化学定律",
            &[("电极", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_electrochemistry_rules() {
        let r = ElectrochemistryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
