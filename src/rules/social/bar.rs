//! 酒吧礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: BarRules,
    name: "酒吧礼仪",
    desc: "酒吧社交礼仪",
    origin: "国际",
    tags: ["社交", "酒吧"]
}

impl BarRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["了解酒单", "适度点酒", "注意小费文化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不大声喧哗", "尊重调酒师", "适度社交"]
    }
}

impl Rule for BarRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("bar")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "酒吧礼仪",
            &[("点酒", &self.section_0()), ("行为", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bar_rules() {
        let r = BarRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
