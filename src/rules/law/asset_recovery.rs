//! 资产追回法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AssetRecoveryRules, name: "资产追回法", desc: "资产追回法律规则", origin: "国际", tags: ["法律", "反腐"] }
impl AssetRecoveryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["冻结追缴"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["国际合作"]
    }
}
impl Rule for AssetRecoveryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("asset_recovery")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "资产追回法",
            &[("程序", &self.section_0()), ("国际", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AssetRecoveryRules::new();
        assert!(!r.explain().is_empty());
    }
}
