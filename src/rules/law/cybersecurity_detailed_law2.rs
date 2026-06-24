//! 网络安全法详解2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CybersecurityDetailedLaw2Rules, name: "网络安全法详解2", desc: "网络安全法详解2", origin: "中国", tags: ["法律", "网络"] }
impl CybersecurityDetailedLaw2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["等级保护"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["关键基础设施"]
    }
}
impl Rule for CybersecurityDetailedLaw2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("cybersecurity_detailed_law2")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "网络安全法详解2",
            &[("等保", &self.section_0()), ("关键", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CybersecurityDetailedLaw2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
