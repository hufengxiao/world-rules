//! 滚球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, format_rule_sections};
use crate::simple_rule;

simple_rule! {
    struct: BocceRules,
    name: "滚球规则",
    desc: "意大利滚球运动规则",
    origin: "意大利",
    tags: ["体育", "休闲"]
}

impl BocceRules {
    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用局数制",
            "每局4球",
            "投掷规则",
            "得分规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 27.5×4.5米",
            "目标球位置",
            "投掷区域",
            "场地边界",
            "表面要求",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投掷技术",
            "滚球技术",
            "瞄准技术",
            "击球技术",
            "控制技术",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "距离目标球最近得分",
            "每局最多4分",
            "得分测量",
            "比分记录",
            "比赛胜负",
        ]
    }

    /// 球的规格
    pub fn ball_specifications(&self) -> Vec<&'static str> {
        vec![
            "大球直径: 107毫米",
            "目标球直径: 40毫米",
            "重量规定",
            "材质要求",
            "颜色区分",
        ]
    }

    /// 团队配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "单人比赛: 各4球",
            "双人比赛: 各4球",
            "四人比赛: 各2球",
            "队员轮换",
            "比赛顺序",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滚球",
            "目标球",
            "测量工具",
            "场地装备",
            "比赛服装",
        ]
    }
}

impl Rule for BocceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bocce")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format_rule_sections("滚球规则", &[
            ("场地规格", &self.court_specifications()),
            ("技术动作", &self.techniques()),
            ("得分规则", &self.scoring()),
            ("装备要求", &self.equipment()),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bocce_rules() {
        let rules = BocceRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}
