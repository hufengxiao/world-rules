//! 儿童健康规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ChildrenHealthRules,
    name: "儿童健康规则",
    desc: "儿童健康护理规则",
    origin: "国际",
    tags: ["健康", "儿童"]
}

impl ChildrenHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["母乳喂养", "辅食添加", "均衡膳食"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["防跌落", "防误食", "防溺水"]
    }
}

impl Rule for ChildrenHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("children_health")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "儿童健康规则",
            &[("营养", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_children_health_rules() {
        let r = ChildrenHealthRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
