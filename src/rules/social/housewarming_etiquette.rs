//! 乔迁之喜礼仪
//!
//! 乔迁新居的入住习俗、待客与送礼礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HousewarmingEtiquetteRules,
    name: "乔迁之喜礼仪",
    desc: "乔迁新居的入住习俗、待客与送礼礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "乔迁", "搬家", "家居"]
}

impl HousewarmingEtiquetteRules {
    /// 乔迁准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "择日前完成搬迁整理",
            "安排暖房活动时段",
            "邀请亲近亲友同庆",
            "准备待客点心饮品",
        ]
    }

    /// 待客迎宾
    pub fn hosting(&self) -> Vec<&'static str> {
        vec![
            "热情迎客引导参观",
            "分享搬家喜悦过程",
            "感谢来宾前来祝贺",
            "照顾第一次到访的客人",
        ]
    }

    /// 随礼答谢
    pub fn gifts(&self) -> Vec<&'static str> {
        vec![
            "接受礼物真诚表达感谢",
            "量力而为不必过于贵重",
            "以点心或宴请等回礼",
            "礼尚往来善待亲友",
        ]
    }

    /// 新居习俗
    pub fn custom(&self) -> Vec<&'static str> {
        vec![
            "尊重不同地区乔迁习俗",
            "不讲晦气话语",
            "注意邻里热情问候新邻居",
            "爱护搬入的公共环境",
        ]
    }
}

impl Rule for HousewarmingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("housewarming")
    }

    fn explain(&self) -> String {
        format!(
            "【乔迁之喜礼仪】\n{}",
            [
                format!(
                    "乔迁准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "待客迎宾：\\n{}",
                    self.hosting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "随礼答谢：\\n{}",
                    self.gifts()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "新居习俗：\\n{}",
                    self.custom()
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
    fn test_housewarmingetiquetterules_basic() {
        let rules = HousewarmingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "乔迁之喜礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.hosting().is_empty());
        assert!(!rules.gifts().is_empty());
        assert!(!rules.custom().is_empty());
    }

    #[test]
    fn test_housewarmingetiquetterules_validation() {
        let rules = HousewarmingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("housewarming"));
    }

    #[test]
    fn test_housewarmingetiquetterules_explain() {
        let rules = HousewarmingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("乔迁准备"));
        assert!(e.contains("待客迎宾"));
        assert!(e.contains("随礼答谢"));
    }
}
