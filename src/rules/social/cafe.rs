//! 咖啡礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CafeRules,
    name: "咖啡礼仪",
    desc: "咖啡社交礼仪",
    origin: "国际",
    tags: ["社交", "咖啡"]
}

impl CafeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["了解咖啡种类", "注意排队礼仪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不发出声响", "使用杯把", "搅拌后取出勺子"]
    }
}

impl Rule for CafeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("cafe")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "咖啡礼仪",
            &[("点单", &self.section_0()), ("饮用", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cafe_rules() {
        let r = CafeRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
