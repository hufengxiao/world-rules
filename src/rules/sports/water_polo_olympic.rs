//! 水球奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WaterPoloOlympicRules, name: "水球奥运规则", desc: "水球奥运会规则", origin: "国际", tags: ["体育", "水上"] }
impl WaterPoloOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4节"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["30秒进攻"]
    }
}
impl Rule for WaterPoloOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_polo_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "水球奥运规则",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WaterPoloOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
