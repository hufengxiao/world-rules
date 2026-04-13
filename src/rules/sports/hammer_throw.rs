//! 链球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 链球规则
pub struct HammerThrowRules {
    metadata: RuleMetadata,
}

impl HammerThrowRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "链球规则",
                "田径链球投掷规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "每人3次试投",
            "取最好成绩",
            "决赛8人",
            "成绩测量",
            "犯规判定",
        ]
    }

    /// 投掷规则
    pub fn throwing_rules(&self) -> Vec<&'static str> {
        vec![
            "旋转投掷",
            "投掷圈直径: 2.135米",
            "出手角度",
            "落地判定",
            "有效区域",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "握持技术",
            "预备姿势",
            "旋转技术",
            "出手技术",
            "跟进动作",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "链球重量: 7.26公斤(男)",
            "链球重量: 4公斤(女)",
            "链球长度",
            "投掷手套",
            "比赛场地",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全围栏",
            "观众距离",
            "裁判监督",
            "场地检查",
            "医疗支持",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "踩出投掷圈",
            "链球落出扇形区",
            "违规动作",
            "超时犯规",
            "装备违规",
        ]
    }

    /// 成绩记录
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "精确到厘米",
            "取最佳成绩",
            "排名规则",
            "同成绩处理",
            "记录标准",
        ]
    }
}

impl Default for HammerThrowRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HammerThrowRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hammer_throw")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【链球规则】\n\n\
            投掷规则:\n{}\n\n\
            技术动作:\n{}\n\n\
            装备要求:\n{}\n\n\
            犯规规则:\n{}\n",
            self.throwing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hammer_throw_rules() {
        let rules = HammerThrowRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}