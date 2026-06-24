//! 血战麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongBloodBattleRules, name: "血战麻将规则", desc: "血战到底麻将", origin: "中国", tags: ["游戏", "麻将"] }
impl MahjongBloodBattleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一家胡后继续"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["查叫惩罚"]
    }
}
impl Rule for MahjongBloodBattleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_blood_battle")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "血战麻将规则",
            &[("血战", &self.section_0()), ("查叫", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongBloodBattleRules::new();
        assert!(!r.explain().is_empty());
    }
}
