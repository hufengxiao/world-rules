//! 中式婚礼礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseWeddingRules, name: "中式婚礼礼仪", desc: "中国传统婚礼礼仪", origin: "中国", tags: ["社交", "婚礼"] }
impl ChineseWeddingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["纳采问名"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拜堂交杯"]
    }
}
impl Rule for ChineseWeddingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_wedding")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中式婚礼礼仪",
            &[("六礼", &self.section_0()), ("仪式", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseWeddingRules::new();
        assert!(!r.explain().is_empty());
    }
}
