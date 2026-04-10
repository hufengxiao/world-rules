//! 保龄球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 保龄球规则
pub struct BowlingRules {
    metadata: RuleMetadata,
}

impl BowlingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "保龄球规则",
                "保龄球比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "室内".into()]),
        }
    }

    /// 场地规格
    pub fn lane_specifications(&self) -> Vec<&'static str> {
        vec![
            "球道长度: 18.29米",
            "球道宽度: 1.05米",
            "球瓶区: 三角形排列",
            "10个球瓶",
            "犯规线: 球道前端",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "一局10格",
            "每格最多两次投球",
            "全倒: 一次击倒所有瓶",
            "补中: 两次击倒所有瓶",
            "全倒得10分加后两球分",
            "补中得10分加后一球分",
        ]
    }

    /// 比赛形式
    pub fn game_formats(&self) -> Vec<&'static str> {
        vec![
            "单局: 10格",
            "三局: 累计计分",
            "系列赛: 多局制",
            "个人赛和团体赛",
            "满分300分",
        ]
    }

    /// 全倒规则
    pub fn strike_rules(&self) -> Vec<&'static str> {
        vec![
            "第一球击倒所有瓶",
            "得10分加后两球得分",
            "连续全倒奖励更高",
            "完美局: 12次全倒",
            "完美局得分300分",
        ]
    }

    /// 补中规则
    pub fn spare_rules(&self) -> Vec<&'static str> {
        vec![
            "两球击倒所有瓶",
            "得10分加后一球得分",
            "第10格补中后可加投",
            "补中符号: /",
            "补中得分较低",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "越过犯规线",
            "犯规球不得分",
            "球瓶重新摆好",
            "电子犯规探测器",
            "连续犯规警告",
        ]
    }

    /// 球瓶排列
    pub fn pin_arrangement(&self) -> Vec<&'static str> {
        vec![
            "10个球瓶三角形排列",
            "第一排: 1个(1号瓶)",
            "第二排: 2个(2、3号瓶)",
            "第三排: 3个(4、5、6号瓶)",
            "第四排: 4个(7、8、9、10号瓶)",
        ]
    }
}

impl Default for BowlingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BowlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bowling")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【保龄球规则】\n\n\
            场地规格:\n{}\n\n\
            计分规则:\n{}\n\n\
            全倒规则:\n{}\n\n\
            补中规则:\n{}\n",
            self.lane_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.strike_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.spare_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bowling_rules() {
        let rules = BowlingRules::new();
        assert!(!rules.lane_specifications().is_empty());
    }
}