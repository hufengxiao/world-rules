//! 冲浪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 冲浪规则
pub struct SurfingRules {
    metadata: RuleMetadata,
}

impl SurfingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "冲浪规则",
                "冲浪比赛基本规则"
            )
            .with_origin("夏威夷")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "积分赛制",
            "淘汰赛制",
            "团队比赛",
            "长板和短板分类",
            "男女分组比赛",
        ]
    }

    /// 比赛时间
    pub fn heat_duration(&self) -> Vec<&'static str> {
        vec![
            "每轮15-30分钟",
            "根据浪况调整",
            "决赛可延长",
            "优先次序制",
            "交叉轮次系统",
        ]
    }

    /// 冲浪板规格
    pub fn board_specifications(&self) -> Vec<&'static str> {
        vec![
            "长板: 9英尺以上",
            "短板: 5-7英尺",
            "鱼板: 短而宽",
            "泡沫板: 初学者使用",
            "桨板: 站立桨冲浪",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            " Commitment: 投入程度",
            "Degree of difficulty: 技术难度",
            "Innovation: 创新性",
            " Combination: 组合技术",
            " Variety: 技术多样性",
            " Speed, Power, Flow: 速度、力量、流畅度",
        ]
    }

    /// 计分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "每浪0.1-10分",
            "5名裁判评分",
            "取最高两浪得分",
            "去掉最高最低分",
            "满分20分",
        ]
    }

    /// 技术动作
    pub fn maneuvers(&self) -> Vec<&'static str> {
        vec![
            "基础动作: 起乘、转向",
            "进阶动作: 切削、浮行",
            "高级动作: 空中动作、管浪",
            "管浪: 最高难度",
            "360度旋转",
        ]
    }

    /// 优先规则
    pub fn priority_rules(&self) -> Vec<&'static str> {
        vec![
            "内侧冲浪者优先",
            "第一优先可自由选浪",
            "第二优先等待",
            "干扰优先扣分",
            "优先交替轮换",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "干扰其他选手",
            "抢浪",
            "危险行为",
            "违规游泳",
            "不当竞争",
        ]
    }
}

impl Default for SurfingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SurfingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("surfing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【冲浪规则】\n\n\
            得分标准:\n{}\n\n\
            计分系统:\n{}\n\n\
            技术动作:\n{}\n\n\
            优先规则:\n{}\n",
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.maneuvers().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.priority_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surfing_rules() {
        let rules = SurfingRules::new();
        assert!(!rules.scoring_criteria().is_empty());
    }
}