//! 缅甸拳详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LethweiMyanmarRules, name: "缅甸拳详细规则", desc: "缅甸拳Lethwei规则", origin: "缅甸", tags: ["体育", "格斗"] }
impl LethweiMyanmarRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["无手套头槌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["KO获胜"]
    }
}
impl Rule for LethweiMyanmarRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("lethwei_myanmar")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "缅甸拳详细规则",
            &[("特殊", &self.section_0()), ("KO", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LethweiMyanmarRules::new();
        assert!(!r.explain().is_empty());
    }
}
