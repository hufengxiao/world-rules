//! 水球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: WaterPoloDetailedRules, name: "水球详细规则", desc: "水球详细比赛规则", origin: "FINA", tags: ["体育", "水上"] }
impl WaterPoloDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4节", "30秒进攻"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["普通犯规", "罚出场"]
    }
}
impl Rule for WaterPoloDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_polo_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "水球详细规则",
            &[("比赛", &self.section_0()), ("犯规", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WaterPoloDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
