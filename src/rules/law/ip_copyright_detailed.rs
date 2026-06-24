//! 版权法详解2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IpCopyrightDetailedRules, name: "版权法详解2", desc: "版权法详解2", origin: "中国", tags: ["法律", "知识产权"] }
impl IpCopyrightDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["著作权邻接权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["合理使用"]
    }
}
impl Rule for IpCopyrightDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ip_copyright_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "版权法详解2",
            &[("权利", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IpCopyrightDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
