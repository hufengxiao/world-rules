//! 环保法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: EnvironmentalDetailed2Rules, name: "环保法详解2", desc: "环保法详解2", origin: "中国", tags: ["法律", "环境"] }
impl EnvironmentalDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["排污许可", "总量控制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生态保护红线", "自然保护区"]
    }
}
impl Rule for EnvironmentalDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("environmental_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "环保法详解2",
            &[("排污", &self.section_0()), ("生态", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnvironmentalDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
