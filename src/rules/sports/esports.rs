//! 电子竞技规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电子竞技规则
pub struct EsportsRules {
    metadata: RuleMetadata,
}

impl EsportsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电子竞技规则",
                "电子竞技比赛规则"
            )
            .with_origin("国际")
            .with_tags(vec!["体育".into(), "电子".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            " MOBA游戏",
            " FPS游戏",
            " RTS游戏",
            "格斗游戏",
            "体育模拟游戏",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "比赛制度",
            "淘汰规则",
            "比赛结束",
            "技术规定",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "队伍人数规定",
            "替补规则",
            "队员身份",
            "教练配置",
            "选手认证",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "违规操作",
            "作弊行为",
            "不当言论",
            "技术犯规",
            "处罚规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "游戏设备",
            "电脑配置",
            "网络连接",
            "比赛服装",
            "防护装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "网络安全",
            "设备安全",
            "选手健康",
            "比赛公平",
            "应急处理",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "胜负判定",
            "积分系统",
            "排名规则",
            "比赛统计",
            "奖金分配",
        ]
    }
}

impl Default for EsportsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EsportsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("esports")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电子竞技规则】\n\n\
            比赛类型:\n{}\n\n\
            犯规规则:\n{}\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esports_rules() {
        let rules = EsportsRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}