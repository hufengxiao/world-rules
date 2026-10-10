//! 学习小组礼仪
//!
//! 参与学习小组或读书会时的相处规则，包括守时、分工、发言分寸与互相尊重，让共同学习高效又愉快。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: StudyGroupEtiquetteRules,
    name: "学习小组礼仪",
    desc: "参与学习小组或读书会时的相处规则，包括守时、分工、发言分寸与互相尊重，让共同学习高效又愉快。",
    origin: "校园与职场学习社群中的常见协作场景",
    tags: ["社交", "学习", "协作", "礼仪"]
}

impl StudyGroupEtiquetteRules {
    /// 基本守则
    pub fn basics(&self) -> Vec<&'static str> {
        vec![
            "按时参加不无故缺席",
            "提前完成分派的任务",
            "缺席须提前告知",
            "保持学习环境安静整洁",
        ]
    }

    /// 讨论发言
    pub fn discussion(&self) -> Vec<&'static str> {
        vec![
            "轮流发言不打断他人",
            "就事论事不带情绪",
            "不懂就问也乐于解答",
            "尊重不同观点",
        ]
    }

    /// 分工协作
    pub fn cooperation(&self) -> Vec<&'static str> {
        vec![
            "主动承担力所能及的部分",
            "按时交付不拖累整体",
            "需要帮助时及时求助",
            "成果共享不独占",
        ]
    }

    /// 相处分寸
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "不贬低他人水平",
            "不炫耀也不自卑",
            "闲聊适度不跑题太久",
            "散会时致谢道别",
        ]
    }
}

impl Rule for StudyGroupEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("study_group")
    }

    fn explain(&self) -> String {
        format!(
            "【学习小组礼仪】\n{}",
            [
                format!(
                    "基本守则：\\n{}",
                    self.basics()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "讨论发言：\\n{}",
                    self.discussion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分工协作：\\n{}",
                    self.cooperation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相处分寸：\\n{}",
                    self.manner()
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
    fn test_studygroupetiquetterules_basic() {
        let rules = StudyGroupEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "学习小组礼仪");
        assert!(!rules.basics().is_empty());
        assert!(!rules.discussion().is_empty());
        assert!(!rules.cooperation().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_studygroupetiquetterules_validation() {
        let rules = StudyGroupEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("study_group"));
    }

    #[test]
    fn test_studygroupetiquetterules_explain() {
        let rules = StudyGroupEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("基本守则"));
        assert!(e.contains("讨论发言"));
        assert!(e.contains("分工协作"));
    }
}
