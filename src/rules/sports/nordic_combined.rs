//! 北欧两项规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 北欧两项规则
pub struct NordicCombinedRules {
    metadata: RuleMetadata,
}

impl NordicCombinedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "北欧两项规则",
                "北欧两项比赛基本规则"
            )
            .with_origin("挪威")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "标准台+10公里",
            "大台+10公里",
            "团体赛",
            "世界杯系列",
            "奥运会项目",
        ]
    }

    /// 跳台滑雪规则
    pub fn ski_jumping_rules(&self) -> Vec<&'static str> {
        vec![
            "每轮1-2次跳跃",
            "距离和姿态评分",
            "积分制",
            "决定越野出发顺序",
            "风况修正",
        ]
    }

    /// 越野滑雪规则
    pub fn cross_country_rules(&self) -> Vec<&'static str> {
        vec![
            "间隔出发",
            "出发顺序由跳台成绩决定",
            "先到终点获胜",
            "自由技术",
            "分段计时",
        ]
    }

    /// 积分转换
    pub fn points_conversion(&self) -> Vec<&'static str> {
        vec![
            "跳台分数转换时间差",
            "每分差多少秒",
            "决定出发间隔",
            "差距追击赛制",
            "公平竞赛原则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "跳台滑雪板: 长度限制",
            "越野滑雪板: 较短",
            "两种固定器",
            "滑雪靴: 两种",
            "头盔: 跳台必须",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "跳台比赛在前",
            "越野比赛在后",
            "两项综合成绩",
            "总时间最短获胜",
            "换装时间计入",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须佩戴(跳台)",
            "场地安全检查",
            "天气条件监控",
            "医疗支持",
            "训练要求",
        ]
    }

    /// 技术要求
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "跳台技术: 飞行姿势",
            "越野技术: 滑行效率",
            "转换能力",
            "耐力和技术结合",
            "全面训练",
        ]
    }
}

impl Default for NordicCombinedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NordicCombinedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nordic_combined")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【北欧两项规则】\n\n\
            比赛项目:\n{}\n\n\
            积分转换:\n{}\n\n\
            比赛规则:\n{}\n\n\
            技术要求:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.points_conversion().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nordic_combined_rules() {
        let rules = NordicCombinedRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}