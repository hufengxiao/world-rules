//! 网络安全法详解

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CybersecurityDetailedRules,
    name: "网络安全法详解",
    desc: "网络安全法律规则详解",
    origin: "中国",
    tags: ["法律", "网络"]
}

impl CybersecurityDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["等级保护制度", "关键基础设施", "安全认证"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["实名制", "禁止传播内容", "日志留存"]
    }
}

impl Rule for CybersecurityDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("cybersecurity_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "网络安全法详解",
            &[
                ("网络运行安全", &self.section_0()),
                ("网络信息安全", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cybersecurity_detailed_rules() {
        let r = CybersecurityDetailedRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
