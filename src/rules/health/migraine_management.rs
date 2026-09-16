//! 偏头痛管理
//!
//! 偏头痛预防、诱因识别与发作期护理的自我管理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MigraineManagementRules,
    name: "偏头痛管理",
    desc: "偏头痛预防、诱因识别与发作期护理的自我管理规则",
    origin: "医学",
    tags: ["健康", "头痛", "偏头痛", "预防"]
}

impl MigraineManagementRules {
    /// 诱因识别
    pub fn triggers(&self) -> Vec<&'static str> {
        vec![
            "记录饮食摄入与发作关联",
            "规律作息避免睡眠剥夺",
            "避免过劳与压力累积",
            "避免强光闪烁与强烈气味",
            "少摄入含酪胺与亚硝酸盐食物",
        ]
    }

    /// 规律生活
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "固定睡眠时间与时长充足",
            "规律三餐不过度空腹",
            "适度规律锻炼",
            "减少酒精与咖啡因依赖",
        ]
    }

    /// 急性期护理
    pub fn acute(&self) -> Vec<&'static str> {
        vec![
            "发作早期及时休息于安静暗处",
            "遵医嘱在早期服用对症药物",
            "冷敷额部或太阳穴缓解",
            "避免剧烈活动加重",
        ]
    }

    /// 就医信号
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "首次剧烈头痛就医排查",
            "伴随发热颈强直立即就医",
            "突发剧痛有神经症状需急诊",
            "携带用药史给医生评估",
        ]
    }

    /// 日常管理
    pub fn daily(&self) -> Vec<&'static str> {
        vec![
            "避免滥用止痛药预防反跳",
            "规律复诊评估发作频率",
            "学习压力放松技巧",
            "如便秘腹胀亦专注身心平衡",
        ]
    }
}

impl Rule for MigraineManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("migraine")
    }

    fn explain(&self) -> String {
        let parts = vec![
            format!(
                "诱因识别：\\n{}",
                self.triggers()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "规律生活：\\n{}",
                self.routine()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "急性期护理：\\n{}",
                self.acute()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "就医信号：\\n{}",
                self.seek_care()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "日常管理：\\n{}",
                self.daily()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
        ];
        format!("【偏头痛管理】\n{}", parts.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_migrainemanagementrules_basic() {
        let rules = MigraineManagementRules::new();
        assert_eq!(rules.metadata().name, "偏头痛管理");
        assert!(!rules.triggers().is_empty());
        assert!(!rules.routine().is_empty());
        assert!(!rules.acute().is_empty());
        assert!(!rules.seek_care().is_empty());
        assert!(!rules.daily().is_empty());
    }

    #[test]
    fn test_migrainemanagementrules_validation() {
        let rules = MigraineManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("migraine"));
    }

    #[test]
    fn test_migrainemanagementrules_explain() {
        let rules = MigraineManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("诱因识别"));
        assert!(e.contains("规律生活"));
        assert!(e.contains("急性期护理"));
    }
}
