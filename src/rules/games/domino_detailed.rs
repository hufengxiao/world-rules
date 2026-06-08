//! 多米诺详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: DominoDetailedRules, name: "多米诺详细规则", desc: "多米诺骨牌详细规则", origin: "国际", tags: ["游戏", "骨牌"] }
impl DominoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["同点数相接", "双牌横向"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["剩余点数", "先出完者胜"]
    }
}
impl Rule for DominoDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("domino_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "多米诺详细规则",
            &[("接龙", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DominoDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
