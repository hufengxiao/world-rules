//! 行为经济学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BehavioralEconomicsRules, name: "行为经济学定律", desc: "行为经济学定律", origin: "国际", tags: ["科学", "经济"] }
impl BehavioralEconomicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["锚定效应", "损失厌恶"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["前景理论", "助推理论"]
    }
}
impl Rule for BehavioralEconomicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("behavioral_economics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "行为经济学定律",
            &[("偏差", &self.section_0()), ("理论", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BehavioralEconomicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
