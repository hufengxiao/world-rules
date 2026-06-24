//! 中国座次礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseSeatingRules, name: "中国座次礼仪", desc: "中国传统座次礼仪", origin: "中国", tags: ["社交", "座次"] }
impl ChineseSeatingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["主位安排"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["座次排列"]
    }
}
impl Rule for ChineseSeatingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_seating")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国座次礼仪",
            &[("宴席", &self.section_0()), ("会议", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseSeatingRules::new();
        assert!(!r.explain().is_empty());
    }
}
