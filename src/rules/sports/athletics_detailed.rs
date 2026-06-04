//! 田径详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: AthleticsDetailedRules, name: "田径详细规则", desc: "田径详细比赛规则", origin: "IAAF", tags: ["体育", "田径"] }
impl AthleticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["起跑规则", "抢跑判罚"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["试跳试投", "成绩测量"]
    }
}
impl Rule for AthleticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("athletics_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "田径详细规则",
            &[("径赛", &self.section_0()), ("田赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AthleticsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
