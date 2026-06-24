//! 自由式滑雪FIS
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FreestyleFisDetailedRules, name: "自由式滑雪FIS", desc: "FIS自由式滑雪规则", origin: "国际", tags: ["体育", "冬季"] }
impl FreestyleFisDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["雪上技巧空中技巧"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["难度完成"]
    }
}
impl Rule for FreestyleFisDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("freestyle_fis_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "自由式滑雪FIS",
            &[("项目", &self.section_0()), ("评分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FreestyleFisDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
