//! 密室逃脱协作礼仪
//!
//! 参与密室逃脱游戏时的团队协作与礼仪规则，包括听从规则、分工配合、尊重队友与爱护道具，提升共同体验。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EscapeRoomMannerRules,
    name: "密室逃脱协作礼仪",
    desc: "参与密室逃脱游戏时的团队协作与礼仪规则，包括听从规则、分工配合、尊重队友与爱护道具，提升共同体验。",
    origin: "现代都市娱乐活动中的团队游戏场景",
    tags: ["社交", "娱乐", "协作", "礼仪"]
}

impl EscapeRoomMannerRules {
    /// 入场守则
    pub fn entry(&self) -> Vec<&'static str> {
        vec![
            "认真听工作人员讲解规则",
            "按时到场不迟到拖延",
            "妥善存放随身贵重物品",
            "不携带违禁或危险物品",
        ]
    }

    /// 团队协作
    pub fn teamwork(&self) -> Vec<&'static str> {
        vec![
            "合理分工各司其职",
            "及时分享发现的线索",
            "不独断也不袖手旁观",
            "遇分歧快速沟通定夺",
        ]
    }

    /// 尊重队友
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "不埋怨出错的人",
            "照顾胆小的队友",
            "不抢功也不甩锅",
            "鼓励为主营造氛围",
        ]
    }

    /// 道具与安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "轻拿轻放机关道具",
            "不暴力破坏设备",
            "遵守安全提示不冒险",
            "结束后归位并道谢",
        ]
    }
}

impl Rule for EscapeRoomMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("escape_room")
    }

    fn explain(&self) -> String {
        format!(
            "【密室逃脱协作礼仪】\n{}",
            [
                format!(
                    "入场守则：\\n{}",
                    self.entry()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "团队协作：\\n{}",
                    self.teamwork()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重队友：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "道具与安全：\\n{}",
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
    fn test_escaperoommannerrules_basic() {
        let rules = EscapeRoomMannerRules::new();
        assert_eq!(rules.metadata().name, "密室逃脱协作礼仪");
        assert!(!rules.entry().is_empty());
        assert!(!rules.teamwork().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_escaperoommannerrules_validation() {
        let rules = EscapeRoomMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("escape_room"));
    }

    #[test]
    fn test_escaperoommannerrules_explain() {
        let rules = EscapeRoomMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("入场守则"));
        assert!(e.contains("团队协作"));
        assert!(e.contains("尊重队友"));
    }
}
