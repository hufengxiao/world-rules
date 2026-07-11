//! 越位规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 越位位置判定
#[derive(Debug, Clone, Copy)]
pub struct OffsidePosition {
    /// 进攻球员位置（距离球门的米数）
    pub attacker_distance: f32,
    /// 最后一名防守球员位置
    pub last_defender_distance: f32,
    /// 球的位置
    pub ball_distance: f32,
    /// 是否在中场半场
    pub in_opponent_half: bool,
}

/// 越位规则详解
pub struct FootballOffsideDetailedRules {
    metadata: RuleMetadata,
}

impl FootballOffsideDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("越位规则详解", "足球越位规则的完整解释")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "越位".into()]),
        }
    }

    /// 越位判定核心原则
    pub fn core_principles(&self) -> Vec<&'static str> {
        vec![
            "进攻球员比球和最后第二名防守球员更靠近球门",
            "必须在对方半场",
            "在传球瞬间判定",
            "不参与比赛不判越位",
            "平行位置不算越位",
        ]
    }

    /// 越位位置条件
    pub fn position_conditions(&self) -> Vec<&'static str> {
        vec![
            "比球更靠近对方球门线",
            "比最后第二名防守球员更靠近对方球门线",
            "处于对方半场",
            "三个条件同时满足才算越位位置",
        ]
    }

    /// 越位犯规触发
    pub fn foul_activation(&self) -> Vec<&'static str> {
        vec![
            "接球触犯越位",
            "干扰对方球员",
            "干扰比赛",
            "获得利益",
            "被动越位不判罚",
        ]
    }

    /// 不判越位情况
    pub fn no_offside_situations(&self) -> Vec<&'static str> {
        vec![
            "球员在中场本方半场",
            "与最后第二名防守球员平行",
            "与球平行或比球更远离球门",
            "直接接球门球",
            "直接接角球",
            "直接接界外球",
        ]
    }

    /// 判定越位
    pub fn check_offside(&self, position: &OffsidePosition) -> bool {
        // 必须在对方半场
        if !position.in_opponent_half {
            return false;
        }
        // 攻击者比最后防守者更靠近球门
        if position.attacker_distance >= position.last_defender_distance {
            return false;
        }
        // 攻击者比球更靠近球门
        if position.attacker_distance >= position.ball_distance {
            return false;
        }
        true
    }

    /// VAR辅助判定
    pub fn var_assistance(&self) -> Vec<&'static str> {
        vec![
            "视频回放辅助",
            "精确线条绘制",
            "毫米级判定",
            "主裁判最终决定",
            "只用于进球/点球/红牌情况",
        ]
    }

    /// 越位判罚后果
    pub fn offside_penalty(&self) -> Vec<&'static str> {
        vec![
            "间接任意球",
            "在越位位置执行",
            "对方球队获得球权",
            "不记录个人犯规",
            "不累计犯规次数",
        ]
    }
}

impl Default for FootballOffsideDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballOffsideDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_offside")
    }

    fn explain(&self) -> String {
        format!(
            "【越位规则详解】\n\n\
            核心原则:\n{}\n\n\
            越位位置条件:\n{}\n\n\
            越位犯规触发:\n{}\n\n\
            不判越位情况:\n{}\n\n\
            VAR辅助:\n{}\n",
            self.core_principles()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.position_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.foul_activation()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.no_offside_situations()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.var_assistance()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offside_detection() {
        let rules = FootballOffsideDetailedRules::new();

        // 越位情况：攻击者比防守者更靠近球门
        let offside_pos = OffsidePosition {
            attacker_distance: 10.0,
            last_defender_distance: 15.0,
            ball_distance: 20.0,
            in_opponent_half: true,
        };
        assert!(rules.check_offside(&offside_pos));

        // 不越位：攻击者与防守者平行
        let not_offside_pos = OffsidePosition {
            attacker_distance: 15.0,
            last_defender_distance: 15.0,
            ball_distance: 20.0,
            in_opponent_half: true,
        };
        assert!(!rules.check_offside(&not_offside_pos));

        // 不越位：在本方半场
        let own_half_pos = OffsidePosition {
            attacker_distance: 10.0,
            last_defender_distance: 15.0,
            ball_distance: 20.0,
            in_opponent_half: false,
        };
        assert!(!rules.check_offside(&own_half_pos));
    }

    #[test]
    fn test_core_principles() {
        let rules = FootballOffsideDetailedRules::new();
        assert_eq!(rules.core_principles().len(), 5);
        assert!(rules.core_principles().contains(&"平行位置不算越位"));
    }

    #[test]
    fn test_no_offside_situations() {
        let rules = FootballOffsideDetailedRules::new();
        let situations = rules.no_offside_situations();
        assert!(situations.contains(&"直接接角球"));
        assert!(situations.contains(&"直接接界外球"));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballOffsideDetailedRules::new();
        assert_eq!(rules.metadata().name, "越位规则详解");
        assert_eq!(rules.category(), RuleCategory::sports("football_offside"));
    }
}
