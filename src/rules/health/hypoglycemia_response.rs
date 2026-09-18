//! 低血糖应对
//!
//! 低血糖头晕心慌时的快速识别与处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HypoglycemiaResponseRules,
    name: "低血糖应对",
    desc: "低血糖头晕心慌时的快速识别与处理",
    origin: "中国",
    tags: ["健康", "低血糖", "急救", "血糖"]
}

impl HypoglycemiaResponseRules {
    /// 识别症状
    pub fn identify(&self) -> Vec<&'static str> {
        vec!["心悸出汗手抖", "头晕乏力心慌", "重者意识模糊", "及时测血糖"]
    }

    /// 快速补糖
    pub fn sugar(&self) -> Vec<&'static str> {
        vec![
            "进食糖果糖水",
            "巧克力含糖饮料",
            "十五分钟复测",
            "未好转再补",
        ]
    }

    /// 严重处理
    pub fn severe(&self) -> Vec<&'static str> {
        vec!["意识不清快就医", "勿强行喂食", "呼叫家人急救", "遵医嘱用药"]
    }

    /// 日常预防
    pub fn prevent(&self) -> Vec<&'static str> {
        vec!["规律餐食定时", "备零食应急", "运动适量", "用药按时"]
    }
}

impl Rule for HypoglycemiaResponseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hypoglycemia")
    }

    fn explain(&self) -> String {
        format!(
            "【低血糖应对】\n{}",
            [
                format!(
                    "识别症状：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "快速补糖：\\n{}",
                    self.sugar()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "严重处理：\\n{}",
                    self.severe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常预防：\\n{}",
                    self.prevent()
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
    fn test_hypoglycemiaresponserules_basic() {
        let rules = HypoglycemiaResponseRules::new();
        assert_eq!(rules.metadata().name, "低血糖应对");
        assert!(!rules.identify().is_empty());
        assert!(!rules.sugar().is_empty());
        assert!(!rules.severe().is_empty());
        assert!(!rules.prevent().is_empty());
    }

    #[test]
    fn test_hypoglycemiaresponserules_validation() {
        let rules = HypoglycemiaResponseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hypoglycemia"));
    }

    #[test]
    fn test_hypoglycemiaresponserules_explain() {
        let rules = HypoglycemiaResponseRules::new();
        let e = rules.explain();
        assert!(e.contains("识别症状"));
        assert!(e.contains("快速补糖"));
        assert!(e.contains("严重处理"));
    }
}
