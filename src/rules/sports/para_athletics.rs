//! 残疾人田径规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ParaAthleticsRules, name: "残疾人田径规则", desc: "残疾人田径规则", origin: "国际", tags: ["体育", "残疾人"] }
impl ParaAthleticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["T/F11到T/F64"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["径赛田赛"]
    }
}
impl Rule for ParaAthleticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_athletics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "残疾人田径规则",
            &[("分级", &self.section_0()), ("项目", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ParaAthleticsRules::new();
        assert!(!r.explain().is_empty());
    }
}
