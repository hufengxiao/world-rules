//! 婚姻法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MarriageDetailed2Rules, name: "婚姻法详解2", desc: "婚姻法详解2", origin: "中国", tags: ["法律", "家庭"] }
impl MarriageDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["共同财产", "个人财产", "财产约定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["抚养权", "抚养费", "探望权"]
    }
}
impl Rule for MarriageDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("marriage_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "婚姻法详解2",
            &[("财产", &self.section_0()), ("子女", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MarriageDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
