//! 民诉法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CivilProcedureDetailed2Rules, name: "民诉法详解2", desc: "民诉法详解2", origin: "中国", tags: ["法律", "诉讼"] }
impl CivilProcedureDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["执行措施", "执行异议"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["财产保全", "行为保全"]
    }
}
impl Rule for CivilProcedureDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_procedure_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "民诉法详解2",
            &[("执行", &self.section_0()), ("保全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilProcedureDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
