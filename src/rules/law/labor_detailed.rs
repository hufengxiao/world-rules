//! 劳动法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: LaborDetailedRules, name: "劳动法详解", desc: "劳动法详解", origin: "中国", tags: ["法律", "劳动"] }
impl LaborDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["固定期限", "试用期"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工时制度", "带薪年假"]
    }
}
impl Rule for LaborDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "劳动法详解",
            &[("合同", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LaborDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
