//! 网络评价礼仪
//!
//! 点评商家与服务时的客观表达与用语规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OnlineReviewMannerRules,
    name: "网络评价礼仪",
    desc: "点评商家与服务时的客观表达与用语规范",
    origin: "网络通用",
    tags: ["线上", "评价", "礼仪"]
}

impl OnlineReviewMannerRules {
    /// 客观真实
    pub fn honesty(&self) -> Vec<&'static str> {
        vec![
            "基于真实消费体验",
            "如实描述不夸大",
            "不恶意抹黑商家",
            "不刷好评或差评",
        ]
    }

    /// 表达方式
    pub fn expression(&self) -> Vec<&'static str> {
        vec![
            "就事论事不对人",
            "描述事实少用辱骂",
            "指出问题给出依据",
            "肯定优点亦需提及",
        ]
    }

    /// 隐私尊重
    pub fn privacy(&self) -> Vec<&'static str> {
        vec![
            "不泄露他人隐私",
            "不上传他人肖像",
            "不公开个人信息",
            "避免涉及他人名誉",
        ]
    }

    /// 纠纷处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec![
            "先与商家沟通解决",
            "差评前给予改正机会",
            "遇欺诈保留证据",
            "依法维权不滥用评价",
        ]
    }
}

impl Rule for OnlineReviewMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("online_review_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【网络评价礼仪】\n{}",
            [
                format!(
                    "客观真实：\\n{}",
                    self.honesty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达方式：\\n{}",
                    self.expression()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "隐私尊重：\\n{}",
                    self.privacy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "纠纷处理：\\n{}",
                    self.dispute()
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
    fn test_onlinereviewmannerrules_basic() {
        let rules = OnlineReviewMannerRules::new();
        assert_eq!(rules.metadata().name, "网络评价礼仪");
        assert!(!rules.honesty().is_empty());
        assert!(!rules.expression().is_empty());
        assert!(!rules.privacy().is_empty());
        assert!(!rules.dispute().is_empty());
    }

    #[test]
    fn test_onlinereviewmannerrules_validation() {
        let rules = OnlineReviewMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("online_review_manner")
        );
    }

    #[test]
    fn test_onlinereviewmannerrules_explain() {
        let rules = OnlineReviewMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("客观真实"));
        assert!(e.contains("表达方式"));
        assert!(e.contains("隐私尊重"));
    }
}
