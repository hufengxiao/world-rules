//! 立直麻将详细规则2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongRiichiDetailed2Rules, name: "立直麻将详细规则2", desc: "日本立直麻将详细", origin: "日本", tags: ["游戏", "麻将"] }
impl MahjongRiichiDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["筋牌", "壁牌", "现物"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["振听规则"]
    }
}
impl Rule for MahjongRiichiDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_riichi_detailed2")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "立直麻将详细规则2",
            &[("防守", &self.section_0()), ("振听", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongRiichiDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
