//! 漂流IRF规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WhiteWaterRaftingIrfRules, name: "漂流IRF规则", desc: "漂流国际联合会规则", origin: "国际", tags: ["体育", "水上"] }
impl WhiteWaterRaftingIrfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["激流赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["救生衣头盔"]
    }
}
impl Rule for WhiteWaterRaftingIrfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("white_water_rafting_irf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "漂流IRF规则",
            &[("比赛", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WhiteWaterRaftingIrfRules::new();
        assert!(!r.explain().is_empty());
    }
}
