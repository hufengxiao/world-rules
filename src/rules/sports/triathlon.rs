//! 铁人三项规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 铁人三项规则
pub struct TriathlonRules {
    metadata: RuleMetadata,
}

impl TriathlonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "铁人三项规则",
                "铁人三项比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "综合".into()]),
        }
    }

    /// 标准距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "奥林匹克标准: 1.5公里游泳+40公里自行车+10公里跑步",
            "半程铁人: 1.9公里游泳+90公里自行车+21.1公里跑步",
            "全程铁人: 3.8公里游泳+180公里自行车+42.2公里跑步",
            "短距离: 0.75公里游泳+20公里自行车+5公里跑步",
            "超距离铁人",
        ]
    }

    /// 比赛流程
    pub fn race_sequence(&self) -> Vec<&'static str> {
        vec![
            "游泳: 第一项目",
            "换项区T1: 游泳到自行车",
            "自行车: 第二项目",
            "换项区T2: 自行车到跑步",
            "跑步: 最后项目",
        ]
    }

    /// 游泳规则
    pub fn swimming_rules(&self) -> Vec<&'static str> {
        vec![
            "公开水域游泳",
            "集体出发",
            "允许穿着泳衣",
            "浮漂辅助",
            "游泳帽佩戴",
        ]
    }

    /// 自行车规则
    pub fn cycling_rules(&self) -> Vec<&'static str> {
        vec![
            "公路自行车",
            "禁止阻挡他人",
            "禁止骑在最前面",
            "头盔必须佩戴",
            "自行车检查",
        ]
    }

    /// 跑步规则
    pub fn running_rules(&self) -> Vec<&'static str> {
        vec![
            "公路跑步",
            "必须完成全程",
            "禁止帮助他人",
            "补给站使用",
            "号码布佩戴",
        ]
    }

    /// 换项区规则
    pub fn transition_rules(&self) -> Vec<&'static str> {
        vec![
            "指定换项区位置",
            "存放自行车和装备",
            "安全换项",
            "不得阻挡他人",
            "换项时间计入总成绩",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "使用机动车帮助",
            "阻挡其他选手",
            "替跑行为",
            "抄近路",
            "危险行为",
        ]
    }

    /// 分组规则
    pub fn categories(&self) -> Vec<&'static str> {
        vec![
            "男子组",
            "女子组",
            "年龄分组",
            "精英组",
            "接力组",
        ]
    }
}

impl Default for TriathlonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TriathlonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("triathlon")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【铁人三项规则】\n\n\
            标准距离:\n{}\n\n\
            比赛流程:\n{}\n\n\
            换项区规则:\n{}\n\n\
            禁止行为:\n{}\n",
            self.distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.race_sequence().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.transition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triathlon_rules() {
        let rules = TriathlonRules::new();
        assert!(!rules.distances().is_empty());
    }
}