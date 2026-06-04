//! 蛋白质组学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ProteomicsRules, name: "蛋白质组学定律", desc: "蛋白质组学定律", origin: "国际", tags: ["科学", "生物"] }
impl ProteomicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["质谱分析", "二维电泳"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["蛋白质互作网络", "翻译后修饰"]
    }
}
impl Rule for ProteomicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("proteomics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "蛋白质组学定律",
            &[("技术", &self.section_0()), ("分析", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ProteomicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
