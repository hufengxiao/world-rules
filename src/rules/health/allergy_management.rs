//! 过敏管理
//!
//! 食物过敏与接触性过敏的识别、规避与应急规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AllergyManagementRules,
    name: "过敏管理",
    desc: "食物过敏与接触性过敏的识别、规避与应急规则",
    origin: "医学",
    tags: ["健康", "过敏", "免疫"]
}

impl AllergyManagementRules {
    /// 识别过敏原
    pub fn identify(&self) -> Vec<&'static str> {
        vec![
            "明确记录已知过敏食物",
            "阅读食品配料表",
            "留意隐形过敏原如酱料",
            "就医做规范过敏原检测",
        ]
    }

    /// 规避措施
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "外出用餐主动告知过敏信息",
            "不抱侥幸尝试疑似过敏食物",
            "注意交叉污染避免共用厨具",
            "携带过敏信息卡片",
        ]
    }

    /// 应急用药
    pub fn emergency(&self) -> Vec<&'static str> {
        vec![
            "严重过敏者随身携带抗过敏药",
            "备好肾上腺素注射剂使用者",
            "出现喉头水肿立即呼救",
            "症状加重不拖延就医",
        ]
    }

    /// 日常管理
    pub fn daily(&self) -> Vec<&'static str> {
        vec![
            "保持居室通风减少尘螨",
            "花粉季减少外出与开窗",
            "宠物接触时注意清洗",
            "遵医嘱规范用药不自行停药",
        ]
    }
}

impl Rule for AllergyManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("allergy")
    }

    fn explain(&self) -> String {
        format!(
            "【过敏管理】\n{}",
            [
                format!(
                    "识别过敏原：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "规避措施：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应急用药：\\n{}",
                    self.emergency()
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
    fn test_allergymanagementrules_basic() {
        let rules = AllergyManagementRules::new();
        assert_eq!(rules.metadata().name, "过敏管理");
        assert!(!rules.identify().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.emergency().is_empty());
        assert!(!rules.daily().is_empty());
    }

    #[test]
    fn test_allergymanagementrules_validation() {
        let rules = AllergyManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("allergy"));
    }

    #[test]
    fn test_allergymanagementrules_explain() {
        let rules = AllergyManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("识别过敏原"));
        assert!(e.contains("规避措施"));
        assert!(e.contains("应急用药"));
    }
}
