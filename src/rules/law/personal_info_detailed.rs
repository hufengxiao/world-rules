//! 个保法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PersonalInfoDetailedRules, name: "个保法详解", desc: "个人信息保护法详解", origin: "中国", tags: ["法律", "数据"] }
impl PersonalInfoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合法必要"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["知情删除"]
    }
}
impl Rule for PersonalInfoDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("personal_info_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "个保法详解",
            &[("原则", &self.section_0()), ("权利", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PersonalInfoDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
