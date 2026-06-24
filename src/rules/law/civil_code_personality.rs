//! 民法典人格权详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodePersonalityRules, name: "民法典人格权详解", desc: "民法典人格权详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodePersonalityRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["生命健康姓名"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["隐私个人信息"]
    }
}
impl Rule for CivilCodePersonalityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_personality")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典人格权详解",
            &[("权利", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodePersonalityRules::new();
        assert!(!r.explain().is_empty());
    }
}
