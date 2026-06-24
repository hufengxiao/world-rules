//! 桑搏FIAS详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SamboFiasDetailedRules, name: "桑搏FIAS详细规则", desc: "桑搏国际规则", origin: "俄罗斯", tags: ["体育", "格斗"] }
impl SamboFiasDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["运动桑搏战斗桑搏"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["投技固技"]
    }
}
impl Rule for SamboFiasDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sambo_fias_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "桑搏FIAS详细规则",
            &[("类型", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SamboFiasDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
