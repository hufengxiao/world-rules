//! 野生动物保护法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: WildlifeProtectionLawRules,
    name: "野生动物保护法",
    desc: "野生动物保护法律规则",
    origin: "中国",
    tags: ["法律", "环境"]
}

impl WildlifeProtectionLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国家重点保护", "省级保护", "三有动物"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特许猎捕", "经营利用许可", "进出口管理"]
    }
}

impl Rule for WildlifeProtectionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("wildlife_protection_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "野生动物保护法",
            &[
                ("保护分级", &self.section_0()),
                ("利用限制", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wildlife_protection_law_rules() {
        let r = WildlifeProtectionLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
