//! 硬地滚球BISFED
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BocceBisfedRules, name: "硬地滚球BISFED", desc: "BISFED硬地滚球规则", origin: "国际", tags: ["体育", "残疾人"] }
impl BocceBisfedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["BC1到BC4"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["投掷距离"]
    }
}
impl Rule for BocceBisfedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bocce_bisfed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "硬地滚球BISFED",
            &[("分级", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BocceBisfedRules::new();
        assert!(!r.explain().is_empty());
    }
}
