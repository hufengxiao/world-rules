//! 合租室友相处礼仪
//!
//! 与室友共同居住的相处规则，包括公共区域卫生、噪音控制、物品借用与费用分摊，维持和谐共居关系。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RoommateLivingMannerRules,
    name: "合租室友相处礼仪",
    desc: "与室友共同居住的相处规则，包括公共区域卫生、噪音控制、物品借用与费用分摊，维持和谐共居关系。",
    origin: "城市合租生活中的常见共居场景",
    tags: ["社交", "生活", "共居", "礼仪"]
}

impl RoommateLivingMannerRules {
    /// 公共区域
    pub fn shared_space(&self) -> Vec<&'static str> {
        vec![
            "用完公共区域及时清理",
            "厨房油污随手擦净",
            "卫生间保持干燥整洁",
            "垃圾按轮值及时清倒",
        ]
    }

    /// 作息噪音
    pub fn noise(&self) -> Vec<&'static str> {
        vec![
            "夜间控制说话与音量",
            "戴耳机看视频听音乐",
            "轻关门窗不甩门",
            "早出晚归尽量放轻脚步",
        ]
    }

    /// 物品与费用
    pub fn belongings(&self) -> Vec<&'static str> {
        vec![
            "借用他人物品先征得同意",
            "及时归还并保持完好",
            "水电燃气按约定分摊",
            "私人食物不擅自取用",
        ]
    }

    /// 沟通分寸
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "有问题及时平和沟通",
            "不当面不说背后议论",
            "尊重彼此隐私与空间",
            "重大变动提前商量",
        ]
    }
}

impl Rule for RoommateLivingMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("roommate")
    }

    fn explain(&self) -> String {
        format!(
            "【合租室友相处礼仪】\n{}",
            [
                format!(
                    "公共区域：\\n{}",
                    self.shared_space()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "作息噪音：\\n{}",
                    self.noise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "物品与费用：\\n{}",
                    self.belongings()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沟通分寸：\\n{}",
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
    fn test_roommatelivingmannerrules_basic() {
        let rules = RoommateLivingMannerRules::new();
        assert_eq!(rules.metadata().name, "合租室友相处礼仪");
        assert!(!rules.shared_space().is_empty());
        assert!(!rules.noise().is_empty());
        assert!(!rules.belongings().is_empty());
        assert!(!rules.communication().is_empty());
    }

    #[test]
    fn test_roommatelivingmannerrules_validation() {
        let rules = RoommateLivingMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("roommate"));
    }

    #[test]
    fn test_roommatelivingmannerrules_explain() {
        let rules = RoommateLivingMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("公共区域"));
        assert!(e.contains("作息噪音"));
        assert!(e.contains("物品与费用"));
    }
}
