//! 精神卫生法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MentalHealthLawRules,
    name: "精神卫生法",
    desc: "精神卫生法律规则",
    origin: "中国",
    tags: ["法律", "医疗"]
}

impl MentalHealthLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人格尊严保护", "隐私保护", "通信自由"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["自愿原则", "非自愿住院条件", "治疗规范"]
    }
}

impl Rule for MentalHealthLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("mental_health_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "精神卫生法",
            &[
                ("患者权益", &self.section_0()),
                ("诊疗规范", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mental_health_law_rules() {
        let r = MentalHealthLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
