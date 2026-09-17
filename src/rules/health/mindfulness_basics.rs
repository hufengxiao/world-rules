//! 正念冥想入门
//!
//! 正念冥想的基础练习、注意力与放松方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MindfulnessBasicsRules,
    name: "正念冥想入门",
    desc: "正念冥想的基础练习、注意力与放松方法",
    origin: "心理学",
    tags: ["健康", "正念", "冥想", "放松", "专注"]
}

impl MindfulnessBasicsRules {
    /// 基础姿势
    pub fn posture(&self) -> Vec<&'static str> {
        vec![
            "选择安静舒适坐位",
            "挺直但放松脊背",
            "双手自然放松",
            "微闭眼或垂视",
        ]
    }

    /// 呼吸正念
    pub fn breath(&self) -> Vec<&'static str> {
        vec![
            "专注观察自然呼吸",
            "觉察吸与呼的流动",
            "走神时温和拉回",
            "不强迫连续",
        ]
    }

    /// 觉察练习
    pub fn awareness(&self) -> Vec<&'static str> {
        vec![
            "以旁观态度审视念头",
            "不去评判好坏",
            "让念头自然来去",
            "保持温和好奇",
        ]
    }

    /// 习惯培养
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "循序渐进从几分钟开始",
            "固定时间坚持练习",
            "把正念融入生活片刻",
            "不追求即刻效果",
        ]
    }
}

impl Rule for MindfulnessBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("mindfulness")
    }

    fn explain(&self) -> String {
        format!(
            "【正念冥想入门】\n{}",
            [
                format!(
                    "基础姿势：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼吸正念：\\n{}",
                    self.breath()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "觉察练习：\\n{}",
                    self.awareness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "习惯培养：\\n{}",
                    self.routine()
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
    fn test_mindfulnessbasicsrules_basic() {
        let rules = MindfulnessBasicsRules::new();
        assert_eq!(rules.metadata().name, "正念冥想入门");
        assert!(!rules.posture().is_empty());
        assert!(!rules.breath().is_empty());
        assert!(!rules.awareness().is_empty());
        assert!(!rules.routine().is_empty());
    }

    #[test]
    fn test_mindfulnessbasicsrules_validation() {
        let rules = MindfulnessBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("mindfulness"));
    }

    #[test]
    fn test_mindfulnessbasicsrules_explain() {
        let rules = MindfulnessBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("基础姿势"));
        assert!(e.contains("呼吸正念"));
        assert!(e.contains("觉察练习"));
    }
}
