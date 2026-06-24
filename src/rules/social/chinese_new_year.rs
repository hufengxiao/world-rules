//! 春节礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseNewYearRules, name: "春节礼仪", desc: "中国春节礼仪", origin: "中国", tags: ["社交", "节日"] }
impl ChineseNewYearRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["年夜饭守岁"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拜年礼节红包"]
    }
}
impl Rule for ChineseNewYearRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_new_year")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "春节礼仪",
            &[("除夕", &self.section_0()), ("拜年", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseNewYearRules::new();
        assert!(!r.explain().is_empty());
    }
}
