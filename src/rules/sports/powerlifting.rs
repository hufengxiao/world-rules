//! 力举规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 力举规则
pub struct PowerliftingRules {
    metadata: RuleMetadata,
}

impl PowerliftingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "力举规则",
                "力举比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "力量".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "深蹲",
            "卧推",
            "硬拉",
            "三项综合",
            "单项比赛",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子: 从52公斤到120公斤以上",
            "女子: 从44公斤到84公斤以上",
            "体重间隔约10公斤",
            "赛前称重",
            "体重限制",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "每人3次试举",
            "取最好成绩",
            "三项成绩相加",
            "总成绩排名",
            "犯规取消",
        ]
    }

    /// 深蹲规则
    pub fn squat_rules(&self) -> Vec<&'static str> {
        vec![
            "臀部低于膝盖",
            "完成深度要求",
            "站立姿势",
            "裁判信号",
            "违规判定",
        ]
    }

    /// 卧推规则
    pub fn bench_press_rules(&self) -> Vec<&'static str> {
        vec![
            "背部接触凳面",
            "双脚着地",
            "暂停信号",
            "推举完成",
            "违规判定",
        ]
    }

    /// 硬拉规则
    pub fn deadlift_rules(&self) -> Vec<&'static str> {
        vec![
            "杠铃离地",
            "双腿伸直",
            "身体直立",
            "锁定姿势",
            "放下控制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "举重腰带",
            "举重鞋",
            "护膝护腕",
            "举重服",
            "无装备比赛",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "深度不够",
            "姿势不完整",
            "犯规动作",
            "超时",
            "重量违规",
        ]
    }
}

impl Default for PowerliftingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PowerliftingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("powerlifting")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【力举规则】\n\n\
            比赛项目:\n{}\n\n\
            深蹲规则:\n{}\n\n\
            卧推规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.squat_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.bench_press_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerlifting_rules() {
        let rules = PowerliftingRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}