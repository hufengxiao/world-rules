//! 肩颈拉伸放松
//!
//! 久坐办公肩颈僵硬的拉伸与放松运动

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NeckShoulderStretchRules,
    name: "肩颈拉伸放松",
    desc: "久坐办公肩颈僵硬的拉伸与放松运动",
    origin: "医学",
    tags: ["健康", "肩颈", "拉伸", "舒缓"]
}

impl NeckShoulderStretchRules {
    /// 拉伸热身
    pub fn warmup(&self) -> Vec<&'static str> {
        vec!["先轻柔活动肩颈", "微微左右转颈", "耸肩放松", "避免猛劲硬拉"]
    }

    /// 舒缓动作
    pub fn stretch(&self) -> Vec<&'static str> {
        vec!["缓慢转头至觉拉伸", "侧头轻拉", "转肩绕环", "保持缓慢深呼"]
    }

    /// 办公间隙
    pub fn breaks(&self) -> Vec<&'static str> {
        vec!["每隔一段起身活动", "改变坐姿缓解", "抬头远望", "避免久低头"]
    }

    /// 注意信号
    pub fn care(&self) -> Vec<&'static str> {
        vec!["无力麻木立即停", "痛感强烈勿强拉", "继不适就医", "保养颈肩"]
    }
}

impl Rule for NeckShoulderStretchRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("neck_stretch")
    }

    fn explain(&self) -> String {
        format!(
            "【肩颈拉伸放松】\n{}",
            [
                format!(
                    "拉伸热身：\\n{}",
                    self.warmup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "舒缓动作：\\n{}",
                    self.stretch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "办公间隙：\\n{}",
                    self.breaks()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意信号：\\n{}",
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
    fn test_neckshoulderstretchrules_basic() {
        let rules = NeckShoulderStretchRules::new();
        assert_eq!(rules.metadata().name, "肩颈拉伸放松");
        assert!(!rules.warmup().is_empty());
        assert!(!rules.stretch().is_empty());
        assert!(!rules.breaks().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_neckshoulderstretchrules_validation() {
        let rules = NeckShoulderStretchRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("neck_stretch"));
    }

    #[test]
    fn test_neckshoulderstretchrules_explain() {
        let rules = NeckShoulderStretchRules::new();
        let e = rules.explain();
        assert!(e.contains("拉伸热身"));
        assert!(e.contains("舒缓动作"));
        assert!(e.contains("办公间隙"));
    }
}
