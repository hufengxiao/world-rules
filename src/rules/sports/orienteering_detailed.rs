//! 定向越野详细
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: OrienteeringDetailedRules, name: "定向越野详细", desc: "定向越野详细规则", origin: "IOF", tags: ["体育", "户外"] }
impl OrienteeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["徒步", "山地车"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["指卡打卡", "路线选择"]
    }
}
impl Rule for OrienteeringDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("orienteering_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "定向越野详细",
            &[("类型", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OrienteeringDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
