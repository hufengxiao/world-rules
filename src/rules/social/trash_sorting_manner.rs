//! 垃圾分类礼仪
//!
//! 垃圾分类投放的公共规范与文明习惯

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TrashSortingMannerRules,
    name: "垃圾分类礼仪",
    desc: "垃圾分类投放的公共规范与文明习惯",
    origin: "中国",
    tags: ["公共场所", "环保", "礼仪"]
}

impl TrashSortingMannerRules {
    /// 分类原则
    pub fn sorting(&self) -> Vec<&'static str> {
        vec![
            "按当地规定分类",
            "可回收与有害分开",
            "厨余与其他分投",
            "认准标识投放",
        ]
    }

    /// 投放规范
    pub fn disposal(&self) -> Vec<&'static str> {
        vec![
            "按规定时间投放",
            "液体倒净再丢",
            "包装压扁省空间",
            "系紧袋口防散落",
        ]
    }

    /// 公共文明
    pub fn civility(&self) -> Vec<&'static str> {
        vec![
            "不随手乱扔",
            "投入指定桶内",
            "投放后盖好桶盖",
            "不翻捡垃圾桶",
        ]
    }

    /// 特殊物品
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "有害垃圾单独投放",
            "大件垃圾预约回收",
            "电池灯管勿乱弃",
            "医疗废物按规处理",
        ]
    }
}

impl Rule for TrashSortingMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("trash_sorting_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【垃圾分类礼仪】\n{}",
            [
                format!(
                    "分类原则：\\n{}",
                    self.sorting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "投放规范：\\n{}",
                    self.disposal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共文明：\\n{}",
                    self.civility()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊物品：\\n{}",
                    self.special()
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
    fn test_trashsortingmannerrules_basic() {
        let rules = TrashSortingMannerRules::new();
        assert_eq!(rules.metadata().name, "垃圾分类礼仪");
        assert!(!rules.sorting().is_empty());
        assert!(!rules.disposal().is_empty());
        assert!(!rules.civility().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_trashsortingmannerrules_validation() {
        let rules = TrashSortingMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("trash_sorting_manner")
        );
    }

    #[test]
    fn test_trashsortingmannerrules_explain() {
        let rules = TrashSortingMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("分类原则"));
        assert!(e.contains("投放规范"));
        assert!(e.contains("公共文明"));
    }
}
