//! 海商法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MaritimeDetailedRules, name: "海商法详解", desc: "海商法详解", origin: "中国", tags: ["法律", "商法"] }
impl MaritimeDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["船舶登记", "优先权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["提单", "货物运输"]
    }
}
impl Rule for MaritimeDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("maritime_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "海商法详解",
            &[("船舶", &self.section_0()), ("运输", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MaritimeDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
