//! 睡前仪式助眠
//!
//! 稳定睡前仪式、放松身心促进入睡的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BedtimeRitualRules,
    name: "睡前仪式助眠",
    desc: "稳定睡前仪式、放松身心促进入睡的规则",
    origin: "医学",
    tags: ["健康", "睡眠", "睡前", "仪式"]
}

impl BedtimeRitualRules {
    /// 固定时间
    pub fn schedule(&self) -> Vec<&'static str> {
        vec![
            "保持规律就寝时间",
            "避免熬夜打乱节律",
            "醒时定时起床",
            "周末也少颠倒",
        ]
    }

    /// 睡前准备
    pub fn prepare(&self) -> Vec<&'static str> {
        vec!["睡前一小时放松", "调暗灯光", "远离屏幕强光", "环境安静舒适"]
    }

    /// 仪式放松
    pub fn relax(&self) -> Vec<&'static str> {
        vec!["温水泡脚助眠", "轻度拉伸放松", "听舒缓音乐", "写写待办释念"]
    }

    /// 避免刺激
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "睡前不喝浓茶咖啡",
            "不剧烈运动",
            "不长时间看屏",
            "不过饱用餐",
        ]
    }
}

impl Rule for BedtimeRitualRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("bedtime")
    }

    fn explain(&self) -> String {
        format!(
            "【睡前仪式助眠】\n{}",
            [
                format!(
                    "固定时间：\\n{}",
                    self.schedule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "睡前准备：\\n{}",
                    self.prepare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "仪式放松：\\n{}",
                    self.relax()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避免刺激：\\n{}",
                    self.avoid()
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
    fn test_bedtimeritualrules_basic() {
        let rules = BedtimeRitualRules::new();
        assert_eq!(rules.metadata().name, "睡前仪式助眠");
        assert!(!rules.schedule().is_empty());
        assert!(!rules.prepare().is_empty());
        assert!(!rules.relax().is_empty());
        assert!(!rules.avoid().is_empty());
    }

    #[test]
    fn test_bedtimeritualrules_validation() {
        let rules = BedtimeRitualRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("bedtime"));
    }

    #[test]
    fn test_bedtimeritualrules_explain() {
        let rules = BedtimeRitualRules::new();
        let e = rules.explain();
        assert!(e.contains("固定时间"));
        assert!(e.contains("睡前准备"));
        assert!(e.contains("仪式放松"));
    }
}
