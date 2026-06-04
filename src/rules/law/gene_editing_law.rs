//! 基因编辑法规

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: GeneEditingLawRules,
    name: "基因编辑法规",
    desc: "基因编辑技术法律规则",
    origin: "国际",
    tags: ["法律", "生物"]
}

impl GeneEditingLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人类胚胎编辑禁止", "知情同意", "伦理审查"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因治疗审批", "转基因监管", "基因检测规范"]
    }
}

impl Rule for GeneEditingLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("gene_editing_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "基因编辑法规",
            &[
                ("研究限制", &self.section_0()),
                ("应用监管", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gene_editing_law_rules() {
        let r = GeneEditingLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
