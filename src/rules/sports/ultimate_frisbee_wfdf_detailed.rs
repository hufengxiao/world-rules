//! 极限飞盘WFDF
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: UltimateFrisbeeWfdfDetailedRules, name: "极限飞盘WFDF", desc: "WFDF极限飞盘规则", origin: "国际", tags: ["体育", "休闲"] }
impl UltimateFrisbeeWfdfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["7v7得分区"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["自行裁决"]
    }
}
impl Rule for UltimateFrisbeeWfdfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ultimate_frisbee_wfdf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "极限飞盘WFDF",
            &[("比赛", &self.section_0()), ("精神", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = UltimateFrisbeeWfdfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
