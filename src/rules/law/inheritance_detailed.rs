//! 继承法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: InheritanceDetailedRules, name: "继承法详解", desc: "继承法详解", origin: "中国", tags: ["法律", "家庭"] }
impl InheritanceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["继承顺序", "代位继承"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["遗嘱形式", "遗嘱效力"]
    }
}
impl Rule for InheritanceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("inheritance_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "继承法详解",
            &[
                ("法定继承", &self.section_0()),
                ("遗嘱继承", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InheritanceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
