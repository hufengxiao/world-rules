//! 寺庙礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseTempleRules, name: "寺庙礼仪", desc: "中国寺庙参拜礼仪", origin: "中国", tags: ["社交", "宗教"] }
impl ChineseTempleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["烧香礼佛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拜佛礼节"]
    }
}
impl Rule for ChineseTempleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_temple")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "寺庙礼仪",
            &[("烧香", &self.section_0()), ("拜佛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseTempleRules::new();
        assert!(!r.explain().is_empty());
    }
}
