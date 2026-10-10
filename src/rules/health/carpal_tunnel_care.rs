//! 腕管综合征护理
//!
//! 手腕正中神经受压引起手指麻木的护理与预防

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CarpalTunnelCareRules,
    name: "腕管综合征护理",
    desc: "手腕正中神经受压引起手指麻木的护理与预防",
    origin: "中国",
    tags: ["骨骼", "神经", "劳损"]
}

impl CarpalTunnelCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "拇指食指中指麻木",
            "夜间麻醒需甩手",
            "握物无力易掉",
            "拇指根部肌肉萎缩",
        ]
    }

    /// 姿势调整
    pub fn posture(&self) -> Vec<&'static str> {
        vec![
            "键盘鼠标置于合适高度",
            "手腕保持中立位",
            "避免长时间屈腕",
            "使用腕托辅助支撑",
        ]
    }

    /// 休息锻炼
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "定时活动手腕手指",
            "手指屈伸放松练习",
            "神经滑动训练",
            "避免重复性手腕动作",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "麻木持续加重应就诊",
            "肌肉萎缩须及时就医",
            "夜间症状明显可评估",
            "必要时遵医嘱治疗",
        ]
    }
}

impl Rule for CarpalTunnelCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("carpal_tunnel_care")
    }

    fn explain(&self) -> String {
        format!(
            "【腕管综合征护理】\n{}",
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
                    "姿势调整：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "休息锻炼：\\n{}",
                    self.exercise()
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
    fn test_carpaltunnelcarerules_basic() {
        let rules = CarpalTunnelCareRules::new();
        assert_eq!(rules.metadata().name, "腕管综合征护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.posture().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_carpaltunnelcarerules_validation() {
        let rules = CarpalTunnelCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("carpal_tunnel_care"));
    }

    #[test]
    fn test_carpaltunnelcarerules_explain() {
        let rules = CarpalTunnelCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("姿势调整"));
        assert!(e.contains("休息锻炼"));
    }
}
