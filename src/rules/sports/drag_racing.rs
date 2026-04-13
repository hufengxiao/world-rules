//! 直线加速赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 直线加速赛规则 (Drag Racing)
pub struct DragRacingRules {
    metadata: RuleMetadata,
}

impl DragRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "直线加速赛规则",
                "直线加速赛比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "赛车".into()]),
        }
    }

    /// 比赛分类
    pub fn competition_classes(&self) -> Vec<&'static str> {
        vec![
            " Top Fuel级别",
            " Funny Car级别",
            " Pro Stock级别",
            "摩托车级别",
            "其他级别",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛距离: 1/4英里或1/8英里",
            "直线跑道",
            "出发信号",
            "计时规则",
            "终点判定",
        ]
    }

    /// 技术规定
    pub fn technical_rules(&self) -> Vec<&'static str> {
        vec![
            "发动机规格",
            "燃料规定",
            "重量限制",
            "安全改装",
            "技术检查",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "防护头盔必须",
            "安全服装",
            "安全系统",
            "医疗支持",
            "救援准备",
        ]
    }

    /// 计时规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "反应时间",
            "完成时间",
            "终点速度",
            "精确计时",
            "成绩记录",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "直线加速赛车",
            "防护头盔",
            "比赛服装",
            "手套护具",
            "安全装备",
        ]
    }

    /// 犯规规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "提前出发",
            "车道违规",
            "技术违规",
            "犯规处罚",
            "取消资格",
        ]
    }
}

impl Default for DragRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DragRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("drag_racing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【直线加速赛规则】\n\n\
            比赛分类:\n{}\n\n\
            技术规定:\n{}\n\n\
            计时规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.timing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_racing_rules() {
        let rules = DragRacingRules::new();
        assert!(!rules.competition_classes().is_empty());
    }
}