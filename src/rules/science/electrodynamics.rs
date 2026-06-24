//! 电动力学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ElectrodynamicsRules, name: "电动力学定律", desc: "电动力学定律", origin: "国际", tags: ["科学", "物理"] }
impl ElectrodynamicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["麦克斯韦方程组"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["波动方程"]
    }
}
impl Rule for ElectrodynamicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("electrodynamics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电动力学定律",
            &[
                ("麦克斯韦", &self.section_0()),
                ("电磁波", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ElectrodynamicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
