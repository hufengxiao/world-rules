//! 航空法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: AviationLawRules, name: "航空法详解", desc: "民用航空法律规则", origin: "中国", tags: ["法律", "航空"] }
impl AviationLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["适航管理", "航线管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["承运人责任", "事故调查"]
    }
}
impl Rule for AviationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("aviation_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "航空法详解",
            &[("运营", &self.section_0()), ("责任", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AviationLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
