//! Bellator MMA规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MmaBellatorRules, name: "Bellator MMA规则", desc: "Bellator MMA规则", origin: "美国", tags: ["体育", "格斗"] }
impl MmaBellatorRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["锦标赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["统一规则"]
    }
}
impl Rule for MmaBellatorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mma_bellator")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Bellator MMA规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MmaBellatorRules::new();
        assert!(!r.explain().is_empty());
    }
}
