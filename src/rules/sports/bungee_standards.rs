//! 蹦极安全标准
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BungeeStandardsRules, name: "蹦极安全标准", desc: "蹦极运动安全标准", origin: "新西兰", tags: ["体育", "极限"] }
impl BungeeStandardsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["设备检查弹力绳"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["桥塔"]
    }
}
impl Rule for BungeeStandardsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bungee_standards")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "蹦极安全标准",
            &[("安全", &self.section_0()), ("场地", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BungeeStandardsRules::new();
        assert!(!r.explain().is_empty());
    }
}
