//! 哺乳问题处理
//!
//! 母乳喂养常见困难的处理与护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BreastfeedingTroubleshootRules,
    name: "哺乳问题处理",
    desc: "母乳喂养常见困难的处理与护理方法",
    origin: "中国",
    tags: ["孕产", "哺乳", "女性"]
}

impl BreastfeedingTroubleshootRules {
    /// 正确含接
    pub fn latch(&self) -> Vec<&'static str> {
        vec![
            "婴儿含住大部分乳晕",
            "下颌贴住乳房",
            "含接正确无疼痛",
            "姿势舒适放松",
        ]
    }

    /// 常见问题
    pub fn problems(&self) -> Vec<&'static str> {
        vec![
            "乳头皲裂注意护理",
            "乳房胀痛及时排空",
            "乳汁淤积热敷按摩",
            "乳腺炎发热须就医",
        ]
    }

    /// 哺乳护理
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "按需哺乳不掐点",
            "两侧交替喂哺",
            "喂后挤出少量乳汁涂乳头",
            "保持乳房清洁干燥",
        ]
    }

    /// 饮食作息
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "哺乳期营养充足",
            "足量饮水保证泌乳",
            "保证休息减少压力",
            "慎用影响乳汁的药物",
        ]
    }
}

impl Rule for BreastfeedingTroubleshootRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("breastfeeding_troubleshoot")
    }

    fn explain(&self) -> String {
        format!(
            "【哺乳问题处理】\n{}",
            [
                format!(
                    "正确含接：\\n{}",
                    self.latch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常见问题：\\n{}",
                    self.problems()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "哺乳护理：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食作息：\\n{}",
                    self.lifestyle()
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
    fn test_breastfeedingtroubleshootrules_basic() {
        let rules = BreastfeedingTroubleshootRules::new();
        assert_eq!(rules.metadata().name, "哺乳问题处理");
        assert!(!rules.latch().is_empty());
        assert!(!rules.problems().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.lifestyle().is_empty());
    }

    #[test]
    fn test_breastfeedingtroubleshootrules_validation() {
        let rules = BreastfeedingTroubleshootRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("breastfeeding_troubleshoot")
        );
    }

    #[test]
    fn test_breastfeedingtroubleshootrules_explain() {
        let rules = BreastfeedingTroubleshootRules::new();
        let e = rules.explain();
        assert!(e.contains("正确含接"));
        assert!(e.contains("常见问题"));
        assert!(e.contains("哺乳护理"));
    }
}
