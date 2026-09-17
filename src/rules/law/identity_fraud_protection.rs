//! 身份信息防骗
//!
//! 保护个人身份信息、识别身份盗用与应对

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: IdentityFraudProtectionRules,
    name: "身份信息防骗",
    desc: "保护个人身份信息、识别身份盗用与应对",
    origin: "中国",
    tags: ["法律", "身份", "防骗", "信息安全"]
}

impl IdentityFraudProtectionRules {
    /// 信息保护
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "不随意透露身份信息",
            "谨慎填写银行卡号",
            "重要单据妥善保管",
            "设置强密码不共用",
        ]
    }

    /// 识别骗局
    pub fn detect(&self) -> Vec<&'static str> {
        vec![
            "警惕索要验证码电话",
            "不轻信中奖转账要求",
            "辨别伪造公文链接",
            "不接不明来源转账指令",
        ]
    }

    /// 发现盗用
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "发现异常交易立即挂失",
            "查询征信报告监控",
            "向银行或机构反馈",
            "保留交易记录证据",
        ]
    }

    /// 依法处理
    pub fn legal(&self) -> Vec<&'static str> {
        vec![
            "身份盗用及时报警",
            "向金融机构投诉",
            "依据事实主张权利",
            "不卷入或不参与洗卡",
        ]
    }
}

impl Rule for IdentityFraudProtectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("identity_fraud")
    }

    fn explain(&self) -> String {
        format!(
            "【身份信息防骗】\n{}",
            [
                format!(
                    "信息保护：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "识别骗局：\\n{}",
                    self.detect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发现盗用：\\n{}",
                    self.response()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "依法处理：\\n{}",
                    self.legal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_identityfraudprotectionrules_basic() {
        let rules = IdentityFraudProtectionRules::new();
        assert_eq!(rules.metadata().name, "身份信息防骗");
        assert!(!rules.protect().is_empty());
        assert!(!rules.detect().is_empty());
        assert!(!rules.response().is_empty());
        assert!(!rules.legal().is_empty());
    }

    #[test]
    fn test_identityfraudprotectionrules_validation() {
        let rules = IdentityFraudProtectionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("identity_fraud"));
    }

    #[test]
    fn test_identityfraudprotectionrules_explain() {
        let rules = IdentityFraudProtectionRules::new();
        let e = rules.explain();
        assert!(e.contains("信息保护"));
        assert!(e.contains("识别骗局"));
        assert!(e.contains("发现盗用"));
    }
}
