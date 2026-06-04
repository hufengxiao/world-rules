//! 电竞详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: EsportsDetailedRules, name: "电竞详细规则", desc: "电子竞技详细比赛规则", origin: "IESF", tags: ["体育", "电子"] }
impl EsportsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["英雄选择", "BO3/BO5"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["回合制", "经济系统"]
    }
}
impl Rule for EsportsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("esports_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "电竞详细规则",
            &[("MOBA", &self.section_0()), ("FPS", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EsportsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
