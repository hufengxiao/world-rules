//! 基因组学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: GenomicLawsRules, name: "基因组学定律", desc: "基因组学定律", origin: "国际", tags: ["科学", "生物"] }
impl GenomicLawsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["二代测序", "三代测序"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["变异检测", "全基因组关联"]
    }
}
impl Rule for GenomicLawsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("genomic_laws")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "基因组学定律",
            &[("测序", &self.section_0()), ("分析", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GenomicLawsRules::new();
        assert!(!r.explain().is_empty());
    }
}
