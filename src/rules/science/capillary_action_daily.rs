//! 毛细现象
//!
//! 液体在细管中上升的毛细作用及其日常应用

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CapillaryActionDailyRules,
    name: "毛细现象",
    desc: "液体在细管中上升的毛细作用及其日常应用",
    origin: "科学常识",
    tags: ["物理", "流体", "生活"]
}

impl CapillaryActionDailyRules {
    /// 现象原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "液体在细管中自动上升",
            "管径越细上升越高",
            "附着与内聚共同作用",
            "润湿液体呈凹液面",
        ]
    }

    /// 两个作用
    pub fn forces(&self) -> Vec<&'static str> {
        vec![
            "附着力使液体沿壁爬升",
            "内聚力使液体互相牵引",
            "两者共同形成毛细现象",
            "不润湿液体则下降",
        ]
    }

    /// 生活现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "毛巾吸水灯芯吸油",
            "纸巾浸水迅速洇湿",
            "土壤水分沿缝隙上移",
            "墙脚返潮与此有关",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "钢笔与棉芯灯使用",
            "植物根部输水依赖",
            "无纺布吸水材料",
            "防水靠阻断毛细通道",
        ]
    }
}

impl Rule for CapillaryActionDailyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("capillary_action_daily")
    }

    fn explain(&self) -> String {
        format!(
            "【毛细现象】\n{}",
            [
                format!(
                    "现象原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "两个作用：\\n{}",
                    self.forces()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活现象：\\n{}",
                    self.phenomena()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应用实例：\\n{}",
                    self.applications()
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
    fn test_capillaryactiondailyrules_basic() {
        let rules = CapillaryActionDailyRules::new();
        assert_eq!(rules.metadata().name, "毛细现象");
        assert!(!rules.principle().is_empty());
        assert!(!rules.forces().is_empty());
        assert!(!rules.phenomena().is_empty());
        assert!(!rules.applications().is_empty());
    }

    #[test]
    fn test_capillaryactiondailyrules_validation() {
        let rules = CapillaryActionDailyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("capillary_action_daily")
        );
    }

    #[test]
    fn test_capillaryactiondailyrules_explain() {
        let rules = CapillaryActionDailyRules::new();
        let e = rules.explain();
        assert!(e.contains("现象原理"));
        assert!(e.contains("两个作用"));
        assert!(e.contains("生活现象"));
    }
}
