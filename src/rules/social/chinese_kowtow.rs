//! 中国跪拜礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseKowtowRules, name: "中国跪拜礼仪", desc: "中国传统跪拜礼仪", origin: "中国", tags: ["社交", "礼仪"] }
impl ChineseKowtowRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["祭祀拜师"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["叩首鞠躬"]
    }
}
impl Rule for ChineseKowtowRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_kowtow")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国跪拜礼仪",
            &[("场合", &self.section_0()), ("礼节", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseKowtowRules::new();
        assert!(!r.explain().is_empty());
    }
}
