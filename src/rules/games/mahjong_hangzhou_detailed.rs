//! 杭州麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongHangzhouDetailedRules, name: "杭州麻将详细规则", desc: "杭州麻将详细", origin: "中国", tags: ["游戏", "麻将"] }
impl MahjongHangzhouDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["财神牌规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["爆胡规则"]
    }
}
impl Rule for MahjongHangzhouDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hangzhou_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "杭州麻将详细规则",
            &[("财神", &self.section_0()), ("爆胡", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongHangzhouDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
