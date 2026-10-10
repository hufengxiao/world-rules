//! 坐骨神经痛缓解
//!
//! 坐骨神经受压引起的下肢放射痛的缓解护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SciaticaReliefRules,
    name: "坐骨神经痛缓解",
    desc: "坐骨神经受压引起的下肢放射痛的缓解护理",
    origin: "中国",
    tags: ["骨骼", "神经", "疼痛"]
}

impl SciaticaReliefRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "腰臀沿腿放射疼痛",
            "多自臀部放射至小腿",
            "咳嗽用力时加重",
            "可伴麻木无力",
        ]
    }

    /// 急性缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "卧床休息选硬板床",
            "侧卧屈膝减轻压迫",
            "急性期局部冷敷",
            "避免弯腰搬重物",
        ]
    }

    /// 康复锻炼
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "仰卧抱膝牵伸腰背",
            "小燕飞强化腰背肌",
            "桥式运动稳定核心",
            "疼痛缓解后逐步进行",
        ]
    }

    /// 就医警示
    pub fn warning(&self) -> Vec<&'static str> {
        vec![
            "大小便失禁须急诊",
            "下肢肌力骤降就医",
            "持续加重应查因",
            "马尾综合征属急症",
        ]
    }
}

impl Rule for SciaticaReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sciatica_relief")
    }

    fn explain(&self) -> String {
        format!(
            "【坐骨神经痛缓解】\n{}",
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
                    "急性缓解：\\n{}",
                    self.relief()
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
                    "就医警示：\\n{}",
                    self.warning()
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
    fn test_sciaticareliefrules_basic() {
        let rules = SciaticaReliefRules::new();
        assert_eq!(rules.metadata().name, "坐骨神经痛缓解");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.warning().is_empty());
    }

    #[test]
    fn test_sciaticareliefrules_validation() {
        let rules = SciaticaReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sciatica_relief"));
    }

    #[test]
    fn test_sciaticareliefrules_explain() {
        let rules = SciaticaReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("急性缓解"));
        assert!(e.contains("康复锻炼"));
    }
}
