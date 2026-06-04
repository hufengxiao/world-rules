//! 职场礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: WorkplaceRules,
    name: "职场礼仪",
    desc: "职场社交礼仪",
    origin: "国际",
    tags: ["社交", "职场"]
}

impl WorkplaceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["邮件礼仪", "会议准时", "尊重上级和同事"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["符合公司文化", "保持整洁", "注意场合"]
    }
}

impl Rule for WorkplaceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("workplace")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "职场礼仪",
            &[("沟通", &self.section_0()), ("着装", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_workplace_rules() {
        let r = WorkplaceRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
