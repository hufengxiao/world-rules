//! IFMA泰拳详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MuayThaiIfmaDetailedRules, name: "IFMA泰拳详细规则", desc: "IFMA泰拳详细规则", origin: "泰国", tags: ["体育", "格斗"] }
impl MuayThaiIfmaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["5回合"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有效打击"]
    }
}
impl Rule for MuayThaiIfmaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("muay_thai_ifma_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "IFMA泰拳详细规则",
            &[("回合", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MuayThaiIfmaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
