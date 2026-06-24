//! 商标法详解2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IpTrademarkDetailedRules, name: "商标法详解2", desc: "商标法详解2", origin: "中国", tags: ["法律", "知识产权"] }
impl IpTrademarkDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["申请审查"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["侵权驰名"]
    }
}
impl Rule for IpTrademarkDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ip_trademark_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "商标法详解2",
            &[("注册", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IpTrademarkDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
