//! 行政法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: AdministrativeDetailed2Rules, name: "行政法详解2", desc: "行政法详解2", origin: "中国", tags: ["法律", "行政"] }
impl AdministrativeDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["设定", "程序", "监督"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["查封", "扣押", "冻结"]
    }
}
impl Rule for AdministrativeDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("administrative_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "行政法详解2",
            &[("许可", &self.section_0()), ("强制", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AdministrativeDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
