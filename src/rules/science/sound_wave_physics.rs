//! 声音波动原理
//!
//! 声音由振动产生、传播与听感特征

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SoundWavePhysicsRules,
    name: "声音波动原理",
    desc: "声音由振动产生、传播与听感特征",
    origin: "物理",
    tags: ["科学", "声音", "波动", "物理"]
}

impl SoundWavePhysicsRules {
    /// 振动发声
    pub fn vibration(&self) -> Vec<&'static str> {
        vec!["物体振动发声", "振动停止声停", "鼓皮琴弦皆如此", "生源基础"]
    }

    /// 传播介质
    pub fn medium(&self) -> Vec<&'static str> {
        vec![
            "空气固体液体传声",
            "真空不能传声",
            "在水中速度较快",
            "实体传导",
        ]
    }

    /// 音调音量
    pub fn pitch(&self) -> Vec<&'static str> {
        vec![
            "振动快音调高",
            "振动慢音调低",
            "振幅大声音响",
            "音量取决于振幅",
        ]
    }

    /// 生活现象
    pub fn example(&self) -> Vec<&'static str> {
        vec!["回声是反射", "隔墙能闻声", "音色辨他人", "声音多样"]
    }
}

impl Rule for SoundWavePhysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("sound")
    }

    fn explain(&self) -> String {
        format!(
            "【声音波动原理】\n{}",
            [
                format!(
                    "振动发声：\\n{}",
                    self.vibration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "传播介质：\\n{}",
                    self.medium()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "音调音量：\\n{}",
                    self.pitch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活现象：\\n{}",
                    self.example()
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
    fn test_soundwavephysicsrules_basic() {
        let rules = SoundWavePhysicsRules::new();
        assert_eq!(rules.metadata().name, "声音波动原理");
        assert!(!rules.vibration().is_empty());
        assert!(!rules.medium().is_empty());
        assert!(!rules.pitch().is_empty());
        assert!(!rules.example().is_empty());
    }

    #[test]
    fn test_soundwavephysicsrules_validation() {
        let rules = SoundWavePhysicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("sound"));
    }

    #[test]
    fn test_soundwavephysicsrules_explain() {
        let rules = SoundWavePhysicsRules::new();
        let e = rules.explain();
        assert!(e.contains("振动发声"));
        assert!(e.contains("传播介质"));
        assert!(e.contains("音调音量"));
    }
}
