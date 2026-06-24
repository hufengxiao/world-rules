//! 四子连规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: QuartoRules, name: "四子连规则", desc: "四子连桌游规则", origin: "瑞士", tags: ["游戏", "棋类"] }
impl QuartoRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4属性4值", "选棋给对手"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先连四者胜"]
    }
}
impl Rule for QuartoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("quarto")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "四子连规则",
            &[("基本", &self.section_0()), ("胜负", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = QuartoRules::new();
        assert!(!r.explain().is_empty());
    }
}
