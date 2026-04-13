//! 拉力赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 拉力赛规则
pub struct RallyRacingRules {
    metadata: RuleMetadata,
}

impl RallyRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "拉力赛规则",
                "汽车拉力赛比赛规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "赛车".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "世界拉力锦标赛(WRC)",
            "区域拉力赛",
            "越野拉力赛",
            "场地拉力赛",
            "耐力拉力赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "分赛段比赛",
            "时间累计",
            "出发顺序",
            "赛段计时",
            "总成绩计算",
        ]
    }

    /// 技术规定
    pub fn technical_rules(&self) -> Vec<&'static str> {
        vec![
            "车辆级别规定",
            "发动机规格",
            "安全改装",
            "车辆检查",
            "技术认证",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全装备必须",
            "防护结构",
            "医疗支持",
            "救援准备",
            "红旗规则",
        ]
    }

    /// 积分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "第一名: 25分",
            "第二名: 18分",
            "第三名: 15分",
            "前10名得分",
            "年度积分",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拉力赛车",
            "防护头盔",
            "安全服装",
            "手套护具",
            "导航设备",
        ]
    }

    /// 犯规规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "超时罚分",
            "技术违规",
            "危险驾驶",
            "罚时规则",
            "取消资格",
        ]
    }
}

impl Default for RallyRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RallyRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rally_racing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【拉力赛规则】\n\n\
            比赛类型:\n{}\n\n\
            技术规定:\n{}\n\n\
            积分系统:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rally_racing_rules() {
        let rules = RallyRacingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}