//! 卡波耶拉roda规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CapoeiraRodaRules, name: "卡波耶拉roda规则", desc: "卡波耶拉传统规则", origin: "巴西", tags: ["体育", "格斗"] }
impl CapoeiraRodaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["圆圈仪式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["音乐节奏"]
    }
}
impl Rule for CapoeiraRodaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("capoeira_roda")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "卡波耶拉roda规则",
            &[("roda", &self.section_0()), ("音乐", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CapoeiraRodaRules::new();
        assert!(!r.explain().is_empty());
    }
}
