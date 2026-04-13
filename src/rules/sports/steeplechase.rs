//! 障碍跑规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 障碍跑规则 (Steeplechase)
pub struct SteeplechaseRules {
    metadata: RuleMetadata,
}

impl SteeplechaseRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "障碍跑规则",
                "田径障碍跑比赛规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "3000米障碍跑",
            "2000米障碍跑",
            "男子标准",
            "女子标准",
            "青少年项目",
        ]
    }

    /// 障碍设置
    pub fn barrier_setup(&self) -> Vec<&'static str> {
        vec![
            "障碍数量: 28个",
            "水坑障碍: 7个",
            "障碍高度: 0.914米(男)",
            "障碍高度: 0.762米(女)",
            "障碍间距",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "跨越障碍",
            "水坑规则",
            "犯规判定",
            "出发规则",
            "终点判定",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "跨越技术",
            "水坑技术",
            "跑步技术",
            "节奏控制",
            "落地技术",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "绕过障碍",
            "踩踏障碍",
            "推挤犯规",
            "干扰对手",
            "违规穿越",
        ]
    }

    /// 场地要求
    pub fn track_requirements(&self) -> Vec<&'static str> {
        vec![
            "标准跑道",
            "水坑设计",
            "障碍位置",
            "安全设施",
            "医疗支持",
        ]
    }

    /// 计时规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "精确到百分秒",
            "成绩记录",
            "排名规则",
            "同时间处理",
        ]
    }
}

impl Default for SteeplechaseRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SteeplechaseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("steeplechase")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【障碍跑规则】\n\n\
            障碍设置:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规规则:\n{}\n\n\
            场地要求:\n{}\n",
            self.barrier_setup().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.track_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steeplechase_rules() {
        let rules = SteeplechaseRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}