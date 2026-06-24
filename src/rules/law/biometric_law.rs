//! 生物识别法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BiometricLawRules, name: "生物识别法", desc: "生物识别法律规则", origin: "国际", tags: ["法律", "数据"] }
impl BiometricLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人脸指纹"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["使用限制"]
    }
}
impl Rule for BiometricLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("biometric_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生物识别法",
            &[("类型", &self.section_0()), ("规范", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BiometricLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
