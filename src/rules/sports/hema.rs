//! 欧洲历史武术规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HemaRules, name: "欧洲历史武术规则", desc: "HEMA欧洲历史武术", origin: "欧洲", tags: ["体育", "格斗"] }
impl HemaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["长剑刺剑"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["计分规则"]
    }
}
impl Rule for HemaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hema")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "欧洲历史武术规则",
            &[("武器", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HemaRules::new();
        assert!(!r.explain().is_empty());
    }
}
