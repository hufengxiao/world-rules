//! 残疾人铁人三项
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ParatriathlonRules, name: "残疾人铁人三项", desc: "残疾人铁人三项规则", origin: "国际", tags: ["体育", "残疾人"] }
impl ParatriathlonRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["PTWC PTS"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["游骑跑"]
    }
}
impl Rule for ParatriathlonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("paratriathlon")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "残疾人铁人三项",
            &[("分级", &self.section_0()), ("项目", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ParatriathlonRules::new();
        assert!(!r.explain().is_empty());
    }
}
