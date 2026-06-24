//! F1 FIA详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: F1FiaDetailedRules, name: "F1 FIA详细规则", desc: "FIA一级方程式规则", origin: "国际", tags: ["体育", "赛车"] }
impl F1FiaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["25-18-15-12-10"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["动力单元轮胎"]
    }
}
impl Rule for F1FiaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("f1_fia_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "F1 FIA详细规则",
            &[("积分", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = F1FiaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
