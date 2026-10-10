//! 公共洗衣房礼仪
//!
//! 小区或宿舍共用洗衣房的轮流使用规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LaundryRoomMannerRules,
    name: "公共洗衣房礼仪",
    desc: "小区或宿舍共用洗衣房的轮流使用规范",
    origin: "中国",
    tags: ["公共场所", "共用", "礼仪"]
}

impl LaundryRoomMannerRules {
    /// 使用秩序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "按先来后到使用",
            "不长时间占用机器",
            "洗完及时取出衣物",
            "高峰期礼让排队",
        ]
    }

    /// 卫生习惯
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "内衣外衣分开洗",
            "不洗沾污严重物品",
            "清理滤网绒毛",
            "保持台面清洁",
        ]
    }

    /// 物品处理
    pub fn belongings(&self) -> Vec<&'static str> {
        vec![
            "他人衣物勿乱动",
            "误拿及时归还",
            "遗失物品交管理处",
            "不占用他人洗涤筐",
        ]
    }

    /// 公共意识
    pub fn awareness(&self) -> Vec<&'static str> {
        vec![
            "节约水电",
            "夜间减少噪音",
            "遵守张贴的规定",
            "损坏设备及时报修",
        ]
    }
}

impl Rule for LaundryRoomMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("laundry_room_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【公共洗衣房礼仪】\n{}",
            [
                format!(
                    "使用秩序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生习惯：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "物品处理：\\n{}",
                    self.belongings()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共意识：\\n{}",
                    self.awareness()
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
    fn test_laundryroommannerrules_basic() {
        let rules = LaundryRoomMannerRules::new();
        assert_eq!(rules.metadata().name, "公共洗衣房礼仪");
        assert!(!rules.order().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.belongings().is_empty());
        assert!(!rules.awareness().is_empty());
    }

    #[test]
    fn test_laundryroommannerrules_validation() {
        let rules = LaundryRoomMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("laundry_room_manner")
        );
    }

    #[test]
    fn test_laundryroommannerrules_explain() {
        let rules = LaundryRoomMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("使用秩序"));
        assert!(e.contains("卫生习惯"));
        assert!(e.contains("物品处理"));
    }
}
