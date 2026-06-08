//! 民法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CivilDetailed2Rules, name: "民法详解2", desc: "民法详解2", origin: "中国", tags: ["法律", "民法"] }
impl CivilDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["所有权", "用益物权", "担保物权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["合同之债", "侵权之债"]
    }
}
impl Rule for CivilDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "民法详解2",
            &[("物权", &self.section_0()), ("债权", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
