//! 空气曲棍球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 空气曲棍球规则 (Air Hockey)
pub struct AirHockeyRules {
    metadata: RuleMetadata,
}

impl AirHockeyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "空气曲棍球规则",
                "空气曲棍球比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用7分制",
            "比赛时间限制",
            "轮流发球",
            "得分规则",
            "比赛结束",
        ]
    }

    /// 球台规格
    pub fn table_specifications(&self) -> Vec<&'static str> {
        vec![
            "球台尺寸: 83×40英寸",
            "球门宽度: 3.75英寸",
            "气流系统",
            "表面设计",
            "球台质量",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "击球技术",
            "防守技术",
            "发球技术",
            "控制技术",
            "战术运用",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "触球犯规",
            "越过中线",
            "阻挡犯规",
            "超时犯规",
            "犯规处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "空气曲棍球台",
            "击球器",
            "比赛用球",
            "计分板",
            "比赛服装",
        ]
    }

    /// 球的规格
    pub fn puck_specifications(&self) -> Vec<&'static str> {
        vec![
            "球直径: 3.25英寸",
            "球重量",
            "材质要求",
            "滑动性能",
            "规格认证",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "禁止危险动作",
            "保持距离",
            "安全操作",
            "场地安全",
            "比赛控制",
        ]
    }
}

impl Default for AirHockeyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AirHockeyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("air_hockey")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【空气曲棍球规则】\n\n\
            球台规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.table_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_air_hockey_rules() {
        let rules = AirHockeyRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}