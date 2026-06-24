//! 矿物学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MineralogyDetailedRules, name: "矿物学详细定律", desc: "矿物学定律", origin: "国际", tags: ["科学", "地球"] }
impl MineralogyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["晶系"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["硬度光泽"]
    }
}
impl Rule for MineralogyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("mineralogy_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "矿物学详细定律",
            &[("结晶", &self.section_0()), ("性质", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MineralogyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
