//! 合同法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ContractDetailedRules, name: "合同法详解", desc: "合同法详解", origin: "中国", tags: ["法律", "民法"] }
impl ContractDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["要约承诺", "格式条款"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["继续履行", "损害赔偿"]
    }
}
impl Rule for ContractDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("contract_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "合同法详解",
            &[("订立", &self.section_0()), ("违约", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ContractDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
