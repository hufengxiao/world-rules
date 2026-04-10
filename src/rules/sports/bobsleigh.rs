//! 雪车规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 雪车规则
pub struct BobsleighRules {
    metadata: RuleMetadata,
}

impl BobsleighRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "雪车规则",
                "雪车比赛基本规则"
            )
            .with_origin("瑞士")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "男子双人雪车",
            "女子双人雪车",
            "男子四人雪车",
            "女子单人雪车",
            "男子单人雪车",
        ]
    }

    /// 雪车规格
    pub fn sled_specifications(&self) -> Vec<&'static str> {
        vec![
            "双人雪车: 最大390公斤(含选手)",
            "四人雪车: 最大630公斤(含选手)",
            "长度限制",
            "宽度限制",
            "材料规定",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "滑行4次",
            "总时间最短获胜",
            "起跑推车",
            "驾驶员控制方向",
            "刹车员刹车",
        ]
    }

    /// 赛道规格
    pub fn track_specifications(&self) -> Vec<&'static str> {
        vec![
            "赛道长度: 1200-1650米",
            "弯道数量: 12-18个",
            "落差: 100-150米",
            "冰面温度控制",
            "安全设施",
        ]
    }

    /// 队员分工
    pub fn team_roles(&self) -> Vec<&'static str> {
        vec![
            "驾驶员: 控制方向",
            "刹车员: 终点刹车",
            "推车员: 起跑推进",
            "团队配合",
            "体重分配",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须佩戴",
            "护具要求",
            "赛道安全检查",
            "天气条件监控",
            "医疗支持",
        ]
    }

    /// 计时规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "精确到0.01秒",
            "起跑计时",
            "终点计时",
            "成绩公布",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超重",
            "违规起跑",
            "超出赛道",
            "设备违规",
            "安全违规",
        ]
    }
}

impl Default for BobsleighRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BobsleighRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bobsleigh")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【雪车规则】\n\n\
            比赛项目:\n{}\n\n\
            比赛规则:\n{}\n\n\
            队员分工:\n{}\n\n\
            安全规则:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.team_roles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bobsleigh_rules() {
        let rules = BobsleighRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}