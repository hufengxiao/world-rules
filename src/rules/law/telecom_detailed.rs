//! 电信法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: TelecomDetailedRules, name: "电信法详解", desc: "电信法详解", origin: "中国", tags: ["法律", "通信"] }
impl TelecomDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["基础电信", "增值电信"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["互联互通", "资费管理"]
    }
}
impl Rule for TelecomDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("telecom_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "电信法详解",
            &[("许可", &self.section_0()), ("监管", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TelecomDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
