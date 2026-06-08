//! 合同法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ContractDetailed2Rules, name: "合同法详解2", desc: "合同法详解2", origin: "中国", tags: ["法律", "民法"] }
impl ContractDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["无效合同", "可撤销合同"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["法定解除", "约定解除"]
    }
}
impl Rule for ContractDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("contract_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "合同法详解2",
            &[("效力", &self.section_0()), ("解除", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ContractDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
