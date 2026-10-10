//! 快递收件礼仪
//!
//! 收寄快递时与快递员及邻里的相处规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PackageDeliveryMannerRules,
    name: "快递收件礼仪",
    desc: "收寄快递时与快递员及邻里的相处规范",
    origin: "中国",
    tags: ["生活", "快递", "礼仪"]
}

impl PackageDeliveryMannerRules {
    /// 收件规范
    pub fn receiving(&self) -> Vec<&'static str> {
        vec![
            "保持电话畅通",
            "及时签收不拖延",
            "核对包裹无误再签",
            "当面验货保权益",
        ]
    }

    /// 与快递员
    pub fn courier(&self) -> Vec<&'static str> {
        vec![
            "态度礼貌致谢",
            "合理时间收件",
            "体谅配送辛苦",
            "有异议理性沟通",
        ]
    }

    /// 邻里相处
    pub fn neighbor(&self) -> Vec<&'static str> {
        vec![
            "代收须征得同意",
            "勿长期占用公共区域",
            "纸箱及时清理",
            "误拿及时归还",
        ]
    }

    /// 异常处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec![
            "丢件及时联系客服",
            "破损拍照留证",
            "合理主张赔偿",
            "依规投诉维权",
        ]
    }
}

impl Rule for PackageDeliveryMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("package_delivery_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【快递收件礼仪】\n{}",
            [
                format!(
                    "收件规范：\\n{}",
                    self.receiving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "与快递员：\\n{}",
                    self.courier()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "邻里相处：\\n{}",
                    self.neighbor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "异常处理：\\n{}",
                    self.dispute()
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
    fn test_packagedeliverymannerrules_basic() {
        let rules = PackageDeliveryMannerRules::new();
        assert_eq!(rules.metadata().name, "快递收件礼仪");
        assert!(!rules.receiving().is_empty());
        assert!(!rules.courier().is_empty());
        assert!(!rules.neighbor().is_empty());
        assert!(!rules.dispute().is_empty());
    }

    #[test]
    fn test_packagedeliverymannerrules_validation() {
        let rules = PackageDeliveryMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("package_delivery_manner")
        );
    }

    #[test]
    fn test_packagedeliverymannerrules_explain() {
        let rules = PackageDeliveryMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("收件规范"));
        assert!(e.contains("与快递员"));
        assert!(e.contains("邻里相处"));
    }
}
