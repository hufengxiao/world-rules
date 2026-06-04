//! 海洋生物学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MarineBiologyRules,
    name: "海洋生物学定律",
    desc: "海洋生物学定律",
    origin: "国际",
    tags: ["科学", "生物"]
}

impl MarineBiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["珊瑚礁生态", "深海热泉生态", "潮间带生态"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["海洋酸化", "海平面上升", "海洋污染"]
    }
}

impl Rule for MarineBiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("marine_biology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "海洋生物学定律",
            &[("生态系统", &self.section_0()), ("环境", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_marine_biology_rules() {
        let r = MarineBiologyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
