//! 合成生物学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: SyntheticBiologyRules,
    name: "合成生物学定律",
    desc: "合成生物学定律",
    origin: "国际",
    tags: ["科学", "生物"]
}

impl SyntheticBiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["模块化设计", "标准化生物部件", "正交性原则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生物燃料", "生物传感器", "基因治疗"]
    }
}

impl Rule for SyntheticBiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("synthetic_biology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "合成生物学定律",
            &[("设计原则", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_synthetic_biology_rules() {
        let r = SyntheticBiologyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
