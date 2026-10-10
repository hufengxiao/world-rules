//! 微波加热食物原理
//!
//! 微波炉加热食物的水分子振动与加热不均原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MicrowaveHeatingFoodRules,
    name: "微波加热食物原理",
    desc: "微波炉加热食物的水分子振动与加热不均原理",
    origin: "科学常识",
    tags: ["物理", "生活", "电磁"]
}

impl MicrowaveHeatingFoodRules {
    /// 加热原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "微波使水分子快速振动",
            "极性分子摩擦生热",
            "热量由食物内部产生",
            "无需介质传热",
        ]
    }

    /// 加热不均
    pub fn unevenness(&self) -> Vec<&'static str> {
        vec![
            "微波在腔内形成驻波",
            "波节波腹冷热交替",
            "转盘使受热更均匀",
            "厚食物内外有差异",
        ]
    }

    /// 适用容器
    pub fn container(&self) -> Vec<&'static str> {
        vec![
            "可用陶瓷玻璃耐热塑料",
            "禁用金属器皿",
            "金属反射微波会打火",
            "封闭容器须留气孔",
        ]
    }

    /// 安全注意
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "带壳蛋不可直接加热",
            "密封液体防过热暴沸",
            "加热后静置片刻",
            "取出时注意烫手",
        ]
    }
}

impl Rule for MicrowaveHeatingFoodRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("microwave_heating_food")
    }

    fn explain(&self) -> String {
        format!(
            "【微波加热食物原理】\n{}",
            [
                format!(
                    "加热原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "加热不均：\\n{}",
                    self.unevenness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "适用容器：\\n{}",
                    self.container()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全注意：\\n{}",
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
    fn test_microwaveheatingfoodrules_basic() {
        let rules = MicrowaveHeatingFoodRules::new();
        assert_eq!(rules.metadata().name, "微波加热食物原理");
        assert!(!rules.principle().is_empty());
        assert!(!rules.unevenness().is_empty());
        assert!(!rules.container().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_microwaveheatingfoodrules_validation() {
        let rules = MicrowaveHeatingFoodRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("microwave_heating_food")
        );
    }

    #[test]
    fn test_microwaveheatingfoodrules_explain() {
        let rules = MicrowaveHeatingFoodRules::new();
        let e = rules.explain();
        assert!(e.contains("加热原理"));
        assert!(e.contains("加热不均"));
        assert!(e.contains("适用容器"));
    }
}
