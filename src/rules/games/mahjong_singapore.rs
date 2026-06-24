//! 新加坡麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongSingaporeRules, name: "新加坡麻将规则", desc: "新加坡麻将规则", origin: "新加坡", tags: ["游戏", "麻将"] }
impl MahjongSingaporeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["简化规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["花牌规则"]
    }
}
impl Rule for MahjongSingaporeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_singapore")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "新加坡麻将规则",
            &[("基本", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongSingaporeRules::new();
        assert!(!r.explain().is_empty());
    }
}
