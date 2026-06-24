//! 书法礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseCalligraphyRules, name: "书法礼仪", desc: "中国传统书法礼仪", origin: "中国", tags: ["社交", "文化"] }
impl ChineseCalligraphyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["磨墨执笔"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["书写规范"]
    }
}
impl Rule for ChineseCalligraphyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_calligraphy")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "书法礼仪",
            &[("笔墨", &self.section_0()), ("书写", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseCalligraphyRules::new();
        assert!(!r.explain().is_empty());
    }
}
