//! 晶体学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CrystallographyRules, name: "晶体学定律", desc: "晶体学定律", origin: "国际", tags: ["科学", "化学"] }
impl CrystallographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["晶格类型", "空间群"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["X射线衍射", "电子衍射"]
    }
}
impl Rule for CrystallographyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("crystallography")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "晶体学定律",
            &[("结构", &self.section_0()), ("分析", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CrystallographyRules::new();
        assert!(!r.explain().is_empty());
    }
}
