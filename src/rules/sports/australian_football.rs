//! 澳式足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 澳式足球规则
pub struct AustralianFootballRules {
    metadata: RuleMetadata,
}

impl AustralianFootballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "澳式足球规则",
                "澳式足球比赛规则"
            )
            .with_origin("澳大利亚")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4节",
            "每节20分钟",
            "有效时间制",
            "节间休息",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "椭圆形场地",
            "长度: 135-185米",
            "宽度: 110-155米",
            "球门柱: 4根",
            "得分区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队18人上场",
            "4名替补",
            "场上位置",
            "换人规则",
            "教练指导",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "踢球技术",
            "手传球",
            "标记技术",
            "奔跑技术",
            "防守技术",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "进球: 6分",
            "射门得分: 1分",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "非法冲撞",
            "持球违规",
            "阻挡犯规",
            "危险动作",
            "犯规处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "澳式足球",
            "比赛服装",
            "防护装备",
            "球鞋",
            "无护具要求",
        ]
    }
}

impl Default for AustralianFootballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AustralianFootballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("australian_football")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【澳式足球规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_australian_football_rules() {
        let rules = AustralianFootballRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}