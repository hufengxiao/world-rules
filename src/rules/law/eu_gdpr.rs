//! GDPR规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EuGdprRules, name: "GDPR规则", desc: "欧盟通用数据保护规则", origin: "欧盟", tags: ["法律", "数据"] }
impl EuGdprRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合法目的最小化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["访问删除可携带"]
    }
}
impl Rule for EuGdprRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("eu_gdpr")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "GDPR规则",
            &[("原则", &self.section_0()), ("权利", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EuGdprRules::new();
        assert!(!r.explain().is_empty());
    }
}
