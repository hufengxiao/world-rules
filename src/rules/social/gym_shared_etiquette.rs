//! 健身房共享礼仪
//!
//! 健身房器材共享、卫生与互助的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GymSharedEtiquetteRules,
    name: "健身房共享礼仪",
    desc: "健身房器材共享、卫生与互助的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "健身房", "共享"]
}

impl GymSharedEtiquetteRules {
    /// 器材共享
    pub fn equip(&self) -> Vec<&'static str> {
        vec![
            "用完器材及时让出",
            "与他人共用排好队",
            "不长时间霸占",
            "有急用礼貌协商",
        ]
    }

    /// 卫生整洁
    pub fn clean(&self) -> Vec<&'static str> {
        vec![
            "运动后用毛巾擦垫",
            "归还器械归位",
            "保持更衣整洁",
            "不遗留个人物品",
        ]
    }

    /// 安静礼貌
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "练习适量不大吵",
            "手机不外放",
            "交谈不过分喧闹",
            "注重个人清洁",
        ]
    }

    /// 互助尊重
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "不评判他人身材",
            "指导需征得同意",
            "礼让新手与长辈",
            "营造友好氛围",
        ]
    }
}

impl Rule for GymSharedEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("gym_shared")
    }

    fn explain(&self) -> String {
        format!(
            "【健身房共享礼仪】\n{}",
            [
                format!(
                    "器材共享：\\n{}",
                    self.equip()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生整洁：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安静礼貌：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "互助尊重：\\n{}",
                    self.respect()
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
    fn test_gymsharedetiquetterules_basic() {
        let rules = GymSharedEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "健身房共享礼仪");
        assert!(!rules.equip().is_empty());
        assert!(!rules.clean().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.respect().is_empty());
    }

    #[test]
    fn test_gymsharedetiquetterules_validation() {
        let rules = GymSharedEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("gym_shared"));
    }

    #[test]
    fn test_gymsharedetiquetterules_explain() {
        let rules = GymSharedEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("器材共享"));
        assert!(e.contains("卫生整洁"));
        assert!(e.contains("安静礼貌"));
    }
}
