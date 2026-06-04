//! 乒乓球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: TableTennisDetailedRules, name: "乒乓球详细规则", desc: "乒乓球详细比赛规则", origin: "ITTF", tags: ["体育", "球类"] }
impl TableTennisDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["11分制", "轮换发球"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["合法发球", "擦网重发"]
    }
}
impl Rule for TableTennisDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("table_tennis_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "乒乓球详细规则",
            &[("计分", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TableTennisDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
