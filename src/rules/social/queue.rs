//! 排队礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: QueueRules,
    name: "排队礼仪",
    desc: "公共排队礼仪",
    origin: "国际",
    tags: ["社交", "公共"]
}

impl QueueRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["先到先排", "不插队", "保持间距"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["老人孕妇优先", "紧急情况说明", "代排需征得同意"]
    }
}

impl Rule for QueueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("queue")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "排队礼仪",
            &[
                ("基本规则", &self.section_0()),
                ("特殊情况", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue_rules() {
        let r = QueueRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
