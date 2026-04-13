//! 沙滩足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 沙滩足球规则
pub struct BeachSoccerRules {
    metadata: RuleMetadata,
}

impl BeachSoccerRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "沙滩足球规则",
                "沙滩足球比赛基本规则"
            )
            .with_origin("巴西")
            .with_tags(vec!["体育".into(), "足球".into()]),
        }
    }

    /// 球场规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "长度: 35-37米",
            "宽度: 26-28米",
            "沙滩地面",
            "无禁区线",
            "门宽: 5.5米×高2.2米",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队5人上场",
            "1名守门员",
            "4名场上球员",
            "替补最多3人",
            "换人次数不限",
        ]
    }

    /// 比赛时间
    pub fn game_duration(&self) -> Vec<&'static str> {
        vec![
            "三节比赛",
            "每节12分钟",
            "节间休息3分钟",
            "总时间36分钟",
            "加时赛3分钟",
        ]
    }

    /// 球规格
    pub fn ball_specifications(&self) -> Vec<&'static str> {
        vec![
            "重量较轻",
            "材质特殊",
            "适合沙滩",
            "弹性较低",
            "比赛专用球",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "无越位规则",
            "犯规直接罚球",
            "累积犯规制",
            "黄牌红牌",
            "两黄一红",
        ]
    }

    /// 罚球规则
    pub fn free_kick_rules(&self) -> Vec<&'static str> {
        vec![
            "所有犯规罚球",
            "守门员防守",
            "其他球员退后5米",
            "直接射门",
            "罚球得分率高",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "世界杯",
            "欧洲杯",
            "国内联赛",
            "沙滩足球锦标赛",
            "友谊赛",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "沙滩安全检查",
            "避免危险动作",
            "医疗支持",
            "天气条件",
            "补水措施",
        ]
    }
}

impl Default for BeachSoccerRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BeachSoccerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("beach_soccer")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【沙滩足球规则】\n\n\
            球场规格:\n{}\n\n\
            比赛时间:\n{}\n\n\
            犯规规则:\n{}\n\n\
            罚球规则:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.game_duration().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.free_kick_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beach_soccer_rules() {
        let rules = BeachSoccerRules::new();
        assert!(!rules.field_specifications().is_empty());
    }
}