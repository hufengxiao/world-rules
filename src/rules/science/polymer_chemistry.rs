//! 高分子化学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: PolymerChemistryRules,
    name: "高分子化学定律",
    desc: "高分子化学定律",
    origin: "国际",
    tags: ["科学", "化学"]
}

impl PolymerChemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["加聚反应", "缩聚反应", "聚合动力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["玻璃化转变", "粘弹性", "降解与老化"]
    }
}

impl Rule for PolymerChemistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("polymer_chemistry")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "高分子化学定律",
            &[("聚合", &self.section_0()), ("性质", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_polymer_chemistry_rules() {
        let r = PolymerChemistryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
