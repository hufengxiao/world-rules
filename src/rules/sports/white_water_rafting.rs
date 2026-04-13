//! 白水漂流规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 白水漂流规则 (White Water Rafting)
pub struct WhiteWaterRaftingRules {
    metadata: RuleMetadata,
}

impl WhiteWaterRaftingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "白水漂流规则",
                "白水漂流运动规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "技术比赛",
            "团队比赛",
            "障碍比赛",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "赛道规定",
            "出发规则",
            "终点判定",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "划桨技术",
            "转向技术",
            "控制技术",
            "过障技术",
            "团队配合",
        ]
    }

    /// 水流分级
    pub fn water_grades(&self) -> Vec<&'static str> {
        vec![
            "一级: 轻松水流",
            "二级: 初级难度",
            "三级: 中级难度",
            "四级: 高级难度",
            "五级: 专家难度",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "救生衣必须",
            "头盔佩戴",
            "安全培训",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "漂流艇",
            "划桨",
            "救生衣",
            "防护头盔",
            "附属装备",
        ]
    }

    /// 团队配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "团队人数: 4-8人",
            "舵手控制",
            "队员配合",
            "角色分配",
            "安全协调",
        ]
    }
}

impl Default for WhiteWaterRaftingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WhiteWaterRaftingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("white_water_rafting")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【白水漂流规则】\n\n\
            比赛类型:\n{}\n\n\
            技术动作:\n{}\n\n\
            水流分级:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.water_grades().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_water_rafting_rules() {
        let rules = WhiteWaterRaftingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}