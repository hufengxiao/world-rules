//! 呼吸健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RespiratoryHealthRules, name: "呼吸健康规则", desc: "呼吸健康护理规则", origin: "国际", tags: ["健康", "呼吸"] }
impl RespiratoryHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["空气质量"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["呼吸训练"]
    }
}
impl Rule for RespiratoryHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("respiratory_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "呼吸健康规则",
            &[("预防", &self.section_0()), ("锻炼", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RespiratoryHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
