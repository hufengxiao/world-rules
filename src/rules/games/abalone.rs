//! 蚌棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AbaloneRules, name: "蚌棋规则", desc: "蚌棋桌游规则", origin: "法国", tags: ["游戏", "棋类"] }
impl AbaloneRules {
    /// 获取基本规则列表
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::abalone::AbaloneRules;
    ///
    /// let rules = AbaloneRules::new();
    /// let basic = rules.section_0();
    /// assert!(!basic.is_empty());
    /// ```
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["推挤对手", "6方向移动"]
    }

    /// 获取胜负规则列表
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::abalone::AbaloneRules;
    ///
    /// let rules = AbaloneRules::new();
    /// let win_conditions = rules.section_1();
    /// assert!(!win_conditions.is_empty());
    /// ```
    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["推出6颗者胜"]
    }
}
impl Rule for AbaloneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("abalone")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "蚌棋规则",
            &[("基本", &self.section_0()), ("胜负", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AbaloneRules::new();
        assert!(!r.explain().is_empty());
    }
}
