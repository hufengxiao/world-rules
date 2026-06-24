//! 中国象棋详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: XiangqiDetailedRules, name: "中国象棋详细规则", desc: "中国象棋详细规则", origin: "中国", tags: ["游戏", "棋类"] }
impl XiangqiDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["车马炮相士帅卒"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["将军应将", "长将长捉"]
    }
}
impl Rule for XiangqiDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("xiangqi_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国象棋详细规则",
            &[("棋子", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = XiangqiDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
