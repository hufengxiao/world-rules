//! 篮球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 篮球规则变体
#[derive(Debug, Clone)]
pub enum BasketballVariant {
    /// NBA规则
    NBA,
    /// FIBA规则 (国际)
    FIBA,
    /// CBA规则 (中国)
    CBA,
}

/// 篮球规则
pub struct BasketballRules {
    metadata: RuleMetadata,
    variant: BasketballVariant,
}

impl BasketballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "篮球规则",
                "标准篮球比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "篮球".into()]),
            variant: BasketballVariant::NBA,
        }
    }

    pub fn with_variant(mut self, variant: BasketballVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 每队人数
    pub fn team_size(&self) -> u8 {
        5
    }

    /// 比赛时长 (分钟)
    pub fn match_duration(&self) -> u16 {
        match self.variant {
            BasketballVariant::NBA => 48, // 4节×12分钟
            BasketballVariant::FIBA => 40, // 4节×10分钟
            BasketballVariant::CBA => 48,
        }
    }

    /// 单节时长
    pub fn quarter_duration(&self) -> u16 {
        match self.variant {
            BasketballVariant::NBA => 12,
            BasketballVariant::FIBA => 10,
            BasketballVariant::CBA => 12,
        }
    }

    /// 三分线距离 (米)
    pub fn three_point_distance(&self) -> f32 {
        match self.variant {
            BasketballVariant::NBA => 7.24,
            BasketballVariant::FIBA => 6.75,
            BasketballVariant::CBA => 7.24,
        }
    }

    /// 罚球得分
    pub fn free_throw_points(&self) -> u8 {
        1
    }

    /// 个人犯规上限
    pub fn personal_foul_limit(&self) -> u8 {
        match self.variant {
            BasketballVariant::NBA => 6,
            BasketballVariant::FIBA => 5,
            BasketballVariant::CBA => 5,
        }
    }

    /// 24秒进攻时限
    pub fn shot_clock(&self) -> u8 {
        match self.variant {
            BasketballVariant::NBA => 24,
            BasketballVariant::FIBA => 24,
            BasketballVariant::CBA => 24,
        }
    }
}

impl Default for BasketballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BasketballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}篮球规则】\n\n\
            比赛时长: {}分钟 (4节×{}分钟)\n\
            每队人数: {}人\n\
            三分线距离: {}米\n\
            进攻时限: {}秒\n\
            个人犯规上限: {}次\n\n\
            得分规则:\n\
            - 罚球: 1分\n\
            - 两分球: 禁区内投篮\n\
            - 三分球: 三分线外投篮\n\n\
            基本规则:\n\
            1. 持球不能走步 (最多两步)\n\
            2. 不能双手运球\n\
            3. 不能撞击防守球员\n\
            4. 犯规上限后罚出场",
            match self.variant {
                BasketballVariant::NBA => "NBA",
                BasketballVariant::FIBA => "FIBA",
                BasketballVariant::CBA => "CBA",
            },
            self.match_duration(),
            self.quarter_duration(),
            self.team_size(),
            self.three_point_distance(),
            self.shot_clock(),
            self.personal_foul_limit()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basketball_rules() {
        let rules = BasketballRules::new();
        assert_eq!(rules.team_size(), 5);
        assert_eq!(rules.match_duration(), 48);
    }
}