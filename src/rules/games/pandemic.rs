//! 瘟疫危机规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PandemicRules, name: "瘟疫危机规则", desc: "瘟疫危机桌游规则", origin: "美国", tags: ["游戏", "桌游"] }
impl PandemicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4种病毒", "治疗目标"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["移动", "治疗", "建设"]
    }
}
impl Rule for PandemicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("pandemic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "瘟疫危机规则",
            &[("合作", &self.section_0()), ("行动", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PandemicRules::new();
        assert!(!r.explain().is_empty());
    }
}
