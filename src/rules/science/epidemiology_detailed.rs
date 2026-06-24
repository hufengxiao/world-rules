//! 流行病学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EpidemiologyDetailedRules, name: "流行病学详细定律", desc: "流行病学定律", origin: "国际", tags: ["科学", "医学"] }
impl EpidemiologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["队列病例对照"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["发病率死亡率"]
    }
}
impl Rule for EpidemiologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("epidemiology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "流行病学详细定律",
            &[("方法", &self.section_0()), ("指标", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EpidemiologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
