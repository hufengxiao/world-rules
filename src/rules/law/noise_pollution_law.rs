//! 噪声污染防治法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: NoisePollutionLawRules,
    name: "噪声污染防治法",
    desc: "噪声污染防治法律规则",
    origin: "中国",
    tags: ["法律", "环境"]
}

impl NoisePollutionLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["工业噪声", "建筑施工噪声", "交通噪声", "社会生活噪声"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["超标排放", "夜间施工违规", "扰民行为"]
    }
}

impl Rule for NoisePollutionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("noise_pollution_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "噪声污染防治法",
            &[("分类", &self.section_0()), ("处罚", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_noise_pollution_law_rules() {
        let r = NoisePollutionLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
