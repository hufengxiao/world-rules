//! 电动方程式规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FormulaERules, name: "电动方程式规则", desc: "FE电动方程式规则", origin: "国际", tags: ["体育", "赛车"] }
impl FormulaERules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["攻击模式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["街道赛"]
    }
}
impl Rule for FormulaERules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("formula_e")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电动方程式规则",
            &[("特殊", &self.section_0()), ("赛道", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FormulaERules::new();
        assert!(!r.explain().is_empty());
    }
}
