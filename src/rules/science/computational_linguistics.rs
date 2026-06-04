//! 计算语言学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ComputationalLinguisticsRules, name: "计算语言学定律", desc: "计算语言学定律", origin: "国际", tags: ["科学", "语言"] }
impl ComputationalLinguisticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["句法分析", "语义解析"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["机器翻译", "文本挖掘"]
    }
}
impl Rule for ComputationalLinguisticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("computational_linguistics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "计算语言学定律",
            &[("方法", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ComputationalLinguisticsRules::new();
        assert!(!r.explain().is_empty());
    }
}
