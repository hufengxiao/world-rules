//! 独木舟规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OutriggerCanoeRules, name: "独木舟规则", desc: "太平洋独木舟规则", origin: "夏威夷", tags: ["体育", "水上"] }
impl OutriggerCanoeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["长距离"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["太平洋传统"]
    }
}
impl Rule for OutriggerCanoeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("outrigger_canoe")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "独木舟规则",
            &[("比赛", &self.section_0()), ("文化", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OutriggerCanoeRules::new();
        assert!(!r.explain().is_empty());
    }
}
