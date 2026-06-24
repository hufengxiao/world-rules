//! 数据伦理定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DataEthicsRules, name: "数据伦理定律", desc: "数据伦理定律", origin: "国际", tags: ["科学", "伦理"] }
impl DataEthicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["数据隐私"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["算法公平"]
    }
}
impl Rule for DataEthicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("data_ethics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数据伦理定律",
            &[("隐私", &self.section_0()), ("公平", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DataEthicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
