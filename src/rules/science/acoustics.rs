//! 声学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AcousticsRules,
    name: "声学定律",
    desc: "声学物理定律",
    origin: "国际",
    tags: ["科学", "物理"]
}

impl AcousticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["声波传播方程", "多普勒效应", "驻波"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["共振频率", "共振条件", "阻尼振动"]
    }
}

impl Rule for AcousticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("acoustics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "声学定律",
            &[("波动", &self.section_0()), ("共振", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_acoustics_rules() {
        let r = AcousticsRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
