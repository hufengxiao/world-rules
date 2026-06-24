//! 辩诉交易法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PleaBargainingRules, name: "辩诉交易法", desc: "辩诉交易法律规则", origin: "美国", tags: ["法律", "刑事"] }
impl PleaBargainingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自愿明知"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["协商量刑"]
    }
}
impl Rule for PleaBargainingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("plea_bargaining")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "辩诉交易法",
            &[("条件", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PleaBargainingRules::new();
        assert!(!r.explain().is_empty());
    }
}
