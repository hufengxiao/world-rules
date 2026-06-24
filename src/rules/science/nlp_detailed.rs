//! 自然语言处理详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NlpDetailedRules, name: "自然语言处理详细", desc: "NLP定律", origin: "国际", tags: ["科学", "计算机"] }
impl NlpDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["词嵌入注意力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["翻译生成"]
    }
}
impl Rule for NlpDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("nlp_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "自然语言处理详细",
            &[("基础", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NlpDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
