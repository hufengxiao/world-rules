//! 慢性病管理规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ChronicDiseaseRules,
    name: "慢性病管理规则",
    desc: "慢性病预防与管理规则",
    origin: "国际",
    tags: ["健康", "医疗"]
}

impl ChronicDiseaseRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["健康饮食", "规律运动", "戒烟限酒"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["定期体检", "遵医嘱用药", "自我监测"]
    }
}

impl Rule for ChronicDiseaseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("chronic_disease")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "慢性病管理规则",
            &[("预防", &self.section_0()), ("管理", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chronic_disease_rules() {
        let r = ChronicDiseaseRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
