//! 税法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: TaxDetailedRules, name: "税法详解", desc: "税法详解", origin: "中国", tags: ["法律", "税法"] }
impl TaxDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["增值税", "消费关税"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["企业所得税", "个人所得税"]
    }
}
impl Rule for TaxDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("tax_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "税法详解",
            &[("流转税", &self.section_0()), ("所得税", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TaxDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
