//! 贫血营养与调理
//!
//! 缺铁等原因所致贫血的营养饮食与就医提示

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AnemiaNutritionRules,
    name: "贫血营养与调理",
    desc: "缺铁等原因所致贫血的营养饮食与就医提示",
    origin: "医学",
    tags: ["健康", "贫血", "铁", "营养"]
}

impl AnemiaNutritionRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "识别面色苍白易疲乏力",
            "注意心悸气短与头晕",
            "留意指甲苍白易昏",
            "持续不明贫血就医",
        ]
    }

    /// 补铁营养
    pub fn iron(&self) -> Vec<&'static str> {
        vec![
            "摄取富含铁的食物",
            "红肉动物血肝适量",
            "深绿蔬菜配合",
            "补铁搭配维C促吸收",
        ]
    }

    /// 预防因素
    pub fn causes(&self) -> Vec<&'static str> {
        vec![
            "女性经期失血需关注营养",
            "慢性出血应查明原因",
            "素食者注意植物铁与维B12",
            "药物治疗遵医嘱",
        ]
    }

    /// 就医检测
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "确诊进行血常规检查",
            "不擅自大量补铁剂",
            "区分缺铁与叶酸等原因",
            "长期贫血由医生评估",
        ]
    }
}

impl Rule for AnemiaNutritionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("anemia_nutrition")
    }

    fn explain(&self) -> String {
        format!(
            "【贫血营养与调理】\n{}",
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
                    "补铁营养：\\n{}",
                    self.iron()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防因素：\\n{}",
                    self.causes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医检测：\\n{}",
                    self.seek_care()
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
    fn test_anemianutritionrules_basic() {
        let rules = AnemiaNutritionRules::new();
        assert_eq!(rules.metadata().name, "贫血营养与调理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.iron().is_empty());
        assert!(!rules.causes().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_anemianutritionrules_validation() {
        let rules = AnemiaNutritionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("anemia_nutrition"));
    }

    #[test]
    fn test_anemianutritionrules_explain() {
        let rules = AnemiaNutritionRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("补铁营养"));
        assert!(e.contains("预防因素"));
    }
}
