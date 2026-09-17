//! 水泡护理
//!
//! 脚部摩擦水泡的处理、防感染与护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BlisterCareRules,
    name: "水泡护理",
    desc: "脚部摩擦水泡的处理、防感染与护理",
    origin: "医学",
    tags: ["健康", "水泡", "护理"]
}

impl BlisterCareRules {
    /// 成因与防范
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "长时间摩擦产生水泡",
            "穿合脚鞋袜减少摩擦",
            "可贴防护垫",
            "及时换干爽袜",
        ]
    }

    /// 正确处理
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "小水泡少挤压自行吸收",
            "不随意挑破",
            "如需排液注意消毒",
            "保持患处清洁干燥",
        ]
    }

    /// 预防感染
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "避免继续摩擦",
            "用清洁敷料覆盖",
            "红肿痛加重就医",
            "有脓液或发热就诊",
        ]
    }

    /// 特殊情况
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "糖尿病患者注意足部",
            "水泡伴感染速就医",
            "足部溃疡勿自行处理",
            "规范护足防并发",
        ]
    }
}

impl Rule for BlisterCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("blister")
    }

    fn explain(&self) -> String {
        format!(
            "【水泡护理】\n{}",
            [
                format!(
                    "成因与防范：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确处理：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防感染：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊情况：\\n{}",
                    self.special()
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
    fn test_blistercarerules_basic() {
        let rules = BlisterCareRules::new();
        assert_eq!(rules.metadata().name, "水泡护理");
        assert!(!rules.avoid().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.protect().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_blistercarerules_validation() {
        let rules = BlisterCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("blister"));
    }

    #[test]
    fn test_blistercarerules_explain() {
        let rules = BlisterCareRules::new();
        let e = rules.explain();
        assert!(e.contains("成因与防范"));
        assert!(e.contains("正确处理"));
        assert!(e.contains("预防感染"));
    }
}
