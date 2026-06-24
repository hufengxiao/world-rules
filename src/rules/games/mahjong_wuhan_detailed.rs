//! 武汉麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongWuhanDetailedRules, name: "武汉麻将详细规则", desc: "武汉麻将详细", origin: "中国", tags: ["游戏", "麻将"] }
impl MahjongWuhanDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赖子牌用法"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["口口番计分"]
    }
}
impl Rule for MahjongWuhanDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_wuhan_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "武汉麻将详细规则",
            &[("赖子", &self.section_0()), ("口口番", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongWuhanDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
