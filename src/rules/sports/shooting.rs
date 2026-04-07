//! 射击规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 射击项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShootingType {
    /// 手枪
    Pistol,
    /// 步枪
    Rifle,
    /// 飞碟
    Shotgun,
}

impl ShootingType {
    pub fn name(&self) -> &'static str {
        match self {
            ShootingType::Pistol => "手枪",
            ShootingType::Rifle => "步枪",
            ShootingType::Shotgun => "飞碟",
        }
    }
}

/// 射击规则
pub struct ShootingRules {
    metadata: RuleMetadata,
}

impl ShootingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "射击规则",
                "ISSF 国际射击运动联合会标准规则"
            )
            .with_origin("ISSF")
            .with_tags(vec!["体育".into(), "射击".into()]),
        }
    }

    /// 手枪项目
    pub fn pistol_events(&self) -> Vec<&'static str> {
        vec![
            "10米气手枪: 60发",
            "25米手枪速射: 60发",
            "25米女子手枪: 60发",
        ]
    }

    /// 步枪项目
    pub fn rifle_events(&self) -> Vec<&'static str> {
        vec![
            "10米气步枪: 60发",
            "50米步枪三姿: 3×40发",
            "50米步枪卧射: 60发",
        ]
    }

    /// 飞碟项目
    pub fn shotgun_events(&self) -> Vec<&'static str> {
        vec![
            "双向飞碟: 125靶",
            "多向飞碟: 125靶",
            "双多向飞碟: 150靶",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "靶纸环值: 1-10环",
            "最高分: 10.9环(精确到小数)",
            "资格赛取前8名进入决赛",
            "决赛采用淘汰制",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "枪口始终指向安全方向",
            "手指不扣扳机时放在护圈外",
            "确认目标前后安全",
            "不射击时枪膛清空",
            "穿戴护目镜和耳罩",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超时射击",
            "站姿违规",
            "装备违规",
            "干扰其他选手",
        ]
    }
}

impl Default for ShootingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShootingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("shooting")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【射击规则】\n\n\
            手枪项目:\n{}\n\n\
            步枪项目:\n{}\n\n\
            飞碟项目:\n{}\n\n\
            计分规则:\n{}\n\n\
            安全规则:\n{}\n",
            self.pistol_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rifle_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.shotgun_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shooting_rules() {
        let rules = ShootingRules::new();
        assert!(!rules.pistol_events().is_empty());
    }
}