//! 帕金森日常照护
//!
//! 帕金森病患者的日常照护规则，包括用药管理、运动康复、居家安全与情绪支持，提升生活质量与自理能力。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ParkinsonCareBasicRules,
    name: "帕金森日常照护",
    desc: "帕金森病患者的日常照护规则，包括用药管理、运动康复、居家安全与情绪支持，提升生活质量与自理能力。",
    origin: "神经内科与老年照护实践建议",
    tags: ["健康", "神经", "老年", "照护"]
}

impl ParkinsonCareBasicRules {
    /// 用药管理
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "严格按时按量服药",
            "不可自行增减或停药",
            "记录药效波动情况",
            "定期复诊调整方案",
        ]
    }

    /// 运动康复
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "坚持适度关节活动训练",
            "练习平衡与步态改善",
            "做面部与言语练习",
            "量力而行循序渐进",
        ]
    }

    /// 居家安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "通道通畅移除绊脚物",
            "浴室加装扶手防跌倒",
            "穿防滑鞋避免拖鞋",
            "起步转身放慢动作",
        ]
    }

    /// 情绪与支持
    pub fn support(&self) -> Vec<&'static str> {
        vec![
            "耐心沟通不催促",
            "鼓励参与社交活动",
            "关注情绪变化防抑郁",
            "家属学习照护知识",
        ]
    }
}

impl Rule for ParkinsonCareBasicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("parkinson")
    }

    fn explain(&self) -> String {
        format!(
            "【帕金森日常照护】\n{}",
            [
                format!(
                    "用药管理：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动康复：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "居家安全：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "情绪与支持：\\n{}",
                    self.support()
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
    fn test_parkinsoncarebasicrules_basic() {
        let rules = ParkinsonCareBasicRules::new();
        assert_eq!(rules.metadata().name, "帕金森日常照护");
        assert!(!rules.medication().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.support().is_empty());
    }

    #[test]
    fn test_parkinsoncarebasicrules_validation() {
        let rules = ParkinsonCareBasicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("parkinson"));
    }

    #[test]
    fn test_parkinsoncarebasicrules_explain() {
        let rules = ParkinsonCareBasicRules::new();
        let e = rules.explain();
        assert!(e.contains("用药管理"));
        assert!(e.contains("运动康复"));
        assert!(e.contains("居家安全"));
    }
}
