//! 象棋960规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: Xiangqi960Rules, name: "象棋960规则", desc: "象棋随机开局规则", origin: "中国", tags: ["游戏", "棋类"] }
impl Xiangqi960Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["随机排列首排", "保留基本规则"]
    }
}
impl Rule for Xiangqi960Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("xiangqi960")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections("象棋960规则", &[("随机", &self.section_0())])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = Xiangqi960Rules::new();
        assert!(!r.explain().is_empty());
    }
}
