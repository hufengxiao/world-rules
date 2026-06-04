//! 三人篮球详细
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: Basketball3x3DetailRules, name: "三人篮球详细", desc: "三人篮球详细规则", origin: "FIBA", tags: ["体育", "球类"] }
impl Basketball3x3DetailRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["10分钟", "21分获胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["12秒进攻"]
    }
}
impl Rule for Basketball3x3DetailRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_3x3_detail")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "三人篮球详细",
            &[("比赛", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = Basketball3x3DetailRules::new();
        assert!(!r.explain().is_empty());
    }
}
