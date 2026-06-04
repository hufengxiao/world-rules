//! 柔道详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: JudoDetailedRules, name: "柔道详细规则", desc: "柔道详细比赛规则", origin: "IJF", tags: ["体育", "格斗"] }
impl JudoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一本", "技有"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["禁止动作", "消极比赛"]
    }
}
impl Rule for JudoDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("judo_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "柔道详细规则",
            &[("得分", &self.section_0()), ("犯规", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = JudoDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
