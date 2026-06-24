//! 蚌棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AbaloneRules, name: "蚌棋规则", desc: "蚌棋桌游规则", origin: "法国", tags: ["游戏", "棋类"] }
impl AbaloneRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["推挤对手", "6方向移动"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["推出6颗者胜"]
    }
}
impl Rule for AbaloneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("abalone")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "蚌棋规则",
            &[("基本", &self.section_0()), ("胜负", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AbaloneRules::new();
        assert!(!r.explain().is_empty());
    }
}
