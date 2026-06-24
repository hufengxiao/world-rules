//! 男性健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MenHealthRules, name: "男性健康规则", desc: "男性健康规则", origin: "国际", tags: ["健康", "男性"] }
impl MenHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["前列腺"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["戒烟限酒"]
    }
}
impl Rule for MenHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("men_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "男性健康规则",
            &[("筛查", &self.section_0()), ("生活", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MenHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
