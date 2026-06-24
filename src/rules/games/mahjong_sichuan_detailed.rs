//! 四川麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongSichuanDetailedRules, name: "四川麻将详细规则", desc: "四川麻将血战到底", origin: "中国", tags: ["游戏", "麻将"] }
impl MahjongSichuanDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["必须缺一门花色"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["胡牌后继续打"]
    }
}
impl Rule for MahjongSichuanDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_sichuan_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "四川麻将详细规则",
            &[("缺门", &self.section_0()), ("血战", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongSichuanDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
