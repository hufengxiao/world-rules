//! 瑜伽课堂礼仪
//!
//! 瑜伽课堂中的安静、尊重与自我限度礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: YogaEtiquetteRules,
    name: "瑜伽课堂礼仪",
    desc: "瑜伽课堂中的安静、尊重与自我限度礼仪",
    origin: "大众",
    tags: ["体育", "瑜伽", "课堂", "礼仪", "静修"]
}

impl YogaEtiquetteRules {
    /// 课前
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前到课备好垫子",
            "手机静音勿扰",
            "告知老师身体不适",
            "不占他人垫位",
        ]
    }

    /// 课中
    pub fn during(&self) -> Vec<&'static str> {
        vec![
            "跟随呼吸与口令",
            "不勉强高难度体式",
            "保持安静专注",
            "不评论他人动作水平",
        ]
    }

    /// 空间与安全
    pub fn space(&self) -> Vec<&'static str> {
        vec![
            "留出与他人合适间距",
            "使用辅具量力",
            "做动作不阻挡他人视线",
            "需要休息可回到婴儿式",
        ]
    }

    /// 共修谦逊
    pub fn metta(&self) -> Vec<&'static str> {
        vec![
            "尊重老师与同修",
            "不过度竞争比较",
            "尊重不同水平的新手",
            "课后向老师致谢",
        ]
    }
}

impl Rule for YogaEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("yoga")
    }

    fn explain(&self) -> String {
        format!(
            "【瑜伽课堂礼仪】\n{}",
            [
                format!(
                    "课前：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "课中：\\n{}",
                    self.during()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "空间与安全：\\n{}",
                    self.space()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共修谦逊：\\n{}",
                    self.metta()
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
    fn test_yogaetiquetterules_basic() {
        let rules = YogaEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "瑜伽课堂礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.during().is_empty());
        assert!(!rules.space().is_empty());
        assert!(!rules.metta().is_empty());
    }

    #[test]
    fn test_yogaetiquetterules_validation() {
        let rules = YogaEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("yoga"));
    }

    #[test]
    fn test_yogaetiquetterules_explain() {
        let rules = YogaEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("课前"));
        assert!(e.contains("课中"));
        assert!(e.contains("空间与安全"));
    }
}
