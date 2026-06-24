//! 乔迁礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseHousewarmingRules, name: "乔迁礼仪", desc: "中国传统乔迁礼仪", origin: "中国", tags: ["社交", "乔迁"] }
impl ChineseHousewarmingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["择日"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["暖房"]
    }
}
impl Rule for ChineseHousewarmingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_housewarming")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "乔迁礼仪",
            &[("搬家", &self.section_0()), ("宴客", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseHousewarmingRules::new();
        assert!(!r.explain().is_empty());
    }
}
