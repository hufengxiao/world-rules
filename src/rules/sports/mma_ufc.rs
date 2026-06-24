//! UFC综合格斗规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MmaUfcRules, name: "UFC综合格斗规则", desc: "UFC综合格斗规则", origin: "美国", tags: ["体育", "格斗"] }
impl MmaUfcRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3回合5分钟"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["男子8级别"]
    }
}
impl Rule for MmaUfcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mma_ufc")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "UFC综合格斗规则",
            &[("回合", &self.section_0()), ("级别", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MmaUfcRules::new();
        assert!(!r.explain().is_empty());
    }
}
