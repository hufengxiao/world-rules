//! 自动驾驶法规

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AutonomousDrivingLawRules,
    name: "自动驾驶法规",
    desc: "自动驾驶法律规则",
    origin: "国际",
    tags: ["法律", "交通"]
}

impl AutonomousDrivingLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["L0到L5自动驾驶分级标准"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["事故责任认定", "保险要求", "数据记录保存"]
    }
}

impl Rule for AutonomousDrivingLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("autonomous_driving_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "自动驾驶法规",
            &[("分级", &self.section_0()), ("责任", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_autonomous_driving_law_rules() {
        let r = AutonomousDrivingLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
