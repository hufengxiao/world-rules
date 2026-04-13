//! 竞走规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 竞走规则
pub struct RaceWalkingRules {
    metadata: RuleMetadata,
}

impl RaceWalkingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "竞走规则",
                "竞走比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "男子20公里竞走",
            "男子50公里竞走",
            "女子20公里竞走",
            "室内5000米竞走",
            "青年组10公里竞走",
        ]
    }

    /// 技术规则
    pub fn technique_rules(&self) -> Vec<&'static str> {
        vec![
            "必须始终保持与地面接触",
            "支撑腿必须伸直",
            "膝关节不能弯曲",
            "前脚落地后脚才能抬起",
            "裁判监督技术",
        ]
    }

    /// 裁判职责
    pub fn judge_duties(&self) -> Vec<&'static str> {
        vec![
            "6-8名裁判沿路线分布",
            "观察运动员技术",
            "发现犯规提出警告",
            "使用红牌系统",
            "主裁判判定取消资格",
        ]
    }

    /// 犯规警告
    pub fn foul_warnings(&self) -> Vec<&'static str> {
        vec![
            "屈膝犯规",
            "腾空犯规",
            "第一次警告",
            "警告累计",
            "不同裁判可分别警告",
        ]
    }

    /// 取消资格
    pub fn disqualification(&self) -> Vec<&'static str> {
        vec![
            "收到3张红牌取消资格",
            "红牌来自不同裁判",
            "通知运动员离场",
            "赛后申诉程序",
            "严重违规直接取消",
        ]
    }

    /// 补给站
    pub fn aid_stations(&self) -> Vec<&'static str> {
        vec![
            "每2-3公里设置补给站",
            "提供水和饮料",
            "个人补给区",
            "教练可在指定区域补给",
            "不得在其他区域补给",
        ]
    }

    /// 比赛路线
    pub fn course_requirements(&self) -> Vec<&'static str> {
        vec![
            "公路或跑道",
            "起点和终点安排",
            "路线测量准确",
            "每公里标记",
            "转弯和爬坡规定",
        ]
    }

    /// 计时规则
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "分段计时",
            "精确到秒",
            "成绩公布",
            "世界纪录认证",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "竞走专用鞋",
            "轻便服装",
            "号码布佩戴",
            "禁止违规装备",
            "计时芯片",
        ]
    }
}

impl Default for RaceWalkingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RaceWalkingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("race_walking")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【竞走规则】\n\n\
            技术规则:\n{}\n\n\
            犯规警告:\n{}\n\n\
            取消资格:\n{}\n\n\
            比赛距离:\n{}\n",
            self.technique_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.foul_warnings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.disqualification().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_walking_rules() {
        let rules = RaceWalkingRules::new();
        assert!(!rules.technique_rules().is_empty());
    }
}