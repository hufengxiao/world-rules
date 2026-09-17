//! 生日庆祝礼仪
//!
//! 生日聚会上的祝福、切蛋糕、送礼与参与礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BirthdayCelebrationRules,
    name: "生日庆祝礼仪",
    desc: "生日聚会上的祝福、切蛋糕、送礼与参与礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "生日", "聚会", "庆祝"]
}

impl BirthdayCelebrationRules {
    /// 主人筹备
    pub fn hosting(&self) -> Vec<&'static str> {
        vec![
            "提前邀请并确认名单",
            "安排场地时间与主题",
            "准备蛋糕与节目环节",
            "照顾到场各年龄段",
        ]
    }

    /// 来宾参与
    pub fn guest(&self) -> Vec<&'static str> {
        vec![
            "准时到场并道贺",
            "送合适的小礼物",
            "融入活动响应氛围",
            "不喧宾夺主抢风头",
        ]
    }

    /// 切蛋糕仪式
    pub fn cutting(&self) -> Vec<&'static str> {
        vec![
            "寿星许愿后进行切蛋糕",
            "先请长辈或寿星开切",
            "分设给各宾客",
            "留第一份给寿星",
        ]
    }

    /// 互动祝福
    pub fn blessing(&self) -> Vec<&'static str> {
        vec![
            "齐唱生日歌带出气氛",
            "真诚祝愿身体健康",
            "拍照留念记录美好",
            "散场致谢来宾",
        ]
    }
}

impl Rule for BirthdayCelebrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("birthday_celeb")
    }

    fn explain(&self) -> String {
        format!(
            "【生日庆祝礼仪】\n{}",
            [
                format!(
                    "主人筹备：\\n{}",
                    self.hosting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "来宾参与：\\n{}",
                    self.guest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "切蛋糕仪式：\\n{}",
                    self.cutting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "互动祝福：\\n{}",
                    self.blessing()
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
    fn test_birthdaycelebrationrules_basic() {
        let rules = BirthdayCelebrationRules::new();
        assert_eq!(rules.metadata().name, "生日庆祝礼仪");
        assert!(!rules.hosting().is_empty());
        assert!(!rules.guest().is_empty());
        assert!(!rules.cutting().is_empty());
        assert!(!rules.blessing().is_empty());
    }

    #[test]
    fn test_birthdaycelebrationrules_validation() {
        let rules = BirthdayCelebrationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("birthday_celeb"));
    }

    #[test]
    fn test_birthdaycelebrationrules_explain() {
        let rules = BirthdayCelebrationRules::new();
        let e = rules.explain();
        assert!(e.contains("主人筹备"));
        assert!(e.contains("来宾参与"));
        assert!(e.contains("切蛋糕仪式"));
    }
}
