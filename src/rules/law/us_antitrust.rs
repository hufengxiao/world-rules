//! 美国反垄断法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: UsAntitrustRules, name: "美国反垄断法", desc: "美国反垄断法律规则", origin: "美国", tags: ["法律", "竞争"] }
impl UsAntitrustRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["垄断协议"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["合并审查"]
    }
}
impl Rule for UsAntitrustRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("us_antitrust")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "美国反垄断法",
            &[("谢尔曼", &self.section_0()), ("克莱顿", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = UsAntitrustRules::new();
        assert!(!r.explain().is_empty());
    }
}
