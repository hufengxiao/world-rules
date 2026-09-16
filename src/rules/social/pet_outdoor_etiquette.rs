//! 宠物户外礼仪
//!
//! 携带宠物外出、散步、入园时的公共礼仪与责任

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PetOutdoorEtiquetteRules,
    name: "宠物户外礼仪",
    desc: "携带宠物外出、散步、入园时的公共礼仪与责任",
    origin: "大众",
    tags: ["社交", "礼仪", "宠物", "户外", "遛狗"]
}

impl PetOutdoorEtiquetteRules {
    /// 牵引与束控
    pub fn leash(&self) -> Vec<&'static str> {
        vec![
            "外出始终牵绳束控宠物",
            "大型或易惊犬带口笼",
            "遇陌生人或他宠收紧牵绳",
            "不交给缺乏控制的儿童牵引",
        ]
    }

    /// 清理粪便
    pub fn cleanup(&self) -> Vec<&'static str> {
        vec![
            "随身携带拾便袋",
            "及时清理宠物粪便",
            "不在他人草地随意排遗",
            "保持公共环境整洁",
        ]
    }

    /// 与人相处
    pub fn with_people(&self) -> Vec<&'static str> {
        vec![
            "不惊吓或扑向路人",
            "进入电梯等封闭空间先询问",
            "不与怕狗者强迫接触",
            "儿童宠物互不冒犯",
        ]
    }

    /// 场所文明
    pub fn places(&self) -> Vec<&'static str> {
        vec![
            "遵守场所能否携宠规定",
            "不在禁止区域带入宠物",
            "餐厅乘车等注意规定",
            "遛宠避开高峰拥挤时段",
        ]
    }
}

impl Rule for PetOutdoorEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("pet_outdoor")
    }

    fn explain(&self) -> String {
        format!(
            "【宠物户外礼仪】\n{}",
            [
                format!(
                    "牵引与束控：\\n{}",
                    self.leash()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清理粪便：\\n{}",
                    self.cleanup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "与人相处：\\n{}",
                    self.with_people()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场所文明：\\n{}",
                    self.places()
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
    fn test_petoutdooretiquetterules_basic() {
        let rules = PetOutdoorEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "宠物户外礼仪");
        assert!(!rules.leash().is_empty());
        assert!(!rules.cleanup().is_empty());
        assert!(!rules.with_people().is_empty());
        assert!(!rules.places().is_empty());
    }

    #[test]
    fn test_petoutdooretiquetterules_validation() {
        let rules = PetOutdoorEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("pet_outdoor"));
    }

    #[test]
    fn test_petoutdooretiquetterules_explain() {
        let rules = PetOutdoorEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("牵引与束控"));
        assert!(e.contains("清理粪便"));
        assert!(e.contains("与人相处"));
    }
}
