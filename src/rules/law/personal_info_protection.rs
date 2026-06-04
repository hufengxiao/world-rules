//! 个人信息保护法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: PersonalInfoProtectionRules,
    name: "个人信息保护法",
    desc: "个人信息保护法律规则",
    origin: "中国",
    tags: ["法律", "数据"]
}

impl PersonalInfoProtectionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合法正当必要", "目的限制", "最小必要"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["知情权", "决定权", "查阅复制权", "删除权"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["安全保障义务", "影响评估", "跨境传输限制"]
    }
}

impl Rule for PersonalInfoProtectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("personal_info_protection")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "个人信息保护法",
            &[
                ("基本原则", &self.section_0()),
                ("个人权利", &self.section_1()),
                ("处理者义务", &self.section_2()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_personal_info_protection_rules() {
        let r = PersonalInfoProtectionRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
