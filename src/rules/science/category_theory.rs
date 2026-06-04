//! 范畴论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CategoryTheoryRules, name: "范畴论定律", desc: "范畴论定律", origin: "国际", tags: ["科学", "数学"] }
impl CategoryTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["函子", "自然变换", "伴随函子"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拓扑斯", "同调代数"]
    }
}
impl Rule for CategoryTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("category_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "范畴论定律",
            &[("基础", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CategoryTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
