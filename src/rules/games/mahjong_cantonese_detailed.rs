//! 广东麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongCantoneseDetailedRules, name: "广东麻将详细规则", desc: "广东麻将详细规则", origin: "中国", tags: ["游戏", "麻将"] }
impl MahjongCantoneseDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["鸡胡可胡"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["推倒胡规则"]
    }
}
impl Rule for MahjongCantoneseDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_cantonese_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "广东麻将详细规则",
            &[("鸡胡", &self.section_0()), ("推倒胡", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongCantoneseDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
