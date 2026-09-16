//! 优质睡眠作息
//!
//! 建立规律作息、改善睡眠质量的睡眠卫生规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SleepQualityRules,
    name: "优质睡眠作息",
    desc: "建立规律作息、改善睡眠质量的睡眠卫生规则",
    origin: "国际",
    tags: ["健康", "睡眠", "作息", "休息", "作息卫生"]
}

impl SleepQualityRules {
    /// 规律作息
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "固定上床与起床时间",
            "保证充足睡眠时长",
            "周末不过度补觉打乱",
            "建立睡前放松习惯",
        ]
    }

    /// 环境优化
    pub fn environment(&self) -> Vec<&'static str> {
        vec![
            "卧室昏暗安静温度适宜",
            "睡前减少强光刺激",
            "规律作息与室温相合",
            "使用遮光窗帘",
        ]
    }

    /// 行为习惯
    pub fn habits(&self) -> Vec<&'static str> {
        vec![
            "睡前避免咖啡因与大量饮酒",
            "睡前不看炫亮屏幕过久",
            "避免夜间饱食",
            "保持适度日间活动",
        ]
    }

    /// 睡眠困扰
    pub fn troubles(&self) -> Vec<&'static str> {
        vec![
            "失眠持续影响白昼就医",
            "打鼾伴呼吸暂停早筛查",
            "长期挣扎入睡留意情绪",
            "不长期滥服助眠药",
        ]
    }
}

impl Rule for SleepQualityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sleep_quality")
    }

    fn explain(&self) -> String {
        format!(
            "【优质睡眠作息】\n{}",
            [
                format!(
                    "规律作息：\\n{}",
                    self.routine()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境优化：\\n{}",
                    self.environment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行为习惯：\\n{}",
                    self.habits()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "睡眠困扰：\\n{}",
                    self.troubles()
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
    fn test_sleepqualityrules_basic() {
        let rules = SleepQualityRules::new();
        assert_eq!(rules.metadata().name, "优质睡眠作息");
        assert!(!rules.routine().is_empty());
        assert!(!rules.environment().is_empty());
        assert!(!rules.habits().is_empty());
        assert!(!rules.troubles().is_empty());
    }

    #[test]
    fn test_sleepqualityrules_validation() {
        let rules = SleepQualityRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sleep_quality"));
    }

    #[test]
    fn test_sleepqualityrules_explain() {
        let rules = SleepQualityRules::new();
        let e = rules.explain();
        assert!(e.contains("规律作息"));
        assert!(e.contains("环境优化"));
        assert!(e.contains("行为习惯"));
    }
}
