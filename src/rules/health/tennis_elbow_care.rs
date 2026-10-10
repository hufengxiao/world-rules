//! 网球肘护理
//!
//! 肘部外侧肌腱劳损的休息与康复护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TennisElbowCareRules,
    name: "网球肘护理",
    desc: "肘部外侧肌腱劳损的休息与康复护理",
    origin: "中国",
    tags: ["骨骼", "肌腱", "劳损"]
}

impl TennisElbowCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "肘外侧按压疼痛",
            "拧毛巾提物时加剧",
            "握力下降",
            "前臂用力时不适",
        ]
    }

    /// 休息制动
    pub fn rest(&self) -> Vec<&'static str> {
        vec![
            "减少重复手腕伸展动作",
            "暂停诱发疼痛的活动",
            "必要时佩戴护肘",
            "急性期冷敷消肿",
        ]
    }

    /// 康复锻炼
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "前臂伸肌牵伸",
            "离心力量训练",
            "握力渐进恢复",
            "疼痛缓解后循序渐进",
        ]
    }

    /// 预防复发
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "调整工具握柄粗细",
            "注意工作姿势",
            "运动前充分热身",
            "避免过度单侧用力",
        ]
    }
}

impl Rule for TennisElbowCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("tennis_elbow_care")
    }

    fn explain(&self) -> String {
        format!(
            "【网球肘护理】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "休息制动：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "康复锻炼：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防复发：\\n{}",
                    self.prevention()
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
    fn test_tenniselbowcarerules_basic() {
        let rules = TennisElbowCareRules::new();
        assert_eq!(rules.metadata().name, "网球肘护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.prevention().is_empty());
    }

    #[test]
    fn test_tenniselbowcarerules_validation() {
        let rules = TennisElbowCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("tennis_elbow_care"));
    }

    #[test]
    fn test_tenniselbowcarerules_explain() {
        let rules = TennisElbowCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("休息制动"));
        assert!(e.contains("康复锻炼"));
    }
}
