//! 病房安静礼仪
//!
//! 病房内保持安静体谅患者的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: QuietWardMannerRules,
    name: "病房安静礼仪",
    desc: "病房内保持安静体谅患者的礼仪",
    origin: "中国",
    tags: ["社交", "病房", "安静", "礼仪"]
}

impl QuietWardMannerRules {
    /// 轻声交谈
    pub fn whisper(&self) -> Vec<&'static str> {
        vec![
            "说话压低音量",
            "不吆喝他人",
            "不在病房大声打电话",
            "照顾同房病友",
        ]
    }

    /// 避开休息
    pub fn quiet_time(&self) -> Vec<&'static str> {
        vec!["午睡时段少打扰", "晚休保持安静", "探访短暂", "不聚群喧哗"]
    }

    /// 走动有序
    pub fn movement(&self) -> Vec<&'static str> {
        vec!["走动脚步轻", "器具轻拿轻放", "不占通道", "有序进出"]
    }

    /// 体谅病友
    pub fn considerate(&self) -> Vec<&'static str> {
        vec!["体恤他人不适", "不评议病情", "尊重隐私", "营造安宁"]
    }
}

impl Rule for QuietWardMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("ward_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【病房安静礼仪】\n{}",
            [
                format!(
                    "轻声交谈：\\n{}",
                    self.whisper()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避开休息：\\n{}",
                    self.quiet_time()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "走动有序：\\n{}",
                    self.movement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体谅病友：\\n{}",
                    self.considerate()
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
    fn test_quietwardmannerrules_basic() {
        let rules = QuietWardMannerRules::new();
        assert_eq!(rules.metadata().name, "病房安静礼仪");
        assert!(!rules.whisper().is_empty());
        assert!(!rules.quiet_time().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.considerate().is_empty());
    }

    #[test]
    fn test_quietwardmannerrules_validation() {
        let rules = QuietWardMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("ward_manner"));
    }

    #[test]
    fn test_quietwardmannerrules_explain() {
        let rules = QuietWardMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("轻声交谈"));
        assert!(e.contains("避开休息"));
        assert!(e.contains("走动有序"));
    }
}
