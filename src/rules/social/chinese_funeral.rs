//! 中国丧葬礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseFuneralRules, name: "中国丧葬礼仪", desc: "中国传统丧葬礼仪", origin: "中国", tags: ["社交", "丧葬"] }
impl ChineseFuneralRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["随礼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["守孝礼节"]
    }
}
impl Rule for ChineseFuneralRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_funeral")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国丧葬礼仪",
            &[("吊唁", &self.section_0()), ("守孝", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseFuneralRules::new();
        assert!(!r.explain().is_empty());
    }
}
