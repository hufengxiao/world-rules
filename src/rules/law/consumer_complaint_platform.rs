//! 消费投诉平台
//!
//! 网购等消费纠纷向平台与监管部门投诉

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ConsumerComplaintPlatformRules,
    name: "消费投诉平台",
    desc: "网购等消费纠纷向平台与监管部门投诉",
    origin: "中国",
    tags: ["消费", "投诉", "维权", "平台"]
}

impl ConsumerComplaintPlatformRules {
    /// 先协商
    pub fn negotiate(&self) -> Vec<&'static str> {
        vec!["先与商家协商退货", "留聊天凭证", "态度理性", "说明诉求"]
    }

    /// 平台介入
    pub fn platform(&self) -> Vec<&'static str> {
        vec!["上平台投诉", "举证据申诉", "找官方渠道", "请求平台处理"]
    }

    /// 官方投诉
    pub fn authority(&self) -> Vec<&'static str> {
        vec!["向监管部门反映", "打投诉热线", "依法提交材料", "跟进结果"]
    }

    /// 信息披露
    pub fn remedy(&self) -> Vec<&'static str> {
        vec!["不夸大不造谣", "依实投诉", "保留全部证据", "合法维权"]
    }
}

impl Rule for ConsumerComplaintPlatformRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("consumer_complaint")
    }

    fn explain(&self) -> String {
        format!(
            "【消费投诉平台】\n{}",
            [
                format!(
                    "先协商：\\n{}",
                    self.negotiate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平台介入：\\n{}",
                    self.platform()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "官方投诉：\\n{}",
                    self.authority()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "信息披露：\\n{}",
                    self.remedy()
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
    fn test_consumercomplaintplatformrules_basic() {
        let rules = ConsumerComplaintPlatformRules::new();
        assert_eq!(rules.metadata().name, "消费投诉平台");
        assert!(!rules.negotiate().is_empty());
        assert!(!rules.platform().is_empty());
        assert!(!rules.authority().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_consumercomplaintplatformrules_validation() {
        let rules = ConsumerComplaintPlatformRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("consumer_complaint"));
    }

    #[test]
    fn test_consumercomplaintplatformrules_explain() {
        let rules = ConsumerComplaintPlatformRules::new();
        let e = rules.explain();
        assert!(e.contains("先协商"));
        assert!(e.contains("平台介入"));
        assert!(e.contains("官方投诉"));
    }
}
