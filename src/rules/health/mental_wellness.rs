//! 心理健康维护规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MentalWellnessRules,
    name: "心理健康维护规则",
    desc: "心理健康维护规则",
    origin: "国际",
    tags: ["健康", "心理"]
}

impl MentalWellnessRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["正念冥想", "深呼吸", "规律作息"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["识别心理问题", "寻求专业帮助", "不讳疾忌医"]
    }
}

impl Rule for MentalWellnessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("mental_wellness")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "心理健康维护规则",
            &[("自我调节", &self.section_0()), ("求助", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mental_wellness_rules() {
        let r = MentalWellnessRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
