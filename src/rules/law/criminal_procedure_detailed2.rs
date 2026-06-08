//! 刑诉法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CriminalProcedureDetailed2Rules, name: "刑诉法详解2", desc: "刑诉法详解2", origin: "中国", tags: ["法律", "诉讼"] }
impl CriminalProcedureDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["讯问", "搜查", "扣押"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["审查起诉", "不起诉", "附条件不起诉"]
    }
}
impl Rule for CriminalProcedureDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_procedure_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "刑诉法详解2",
            &[("侦查", &self.section_0()), ("检察", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalProcedureDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
