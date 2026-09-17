//! 家庭药箱
//!
//! 家庭常备药、管理用药与安全存放

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HomeMedicineKitRules,
    name: "家庭药箱",
    desc: "家庭常备药、管理用药与安全存放",
    origin: "医学",
    tags: ["健康", "药箱", "常备药", "用药"]
}

impl HomeMedicineKitRules {
    /// 常备药
    pub fn stock(&self) -> Vec<&'static str> {
        vec!["备退烧止痛药", "备肠胃药", "外伤消毒敷料", "按需备慢性药"]
    }

    /// 合理用药
    pub fn usage(&self) -> Vec<&'static str> {
        vec!["按说明书服用", "不超量不过频", "不擅配抗生素", "留意过敏史"]
    }

    /// 存放管理
    pub fn storage(&self) -> Vec<&'static str> {
        vec!["阴凉干燥存放", "远离小孩", "定期查效期", "过期药处理"]
    }

    /// 安全警示
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["儿童防误服", "用药异常就医", "慢性病遵嘱", "配方清晰"]
    }
}

impl Rule for HomeMedicineKitRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("medicine_kit")
    }

    fn explain(&self) -> String {
        format!(
            "【家庭药箱】\n{}",
            [
                format!(
                    "常备药：\\n{}",
                    self.stock()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理用药：\\n{}",
                    self.usage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "存放管理：\\n{}",
                    self.storage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全警示：\\n{}",
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
    fn test_homemedicinekitrules_basic() {
        let rules = HomeMedicineKitRules::new();
        assert_eq!(rules.metadata().name, "家庭药箱");
        assert!(!rules.stock().is_empty());
        assert!(!rules.usage().is_empty());
        assert!(!rules.storage().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_homemedicinekitrules_validation() {
        let rules = HomeMedicineKitRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("medicine_kit"));
    }

    #[test]
    fn test_homemedicinekitrules_explain() {
        let rules = HomeMedicineKitRules::new();
        let e = rules.explain();
        assert!(e.contains("常备药"));
        assert!(e.contains("合理用药"));
        assert!(e.contains("存放管理"));
    }
}
