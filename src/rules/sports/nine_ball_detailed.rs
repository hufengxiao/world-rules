//! 九球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 九球详细规则
pub struct NineBallDetailedRules {
    metadata: RuleMetadata,
}

impl NineBallDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "九球详细规则",
                "九球台球详细比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "台球".into()]),
        }
    }

    /// 球台规格
    pub fn table_specifications(&self) -> Vec<&'static str> {
        vec![
            "球台尺寸: 9×4.5英尺",
            "袋口尺寸",
            "球台高度",
            "台面要求",
            "边框规格",
        ]
    }

    /// 球的配置
    pub fn ball_setup(&self) -> Vec<&'static str> {
        vec![
            "1-9号球",
            "菱形摆放",
            "9球在中间",
            "1球在前端",
            "其他球随机",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "必须按顺序击打",
            "最低球先击",
            "9球可直接获胜",
            "犯规规则",
            "比赛控制",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "击球技术",
            "组合球技术",
            "防守技术",
            "开球技术",
            "精确控制",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "击打错误球",
            "球未碰库边",
            "球台犯规",
            "超时犯规",
            "犯规处罚",
        ]
    }

    /// 开球规则
    pub fn break_rules(&self) -> Vec<&'static str> {
        vec![
            "开球位置",
            "开球要求",
            "开球进球",
            "开球犯规",
            "开球控制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "九球球杆",
            "比赛用球",
            "球台装备",
            "架桥装备",
            "比赛服装",
        ]
    }
}

impl Default for NineBallDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NineBallDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nine_ball_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【九球详细规则】\n\n\
            球的配置:\n{}\n\n\
            比赛规则:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规规则:\n{}\n",
            self.ball_setup().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nine_ball_detailed_rules() {
        let rules = NineBallDetailedRules::new();
        assert!(!rules.ball_setup().is_empty());
    }
}