//! 点格棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DotsAndBoxesRules, name: "点格棋规则", desc: "点格棋游戏规则", origin: "国际", tags: ["游戏", "棋类"] }
impl DotsAndBoxesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["连接点", "形成方格"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["让对手开链"]
    }
}
impl Rule for DotsAndBoxesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("dots_and_boxes")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "点格棋规则",
            &[("基本", &self.section_0()), ("策略", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DotsAndBoxesRules::new();
        assert!(!r.explain().is_empty());
    }
}
