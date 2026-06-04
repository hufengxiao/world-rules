//! 电力法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ElectricityDetailedRules, name: "电力法详解", desc: "电力法详解", origin: "中国", tags: ["法律", "能源"] }
impl ElectricityDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["可再生能源", "核电安全"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["电力调度", "电价管理"]
    }
}
impl Rule for ElectricityDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("electricity_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "电力法详解",
            &[("发电", &self.section_0()), ("供应", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ElectricityDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
