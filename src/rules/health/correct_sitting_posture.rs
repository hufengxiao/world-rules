//! 正确坐姿
//!
//! 学习办公正确坐姿与腰颈保护

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CorrectSittingPostureRules,
    name: "正确坐姿",
    desc: "学习办公正确坐姿与腰颈保护",
    origin: "医学",
    tags: ["健康", "坐姿", "脊柱", "护腰"]
}

impl CorrectSittingPostureRules {
    /// 坐姿要素
    pub fn basics(&self) -> Vec<&'static str> {
        vec!["背靠椅背坐正", "腰曲自然", "头正对面上方", "不歪不驼"]
    }

    /// 桌椅高度
    pub fn height(&self) -> Vec<&'static str> {
        vec!["桌椅高度合适", "脚掌平踩地", "屏幕与眼平", "腰挺不塌"]
    }

    /// 时长调整
    pub fn adjust(&self) -> Vec<&'static str> {
        vec!["每坐多时防腰酸", "起身活动", "变换姿势", "避免久坐"]
    }

    /// 脊柱爱护
    pub fn care(&self) -> Vec<&'static str> {
        vec!["不弯腰搬重物", "前倾适度", "撑腰无力即休", "护脊健康"]
    }
}

impl Rule for CorrectSittingPostureRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sitting_posture")
    }

    fn explain(&self) -> String {
        format!(
            "【正确坐姿】\n{}",
            [
                format!(
                    "坐姿要素：\\n{}",
                    self.basics()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "桌椅高度：\\n{}",
                    self.height()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "时长调整：\\n{}",
                    self.adjust()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "脊柱爱护：\\n{}",
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
    fn test_correctsittingposturerules_basic() {
        let rules = CorrectSittingPostureRules::new();
        assert_eq!(rules.metadata().name, "正确坐姿");
        assert!(!rules.basics().is_empty());
        assert!(!rules.height().is_empty());
        assert!(!rules.adjust().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_correctsittingposturerules_validation() {
        let rules = CorrectSittingPostureRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sitting_posture"));
    }

    #[test]
    fn test_correctsittingposturerules_explain() {
        let rules = CorrectSittingPostureRules::new();
        let e = rules.explain();
        assert!(e.contains("坐姿要素"));
        assert!(e.contains("桌椅高度"));
        assert!(e.contains("时长调整"));
    }
}
