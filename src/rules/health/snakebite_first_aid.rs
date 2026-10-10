//! 蛇咬伤急救
//!
//! 毒蛇咬伤后的正确急救与禁忌处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SnakebiteFirstAidRules,
    name: "蛇咬伤急救",
    desc: "毒蛇咬伤后的正确急救与禁忌处理",
    origin: "中国",
    tags: ["急救", "咬伤", "应急"]
}

impl SnakebiteFirstAidRules {
    /// 现场处理
    pub fn onsite(&self) -> Vec<&'static str> {
        vec![
            "让伤者安静平卧",
            "伤肢制动放低",
            "迅速远离蛇出没处",
            "记录蛇的外形特征",
        ]
    }

    /// 伤口处理
    pub fn wound(&self) -> Vec<&'static str> {
        vec![
            "用清水冲洗伤口",
            "移除戒指手镯等束缚",
            "覆盖清洁敷料",
            "尽快送医注射抗毒血清",
        ]
    }

    /// 急救禁忌
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "不可切开伤口吸毒",
            "不可用止血带捆扎",
            "不可冰敷或烧灼",
            "不可让伤者奔跑",
        ]
    }

    /// 送医要点
    pub fn hospital(&self) -> Vec<&'static str> {
        vec![
            "尽快送往有血清医院",
            "记录咬伤时间",
            "观察症状变化",
            "保持伤者镇静",
        ]
    }
}

impl Rule for SnakebiteFirstAidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("snakebite_first_aid")
    }

    fn explain(&self) -> String {
        format!(
            "【蛇咬伤急救】\n{}",
            [
                format!(
                    "现场处理：\\n{}",
                    self.onsite()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "伤口处理：\\n{}",
                    self.wound()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "急救禁忌：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "送医要点：\\n{}",
                    self.hospital()
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
    fn test_snakebitefirstaidrules_basic() {
        let rules = SnakebiteFirstAidRules::new();
        assert_eq!(rules.metadata().name, "蛇咬伤急救");
        assert!(!rules.onsite().is_empty());
        assert!(!rules.wound().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.hospital().is_empty());
    }

    #[test]
    fn test_snakebitefirstaidrules_validation() {
        let rules = SnakebiteFirstAidRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("snakebite_first_aid")
        );
    }

    #[test]
    fn test_snakebitefirstaidrules_explain() {
        let rules = SnakebiteFirstAidRules::new();
        let e = rules.explain();
        assert!(e.contains("现场处理"));
        assert!(e.contains("伤口处理"));
        assert!(e.contains("急救禁忌"));
    }
}
