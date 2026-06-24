//! 隐私权法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PrivacyRightsRules, name: "隐私权法", desc: "隐私权保障法律", origin: "国际", tags: ["法律", "隐私"] }
impl PrivacyRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "私人生活安宁:不受他人侵扰",
            "私人信息保密:个人数据受保护",
            "私人空间:住宅不受非法侵入",
            "通信自由:通信内容不受非法查看",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "GDPR:欧盟通用数据保护条例",
            "CCPA:加州消费者隐私法",
            "中国个人信息保护法:2021年实施",
            "核心原则:合法/正当/必要/诚信",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "公共利益:国家安全/公共卫生",
            "知情同意:数据处理需获得同意",
            "匿名化:去除个人标识信息",
            "数据泄露通知:发生泄露需及时通知",
        ]
    }
}
impl Rule for PrivacyRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("privacy_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "隐私权法",
            &[
                ("隐私权内容", &self.section_0()),
                ("数据保护", &self.section_1()),
                ("限制与例外", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PrivacyRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
