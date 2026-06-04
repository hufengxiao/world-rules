//! 森林法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ForestDetailedRules, name: "森林法详解", desc: "森林法详解", origin: "中国", tags: ["法律", "资源"] }
impl ForestDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国有林", "集体林"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["采伐限额", "天然林保护"]
    }
}
impl Rule for ForestDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("forest_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "森林法详解",
            &[("权属", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ForestDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
