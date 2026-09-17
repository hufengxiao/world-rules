//! 观赛观众礼仪
//!
//! 现场观赛时的鼓掌、秩序与尊重对手观众的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SpectatorBehaviorRules,
    name: "观赛观众礼仪",
    desc: "现场观赛时的鼓掌、秩序与尊重对手观众的礼仪",
    origin: "国际",
    tags: ["体育", "观众", "观赛", "礼仪", "现场"]
}

impl SpectatorBehaviorRules {
    /// 观赛秩序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "按票入座有序观赛",
            "不喧哗影响他人观赛",
            "入场退场听从引导",
            "保住整洁不留垃圾",
        ]
    }

    /// 加油有度
    pub fn cheering(&self) -> Vec<&'static str> {
        vec![
            "加油鼓掌文明有度",
            "不辱骂对手与球员",
            "不朝场内投掷物品",
            "尊重每一支参赛队伍",
        ]
    }

    /// 文明互动
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "尊重客场球迷",
            "不挑衅引发冲突",
            "配合安保与现场管理",
            "遵守场馆观赛须知",
        ]
    }

    /// 特殊情况
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "比赛中断保持安静配合",
            "伤病球员出现给予尊重掌声",
            "尊重颁奖礼仪",
            "避免干扰判罚现场",
        ]
    }
}

impl Rule for SpectatorBehaviorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("spectator_behavior")
    }

    fn explain(&self) -> String {
        format!(
            "【观赛观众礼仪】\n{}",
            [
                format!(
                    "观赛秩序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "加油有度：\\n{}",
                    self.cheering()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "文明互动：\\n{}",
                    self.interaction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊情况：\\n{}",
                    self.special()
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
    fn test_spectatorbehaviorrules_basic() {
        let rules = SpectatorBehaviorRules::new();
        assert_eq!(rules.metadata().name, "观赛观众礼仪");
        assert!(!rules.order().is_empty());
        assert!(!rules.cheering().is_empty());
        assert!(!rules.interaction().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_spectatorbehaviorrules_validation() {
        let rules = SpectatorBehaviorRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("spectator_behavior"));
    }

    #[test]
    fn test_spectatorbehaviorrules_explain() {
        let rules = SpectatorBehaviorRules::new();
        let e = rules.explain();
        assert!(e.contains("观赛秩序"));
        assert!(e.contains("加油有度"));
        assert!(e.contains("文明互动"));
    }
}
