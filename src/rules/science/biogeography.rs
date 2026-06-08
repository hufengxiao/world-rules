//! 生物地理学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BiogeographyRules, name: "生物地理学定律", desc: "生物地理学定律", origin: "国际", tags: ["科学", "生物"] }
impl BiogeographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["隔离分化", "扩散", "板块构造"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["物种面积关系", "距离衰减"]
    }
}
impl Rule for BiogeographyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("biogeography")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "生物地理学定律",
            &[("分布", &self.section_0()), ("规律", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BiogeographyRules::new();
        assert!(!r.explain().is_empty());
    }
}
