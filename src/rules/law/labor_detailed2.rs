//! 劳动法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: LaborDetailed2Rules, name: "劳动法详解2", desc: "劳动法详解2", origin: "中国", tags: ["法律", "劳动"] }
impl LaborDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["标准工时", "综合工时", "不定时"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["最低工资", "加班费", "社保"]
    }
}
impl Rule for LaborDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "劳动法详解2",
            &[("工时", &self.section_0()), ("工资", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LaborDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
