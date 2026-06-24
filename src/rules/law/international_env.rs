//! 国际环境法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalEnvRules, name: "国际环境法", desc: "国际环境法律规则", origin: "国际", tags: ["法律", "环境"] }
impl InternationalEnvRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["巴黎协定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生物多样性公约"]
    }
}
impl Rule for InternationalEnvRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_env")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际环境法",
            &[("气候", &self.section_0()), ("生物", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalEnvRules::new();
        assert!(!r.explain().is_empty());
    }
}
