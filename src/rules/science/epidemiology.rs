//! 流行病学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: EpidemiologyRules, name: "流行病学定律", desc: "流行病学定律", origin: "国际", tags: ["科学", "医学"] }
impl EpidemiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["队列研究", "病例对照"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["发病率", "相对风险"]
    }
}
impl Rule for EpidemiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("epidemiology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "流行病学定律",
            &[("方法", &self.section_0()), ("指标", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EpidemiologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
