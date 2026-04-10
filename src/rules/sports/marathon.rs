//! 马拉松规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 马拉松规则
pub struct MarathonRules {
    metadata: RuleMetadata,
}

impl MarathonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "马拉松规则",
                "马拉松比赛基本规则"
            )
            .with_origin("希腊")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 标准距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "全程马拉松: 42.195公里",
            "半程马拉松: 21.0975公里",
            "四分马拉松: 10.5公里",
            "迷你马拉松: 5公里",
            "超马拉松: 超过42.195公里",
        ]
    }

    /// 比赛路线
    pub fn course_requirements(&self) -> Vec<&'static str> {
        vec![
            "公路或城市道路",
            "起点和终点安排",
            "补给站设置",
            "医疗站配置",
            "测量精度要求",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "年龄要求: 通常18岁以上",
            "健康证明要求",
            "训练建议: 3-6个月准备",
            "报名审核",
            "经验要求: 大型赛事",
        ]
    }

    /// 补给规则
    pub fn aid_station_rules(&self) -> Vec<&'static str> {
        vec![
            "补给站每5公里设置",
            "提供水和运动饮料",
            "能量食品",
            "医疗支持",
            "私人补给区",
        ]
    }

    /// 赛事规则
    pub fn race_rules(&self) -> Vec<&'static str> {
        vec![
            "号码布佩戴",
            "计时芯片使用",
            "禁止替跑",
            "遵守路线",
            "关门时间",
        ]
    }

    /// 分组规则
    pub fn categories(&self) -> Vec<&'static str> {
        vec![
            "男子组",
            "女子组",
            "年龄分组",
            "轮椅组",
            "精英组和大众组",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "替跑行为",
            "抄近路",
            "携带违规物品",
            "干扰其他选手",
            "破坏补给",
        ]
    }

    /// 计时规则
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "枪声成绩: 从发枪开始",
            "净成绩: 从过起点线开始",
            "分段计时",
            "官方成绩公布",
            "成绩认证",
        ]
    }
}

impl Default for MarathonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MarathonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("marathon")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【马拉松规则】\n\n\
            标准距离:\n{}\n\n\
            补给规则:\n{}\n\n\
            赛事规则:\n{}\n\n\
            计时规则:\n{}\n",
            self.distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.aid_station_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.race_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.timing().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marathon_rules() {
        let rules = MarathonRules::new();
        assert!(!rules.distances().is_empty());
    }
}