//! 茶水间礼仪
//!
//! 办公室茶水间的共用设备与卫生规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OfficeKitchenMannerRules,
    name: "茶水间礼仪",
    desc: "办公室茶水间的共用设备与卫生规范",
    origin: "职场通用",
    tags: ["职场", "共用", "礼仪"]
}

impl OfficeKitchenMannerRules {
    /// 设备使用
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "咖啡机用完清理",
            "微波炉加盖加热",
            "饮水机及时补水",
            "设备故障及时报修",
        ]
    }

    /// 卫生规范
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "用后清洗杯具",
            "台面溅洒及时擦",
            "垃圾随手清理",
            "不留下个人杂物",
        ]
    }

    /// 共享礼节
    pub fn sharing(&self) -> Vec<&'static str> {
        vec![
            "公用物品用后归位",
            "补充用完的耗材",
            "不独占空间",
            "排队礼让",
        ]
    }

    /// 相处分寸
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "闲聊适可而止",
            "不议论同事是非",
            "不长时间逗留",
            "保持轻松氛围",
        ]
    }
}

impl Rule for OfficeKitchenMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("office_kitchen_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【茶水间礼仪】\n{}",
            [
                format!(
                    "设备使用：\\n{}",
                    self.equipment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生规范：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共享礼节：\\n{}",
                    self.sharing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相处分寸：\\n{}",
                    self.interaction()
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
    fn test_officekitchenmannerrules_basic() {
        let rules = OfficeKitchenMannerRules::new();
        assert_eq!(rules.metadata().name, "茶水间礼仪");
        assert!(!rules.equipment().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.sharing().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_officekitchenmannerrules_validation() {
        let rules = OfficeKitchenMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("office_kitchen_manner")
        );
    }

    #[test]
    fn test_officekitchenmannerrules_explain() {
        let rules = OfficeKitchenMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("设备使用"));
        assert!(e.contains("卫生规范"));
        assert!(e.contains("共享礼节"));
    }
}
