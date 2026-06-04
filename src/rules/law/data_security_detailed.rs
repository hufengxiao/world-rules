//! 数据安全法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: DataSecurityDetailedRules,
    name: "数据安全法",
    desc: "数据安全法律规则",
    origin: "中国",
    tags: ["法律", "数据"]
}

impl DataSecurityDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一般数据", "重要数据", "核心数据"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["风险评估", "安全审查", "出境安全评估"]
    }
}

impl Rule for DataSecurityDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("data_security_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "数据安全法",
            &[
                ("分类分级", &self.section_0()),
                ("安全义务", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_data_security_law_rules() {
        let r = DataSecurityDetailedRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
