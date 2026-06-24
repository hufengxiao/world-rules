//! 玉米洞ACL详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CornholeAclDetailedRules, name: "玉米洞ACL详细", desc: "ACL玉米洞规则", origin: "美国", tags: ["体育", "休闲"] }
impl CornholeAclDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["21分获胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["板1分洞3分"]
    }
}
impl Rule for CornholeAclDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cornhole_acl_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "玉米洞ACL详细",
            &[("比赛", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CornholeAclDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
