//! 气象学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MeteorologyDetailedRules, name: "气象学详细定律", desc: "气象学定律", origin: "国际", tags: ["科学", "地球"] }
impl MeteorologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["气压温度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["锋面气旋"]
    }
}
impl Rule for MeteorologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("meteorology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "气象学详细定律",
            &[("大气", &self.section_0()), ("天气", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MeteorologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
