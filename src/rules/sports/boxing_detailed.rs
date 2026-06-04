//! 拳击详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BoxingDetailedRules, name: "拳击详细规则", desc: "拳击详细比赛规则", origin: "WBA", tags: ["体育", "格斗"] }
impl BoxingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3分钟一回合", "KO判定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有效打击", "点数判定"]
    }
}
impl Rule for BoxingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "拳击详细规则",
            &[("回合", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BoxingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
