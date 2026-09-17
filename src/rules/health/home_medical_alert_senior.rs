//! 老人居家呼救
//!
//! 居家老人紧急呼救、电子呼救与突发应对

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HomeMedicalAlertSeniorRules,
    name: "老人居家呼救",
    desc: "居家老人紧急呼救、电子呼救与突发应对",
    origin: "医学",
    tags: ["健康", "老人", "呼救", "居家安全"]
}

impl HomeMedicalAlertSeniorRules {
    /// 呼救设备
    pub fn device(&self) -> Vec<&'static str> {
        vec![
            "备紧急呼叫铃",
            "手机设快捷联系",
            "可佩戴呼叫按钮",
            "邻居家人留号",
        ]
    }

    /// 突发应对
    pub fn emergency(&self) -> Vec<&'static str> {
        vec![
            "跌倒不勉强起身",
            "大声或按键呼救",
            "家属定联系",
            "急救电话记牢",
        ]
    }

    /// 为了及时
    pub fn readiness(&self) -> Vec<&'static str> {
        vec![
            "每天有人照看",
            "固定时段视频问",
            "检测异常察觉",
            "不孤居无援",
        ]
    }

    /// 家人协助
    pub fn family(&self) -> Vec<&'static str> {
        vec![
            "备好常吃药到身边",
            "病史资料放好",
            "定期健康监测",
            "关怀不间断",
        ]
    }
}

impl Rule for HomeMedicalAlertSeniorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("home_alert")
    }

    fn explain(&self) -> String {
        format!(
            "【老人居家呼救】\n{}",
            [
                format!(
                    "呼救设备：\\n{}",
                    self.device()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "突发应对：\\n{}",
                    self.emergency()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "为了及时：\\n{}",
                    self.readiness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家人协助：\\n{}",
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
    fn test_homemedicalalertseniorrules_basic() {
        let rules = HomeMedicalAlertSeniorRules::new();
        assert_eq!(rules.metadata().name, "老人居家呼救");
        assert!(!rules.device().is_empty());
        assert!(!rules.emergency().is_empty());
        assert!(!rules.readiness().is_empty());
        assert!(!rules.family().is_empty());
    }

    #[test]
    fn test_homemedicalalertseniorrules_validation() {
        let rules = HomeMedicalAlertSeniorRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("home_alert"));
    }

    #[test]
    fn test_homemedicalalertseniorrules_explain() {
        let rules = HomeMedicalAlertSeniorRules::new();
        let e = rules.explain();
        assert!(e.contains("呼救设备"));
        assert!(e.contains("突发应对"));
        assert!(e.contains("为了及时"));
    }
}
