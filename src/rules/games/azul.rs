//! 花砖物语规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AzulRules, name: "花砖物语规则", desc: "花砖物语桌游规则", origin: "葡萄牙", tags: ["游戏", "桌游"] }
impl AzulRules {
    /// 获取基本规则列表
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::azul::AzulRules;
    ///
    /// let rules = AzulRules::new();
    /// let basic = rules.section_0();
    /// assert!(!basic.is_empty());
    /// ```
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["选砖", "铺墙"]
    }

    /// 获取计分规则列表
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::azul::AzulRules;
    ///
    /// let rules = AzulRules::new();
    /// let scoring = rules.section_1();
    /// assert!(!scoring.is_empty());
    /// ```
    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["相邻加分", "扣分"]
    }
}
impl Rule for AzulRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("azul")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "花砖物语规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AzulRules::new();
        assert!(!r.explain().is_empty());
    }
}
