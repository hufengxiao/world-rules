//! 斯诺克详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 斯诺克详细规则
pub struct SnookerDetailedRules {
    metadata: RuleMetadata,
}

impl SnookerDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "斯诺克详细规则",
                "斯诺克台球详细比赛规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "台球".into()]),
        }
    }

    /// 球台规格
    pub fn table_specifications(&self) -> Vec<&'static str> {
        vec![
            "球台尺寸: 12×6英尺",
            "袋口尺寸",
            "球台高度",
            "台面要求",
            "边框规格",
        ]
    }

    /// 球的配置
    pub fn ball_setup(&self) -> Vec<&'static str> {
        vec![
            "15颗红球: 每颗1分",
            "黄球: 2分",
            "绿球: 3分",
            "棕球: 4分",
            "蓝球: 5分",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "红球: 1分",
            "彩球按颜色得分",
            "最高单杆147分",
            "犯规罚分",
            "比分记录",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用局数制",
            "红球彩球交替击打",
            "清台规则",
            "犯规规则",
            "比赛控制",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "击球技术",
            "防守技术",
            "斯诺克技术",
            "解球技术",
            "精确控制",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "最少罚分4分",
            "击打错误球",
            "球台犯规",
            "超时犯规",
            "犯规处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "斯诺克球杆",
            "比赛用球",
            "球台装备",
            "架桥装备",
            "比赛服装",
        ]
    }
}

impl Default for SnookerDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SnookerDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("snooker_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【斯诺克详细规则】\n\n\
            球的配置:\n{}\n\n\
            得分规则:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规规则:\n{}\n",
            self.ball_setup().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snooker_detailed_rules() {
        let rules = SnookerDetailedRules::new();
        assert!(!rules.ball_setup().is_empty());
    }
}