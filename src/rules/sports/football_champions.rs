//! 欧冠规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FootballChampionsRules, name: "欧冠规则", desc: "欧洲冠军联赛规则", origin: "欧洲", tags: ["体育", "球类"] }
impl FootballChampionsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["小组赛", "淘汰赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["种子排名"]
    }
}
impl Rule for FootballChampionsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_champions")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "欧冠规则",
            &[("赛制", &self.section_0()), ("种子", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballChampionsRules::new();
        assert!(!r.explain().is_empty());
    }
}
