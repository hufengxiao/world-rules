//! 沙排详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BeachVolleyballDetailRules, name: "沙排详细规则", desc: "沙滩排球详细规则", origin: "FIVB", tags: ["体育", "沙滩"] }
impl BeachVolleyballDetailRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["2人制", "15分制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["换人限制"]
    }
}
impl Rule for BeachVolleyballDetailRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("beach_volleyball_detail")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "沙排详细规则",
            &[("比赛", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BeachVolleyballDetailRules::new();
        assert!(!r.explain().is_empty());
    }
}
