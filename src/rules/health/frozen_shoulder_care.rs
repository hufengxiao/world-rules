//! 肩周炎护理
//!
//! 肩关节粘连僵硬的康复锻炼与日常护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FrozenShoulderCareRules,
    name: "肩周炎护理",
    desc: "肩关节粘连僵硬的康复锻炼与日常护理",
    origin: "中国",
    tags: ["骨骼", "关节", "康复"]
}

impl FrozenShoulderCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "肩部疼痛夜间加重",
            "活动范围逐渐受限",
            "梳头穿衣困难",
            "常见于中年人群",
        ]
    }

    /// 康复锻炼
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "钟摆式前后摆动",
            "爬墙动作逐步抬高",
            "毛巾背后拉伸",
            "循序渐进勿强拉",
        ]
    }

    /// 日常护理
    pub fn daily_care(&self) -> Vec<&'static str> {
        vec![
            "肩部保暖避免受凉",
            "避免提重物与突然用力",
            "保持良好坐姿",
            "睡眠避免压迫患肩",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "疼痛剧烈影响睡眠就医",
            "活动受限加重应就诊",
            "康复训练遵医嘱进行",
            "长期不愈须影像检查",
        ]
    }
}

impl Rule for FrozenShoulderCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("frozen_shoulder_care")
    }

    fn explain(&self) -> String {
        format!(
            "【肩周炎护理】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "康复锻炼：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常护理：\\n{}",
                    self.daily_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_frozenshouldercarerules_basic() {
        let rules = FrozenShoulderCareRules::new();
        assert_eq!(rules.metadata().name, "肩周炎护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.daily_care().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_frozenshouldercarerules_validation() {
        let rules = FrozenShoulderCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("frozen_shoulder_care")
        );
    }

    #[test]
    fn test_frozenshouldercarerules_explain() {
        let rules = FrozenShoulderCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("康复锻炼"));
        assert!(e.contains("日常护理"));
    }
}
