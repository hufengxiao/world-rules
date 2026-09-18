//! 发热家庭照护
//!
//! 发热的测量、补水与退热方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FeverManagementHomeRules,
    name: "发热家庭照护",
    desc: "发热的测量、补水与退热方法",
    origin: "医学",
    tags: ["健康", "发热", "退烧", "护理"]
}

impl FeverManagementHomeRules {
    /// 正确测体
    pub fn measure(&self) -> Vec<&'static str> {
        vec!["用体温计测量", "测腋下或前额", "了解正常值", "记录变化"]
    }

    /// 多补水
    pub fn hydrate(&self) -> Vec<&'static str> {
        vec!["多喝温水", "清淡易消化", "防脱水", "补充电解质"]
    }

    /// 物理退热
    pub fn cooling(&self) -> Vec<&'static str> {
        vec!["温水擦浴", "少穿盖透气", "不捂汗", "阴凉通风"]
    }

    /// 用药就医
    pub fn medicate(&self) -> Vec<&'static str> {
        vec!["高热用退热药", "按体重剂量", "持续高热就医", "幼儿尤慎"]
    }
}

impl Rule for FeverManagementHomeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("fever")
    }

    fn explain(&self) -> String {
        format!(
            "【发热家庭照护】\n{}",
            [
                format!(
                    "正确测体：\\n{}",
                    self.measure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "多补水：\\n{}",
                    self.hydrate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "物理退热：\\n{}",
                    self.cooling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用药就医：\\n{}",
                    self.medicate()
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
    fn test_fevermanagementhomerules_basic() {
        let rules = FeverManagementHomeRules::new();
        assert_eq!(rules.metadata().name, "发热家庭照护");
        assert!(!rules.measure().is_empty());
        assert!(!rules.hydrate().is_empty());
        assert!(!rules.cooling().is_empty());
        assert!(!rules.medicate().is_empty());
    }

    #[test]
    fn test_fevermanagementhomerules_validation() {
        let rules = FeverManagementHomeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("fever"));
    }

    #[test]
    fn test_fevermanagementhomerules_explain() {
        let rules = FeverManagementHomeRules::new();
        let e = rules.explain();
        assert!(e.contains("正确测体"));
        assert!(e.contains("多补水"));
        assert!(e.contains("物理退热"));
    }
}
