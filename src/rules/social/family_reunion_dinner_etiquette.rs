//! 家庭聚餐礼仪
//!
//! 一家人团聚用餐时的座次与相处礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FamilyReunionDinnerEtiquetteRules,
    name: "家庭聚餐礼仪",
    desc: "一家人团聚用餐时的座次与相处礼节",
    origin: "中国",
    tags: ["家庭", "聚餐", "礼仪"]
}

impl FamilyReunionDinnerEtiquetteRules {
    /// 座次安排
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "长辈居主位",
            "长者上座晚辈下座",
            "夫妻可相邻而坐",
            "让位给来客",
        ]
    }

    /// 用餐礼节
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "长者先动筷",
            "为长辈夹菜添饭",
            "吃饭不出声不挑拣",
            "公筷公勺讲卫生",
        ]
    }

    /// 交谈分寸
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "多聊家常少揭短",
            "不谈令人不快话题",
            "尊重各人选择",
            "不当众攀比议论",
        ]
    }

    /// 晚辈之道
    pub fn junior_duty(&self) -> Vec<&'static str> {
        vec![
            "主动帮忙张罗",
            "照顾老人小孩",
            "饭后帮忙收拾",
            "离席道别致谢",
        ]
    }
}

impl Rule for FamilyReunionDinnerEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("family_reunion_dinner_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【家庭聚餐礼仪】\n{}",
            [
                format!(
                    "座次安排：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用餐礼节：\\n{}",
                    self.dining()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交谈分寸：\\n{}",
                    self.conversation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "晚辈之道：\\n{}",
                    self.junior_duty()
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
    fn test_familyreuniondinneretiquetterules_basic() {
        let rules = FamilyReunionDinnerEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "家庭聚餐礼仪");
        assert!(!rules.seating().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.junior_duty().is_empty());
    }

    #[test]
    fn test_familyreuniondinneretiquetterules_validation() {
        let rules = FamilyReunionDinnerEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("family_reunion_dinner_etiquette")
        );
    }

    #[test]
    fn test_familyreuniondinneretiquetterules_explain() {
        let rules = FamilyReunionDinnerEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("座次安排"));
        assert!(e.contains("用餐礼节"));
        assert!(e.contains("交谈分寸"));
    }
}
