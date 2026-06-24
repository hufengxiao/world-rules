//! 散打CWUF规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SandaCwufRules, name: "散打CWUF规则", desc: "中国散打竞赛规则", origin: "中国", tags: ["体育", "格斗"] }
impl SandaCwufRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["拳腿摔"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["体重分级"]
    }
}
impl Rule for SandaCwufRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sanda_cwuf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "散打CWUF规则",
            &[("得分", &self.section_0()), ("级别", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SandaCwufRules::new();
        assert!(!r.explain().is_empty());
    }
}
