//! 围棋详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GoDetailedRules, name: "围棋详细规则", desc: "围棋详细规则", origin: "中国", tags: ["游戏", "棋类"] }
impl GoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["落子", "提子", "禁着点"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["中国规则", "日本规则", "贴目"]
    }
}
impl Rule for GoDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "围棋详细规则",
            &[("基本", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GoDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
