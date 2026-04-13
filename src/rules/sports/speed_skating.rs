//! 速度滑冰规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 速度滑冰规则
pub struct SpeedSkatingRules {
    metadata: RuleMetadata,
}

impl SpeedSkatingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "速度滑冰规则",
                "速度滑冰比赛规则"
            )
            .with_origin("荷兰")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "500米",
            "1000米",
            "1500米",
            "5000米",
            "10000米",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "双跑道比赛",
            "内道外道交替",
            "换道规则",
            "出发规则",
            "终点判定",
        ]
    }

    /// 滑冰技术
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "起跑技术",
            "滑行技术",
            "弯道技术",
            "换道技术",
            "终点冲刺",
        ]
    }

    /// 场地规格
    pub fn track_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准跑道: 400米",
            "跑道宽度",
            "内道外道",
            "换道区域",
            "冰面质量",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "速滑冰刀",
            "比赛服装",
            "防护装备",
            "头盔",
            "冰鞋",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "冰面安全",
            "换道安全",
            "摔倒处理",
            "医疗支持",
            "比赛控制",
        ]
    }

    /// 计时规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "手动备用",
            "精确到百分秒",
            "成绩记录",
            "同分处理",
        ]
    }
}

impl Default for SpeedSkatingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SpeedSkatingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("speed_skating")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【速度滑冰规则】\n\n\
            比赛项目:\n{}\n\n\
            滑冰技术:\n{}\n\n\
            场地规格:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.track_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_skating_rules() {
        let rules = SpeedSkatingRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}