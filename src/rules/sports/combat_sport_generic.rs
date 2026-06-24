//! 格斗通用规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CombatSportGenericRules, name: "格斗通用规则", desc: "格斗运动通用规则", origin: "国际", tags: ["体育", "格斗"] }
impl CombatSportGenericRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["体重分级"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["护具拳套"]
    }
}
impl Rule for CombatSportGenericRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("combat_sport_generic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "格斗通用规则",
            &[("级别", &self.section_0()), ("装备", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CombatSportGenericRules::new();
        assert!(!r.explain().is_empty());
    }
}
