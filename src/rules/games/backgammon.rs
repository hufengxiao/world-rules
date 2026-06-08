//! 西洋双陆棋规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BackgammonRules, name: "西洋双陆棋规则", desc: "西洋双陆棋规则", origin: "国际", tags: ["游戏", "棋类"] }
impl BackgammonRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["掷骰子移动", "点数对应步数"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["建立防线", "封锁对手", " bearing off"]
    }
}
impl Rule for BackgammonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("backgammon")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "西洋双陆棋规则",
            &[("走法", &self.section_0()), ("策略", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BackgammonRules::new();
        assert!(!r.explain().is_empty());
    }
}
