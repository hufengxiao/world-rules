//! IJF柔道详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: JudoIjfDetailedRules, name: "IJF柔道详细规则", desc: "IJF柔道详细规则", origin: "日本", tags: ["体育", "格斗"] }
impl JudoIjfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一本", "技有"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["禁止动作"]
    }
}
impl Rule for JudoIjfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("judo_ijf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "IJF柔道详细规则",
            &[("得分", &self.section_0()), ("禁止", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = JudoIjfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
