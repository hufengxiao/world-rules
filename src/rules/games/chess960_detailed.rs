//! 费舍尔随机棋详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: Chess960DetailedRules, name: "费舍尔随机棋详细", desc: "Chess960详细规则", origin: "国际", tags: ["游戏", "棋类"] }
impl Chess960DetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["960种随机排列"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["易位规则调整"]
    }
}
impl Rule for Chess960DetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess960_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "费舍尔随机棋详细",
            &[("开局", &self.section_0()), ("易位", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = Chess960DetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
