//! 接力跑规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 接力跑规则
pub struct RelayRules {
    metadata: RuleMetadata,
}

impl RelayRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "接力跑规则",
                "接力跑比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 项目分类
    pub fn event_types(&self) -> Vec<&'static str> {
        vec![
            "4×100米接力",
            "4×200米接力",
            "4×400米接力",
            "4×800米接力",
            "混合接力",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队4名运动员",
            "每人跑一段距离",
            "替补队员可替换",
            "接力棒传递",
            "顺序不能改变",
        ]
    }

    /// 交接规则
    pub fn baton_passing(&self) -> Vec<&'static str> {
        vec![
            "必须在交接区内传递",
            "交接区长度: 20米(100米接力)",
            "接力棒必须传递",
            "接棒者可以在预跑区起跑",
            "不允许扔接棒",
        ]
    }

    /// 交接区规格
    pub fn passing_zone(&self) -> Vec<&'static str> {
        vec![
            "4×100米: 20米交接区",
            "4×400米: 20米交接区",
            "预跑区: 10米(仅100米接力)",
            "标志线",
            "起跑线位置",
        ]
    }

    /// 起跑规则
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "第一棒蹲式起跑",
            "使用起跑器",
            "起跑信号",
            "抢跑取消资格",
            "反应时间检测",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "交接区外传递",
            "掉棒后捡棒",
            "阻挡其他队伍",
            "跑出跑道",
            "故意干扰",
        ]
    }

    /// 风速限制
    pub fn wind_limits(&self) -> Vec<&'static str> {
        vec![
            "顺风超过2米/秒记录无效",
            "风速测量设备",
            "室内比赛无风速限制",
            "风速记录",
            "影响世界纪录认证",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "预赛和决赛",
            "分组比赛",
            "接力赛决赛",
            "世界纪录认证",
            "奥运项目",
        ]
    }
}

impl Default for RelayRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RelayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("relay")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【接力跑规则】\n\n\
            项目分类:\n{}\n\n\
            交接规则:\n{}\n\n\
            交接区规格:\n{}\n\n\
            犯规规则:\n{}\n",
            self.event_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.baton_passing().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.passing_zone().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_rules() {
        let rules = RelayRules::new();
        assert!(!rules.event_types().is_empty());
    }
}