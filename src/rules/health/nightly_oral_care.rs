//! 睡前口腔护理
//!
//! 睡前刷牙、清舌与避免夜间伤牙习惯

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NightlyOralCareRules,
    name: "睡前口腔护理",
    desc: "睡前刷牙、清舌与避免夜间伤牙习惯",
    origin: "牙科",
    tags: ["健康", "口腔", "睡前", "护理"]
}

impl NightlyOralCareRules {
    /// 睡前刷牙
    pub fn brush(&self) -> Vec<&'static str> {
        vec!["睡前认真刷牙", "餐后清水漱", "不带着食渣睡", "逐面清理"]
    }

    /// 清洁舌面
    pub fn tongue(&self) -> Vec<&'static str> {
        vec!["软刷刷舌面", "轻刮不伤", "减少口臭", "保持洁净"]
    }

    /// 夜宵注意
    pub fn snack(&self) -> Vec<&'static str> {
        vec!["晚食后刷牙", "吃完再刷", "少甜食饮品", "护牙防蛀"]
    }

    /// 养成习惯
    pub fn habit(&self) -> Vec<&'static str> {
        vec!["固定睡前流程", "孩子家长帮刷", "定期检查", "护牙好习惯"]
    }
}

impl Rule for NightlyOralCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nightly_oral")
    }

    fn explain(&self) -> String {
        format!(
            "【睡前口腔护理】\n{}",
            [
                format!(
                    "睡前刷牙：\\n{}",
                    self.brush()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清洁舌面：\\n{}",
                    self.tongue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "夜宵注意：\\n{}",
                    self.snack()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "养成习惯：\\n{}",
                    self.habit()
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
    fn test_nightlyoralcarerules_basic() {
        let rules = NightlyOralCareRules::new();
        assert_eq!(rules.metadata().name, "睡前口腔护理");
        assert!(!rules.brush().is_empty());
        assert!(!rules.tongue().is_empty());
        assert!(!rules.snack().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_nightlyoralcarerules_validation() {
        let rules = NightlyOralCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nightly_oral"));
    }

    #[test]
    fn test_nightlyoralcarerules_explain() {
        let rules = NightlyOralCareRules::new();
        let e = rules.explain();
        assert!(e.contains("睡前刷牙"));
        assert!(e.contains("清洁舌面"));
        assert!(e.contains("夜宵注意"));
    }
}
