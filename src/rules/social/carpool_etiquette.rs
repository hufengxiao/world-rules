//! 拼车与人乘车礼仪
//!
//! 拼车、搭车及车内的出行礼仪、分担与合作规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CarpoolEtiquetteRules,
    name: "拼车与人乘车礼仪",
    desc: "拼车、搭车及车内的出行礼仪、分担与合作规范",
    origin: "出行",
    tags: ["社交", "礼仪", "拼车", "乘车", "出宿"]
}

impl CarpoolEtiquetteRules {
    /// 预约与守时
    pub fn punctuality(&self) -> Vec<&'static str> {
        vec![
            "约定时间地点并守时",
            "临时变动及时告知对方",
            "不无故取消或迟到",
            "确认路线与费用分担方式",
        ]
    }

    /// 行为自律
    pub fn behaviour(&self) -> Vec<&'static str> {
        vec![
            "系好安全带注意安全",
            "不超载不超速",
            "车内不在驾驶时干扰驾驶员",
            "保持车内整洁不下向丢弃",
        ]
    }

    /// 分担与礼貌
    pub fn contributions(&self) -> Vec<&'static str> {
        vec![
            "主动分担油费高速费",
            "对司机表达感谢",
            "调整座位尊重其他乘客",
            "不在车内大声喧哗",
        ]
    }

    /// 沟通与协调
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "提前说明携带物品体积",
            "有宠物或异味告知对方",
            "协商好交接地点",
            "尊重司机与同行者意见",
        ]
    }
}

impl Rule for CarpoolEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("carpool")
    }

    fn explain(&self) -> String {
        format!(
            "【拼车与人乘车礼仪】\n{}",
            [
                format!(
                    "预约与守时：\\n{}",
                    self.punctuality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行为自律：\\n{}",
                    self.behaviour()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分担与礼貌：\\n{}",
                    self.contributions()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沟通与协调：\\n{}",
                    self.communication()
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
    fn test_carpooletiquetterules_basic() {
        let rules = CarpoolEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "拼车与人乘车礼仪");
        assert!(!rules.punctuality().is_empty());
        assert!(!rules.behaviour().is_empty());
        assert!(!rules.contributions().is_empty());
        assert!(!rules.communication().is_empty());
    }

    #[test]
    fn test_carpooletiquetterules_validation() {
        let rules = CarpoolEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("carpool"));
    }

    #[test]
    fn test_carpooletiquetterules_explain() {
        let rules = CarpoolEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("预约与守时"));
        assert!(e.contains("行为自律"));
        assert!(e.contains("分担与礼貌"));
    }
}
