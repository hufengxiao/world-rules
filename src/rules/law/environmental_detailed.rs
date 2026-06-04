//! 环保法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: EnvironmentalDetailedRules, name: "环保法详解", desc: "环保法详解", origin: "中国", tags: ["法律", "环境"] }
impl EnvironmentalDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["环评制度", "排污许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["公益诉讼", "按日计罚"]
    }
}
impl Rule for EnvironmentalDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("environmental_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "环保法详解",
            &[("制度", &self.section_0()), ("责任", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnvironmentalDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
