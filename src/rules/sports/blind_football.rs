//! 盲人足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 盲人足球规则
pub struct BlindFootballRules {
    metadata: RuleMetadata,
}

impl BlindFootballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "盲人足球规则",
                "盲人足球比赛基本规则"
            )
            .with_origin("西班牙")
            .with_tags(vec!["体育".into(), "残奥".into()]),
        }
    }

    /// 球员分类
    pub fn player_classification(&self) -> Vec<&'static str> {
        vec![
            "B1级: 全盲球员",
            "B2/B3级: 低视力门将",
            "守门员可为视力正常",
            "分级认证",
            "医学评估",
        ]
    }

    /// 球场规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "尺寸: 40×20米",
            "围板围场",
            "地面平整",
            "门宽: 3米",
            "门高: 2米",
        ]
    }

    /// 球规格
    pub fn ball_specifications(&self) -> Vec<&'static str> {
        vec![
            "内置发声装置",
            "声音引导球员",
            "球体特殊设计",
            "标准尺寸",
            "重量规定",
        ]
    }

    /// 比赛时间
    pub fn game_duration(&self) -> Vec<&'static str> {
        vec![
            "两半场各25分钟",
            "半场休息10分钟",
            "加时赛",
            "时间精确控制",
            "暂停规则",
        ]
    }

    /// 辅助系统
    pub fn guidance_system(&self) -> Vec<&'static str> {
        vec![
            "引导员在场外指导",
            "教练可进入区域",
            "门将可指导防守",
            "听觉指引",
            "语音沟通",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "碰撞犯规",
            "危险动作",
            "故意干扰",
            "手球",
            "不当行为",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "眼部保护装置",
            "避免危险动作",
            "围板安全",
            "医疗支持",
            "紧急预案",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队5人上场",
            "4名盲人球员",
            "1名门将",
            "替补队员",
            "引导员辅助",
        ]
    }
}

impl Default for BlindFootballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BlindFootballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("blind_football")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【盲人足球规则】\n\n\
            球员分类:\n{}\n\n\
            辅助系统:\n{}\n\n\
            犯规规则:\n{}\n\n\
            安全规则:\n{}\n",
            self.player_classification().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.guidance_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blind_football_rules() {
        let rules = BlindFootballRules::new();
        assert!(!rules.player_classification().is_empty());
    }
}