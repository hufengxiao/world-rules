//! 数据库理论定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DatabaseTheoryRules, name: "数据库理论定律", desc: "数据库理论定律", origin: "国际", tags: ["科学", "计算机"] }
impl DatabaseTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["SQL范式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["MongoDB Redis"]
    }
}
impl Rule for DatabaseTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("database_theory")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数据库理论定律",
            &[("关系", &self.section_0()), ("NoSQL", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DatabaseTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
