//! 直播礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: LiveStreamingRules,
    name: "直播礼仪",
    desc: "直播社交礼仪",
    origin: "中国",
    tags: ["社交", "直播"]
}

impl LiveStreamingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["文明直播", "不诱导打赏", "保护未成年人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["文明弹幕", "不人身攻击", "理性消费"]
    }
}

impl Rule for LiveStreamingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("live_streaming")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "直播礼仪",
            &[("主播", &self.section_0()), ("观众", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_live_streaming_rules() {
        let r = LiveStreamingRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
