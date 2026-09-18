//! 敬酒倒酒礼仪
//!
//! 倒酒满度、顺序与敬酒举杯礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WinePouringMethodRules,
    name: "敬酒倒酒礼仪",
    desc: "倒酒满度、顺序与敬酒举杯礼节",
    origin: "中国",
    tags: ["社交", "倒酒", "敬酒", "礼仪"]
}

impl WinePouringMethodRules {
    /// 倒酒满度
    pub fn level(&self) -> Vec<&'static str> {
        vec!["白酒八分满", "红酒约三分之一", "啤酒倒起泡少", "依酒定度"]
    }

    /// 倒酒顺序
    pub fn order(&self) -> Vec<&'static str> {
        vec!["先客后主", "先长辈后晚辈", "顺向依次", "双手捧瓶替他人倒"]
    }

    /// 敬酒姿态
    pub fn toast(&self) -> Vec<&'static str> {
        vec!["举杯欠身", "碰杯低于长辈", "说祝酒辞", "不过度劝酒"]
    }

    /// 礼貌回敬
    pub fn response(&self) -> Vec<&'static str> {
        vec!["受敬起身回应", "量力而饮", "不胜酒力明说", "把酒言欢"]
    }
}

impl Rule for WinePouringMethodRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wine_pouring")
    }

    fn explain(&self) -> String {
        format!(
            "【敬酒倒酒礼仪】\n{}",
            [
                format!(
                    "倒酒满度：\\n{}",
                    self.level()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "倒酒顺序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "敬酒姿态：\\n{}",
                    self.toast()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼貌回敬：\\n{}",
                    self.response()
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
    fn test_winepouringmethodrules_basic() {
        let rules = WinePouringMethodRules::new();
        assert_eq!(rules.metadata().name, "敬酒倒酒礼仪");
        assert!(!rules.level().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.toast().is_empty());
        assert!(!rules.response().is_empty());
    }

    #[test]
    fn test_winepouringmethodrules_validation() {
        let rules = WinePouringMethodRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("wine_pouring"));
    }

    #[test]
    fn test_winepouringmethodrules_explain() {
        let rules = WinePouringMethodRules::new();
        let e = rules.explain();
        assert!(e.contains("倒酒满度"));
        assert!(e.contains("倒酒顺序"));
        assert!(e.contains("敬酒姿态"));
    }
}
