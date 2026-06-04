//! 台球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BilliardsDetailedRules, name: "台球详细规则", desc: "台球详细比赛规则", origin: "WPBSA", tags: ["体育", "桌球"] }
impl BilliardsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["15红球", "清台"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["开球", "赢球局"]
    }
}
impl Rule for BilliardsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("billiards_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "台球详细规则",
            &[("斯诺克", &self.section_0()), ("九球", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BilliardsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
