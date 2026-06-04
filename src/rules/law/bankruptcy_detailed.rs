//! 破产法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BankruptcyDetailedRules, name: "破产法详解", desc: "破产法详解", origin: "中国", tags: ["法律", "商法"] }
impl BankruptcyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["破产申请", "管理人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["重整计划", "和解"]
    }
}
impl Rule for BankruptcyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("bankruptcy_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "破产法详解",
            &[("程序", &self.section_0()), ("重整", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BankruptcyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
