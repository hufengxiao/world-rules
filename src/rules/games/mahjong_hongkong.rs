//! 香港麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongHongkongRules, name: "香港麻将规则", desc: "香港麻将规则", origin: "香港", tags: ["游戏", "麻将"] }
impl MahjongHongkongRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["鸡胡"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["翻数计分"]
    }
}
impl Rule for MahjongHongkongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hongkong")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "香港麻将规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongHongkongRules::new();
        assert!(!r.explain().is_empty());
    }
}
