//! 中国茶道礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseTeaCeremonyRules, name: "中国茶道礼仪", desc: "中国茶道礼仪", origin: "中国", tags: ["社交", "茶道"] }
impl ChineseTeaCeremonyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["温壶投茶"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["闻香品茗"]
    }
}
impl Rule for ChineseTeaCeremonyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_tea_ceremony")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国茶道礼仪",
            &[("泡茶", &self.section_0()), ("品茶", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseTeaCeremonyRules::new();
        assert!(!r.explain().is_empty());
    }
}
