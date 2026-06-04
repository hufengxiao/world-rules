//! 生物信息学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: BioinformaticsRules,
    name: "生物信息学定律",
    desc: "生物信息学定律",
    origin: "国际",
    tags: ["科学", "生物"]
}

impl BioinformaticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["序列比对算法", "BLAST搜索", "多序列比对"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因预测", "基因组组装", "比较基因组学"]
    }
}

impl Rule for BioinformaticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("bioinformatics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "生物信息学定律",
            &[
                ("序列分析", &self.section_0()),
                ("基因组", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bioinformatics_rules() {
        let r = BioinformaticsRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
