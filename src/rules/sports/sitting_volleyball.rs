//! 坐式排球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 坐式排球规则
pub struct SittingVolleyballRules {
    metadata: RuleMetadata,
}

impl SittingVolleyballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "坐式排球规则",
                "坐式排球比赛基本规则"
            )
            .with_origin("荷兰")
            .with_tags(vec!["体育".into(), "残奥".into()]),
        }
    }

    /// 球场规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 10×6米",
            "网高: 1.15米(男子)",
            "网高: 1.05米(女子)",
            "地面平整",
            "进攻线距离",
        ]
    }

    /// 球员分类
    pub fn player_classification(&self) -> Vec<&'static str> {
        vec![
            "VS级: 轻度残疾",
            "VD级: 重度残疾",
            "每队最多VS级球员",
            "分级认证",
            "医学评估",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "每队6人上场",
            "坐姿比赛",
            "臀部必须接触地面",
            "发球规则",
            "轮转规则",
        ]
    }

    /// 技术规则
    pub fn technique_rules(&self) -> Vec<&'static str> {
        vec![
            "臀部接触地面",
            "站立瞬间违规",
            "传球规则",
            "扣球规则",
            "拦网规则",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每局25分",
            "至少领先2分",
            "五局三胜",
            "决胜局15分",
            "每球得分制",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "臀部离开地面",
            "站立比赛",
            "触碰网",
            "位置错误",
            "不当行为",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "地面平整",
            "防滑处理",
            "医疗支持",
            "紧急预案",
            "避免碰撞",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "标准排球",
            "服装要求",
            "护膝可选",
            "号码布",
            "无鞋比赛",
        ]
    }
}

impl Default for SittingVolleyballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SittingVolleyballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sitting_volleyball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【坐式排球规则】\n\n\
            球场规格:\n{}\n\n\
            技术规则:\n{}\n\n\
            犯规规则:\n{}\n\n\
            得分规则:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sitting_volleyball_rules() {
        let rules = SittingVolleyballRules::new();
        assert!(!rules.court_specifications().is_empty());
    }
}