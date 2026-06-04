//! 信息论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: InformationTheoryRules, name: "信息论定律", desc: "香农信息论定律", origin: "国际", tags: ["科学", "数学"] }
impl InformationTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["信息熵", "信道容量", "数据压缩"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈夫曼编码", "纠错码"]
    }
}
impl Rule for InformationTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("information_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "信息论定律",
            &[("基础", &self.section_0()), ("编码", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InformationTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
