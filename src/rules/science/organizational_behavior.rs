//! 组织行为学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: OrganizationalBehaviorRules,
    name: "组织行为学定律",
    desc: "组织行为学定律",
    origin: "国际",
    tags: ["科学", "管理"]
}

impl OrganizationalBehaviorRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["动机理论", "人格与工作匹配", "工作满意度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["团队动力学", "领导力理论", "冲突管理"]
    }
}

impl Rule for OrganizationalBehaviorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("organizational_behavior")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "组织行为学定律",
            &[("个体", &self.section_0()), ("群体", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_organizational_behavior_rules() {
        let r = OrganizationalBehaviorRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
