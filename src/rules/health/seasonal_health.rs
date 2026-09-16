//! 换季健康防护
//!
//! 春夏季交替时季节性疾病与身体适应的防护规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SeasonalHealthRules,
    name: "换季健康防护",
    desc: "春夏季交替时季节性疾病与身体适应的防护规则",
    origin: "医学",
    tags: ["健康", "换季", "季节", "预防", "免疫"]
}

impl SeasonalHealthRules {
    /// 起居适应
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "根据气温增减衣物",
            "保持作息规律充足睡眠",
            "居家通风减少密闭",
            "避免乍暖乍寒急剧受凉",
        ]
    }

    /// 饮食调理
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "多吃时令蔬果补维C",
            "清淡饮食少油腻辛辣",
            "多喝温水保润",
            "增强体质均衡营养",
        ]
    }

    /// 特殊防护
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "花粉过敏者减少高峰期外出",
            "出门佩戴口罩护口鼻",
            "过敏体质备应急药",
            "感冒流行期勤洗手",
        ]
    }

    /// 身体信号
    pub fn warning(&self) -> Vec<&'static str> {
        vec![
            "持续不适不拖延",
            "咳嗽发热多日就医",
            "过敏症状加重需及时就诊",
            "老人儿童更注重保暖防护",
        ]
    }
}

impl Rule for SeasonalHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("seasonal")
    }

    fn explain(&self) -> String {
        format!(
            "【换季健康防护】\n{}",
            [
                format!(
                    "起居适应：\\n{}",
                    self.routine()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食调理：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊防护：\\n{}",
                    self.prevention()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "身体信号：\\n{}",
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
    fn test_seasonalhealthrules_basic() {
        let rules = SeasonalHealthRules::new();
        assert_eq!(rules.metadata().name, "换季健康防护");
        assert!(!rules.routine().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.prevention().is_empty());
        assert!(!rules.warning().is_empty());
    }

    #[test]
    fn test_seasonalhealthrules_validation() {
        let rules = SeasonalHealthRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("seasonal"));
    }

    #[test]
    fn test_seasonalhealthrules_explain() {
        let rules = SeasonalHealthRules::new();
        let e = rules.explain();
        assert!(e.contains("起居适应"));
        assert!(e.contains("饮食调理"));
        assert!(e.contains("特殊防护"));
    }
}
