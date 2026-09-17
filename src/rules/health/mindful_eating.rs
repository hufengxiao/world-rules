//! 正念饮食
//!
//! 细嚼慢咽、品味食物与觉察饱足的正念饮食

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MindfulEatingRules,
    name: "正念饮食",
    desc: "细嚼慢咽、品味食物与觉察饱足的正念饮食",
    origin: "心理学",
    tags: ["健康", "正念", "饮食", "减重"]
}

impl MindfulEatingRules {
    /// 专心进食
    pub fn focus(&self) -> Vec<&'static str> {
        vec![
            "吃饭不看电视手机",
            "专心品味食物",
            "感知色香口感",
            "减少分心进食",
        ]
    }

    /// 细嚼慢咽
    pub fn slow(&self) -> Vec<&'static str> {
        vec!["放慢用餐速度", "充分咀嚼", "放下筷子稍停", "与身体节奏同步"]
    }

    /// 觉察饱腹
    pub fn full(&self) -> Vec<&'static str> {
        vec![
            "留意吃饱信号",
            "不硬撑不过量",
            "区分嘴馋与饥饿",
            "吃到适可而止",
        ]
    }

    /// 情绪进食
    pub fn emotion(&self) -> Vec<&'static str> {
        vec![
            "识别焦躁时进食",
            "不以食物解压",
            "情绪波动另找出口",
            "健康享受每一餐",
        ]
    }
}

impl Rule for MindfulEatingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("mindful_eating")
    }

    fn explain(&self) -> String {
        format!(
            "【正念饮食】\n{}",
            [
                format!(
                    "专心进食：\\n{}",
                    self.focus()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "细嚼慢咽：\\n{}",
                    self.slow()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "觉察饱腹：\\n{}",
                    self.full()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "情绪进食：\\n{}",
                    self.emotion()
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
    fn test_mindfuleatingrules_basic() {
        let rules = MindfulEatingRules::new();
        assert_eq!(rules.metadata().name, "正念饮食");
        assert!(!rules.focus().is_empty());
        assert!(!rules.slow().is_empty());
        assert!(!rules.full().is_empty());
        assert!(!rules.emotion().is_empty());
    }

    #[test]
    fn test_mindfuleatingrules_validation() {
        let rules = MindfulEatingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("mindful_eating"));
    }

    #[test]
    fn test_mindfuleatingrules_explain() {
        let rules = MindfulEatingRules::new();
        let e = rules.explain();
        assert!(e.contains("专心进食"));
        assert!(e.contains("细嚼慢咽"));
        assert!(e.contains("觉察饱腹"));
    }
}
