//! 民法典继承详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodeInheritanceRules, name: "民法典继承详解", desc: "民法典继承详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodeInheritanceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["继承顺序"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["遗嘱形式"]
    }
}
impl Rule for CivilCodeInheritanceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_inheritance")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典继承详解",
            &[("法定", &self.section_0()), ("遗嘱", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodeInheritanceRules::new();
        assert!(!r.explain().is_empty());
    }
}
