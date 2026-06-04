//! 知识产权详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: IpDetailedRules, name: "知识产权详解", desc: "知识产权法详解", origin: "中国", tags: ["法律", "知识产权"] }
impl IpDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发明专利", "实用新型"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["著作权", "合理使用"]
    }
}
impl Rule for IpDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ip_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "知识产权详解",
            &[("专利", &self.section_0()), ("版权", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IpDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
