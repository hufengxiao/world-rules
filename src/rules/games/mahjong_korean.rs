//! 韩国麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongKoreanRules, name: "韩国麻将规则", desc: "韩国麻将规则", origin: "韩国", tags: ["游戏", "麻将"] }
impl MahjongKoreanRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["简化规则", "无风牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["简化计分"]
    }
}
impl Rule for MahjongKoreanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_korean")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "韩国麻将规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongKoreanRules::new();
        assert!(!r.explain().is_empty());
    }
}
