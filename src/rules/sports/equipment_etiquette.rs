//! 运动器材使用礼仪
//!
//! 共用运动器材、场地时的轮流使用与爱护礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EquipmentEtiquetteRules,
    name: "运动器材使用礼仪",
    desc: "共用运动器材、场地时的轮流使用与爱护礼仪",
    origin: "国际",
    tags: ["体育", "器材", "礼仪", "分享", "场地"]
}

impl EquipmentEtiquetteRules {
    /// 轮流使用
    pub fn rotation(&self) -> Vec<&'static str> {
        vec![
            "公共器材轮流使用不霸占",
            "练完及时让出",
            "高峰期缩短个人用时",
            "尊重先来后到的次序",
        ]
    }

    /// 爱护器材
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "使用规范避免损坏",
            "用后归还原位",
            "发现损坏及时报备",
            "不私自拆卸器材",
        ]
    }

    /// 场地卫生
    pub fn cleanliness(&self) -> Vec<&'static str> {
        vec![
            "器材用后擦拭干净",
            "不在场地遗留物品",
            "汗渍毛巾等及时清理",
            "爱护共同健身环境",
        ]
    }

    /// 安全共享
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "运动前检查器材安全",
            "不与他人抢争同一器材距离",
            "重的器材注意搬运安全",
            "手与器材不扰他人",
        ]
    }
}

impl Rule for EquipmentEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("equipment")
    }

    fn explain(&self) -> String {
        format!(
            "【运动器材使用礼仪】\n{}",
            [
                format!(
                    "轮流使用：\\n{}",
                    self.rotation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "爱护器材：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地卫生：\\n{}",
                    self.cleanliness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全共享：\\n{}",
                    self.safety()
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
    fn test_equipmentetiquetterules_basic() {
        let rules = EquipmentEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "运动器材使用礼仪");
        assert!(!rules.rotation().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.cleanliness().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_equipmentetiquetterules_validation() {
        let rules = EquipmentEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("equipment"));
    }

    #[test]
    fn test_equipmentetiquetterules_explain() {
        let rules = EquipmentEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("轮流使用"));
        assert!(e.contains("爱护器材"));
        assert!(e.contains("场地卫生"));
    }
}
