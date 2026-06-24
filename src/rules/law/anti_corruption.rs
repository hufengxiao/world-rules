//! 反腐败法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AntiCorruptionRules, name: "反腐败法", desc: "反腐败法律规则", origin: "国际", tags: ["法律", "反腐"] }
impl AntiCorruptionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["联合国反腐公约"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["财产申报"]
    }
}
impl Rule for AntiCorruptionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("anti_corruption")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "反腐败法",
            &[("公约", &self.section_0()), ("制度", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AntiCorruptionRules::new();
        assert!(!r.explain().is_empty());
    }
}
