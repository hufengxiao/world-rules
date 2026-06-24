//! 长沙麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongChangshaDetailedRules, name: "长沙麻将详细规则", desc: "长沙麻将详细", origin: "中国", tags: ["游戏", "麻将"] }
impl MahjongChangshaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["口口番规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["赖子牌规则"]
    }
}
impl Rule for MahjongChangshaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_changsha_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "长沙麻将详细规则",
            &[("口口番", &self.section_0()), ("赖子", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongChangshaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
