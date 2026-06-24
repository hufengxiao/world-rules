//! WKF空手道详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KarateWkfDetailedRules, name: "WKF空手道详细规则", desc: "WKF空手道详细规则", origin: "日本", tags: ["体育", "格斗"] }
impl KarateWkfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["得分区域"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["规定型自选型"]
    }
}
impl Rule for KarateWkfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("karate_wkf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WKF空手道详细规则",
            &[("组手", &self.section_0()), ("型", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KarateWkfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
