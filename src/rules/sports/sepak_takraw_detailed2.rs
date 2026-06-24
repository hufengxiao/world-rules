//! 藤球详细规则2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SepakTakrawDetailed2Rules, name: "藤球详细规则2", desc: "藤球运动详细规则", origin: "东南亚", tags: ["体育", "球类"] }
impl SepakTakrawDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["脚踢头顶"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["3局2胜"]
    }
}
impl Rule for SepakTakrawDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sepak_takraw_detailed2")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "藤球详细规则2",
            &[("技术", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SepakTakrawDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
