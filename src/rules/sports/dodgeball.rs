//! 躲避球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DodgeballRules, name: "躲避球规则", desc: "躲避球竞赛规则", origin: "国际", tags: ["体育", "休闲"] }
impl DodgeballRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["6人制击中出局"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["3局2胜"]
    }
}
impl Rule for DodgeballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("dodgeball")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "躲避球规则",
            &[("基本", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DodgeballRules::new();
        assert!(!r.explain().is_empty());
    }
}
