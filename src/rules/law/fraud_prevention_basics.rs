//! 诈骗防范要点
//!
//! 识别常见骗局、保护财产与受骗后的补救

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FraudPreventionBasicsRules,
    name: "诈骗防范要点",
    desc: "识别常见骗局、保护财产与受骗后的补救",
    origin: "中国",
    tags: ["法律", "防骗", "诈骗", "安全"]
}

impl FraudPreventionBasicsRules {
    /// 识别骗术
    pub fn detect(&self) -> Vec<&'static str> {
        vec![
            "警惕索要验证码密码",
            "不轻信高收益投资",
            "辨别冒充客服公检法",
            "冷静不跟风转账",
        ]
    }

    /// 保护财产
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "不向陌生账户转账",
            "设置防护不乱点",
            "电商退款走平台",
            "重要信息不泄露",
        ]
    }

    /// 受骗应对
    pub fn respond(&self) -> Vec<&'static str> {
        vec![
            "发现受骗立即停止转账",
            "速报平台或银行止付",
            "保留聊天与转账凭证",
            "必要时报警处理",
        ]
    }

    /// 家庭防护
    pub fn family(&self) -> Vec<&'static str> {
        vec![
            "提醒家中老人防骗",
            "不独断大额转账",
            "重要变动多核实",
            "增强信息安全意识",
        ]
    }
}

impl Rule for FraudPreventionBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("fraud_prevention")
    }

    fn explain(&self) -> String {
        format!(
            "【诈骗防范要点】\n{}",
            [
                format!(
                    "识别骗术：\\n{}",
                    self.detect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保护财产：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "受骗应对：\\n{}",
                    self.respond()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家庭防护：\\n{}",
                    self.family()
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
    fn test_fraudpreventionbasicsrules_basic() {
        let rules = FraudPreventionBasicsRules::new();
        assert_eq!(rules.metadata().name, "诈骗防范要点");
        assert!(!rules.detect().is_empty());
        assert!(!rules.protect().is_empty());
        assert!(!rules.respond().is_empty());
        assert!(!rules.family().is_empty());
    }

    #[test]
    fn test_fraudpreventionbasicsrules_validation() {
        let rules = FraudPreventionBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("fraud_prevention"));
    }

    #[test]
    fn test_fraudpreventionbasicsrules_explain() {
        let rules = FraudPreventionBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("识别骗术"));
        assert!(e.contains("保护财产"));
        assert!(e.contains("受骗应对"));
    }
}
