//! 婚姻法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MarriageDetailedRules, name: "婚姻法详解", desc: "婚姻法详解", origin: "中国", tags: ["法律", "家庭"] }
impl MarriageDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["结婚条件", "无效婚姻"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["协议离婚", "子女抚养"]
    }
}
impl Rule for MarriageDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("marriage_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "婚姻法详解",
            &[("结婚", &self.section_0()), ("离婚", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MarriageDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
