//! 房车世界杯规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WtcrRules, name: "房车世界杯规则", desc: "WTCR房车世界杯", origin: "国际", tags: ["体育", "赛车"] }
impl WtcrRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["冲刺赛正赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["BoP平衡"]
    }
}
impl Rule for WtcrRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wtcr")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "房车世界杯规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WtcrRules::new();
        assert!(!r.explain().is_empty());
    }
}
