//! 武术IWUF规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WushuIwufRules, name: "武术IWUF规则", desc: "国际武术联合会规则", origin: "中国", tags: ["体育", "格斗"] }
impl WushuIwufRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["长拳南拳太极"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["散打规则"]
    }
}
impl Rule for WushuIwufRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wushu_iwuf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "武术IWUF规则",
            &[("套路", &self.section_0()), ("散打", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WushuIwufRules::new();
        assert!(!r.explain().is_empty());
    }
}
