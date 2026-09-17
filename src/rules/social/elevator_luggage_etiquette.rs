//! 电梯与行李礼仪
//!
//! 乘电梯与携带行李时的礼让与有序礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElevatorLuggageEtiquetteRules,
    name: "电梯与行李礼仪",
    desc: "乘电梯与携带行李时的礼让与有序礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "电梯", "行李", "礼让"]
}

impl ElevatorLuggageEtiquetteRules {
    /// 进出电梯
    pub fn boarding(&self) -> Vec<&'static str> {
        vec![
            "先下后上按次序进出",
            "不拥挤抢门禁",
            "靠近门口者先出",
            "让有急事者先行",
        ]
    }

    /// 电梯行为
    pub fn inside(&self) -> Vec<&'static str> {
        vec![
            "压低音量轻交谈",
            "不霸占按键区",
            "手机保持安静",
            "不大声嬉闹",
        ]
    }

    /// 行李物品
    pub fn luggage(&self) -> Vec<&'static str> {
        vec![
            "行李靠边不占通道",
            "大件尽量避开高峰",
            "推车进出注意他人",
            "不使行李碰触他人",
        ]
    }

    /// 体谅礼让
    pub fn consider(&self) -> Vec<&'static str> {
        vec![
            "主动为长者扶门",
            "助有困难者按层",
            "避免站位挡出口",
            "保持空间不挤他人",
        ]
    }
}

impl Rule for ElevatorLuggageEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("elevator_luggage")
    }

    fn explain(&self) -> String {
        format!(
            "【电梯与行李礼仪】\n{}",
            [
                format!(
                    "进出电梯：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "电梯行为：\\n{}",
                    self.inside()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行李物品：\\n{}",
                    self.luggage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体谅礼让：\\n{}",
                    self.consider()
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
    fn test_elevatorluggageetiquetterules_basic() {
        let rules = ElevatorLuggageEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "电梯与行李礼仪");
        assert!(!rules.boarding().is_empty());
        assert!(!rules.inside().is_empty());
        assert!(!rules.luggage().is_empty());
        assert!(!rules.consider().is_empty());
    }

    #[test]
    fn test_elevatorluggageetiquetterules_validation() {
        let rules = ElevatorLuggageEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("elevator_luggage"));
    }

    #[test]
    fn test_elevatorluggageetiquetterules_explain() {
        let rules = ElevatorLuggageEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("进出电梯"));
        assert!(e.contains("电梯行为"));
        assert!(e.contains("行李物品"));
    }
}
