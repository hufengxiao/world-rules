//! 水橇IWWF规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WaterSkiingIwwfRules, name: "水橇IWWF规则", desc: "水橇国际规则", origin: "国际", tags: ["体育", "水上"] }
impl WaterSkiingIwwfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["回旋跳跃花样"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["救生衣"]
    }
}
impl Rule for WaterSkiingIwwfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_skiing_iwwf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "水橇IWWF规则",
            &[("项目", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WaterSkiingIwwfRules::new();
        assert!(!r.explain().is_empty());
    }
}
