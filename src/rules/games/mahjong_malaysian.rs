//! 马来西亚麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongMalaysianRules, name: "马来西亚麻将规则", desc: "马来西亚麻将规则", origin: "马来西亚", tags: ["游戏", "麻将"] }
impl MahjongMalaysianRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["动物牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特殊番种"]
    }
}
impl Rule for MahjongMalaysianRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_malaysian")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "马来西亚麻将规则",
            &[("基本", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongMalaysianRules::new();
        assert!(!r.explain().is_empty());
    }
}
