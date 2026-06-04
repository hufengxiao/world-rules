//! 无人机法规

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: DroneLawRules,
    name: "无人机法规",
    desc: "无人机飞行法律规则",
    origin: "中国",
    tags: ["法律", "航空"]
}

impl DroneLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["禁飞区域", "飞行高度限制", "实名登记"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["违规飞行处罚", "隐私侵权责任", "安全事故"]
    }
}

impl Rule for DroneLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("drone_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "无人机法规",
            &[("飞行规则", &self.section_0()), ("处罚", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_drone_law_rules() {
        let r = DroneLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
