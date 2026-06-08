//! 德州扑克详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: TexasHoldemDetailedRules, name: "德州扑克详细规则", desc: "德州扑克详细比赛规则", origin: "美国", tags: ["游戏", "扑克"] }
impl TexasHoldemDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["庄家", "小盲注", "大盲注"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["翻牌前", "翻牌", "转牌", "河牌"]
    }
}
impl Rule for TexasHoldemDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("texas_holdem_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "德州扑克详细规则",
            &[("位置", &self.section_0()), ("下注轮", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TexasHoldemDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
