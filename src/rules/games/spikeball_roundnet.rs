//! 圆网球
//!
//! 围绕弹性网面拍击球的四人对抗运动游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SpikeballRoundnetRules,
    name: "圆网球",
    desc: "围绕弹性网面拍击球的四人对抗运动游戏",
    origin: "美国",
    tags: ["户外", "球类", "对抗"]
}

impl SpikeballRoundnetRules {
    /// 场地装备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "圆形弹性网架",
            "小号弹球一只",
            "四人两队各两人",
            "围网均分站位",
        ]
    }

    /// 发球规则
    pub fn serving(&self) -> Vec<&'static str> {
        vec![
            "发球者距网规定距离",
            "将球击向网面",
            "球须弹起且不过高",
            "接方有三次触球机会",
        ]
    }

    /// 回合对抗
    pub fn rally(&self) -> Vec<&'static str> {
        vec![
            "接方三次触球内回击",
            "球须击向网面反弹",
            "球落地或出界即失分",
            "可旋转方向组织进攻",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "先达二十一分者胜",
            "须领先两分方胜",
            "失分方下轮发球",
            "两队交换发球权",
        ]
    }
}

impl Rule for SpikeballRoundnetRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("spikeball_roundnet")
    }

    fn explain(&self) -> String {
        format!(
            "【圆网球】\n{}",
            [
                format!(
                    "场地装备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发球规则：\\n{}",
                    self.serving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合对抗：\\n{}",
                    self.rally()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_spikeballroundnetrules_basic() {
        let rules = SpikeballRoundnetRules::new();
        assert_eq!(rules.metadata().name, "圆网球");
        assert!(!rules.setup().is_empty());
        assert!(!rules.serving().is_empty());
        assert!(!rules.rally().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_spikeballroundnetrules_validation() {
        let rules = SpikeballRoundnetRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("spikeball_roundnet"));
    }

    #[test]
    fn test_spikeballroundnetrules_explain() {
        let rules = SpikeballRoundnetRules::new();
        let e = rules.explain();
        assert!(e.contains("场地装备"));
        assert!(e.contains("发球规则"));
        assert!(e.contains("回合对抗"));
    }
}
