//! 旅行健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TravelHealthRules, name: "旅行健康规则", desc: "旅行健康规则", origin: "国际", tags: ["健康", "旅行"] }
impl TravelHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["疫苗药物"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["饮水安全"]
    }
}
impl Rule for TravelHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("travel_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "旅行健康规则",
            &[("预防", &self.section_0()), ("饮食", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TravelHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
