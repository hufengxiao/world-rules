//! 私信礼节
//!
//! 一对一私信沟通的用语与时间分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PrivateMessageCourtesyRules,
    name: "私信礼节",
    desc: "一对一私信沟通的用语与时间分寸",
    origin: "网络通用",
    tags: ["线上", "私信", "礼仪"]
}

impl PrivateMessageCourtesyRules {
    /// 时间分寸
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "避免深夜打扰他人",
            "工作私信按工作时间",
            "非急事不连续催促",
            "尊重对方回复节奏",
        ]
    }

    /// 用语规范
    pub fn wording(&self) -> Vec<&'static str> {
        vec![
            "开头礼貌称呼",
            "说明来意简洁明了",
            "不群发无差别广告",
            "请求帮忙用商量语气",
        ]
    }

    /// 内容边界
    pub fn boundaries(&self) -> Vec<&'static str> {
        vec![
            "不转发谣言与垃圾信息",
            "不追问他人隐私",
            "不发令人不适内容",
            "未经同意不拉群",
        ]
    }

    /// 回应礼节
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "收到重要信息及时回复",
            "不能立即处理先告知",
            "结束对话有收尾",
            "误发信息及时更正",
        ]
    }
}

impl Rule for PrivateMessageCourtesyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("private_message_courtesy")
    }

    fn explain(&self) -> String {
        format!(
            "【私信礼节】\n{}",
            [
                format!(
                    "时间分寸：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用语规范：\\n{}",
                    self.wording()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "内容边界：\\n{}",
                    self.boundaries()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回应礼节：\\n{}",
                    self.response()
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
    fn test_privatemessagecourtesyrules_basic() {
        let rules = PrivateMessageCourtesyRules::new();
        assert_eq!(rules.metadata().name, "私信礼节");
        assert!(!rules.timing().is_empty());
        assert!(!rules.wording().is_empty());
        assert!(!rules.boundaries().is_empty());
        assert!(!rules.response().is_empty());
    }

    #[test]
    fn test_privatemessagecourtesyrules_validation() {
        let rules = PrivateMessageCourtesyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("private_message_courtesy")
        );
    }

    #[test]
    fn test_privatemessagecourtesyrules_explain() {
        let rules = PrivateMessageCourtesyRules::new();
        let e = rules.explain();
        assert!(e.contains("时间分寸"));
        assert!(e.contains("用语规范"));
        assert!(e.contains("内容边界"));
    }
}
