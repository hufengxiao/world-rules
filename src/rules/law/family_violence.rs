//! 反家暴法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: FamilyViolenceRules, name: "反家暴法", desc: "反家庭暴力法律规则", origin: "中国", tags: ["法律", "家庭"] }
impl FamilyViolenceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人身安全保护令", "告诫书"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["强制报告", "法治宣传"]
    }
}
impl Rule for FamilyViolenceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("family_violence")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "反家暴法",
            &[("措施", &self.section_0()), ("预防", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FamilyViolenceRules::new();
        assert!(!r.explain().is_empty());
    }
}
