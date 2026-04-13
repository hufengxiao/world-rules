//! 摩托艇规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 摩托艇规则
pub struct MotorboatRacingRules {
    metadata: RuleMetadata,
}

impl MotorboatRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "摩托艇规则",
                "摩托艇比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "绕标比赛",
            "耐力赛",
            "拉力赛",
            "自由表演",
        ]
    }

    /// 船艇分类
    pub fn boat_classes(&self) -> Vec<&'static str> {
        vec![
            "F1摩托艇",
            "F2摩托艇",
            "F3摩托艇",
            "近海赛艇",
            "喷射艇",
        ]
    }

    /// 赛道规格
    pub fn course_specifications(&self) -> Vec<&'static str> {
        vec![
            "水上赛道",
            "浮标标记",
            "安全区域",
            "观众距离",
            "转弯点设置",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "救生衣必须",
            "头盔必须",
            "安全艇在场",
            "医疗支持",
            "天气条件监控",
        ]
    }

    /// 起航规则
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "统一起航",
            "信号灯控制",
            "起航线",
            "抢航处罚",
            "延时起航",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "抢航",
            "超出赛道",
            "危险驾驶",
            "碰撞他人",
            "违规超车",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "摩托艇",
            "救生衣",
            "头盔",
            "通讯设备",
            "安全装备",
        ]
    }

    /// 驾驶员要求
    pub fn pilot_requirements(&self) -> Vec<&'static str> {
        vec![
            "驾照要求",
            "体检合格",
            "训练认证",
            "保险要求",
            "经验要求",
        ]
    }
}

impl Default for MotorboatRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MotorboatRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("motorboat_racing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【摩托艇规则】\n\n\
            船艇分类:\n{}\n\n\
            安全规则:\n{}\n\n\
            犯规规则:\n{}\n\n\
            驾驶员要求:\n{}\n",
            self.boat_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pilot_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motorboat_racing_rules() {
        let rules = MotorboatRacingRules::new();
        assert!(!rules.competition_formats().is_empty());
    }
}