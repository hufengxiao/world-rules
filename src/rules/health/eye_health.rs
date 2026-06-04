//! 眼睛健康规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: EyeHealthRules,
    name: "眼睛健康规则",
    desc: "眼睛健康保护规则",
    origin: "国际",
    tags: ["健康", "视力"]
}

impl EyeHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["20-20-20法则", "保持距离", "充足光线"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["佩戴太阳镜", "防蓝光", "眼保健操"]
    }
}

impl Rule for EyeHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("eye_health")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "眼睛健康规则",
            &[("用眼", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eye_health_rules() {
        let r = EyeHealthRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
