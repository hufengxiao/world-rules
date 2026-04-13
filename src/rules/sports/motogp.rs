//! MotoGP赛车规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// MotoGP赛车规则
pub struct MotoGPRules {
    metadata: RuleMetadata,
}

impl MotoGPRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "MotoGP赛车规则",
                " MotoGP摩托车比赛规则"
            )
            .with_origin("国际")
            .with_tags(vec!["体育".into(), "赛车".into()]),
        }
    }

    /// 比赛分类
    pub fn competition_classes(&self) -> Vec<&'static str> {
        vec![
            " MotoGP级别",
            " Moto2级别",
            " Moto3级别",
            "电动级别",
            "级别规定",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛距离",
            "圈数规定",
            "时间限制",
            "出发规则",
            "终点判定",
        ]
    }

    /// 技术规定
    pub fn technical_rules(&self) -> Vec<&'static str> {
        vec![
            "发动机容量限制",
            " MotoGP: 1000cc",
            " Moto2: 765cc",
            " Moto3: 250cc",
            "重量限制",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "防护头盔必须",
            "防护服装",
            "安全车规则",
            "红旗规则",
            "医疗支持",
        ]
    }

    /// 积分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "第一名: 25分",
            "第二名: 20分",
            "第三名: 16分",
            "前15名得分",
            "年度积分",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛摩托车",
            "防护头盔",
            "比赛服装",
            "手套护具",
            "靴子",
        ]
    }

    /// 犯规规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "危险驾驶",
            "非法超车",
            "犯规处罚",
            "罚时规则",
            "取消资格",
        ]
    }
}

impl Default for MotoGPRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MotoGPRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("motogp")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【 MotoGP赛车规则】\n\n\
            比赛分类:\n{}\n\n\
            技术规定:\n{}\n\n\
            积分系统:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_motogp_rules() {
        let rules = MotoGPRules::new();
        assert!(!rules.competition_classes().is_empty());
    }
}