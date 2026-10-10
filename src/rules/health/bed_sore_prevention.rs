//! 压疮（褥疮）预防
//!
//! 长期卧床者压疮的预防与护理规则，包括定时翻身、皮肤护理、减压用具与营养支持，降低皮肤破损风险。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BedSorePreventionRules,
    name: "压疮（褥疮）预防",
    desc: "长期卧床者压疮的预防与护理规则，包括定时翻身、皮肤护理、减压用具与营养支持，降低皮肤破损风险。",
    origin: "老年护理与临床压疮防治指南",
    tags: ["健康", "护理", "卧床", "预防"]
}

impl BedSorePreventionRules {
    /// 体位变换
    pub fn positioning(&self) -> Vec<&'static str> {
        vec![
            "定时协助翻身变换体位",
            "骨突部位避免持续受压",
            "使用软枕垫托支撑",
            "坐位者也要定时减压",
        ]
    }

    /// 皮肤护理
    pub fn skin_care(&self) -> Vec<&'static str> {
        vec![
            "保持皮肤清洁干燥",
            "大小便后及时清理",
            "避免用力拖拽摩擦皮肤",
            "观察发红处勿按摩破损区",
        ]
    }

    /// 减压用具
    pub fn support(&self) -> Vec<&'static str> {
        vec![
            "使用气垫床或减压垫",
            "床单平整无褶皱",
            "衣物宽松无硬物",
            "保持床铺清洁干爽",
        ]
    }

    /// 营养与观察
    pub fn nutrition(&self) -> Vec<&'static str> {
        vec![
            "保证蛋白质与维生素摄入",
            "适量饮水保持皮肤弹性",
            "每日检查受压部位皮肤",
            "出现破溃及时就医处理",
        ]
    }
}

impl Rule for BedSorePreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("bed_sore")
    }

    fn explain(&self) -> String {
        format!(
            "【压疮（褥疮）预防】\n{}",
            [
                format!(
                    "体位变换：\\n{}",
                    self.positioning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "皮肤护理：\\n{}",
                    self.skin_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "减压用具：\\n{}",
                    self.support()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营养与观察：\\n{}",
                    self.nutrition()
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
    fn test_bedsorepreventionrules_basic() {
        let rules = BedSorePreventionRules::new();
        assert_eq!(rules.metadata().name, "压疮（褥疮）预防");
        assert!(!rules.positioning().is_empty());
        assert!(!rules.skin_care().is_empty());
        assert!(!rules.support().is_empty());
        assert!(!rules.nutrition().is_empty());
    }

    #[test]
    fn test_bedsorepreventionrules_validation() {
        let rules = BedSorePreventionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("bed_sore"));
    }

    #[test]
    fn test_bedsorepreventionrules_explain() {
        let rules = BedSorePreventionRules::new();
        let e = rules.explain();
        assert!(e.contains("体位变换"));
        assert!(e.contains("皮肤护理"));
        assert!(e.contains("减压用具"));
    }
}
