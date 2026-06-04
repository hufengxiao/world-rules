//! 吸烟礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: SmokingRules,
    name: "吸烟礼仪",
    desc: "吸烟社交礼仪",
    origin: "国际",
    tags: ["社交", "公共"]
}

impl SmokingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["遵守禁烟规定", "找吸烟区", "室外注意风向"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先询问再敬烟", "不强迫他人", "尊重非吸烟者"]
    }
}

impl Rule for SmokingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("smoking")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "吸烟礼仪",
            &[("场所", &self.section_0()), ("社交", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smoking_rules() {
        let r = SmokingRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
