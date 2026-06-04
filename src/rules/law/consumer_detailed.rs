//! 消费者权益详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ConsumerDetailedRules, name: "消费者权益详解", desc: "消费者权益法详解", origin: "中国", tags: ["法律", "消费"] }
impl ConsumerDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["知情权", "选择权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["七天无理由退货", "惩罚性赔偿"]
    }
}
impl Rule for ConsumerDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("consumer_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "消费者权益详解",
            &[("权利", &self.section_0()), ("救济", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ConsumerDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
