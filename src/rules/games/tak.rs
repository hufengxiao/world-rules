//! 塔克棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TakRules, name: "塔克棋规则", desc: "塔克棋桌游规则", origin: "美国", tags: ["游戏", "棋类"] }
impl TakRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["铺路", "叠棋"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["连接两边"]
    }
}
impl Rule for TakRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("tak")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "塔克棋规则",
            &[("基本", &self.section_0()), ("目标", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TakRules::new();
        assert!(!r.explain().is_empty());
    }
}
