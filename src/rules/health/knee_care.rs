//! 膝关节保养
//!
//! 减少膝关节磨损、护膝与运动保护的健康规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KneeCareRules,
    name: "膝关节保养",
    desc: "减少膝关节磨损、护膝与运动保护的健康规则",
    origin: "国际",
    tags: ["健康", "膝盖", "关节", "护膝", "运动"]
}

impl KneeCareRules {
    /// 控制负荷
    pub fn load(&self) -> Vec<&'static str> {
        vec![
            "减少持续的上下台阶与深蹲过度",
            "控制体重减轻膝负担",
            "避免长时间跪姿或蹲姿",
            "选择对膝友好的运动",
        ]
    }

    /// 腿部力量
    pub fn strength(&self) -> Vec<&'static str> {
        vec![
            "强化股四头肌与腿后肌",
            "进行股四头等长训练",
            "平衡练习稳固关节",
            "循序渐进不过度",
        ]
    }

    /// 运动防护
    pub fn protection(&self) -> Vec<&'static str> {
        vec![
            "运动前充分热身",
            "剧烈运动必要时护膝",
            "落地缓冲保护膝盖",
            "出现疼痛及时停下来休",
        ]
    }

    /// 就医提示
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "持续肿胀或晨僵就医",
            "活动时卡顿或弹响明显检查",
            "老人膝痛影响活动需评估",
            "避免自行依赖药物止痛",
        ]
    }
}

impl Rule for KneeCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("knee_care")
    }

    fn explain(&self) -> String {
        format!(
            "【膝关节保养】\n{}",
            [
                format!(
                    "控制负荷：\\n{}",
                    self.load()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "腿部力量：\\n{}",
                    self.strength()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动防护：\\n{}",
                    self.protection()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
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
    fn test_kneecarerules_basic() {
        let rules = KneeCareRules::new();
        assert_eq!(rules.metadata().name, "膝关节保养");
        assert!(!rules.load().is_empty());
        assert!(!rules.strength().is_empty());
        assert!(!rules.protection().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_kneecarerules_validation() {
        let rules = KneeCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("knee_care"));
    }

    #[test]
    fn test_kneecarerules_explain() {
        let rules = KneeCareRules::new();
        let e = rules.explain();
        assert!(e.contains("控制负荷"));
        assert!(e.contains("腿部力量"));
        assert!(e.contains("运动防护"));
    }
}
