//! 节日饮食礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseFestivalFoodRules, name: "节日饮食礼仪", desc: "中国节日饮食礼仪", origin: "中国", tags: ["社交", "饮食"] }
impl ChineseFestivalFoodRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["团圆"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["分享"]
    }
}
impl Rule for ChineseFestivalFoodRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_festival_food")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "节日饮食礼仪",
            &[("年夜饭", &self.section_0()), ("月饼", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseFestivalFoodRules::new();
        assert!(!r.explain().is_empty());
    }
}
