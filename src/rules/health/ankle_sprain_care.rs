//! 踝扭伤处理
//!
//! 脚踝关节扭伤的即时处理、休息与康复规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AnkleSprainCareRules,
    name: "踝扭伤处理",
    desc: "脚踝关节扭伤的即时处理、休息与康复规则",
    origin: "医学",
    tags: ["健康", "扭伤", "脚踝", "康复", "运动伤"]
}

impl AnkleSprainCareRules {
    /// 即时处理
    pub fn immediate(&self) -> Vec<&'static str> {
        vec![
            "立即停止活动并抬高患肢",
            "用冰袋冷敷减轻肿胀",
            "冷敷每次15至20分钟",
            "用弹性绷带适度加压固定",
        ]
    }

    /// 休息与制动
    pub fn immobilize(&self) -> Vec<&'static str> {
        vec![
            "受伤初期减少负重行走",
            "使用拐杖或扶手减轻承重",
            "不过早恢复剧烈活动",
            "保持患肢抬高利于消肿",
        ]
    }

    /// 就医判断
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "严重肿胀淤青或无法负重就医",
            "疼痛剧烈疑似脱位骨折",
            "多次反复扭伤需评估韧带",
            "儿童或老人扭伤应更谨慎",
        ]
    }

    /// 康复训练
    pub fn rehab(&self) -> Vec<&'static str> {
        vec![
            "肿消后循序渐进活动关节",
            "进行平衡与强化肌群训练",
            "逐步恢复正常负重",
            "复健期间疼痛明显即休息",
        ]
    }
}

impl Rule for AnkleSprainCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("ankle_sprain")
    }

    fn explain(&self) -> String {
        format!(
            "【踝扭伤处理】\n{}",
            [
                format!(
                    "即时处理：\\n{}",
                    self.immediate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "休息与制动：\\n{}",
                    self.immobilize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
                    self.seek_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "康复训练：\\n{}",
                    self.rehab()
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
    fn test_anklespraincarerules_basic() {
        let rules = AnkleSprainCareRules::new();
        assert_eq!(rules.metadata().name, "踝扭伤处理");
        assert!(!rules.immediate().is_empty());
        assert!(!rules.immobilize().is_empty());
        assert!(!rules.seek_care().is_empty());
        assert!(!rules.rehab().is_empty());
    }

    #[test]
    fn test_anklespraincarerules_validation() {
        let rules = AnkleSprainCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("ankle_sprain"));
    }

    #[test]
    fn test_anklespraincarerules_explain() {
        let rules = AnkleSprainCareRules::new();
        let e = rules.explain();
        assert!(e.contains("即时处理"));
        assert!(e.contains("休息与制动"));
        assert!(e.contains("就医判断"));
    }
}
