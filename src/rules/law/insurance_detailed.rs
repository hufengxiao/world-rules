//! 保险法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: InsuranceDetailedRules, name: "保险法详解", desc: "保险法律规则", origin: "中国", tags: ["法律", "金融"] }
impl InsuranceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["保险利益", "如实告知"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["偿付能力", "准备金"]
    }
}
impl Rule for InsuranceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("insurance_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "保险法详解",
            &[("合同", &self.section_0()), ("监管", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InsuranceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
