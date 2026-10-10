//! 共享冰箱礼仪
//!
//! 办公室共用冰箱的食物存放与使用规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SharedFridgeMannerRules,
    name: "共享冰箱礼仪",
    desc: "办公室共用冰箱的食物存放与使用规范",
    origin: "职场通用",
    tags: ["职场", "共用", "礼仪"]
}

impl SharedFridgeMannerRules {
    /// 存放规范
    pub fn storage(&self) -> Vec<&'static str> {
        vec![
            "食物标注姓名日期",
            "密封保存防串味",
            "易腐食物及时冷藏",
            "不占用过多空间",
        ]
    }

    /// 取用礼节
    pub fn taking(&self) -> Vec<&'static str> {
        vec![
            "只取自己的食物",
            "勿擅动他人餐食",
            "不试吃他人食物",
            "取物后关好冰箱门",
        ]
    }

    /// 清洁维护
    pub fn cleaning(&self) -> Vec<&'static str> {
        vec![
            "洒漏及时清理",
            "过期食物自行处理",
            "定期清理不留残渣",
            "发现异味及时报告",
        ]
    }

    /// 公共意识
    pub fn awareness(&self) -> Vec<&'static str> {
        vec![
            "不存放气味浓烈食物",
            "遵守部门张贴规定",
            "勿长期囤积物品",
            "共同维护卫生",
        ]
    }
}

impl Rule for SharedFridgeMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("shared_fridge_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【共享冰箱礼仪】\n{}",
            [
                format!(
                    "存放规范：\\n{}",
                    self.storage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "取用礼节：\\n{}",
                    self.taking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清洁维护：\\n{}",
                    self.cleaning()
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
    fn test_sharedfridgemannerrules_basic() {
        let rules = SharedFridgeMannerRules::new();
        assert_eq!(rules.metadata().name, "共享冰箱礼仪");
        assert!(!rules.storage().is_empty());
        assert!(!rules.taking().is_empty());
        assert!(!rules.cleaning().is_empty());
        assert!(!rules.awareness().is_empty());
    }

    #[test]
    fn test_sharedfridgemannerrules_validation() {
        let rules = SharedFridgeMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("shared_fridge_manner")
        );
    }

    #[test]
    fn test_sharedfridgemannerrules_explain() {
        let rules = SharedFridgeMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("存放规范"));
        assert!(e.contains("取用礼节"));
        assert!(e.contains("清洁维护"));
    }
}
