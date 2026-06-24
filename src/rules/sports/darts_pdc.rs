//! PDC飞镖规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DartsPdcRules, name: "PDC飞镖规则", desc: "PDC飞镖锦标赛", origin: "英国", tags: ["体育", "休闲"] }
impl DartsPdcRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["501减分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["双倍结束"]
    }
}
impl Rule for DartsPdcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("darts_pdc")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "PDC飞镖规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DartsPdcRules::new();
        assert!(!r.explain().is_empty());
    }
}
