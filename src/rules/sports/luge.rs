//! 雪橇规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 雪橇规则 (Luge)
pub struct LugeRules {
    metadata: RuleMetadata,
}

impl LugeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "雪橇规则",
                "雪橇竞速规则"
            )
            .with_origin("瑞士")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "单人雪橇",
            "双人雪橇",
            "团体接力",
            "男子项目",
            "女子项目",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛轮次",
            "时间计算",
            "出发规则",
            "赛道要求",
            "终点判定",
        ]
    }

    /// 赛道规格
    pub fn track_specifications(&self) -> Vec<&'static str> {
        vec![
            "赛道长度: 1200-1650米",
            "坡度要求",
            "弯道设计",
            "冰面质量",
            "安全设施",
        ]
    }

    /// 技术要求
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "仰卧滑行",
            "身体控制",
            "弯道技术",
            "出发技术",
            "终点冲刺",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "雪橇装备",
            "防护头盔",
            "比赛服装",
            "手套护具",
            "靴子",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须",
            "赛道安全",
            "速度限制",
            "医疗支持",
            "应急处理",
        ]
    }

    /// 计时规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "精确到千分秒",
            "总时间计算",
            "成绩记录",
            "同分处理",
        ]
    }
}

impl Default for LugeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LugeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("luge")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【雪橇规则】\n\n\
            比赛项目:\n{}\n\n\
            技术要求:\n{}\n\n\
            装备要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luge_rules() {
        let rules = LugeRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}