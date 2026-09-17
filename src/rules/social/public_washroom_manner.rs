//! 公共厕所礼仪
//!
//! 公共厕所使用、卫生与文明礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PublicWashroomMannerRules,
    name: "公共厕所礼仪",
    desc: "公共厕所使用、卫生与文明礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "公共厕所", "卫生"]
}

impl PublicWashroomMannerRules {
    /// 有序使用
    pub fn queue(&self) -> Vec<&'static str> {
        vec!["排队依次入内", "开门前先敲门", "按标识男女", "不乱闯占用"]
    }

    /// 爱护卫生
    pub fn clean(&self) -> Vec<&'static str> {
        vec!["用完冲水", "手纸归专用", "不留污渍", "保持地面整洁"]
    }

    /// 体谅他人
    pub fn consider(&self) -> Vec<&'static str> {
        vec![
            "不在内大声交谈",
            "快速使用不占用",
            "礼让残障与儿童设施",
            "洗手龙头轻关",
        ]
    }

    /// 公德守序
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["不涂鸦破坏", "节约用水", "文明用厕", "共同维护干净"]
    }
}

impl Rule for PublicWashroomMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("washroom")
    }

    fn explain(&self) -> String {
        format!(
            "【公共厕所礼仪】\n{}",
            [
                format!(
                    "有序使用：\\n{}",
                    self.queue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "爱护卫生：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体谅他人：\\n{}",
                    self.consider()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公德守序：\\n{}",
                    self.manner()
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
    fn test_publicwashroommannerrules_basic() {
        let rules = PublicWashroomMannerRules::new();
        assert_eq!(rules.metadata().name, "公共厕所礼仪");
        assert!(!rules.queue().is_empty());
        assert!(!rules.clean().is_empty());
        assert!(!rules.consider().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_publicwashroommannerrules_validation() {
        let rules = PublicWashroomMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("washroom"));
    }

    #[test]
    fn test_publicwashroommannerrules_explain() {
        let rules = PublicWashroomMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("有序使用"));
        assert!(e.contains("爱护卫生"));
        assert!(e.contains("体谅他人"));
    }
}
