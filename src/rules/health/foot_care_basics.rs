//! 足部护理基础
//!
//! 日常足部清洁、保湿与护足注意事项

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FootCareBasicsRules,
    name: "足部护理基础",
    desc: "日常足部清洁、保湿与护足注意事项",
    origin: "医学",
    tags: ["健康", "足部", "护理"]
}

impl FootCareBasicsRules {
    /// 清洁保湿
    pub fn clean(&self) -> Vec<&'static str> {
        vec!["每日温水洗足", "足趾缝擦干", "涂抹保湿霜", "保持干爽"]
    }

    /// 剪甲护足
    pub fn nails(&self) -> Vec<&'static str> {
        vec!["指甲平剪防扎", "不剪过深", "防甲沟炎", "选合脚鞋袜"]
    }

    /// 护足防病
    pub fn prevent(&self) -> Vec<&'static str> {
        vec![
            "留意足部破损",
            "脚趾缝发痒注意",
            "及时处理茧鸡眼",
            "糖尿病患者常检",
        ]
    }

    /// 就医提示
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "足部红肿感染就医",
            "伤口不愈合检查",
            "麻木不适评估",
            "规范护足",
        ]
    }
}

impl Rule for FootCareBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("foot_care")
    }

    fn explain(&self) -> String {
        format!(
            "【足部护理基础】\n{}",
            [
                format!(
                    "清洁保湿：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "剪甲护足：\\n{}",
                    self.nails()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护足防病：\\n{}",
                    self.prevent()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.care()
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
    fn test_footcarebasicsrules_basic() {
        let rules = FootCareBasicsRules::new();
        assert_eq!(rules.metadata().name, "足部护理基础");
        assert!(!rules.clean().is_empty());
        assert!(!rules.nails().is_empty());
        assert!(!rules.prevent().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_footcarebasicsrules_validation() {
        let rules = FootCareBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("foot_care"));
    }

    #[test]
    fn test_footcarebasicsrules_explain() {
        let rules = FootCareBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁保湿"));
        assert!(e.contains("剪甲护足"));
        assert!(e.contains("护足防病"));
    }
}
