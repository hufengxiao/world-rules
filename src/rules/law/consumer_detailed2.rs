//! 消费者权益详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ConsumerDetailed2Rules, name: "消费者权益详解2", desc: "消费者权益法详解2", origin: "中国", tags: ["法律", "消费"] }
impl ConsumerDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["修理", "更换", "退货"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["投诉", "调解", "仲裁", "诉讼"]
    }
}
impl Rule for ConsumerDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("consumer_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "消费者权益详解2",
            &[("三包", &self.section_0()), ("维权", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ConsumerDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
