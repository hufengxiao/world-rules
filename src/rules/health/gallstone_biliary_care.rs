//! 胆囊结石护理
//!
//! 胆囊结石与胆道健康的日常调理规则，涵盖低脂饮食、规律进餐、体重管理与症状识别，减少胆绞痛发作风险。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GallstoneBiliaryCareRules,
    name: "胆囊结石护理",
    desc: "胆囊结石与胆道健康的日常调理规则，涵盖低脂饮食、规律进餐、体重管理与症状识别，减少胆绞痛发作风险。",
    origin: "消化内科与肝胆外科临床建议",
    tags: ["健康", "胆囊", "结石", "饮食"]
}

impl GallstoneBiliaryCareRules {
    /// 饮食原则
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "控制脂肪与胆固醇摄入",
            "少食油炸肥腻食物",
            "多吃蔬菜与膳食纤维",
            "适量优质蛋白",
        ]
    }

    /// 进餐规律
    pub fn meal(&self) -> Vec<&'static str> {
        vec![
            "三餐定时不长时间空腹",
            "早餐不可省略",
            "避免暴饮暴食",
            "少量多餐减轻胆囊负担",
        ]
    }

    /// 生活管理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "控制体重避免快速减肥",
            "适度运动促进代谢",
            "保持情绪平稳",
            "规律作息不熬夜",
        ]
    }

    /// 症状与就医
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "右上腹疼痛放射至肩背",
            "进食油腻后腹胀恶心",
            "出现黄疸或高热须急诊",
            "定期超声复查结石变化",
        ]
    }
}

impl Rule for GallstoneBiliaryCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("gallstone")
    }

    fn explain(&self) -> String {
        format!(
            "【胆囊结石护理】\n{}",
            [
                format!(
                    "饮食原则：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "进餐规律：\\n{}",
                    self.meal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活管理：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "症状与就医：\\n{}",
                    self.medical()
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
    fn test_gallstonebiliarycarerules_basic() {
        let rules = GallstoneBiliaryCareRules::new();
        assert_eq!(rules.metadata().name, "胆囊结石护理");
        assert!(!rules.diet().is_empty());
        assert!(!rules.meal().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_gallstonebiliarycarerules_validation() {
        let rules = GallstoneBiliaryCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("gallstone"));
    }

    #[test]
    fn test_gallstonebiliarycarerules_explain() {
        let rules = GallstoneBiliaryCareRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食原则"));
        assert!(e.contains("进餐规律"));
        assert!(e.contains("生活管理"));
    }
}
