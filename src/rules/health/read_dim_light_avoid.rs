//! 阅读光线注意
//!
//! 避免暗光阅读、正确采光的护眼

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReadDimLightAvoidRules,
    name: "阅读光线注意",
    desc: "避免暗光阅读、正确采光的护眼",
    origin: "医学",
    tags: ["健康", "阅读", "光线", "护眼"]
}

impl ReadDimLightAvoidRules {
    /// 光线充足
    pub fn adequate(&self) -> Vec<&'static str> {
        vec!["阅读环境明亮", "光线从侧边来", "不直射眼睛", "柔和均匀"]
    }

    /// 避免暗处
    pub fn avoid_dim(&self) -> Vec<&'static str> {
        vec!["不在黑暗中看书", "关灯玩手机伤眼", "开台灯辅光", "保护视力"]
    }

    /// 设备亮调
    pub fn screen_bright(&self) -> Vec<&'static str> {
        vec!["屏幕随环境调", "不过亮刺眼", "亮暗适宜", "舒适用眼"]
    }

    /// 定期保养
    pub fn care(&self) -> Vec<&'static str> {
        vec!["阅读间隙远眺", "少熬夜用眼", "均衡营养", "护眼长久"]
    }
}

impl Rule for ReadDimLightAvoidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("read_light")
    }

    fn explain(&self) -> String {
        format!(
            "【阅读光线注意】\n{}",
            [
                format!(
                    "光线充足：\\n{}",
                    self.adequate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避免暗处：\\n{}",
                    self.avoid_dim()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "设备亮调：\\n{}",
                    self.screen_bright()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "定期保养：\\n{}",
                    self.care()
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
    fn test_readdimlightavoidrules_basic() {
        let rules = ReadDimLightAvoidRules::new();
        assert_eq!(rules.metadata().name, "阅读光线注意");
        assert!(!rules.adequate().is_empty());
        assert!(!rules.avoid_dim().is_empty());
        assert!(!rules.screen_bright().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_readdimlightavoidrules_validation() {
        let rules = ReadDimLightAvoidRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("read_light"));
    }

    #[test]
    fn test_readdimlightavoidrules_explain() {
        let rules = ReadDimLightAvoidRules::new();
        let e = rules.explain();
        assert!(e.contains("光线充足"));
        assert!(e.contains("避免暗处"));
        assert!(e.contains("设备亮调"));
    }
}
