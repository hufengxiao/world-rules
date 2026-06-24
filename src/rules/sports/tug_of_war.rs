//! 拔河规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TugOfWarRules, name: "拔河规则", desc: "拔河竞赛规则", origin: "国际", tags: ["体育", "休闲"] }
impl TugOfWarRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["6人制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["体重分级"]
    }
}
impl Rule for TugOfWarRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tug_of_war")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "拔河规则",
            &[("规则", &self.section_0()), ("级别", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TugOfWarRules::new();
        assert!(!r.explain().is_empty());
    }
}
