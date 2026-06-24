//! 祭祖礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseAncestorWorshipRules, name: "祭祖礼仪", desc: "中国传统祭祖礼仪", origin: "中国", tags: ["社交", "祭祀"] }
impl ChineseAncestorWorshipRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["三牲"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["叩拜"]
    }
}
impl Rule for ChineseAncestorWorshipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_ancestor_worship")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "祭祖礼仪",
            &[("祭品", &self.section_0()), ("仪式", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseAncestorWorshipRules::new();
        assert!(!r.explain().is_empty());
    }
}
