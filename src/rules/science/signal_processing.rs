//! 信号处理定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: SignalProcessingRules, name: "信号处理定律", desc: "信号处理定律", origin: "国际", tags: ["科学", "工程"] }
impl SignalProcessingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["傅里叶变换", "小波变换"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["FIR滤波器", "卡尔曼滤波"]
    }
}
impl Rule for SignalProcessingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("signal_processing")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "信号处理定律",
            &[("变换", &self.section_0()), ("滤波", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SignalProcessingRules::new();
        assert!(!r.explain().is_empty());
    }
}
