//! 凝聚态物理定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CondensedMatterRules,
    name: "凝聚态物理定律",
    desc: "凝聚态物理定律",
    origin: "国际",
    tags: ["科学", "物理"]
}

impl CondensedMatterRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["布拉格衍射定律", "晶格振动声子", "能带理论"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["BCS理论", "迈斯纳效应", "约瑟夫森效应"]
    }
}

impl Rule for CondensedMatterRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("condensed_matter")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "凝聚态物理定律",
            &[("晶体", &self.section_0()), ("超导", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_condensed_matter_rules() {
        let r = CondensedMatterRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
