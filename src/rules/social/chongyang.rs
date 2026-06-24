//! 重阳节礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChongyangRules, name: "重阳节礼仪", desc: "重阳节传统礼仪", origin: "中国", tags: ["社交", "节日"] }
impl ChongyangRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["登高习俗"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["敬老礼节"]
    }
}
impl Rule for ChongyangRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chongyang")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "重阳节礼仪",
            &[("登高", &self.section_0()), ("敬老", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChongyangRules::new();
        assert!(!r.explain().is_empty());
    }
}
