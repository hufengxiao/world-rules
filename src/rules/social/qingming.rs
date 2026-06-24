//! 清明节礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: QingmingRules, name: "清明节礼仪", desc: "清明节传统礼仪", origin: "中国", tags: ["社交", "节日"] }
impl QingmingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["祭扫礼节"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["踏青习俗"]
    }
}
impl Rule for QingmingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("qingming")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "清明节礼仪",
            &[("扫墓", &self.section_0()), ("踏青", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = QingmingRules::new();
        assert!(!r.explain().is_empty());
    }
}
