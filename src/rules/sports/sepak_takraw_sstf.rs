//! 藤球SSTF规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SepakTakrawSstfRules, name: "藤球SSTF规则", desc: "SSTF藤球规则", origin: "韩国", tags: ["体育", "球类"] }
impl SepakTakrawSstfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3局2胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["脚踢头顶"]
    }
}
impl Rule for SepakTakrawSstfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sepak_takraw_sstf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "藤球SSTF规则",
            &[("比赛", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SepakTakrawSstfRules::new();
        assert!(!r.explain().is_empty());
    }
}
