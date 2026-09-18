//! 手脚冰凉调理
//!
//! 手足易冷之人的保暖与促进循环养生方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ColdHandsFeetWarmRules,
    name: "手脚冰凉调理",
    desc: "手足易冷之人的保暖与促进循环养生方法",
    origin: "中国",
    tags: ["健康", "手脚", "保暖", "循环"]
}

impl ColdHandsFeetWarmRules {
    /// 原因认识
    pub fn cause(&self) -> Vec<&'static str> {
        vec!["末梢循环偏弱", "保暖不足", "压力紧张", "体质差异"]
    }

    /// 即时保暖
    pub fn warm(&self) -> Vec<&'static str> {
        vec!["热水泡手泡脚", "戴手套厚袜", "加衣护颈背", "搓手活动促循环"]
    }

    /// 日常血循环
    pub fn circulate(&self) -> Vec<&'static str> {
        vec!["适度运动快走", "温热饮食", "少烟酒", "情绪放松"]
    }

    /// 就医注意
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec![
            "指端明显变色青紫",
            "冰凉伴麻木疼痛",
            "皮温改变规律",
            "及时就诊排查",
        ]
    }
}

impl Rule for ColdHandsFeetWarmRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("cold_hands_feet")
    }

    fn explain(&self) -> String {
        format!(
            "【手脚冰凉调理】\n{}",
            [
                format!(
                    "原因认识：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "即时保暖：\\n{}",
                    self.warm()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常血循环：\\n{}",
                    self.circulate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医注意：\\n{}",
                    self.seek_help()
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
    fn test_coldhandsfeetwarmrules_basic() {
        let rules = ColdHandsFeetWarmRules::new();
        assert_eq!(rules.metadata().name, "手脚冰凉调理");
        assert!(!rules.cause().is_empty());
        assert!(!rules.warm().is_empty());
        assert!(!rules.circulate().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_coldhandsfeetwarmrules_validation() {
        let rules = ColdHandsFeetWarmRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("cold_hands_feet"));
    }

    #[test]
    fn test_coldhandsfeetwarmrules_explain() {
        let rules = ColdHandsFeetWarmRules::new();
        let e = rules.explain();
        assert!(e.contains("原因认识"));
        assert!(e.contains("即时保暖"));
        assert!(e.contains("日常血循环"));
    }
}
