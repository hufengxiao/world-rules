//! 动物行为学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: AnimalBehaviorRules, name: "动物行为学定律", desc: "动物行为学定律", origin: "国际", tags: ["科学", "生物"] }
impl AnimalBehaviorRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["固定动作模式", "释放机制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["条件反射", "印记", "观察学习"]
    }
}
impl Rule for AnimalBehaviorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("animal_behavior")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "动物行为学定律",
            &[("本能", &self.section_0()), ("学习", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AnimalBehaviorRules::new();
        assert!(!r.explain().is_empty());
    }
}
