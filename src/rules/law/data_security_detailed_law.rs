//! 数据安全法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DataSecurityDetailedLawRules, name: "数据安全法详解", desc: "数据安全法详解", origin: "中国", tags: ["法律", "数据"] }
impl DataSecurityDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一般重要核心"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["安全评估"]
    }
}
impl Rule for DataSecurityDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("data_security_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数据安全法详解",
            &[("分级", &self.section_0()), ("出境", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DataSecurityDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
