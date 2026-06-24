//! 元宵节礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: YuanxiaoRules, name: "元宵节礼仪", desc: "元宵节传统礼仪", origin: "中国", tags: ["社交", "节日"] }
impl YuanxiaoRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赏灯习俗"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["吃汤圆"]
    }
}
impl Rule for YuanxiaoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("yuanxiao")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "元宵节礼仪",
            &[("灯会", &self.section_0()), ("汤圆", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = YuanxiaoRules::new();
        assert!(!r.explain().is_empty());
    }
}
