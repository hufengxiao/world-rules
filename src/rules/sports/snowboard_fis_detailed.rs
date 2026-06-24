//! 单板滑雪FIS
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SnowboardFisDetailedRules, name: "单板滑雪FIS", desc: "FIS单板滑雪规则", origin: "国际", tags: ["体育", "冬季"] }
impl SnowboardFisDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["U型场地大跳台"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["难度流畅"]
    }
}
impl Rule for SnowboardFisDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("snowboard_fis_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "单板滑雪FIS",
            &[("项目", &self.section_0()), ("评分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SnowboardFisDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
