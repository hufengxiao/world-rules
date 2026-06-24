//! 专利法详解2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IpPatentDetailedRules, name: "专利法详解2", desc: "专利法详解2", origin: "中国", tags: ["法律", "知识产权"] }
impl IpPatentDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发明实用新型"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["侵权判定"]
    }
}
impl Rule for IpPatentDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ip_patent_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "专利法详解2",
            &[("申请", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IpPatentDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
