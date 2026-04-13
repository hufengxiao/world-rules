//! 滑翔伞规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 滑翔伞规则
pub struct ParaglidingRules {
    metadata: RuleMetadata,
}

impl ParaglidingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "滑翔伞规则",
                "滑翔伞比赛基本规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "航空".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "距离比赛",
            "定点比赛",
            "自由飞行",
            "越野飞行",
        ]
    }

    /// 竞速规则
    pub fn speed_race_rules(&self) -> Vec<&'static str> {
        vec![
            "指定路线",
            "转弯点通过",
            "GPS记录",
            "速度计算",
            "最快者获胜",
        ]
    }

    /// 定点规则
    pub fn accuracy_rules(&self) -> Vec<&'static str> {
        vec![
            "定点降落",
            "目标圈测量",
            "精确到厘米",
            "距离目标计算",
            "多次尝试",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑翔伞翼",
            "座袋",
            "头盔: 必须佩戴",
            "高度仪表",
            "GPS设备",
        ]
    }

    /// 气象要求
    pub fn weather_requirements(&self) -> Vec<&'static str> {
        vec![
            "风力限制",
            "风向监测",
            "云层状况",
            "温度影响",
            "天气预警",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须佩戴",
            "训练认证要求",
            "飞行计划",
            "通讯设备",
            "紧急预案",
        ]
    }

    /// 认证等级
    pub fn certification_levels(&self) -> Vec<&'static str> {
        vec![
            "初级飞行员",
            "中级飞行员",
            "高级飞行员",
            "教练等级",
            "竞赛等级",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "危险飞行",
            "违规进入禁飞区",
            "无认证飞行",
            "恶劣天气飞行",
            "违规比赛行为",
        ]
    }
}

impl Default for ParaglidingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaglidingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("paragliding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【滑翔伞规则】\n\n\
            比赛形式:\n{}\n\n\
            安全规则:\n{}\n\n\
            认证等级:\n{}\n\n\
            禁止行为:\n{}\n",
            self.competition_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.certification_levels().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragliding_rules() {
        let rules = ParaglidingRules::new();
        assert!(!rules.competition_formats().is_empty());
    }
}