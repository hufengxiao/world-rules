//! 骰子游戏规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DiceGameRules, name: "骰子游戏规则", desc: "骰子游戏通用规则", origin: "国际", tags: ["游戏", "骰子"] }
impl DiceGameRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["比大小凑组合"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["概率计算"]
    }
}
impl Rule for DiceGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("dice_game")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "骰子游戏规则",
            &[("玩法", &self.section_0()), ("策略", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DiceGameRules::new();
        assert!(!r.explain().is_empty());
    }
}
