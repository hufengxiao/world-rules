//! Risk世界征服规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: RiskRules, name: "Risk世界征服规则", desc: "Risk桌游规则", origin: "法国", tags: ["游戏", "桌游"] }
impl RiskRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["占领全部领土获胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["获得增援", "进攻相邻", "调防"]
    }
}
impl Rule for RiskRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("risk")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "Risk世界征服规则",
            &[("游戏目标", &self.section_0()), ("回合", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RiskRules::new();
        assert!(!r.explain().is_empty());
    }
}
