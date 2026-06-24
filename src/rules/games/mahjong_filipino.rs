//! 菲律宾麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongFilipinoRules, name: "菲律宾麻将规则", desc: "菲律宾麻将规则", origin: "菲律宾", tags: ["游戏", "麻将"] }
impl MahjongFilipinoRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["简化计分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特殊胡牌"]
    }
}
impl Rule for MahjongFilipinoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_filipino")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "菲律宾麻将规则",
            &[("基本", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongFilipinoRules::new();
        assert!(!r.explain().is_empty());
    }
}
