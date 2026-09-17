//! 癫痫发作应对
//!
//! 癫痫等抽搐发作时的保护、观察与急救要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SeizureResponseRules,
    name: "癫痫发作应对",
    desc: "癫痫等抽搐发作时的保护、观察与急救要点",
    origin: "医学",
    tags: ["健康", "癫痫", "抽搐", "急救", "照护"]
}

impl SeizureResponseRules {
    /// 保护患者
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "保持冷静不强行按压",
            "移开周围硬物和锐器",
            "垫护头部减少磕碰",
            "保持周围空间通畅",
        ]
    }

    /// 观察记录
    pub fn observe(&self) -> Vec<&'static str> {
        vec![
            "记录发作开始与持续时间",
            "观察身体哪部分抽搐",
            "留意意识与肤色变化",
            "勿把物品塞入患者口中",
        ]
    }

    /// 体位安全
    pub fn position(&self) -> Vec<&'static str> {
        vec![
            "将患者侧卧利于呼吸道",
            "解开紧身衣物颈部",
            "勿过度束缚肢体",
            "抽搐停止后仍观察呼吸",
        ]
    }

    /// 就医判断
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "首次发作应就医",
            "发作超5分钟需急救",
            "连续发作或意识不恢复就医",
            "发作后头部受伤需评估",
        ]
    }
}

impl Rule for SeizureResponseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("seizure_response")
    }

    fn explain(&self) -> String {
        format!(
            "【癫痫发作应对】\n{}",
            [
                format!(
                    "保护患者：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观察记录：\\n{}",
                    self.observe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体位安全：\\n{}",
                    self.position()
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
    fn test_seizureresponserules_basic() {
        let rules = SeizureResponseRules::new();
        assert_eq!(rules.metadata().name, "癫痫发作应对");
        assert!(!rules.protect().is_empty());
        assert!(!rules.observe().is_empty());
        assert!(!rules.position().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_seizureresponserules_validation() {
        let rules = SeizureResponseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("seizure_response"));
    }

    #[test]
    fn test_seizureresponserules_explain() {
        let rules = SeizureResponseRules::new();
        let e = rules.explain();
        assert!(e.contains("保护患者"));
        assert!(e.contains("观察记录"));
        assert!(e.contains("体位安全"));
    }
}
