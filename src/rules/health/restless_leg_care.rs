//! 不宁腿缓解
//!
//! 不宁腿综合征的日常缓解与调理规则，包括作息规律、适度活动、饮食调整与就医评估，改善夜间腿部不适。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RestlessLegCareRules,
    name: "不宁腿缓解",
    desc: "不宁腿综合征的日常缓解与调理规则，包括作息规律、适度活动、饮食调整与就医评估，改善夜间腿部不适。",
    origin: "神经内科睡眠障碍相关健康建议",
    tags: ["健康", "睡眠", "腿部", "调理"]
}

impl RestlessLegCareRules {
    /// 作息调理
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "保持规律作息与睡眠时间",
            "睡前放松避免过度兴奋",
            "营造安静舒适的睡眠环境",
            "避免熬夜加重症状",
        ]
    }

    /// 活动缓解
    pub fn activity(&self) -> Vec<&'static str> {
        vec![
            "发作时起身走动缓解",
            "适度拉伸小腿肌肉",
            "睡前温水泡脚放松",
            "白天规律运动但避免睡前剧烈",
        ]
    }

    /// 饮食注意
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "减少咖啡因与酒精摄入",
            "睡前避免浓茶咖啡",
            "注意铁与叶酸营养",
            "均衡饮食不偏食",
        ]
    }

    /// 就医评估
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "症状频繁影响睡眠须就诊",
            "排查缺铁或慢性疾病",
            "遵医嘱用药不自行加量",
            "记录发作规律便于诊疗",
        ]
    }
}

impl Rule for RestlessLegCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("restless_leg")
    }

    fn explain(&self) -> String {
        format!(
            "【不宁腿缓解】\n{}",
            [
                format!(
                    "作息调理：\\n{}",
                    self.routine()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "活动缓解：\\n{}",
                    self.activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食注意：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医评估：\\n{}",
                    self.medical()
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
    fn test_restlesslegcarerules_basic() {
        let rules = RestlessLegCareRules::new();
        assert_eq!(rules.metadata().name, "不宁腿缓解");
        assert!(!rules.routine().is_empty());
        assert!(!rules.activity().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_restlesslegcarerules_validation() {
        let rules = RestlessLegCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("restless_leg"));
    }

    #[test]
    fn test_restlesslegcarerules_explain() {
        let rules = RestlessLegCareRules::new();
        let e = rules.explain();
        assert!(e.contains("作息调理"));
        assert!(e.contains("活动缓解"));
        assert!(e.contains("饮食注意"));
    }
}
