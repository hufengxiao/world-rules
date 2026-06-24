//! 合气道韩国规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HapkidoKhfRules, name: "合气道韩国规则", desc: "韩国合气道联合会规则", origin: "韩国", tags: ["体育", "格斗"] }
impl HapkidoKhfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["圆形运动关节技"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["短棍绳鞭"]
    }
}
impl Rule for HapkidoKhfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hapkido_khf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "合气道韩国规则",
            &[("技术", &self.section_0()), ("武器", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HapkidoKhfRules::new();
        assert!(!r.explain().is_empty());
    }
}
