//! 在线课堂礼仪
//!
//! 视频会议、网课与线上课堂中的参与与互动礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OnlineClassesEtiquetteRules,
    name: "在线课堂礼仪",
    desc: "视频会议、网课与线上课堂中的参与与互动礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "在线视频", "网课", "会议"]
}

impl OnlineClassesEtiquetteRules {
    /// 课前准备
    pub fn prep(&self) -> Vec<&'static str> {
        vec![
            "提前测试网络与音视频设备",
            "准时进入在线课堂",
            "背景整洁避免过于杂乱",
            "准备单向时开麦克风",
        ]
    }

    /// 课堂互动
    pub fn interact(&self) -> Vec<&'static str> {
        vec![
            "按主持人或老师引导发言",
            "不打断他人提问",
            "发言清晰条理",
            "需待答再反馈互动",
        ]
    }

    /// 礼仪细节
    pub fn etiquette(&self) -> Vec<&'static str> {
        vec![
            "中途静音保持安静",
            "进出入课堂说明去向",
            "不使用不雅昵称或头像",
            "尊重教师与同学",
        ]
    }

    /// 课后沟通
    pub fn after(&self) -> Vec<&'static str> {
        vec!["课后有序提问答疑", "不散播课堂隐私", "按时提交课业"]
    }
}

impl Rule for OnlineClassesEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("online_class")
    }

    fn explain(&self) -> String {
        format!(
            "【在线课堂礼仪】\n{}",
            [
                format!(
                    "课前准备：\\n{}",
                    self.prep()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "课堂互动：\\n{}",
                    self.interact()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼仪细节：\\n{}",
                    self.etiquette()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "课后沟通：\\n{}",
                    self.after()
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
    fn test_onlineclassesetiquetterules_basic() {
        let rules = OnlineClassesEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "在线课堂礼仪");
        assert!(!rules.prep().is_empty());
        assert!(!rules.interact().is_empty());
        assert!(!rules.etiquette().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_onlineclassesetiquetterules_validation() {
        let rules = OnlineClassesEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("online_class"));
    }

    #[test]
    fn test_onlineclassesetiquetterules_explain() {
        let rules = OnlineClassesEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("课前准备"));
        assert!(e.contains("课堂互动"));
        assert!(e.contains("礼仪细节"));
    }
}
