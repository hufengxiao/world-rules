//! 任意球规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 任意球类型
#[derive(Debug, Clone, PartialEq)]
pub enum FreeKickType {
    /// 直接任意球（可直接射门）
    Direct,
    /// 间接任意球（需经他人触球）
    Indirect,
}

/// 任意球规则详解
pub struct FootballFreeKickRules {
    metadata: RuleMetadata,
}

impl FootballFreeKickRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("任意球规则详解", "足球任意球判罚和执行的完整规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "任意球".into()]),
        }
    }

    /// 直接任意球判罚条件
    pub fn direct_free_kick_conditions(&self) -> Vec<&'static str> {
        vec![
            "踢或企图踢对方球员",
            "绊摔或企图绊摔对方球员",
            "跳向对方球员",
            "冲撞对方球员",
            "打或企图打对方球员",
            "推对方球员",
            "抢截时触球前触人",
            "拉扯对方球员",
            "向对方球员吐唾沫",
            "故意手球（门将除外）",
        ]
    }

    /// 间接任意球判罚条件
    pub fn indirect_free_kick_conditions(&self) -> Vec<&'static str> {
        vec![
            "危险动作犯规",
            "阻挡对方球员",
            "阻挡门将发球",
            "门将持球超过6秒",
            "门将二次触球",
            "门将用手接回传球",
            "越位犯规",
            "其他技术犯规",
        ]
    }

    /// 任意球执行程序
    pub fn execution_procedure(&self) -> Vec<&'static str> {
        vec![
            "球必须静止",
            "裁判确认罚球位置",
            "对手退后至少9.15米",
            "哨响后才能执行",
            "一脚完成罚球",
            "罚球后不能重复触球",
        ]
    }

    /// 快速任意球规定
    pub fn quick_free_kick_rules(&self) -> Vec<&'static str> {
        vec![
            "可以不等对手退后",
            "裁判未制止即可执行",
            "对手自己承担责任",
            "对手故意阻挡可判犯规",
            "裁判可以要求重罚",
        ]
    }

    /// 任意球位置规则
    pub fn position_rules(&self) -> Vec<&'static str> {
        vec![
            "犯规发生位置",
            "禁区内犯规判点球",
            "本方禁区内间接任意球",
            "对方禁区内任意球可射门",
            "特定位置有特殊规定",
        ]
    }

    /// 人墙规定
    pub fn wall_rules(&self) -> Vec<&'static str> {
        vec![
            "至少距球9.15米",
            "防守方组成人墙",
            "进攻方可以站位",
            "不能移动人墙位置",
            "人墙阻挡可判犯规",
        ]
    }

    /// 任意球结果处理
    pub fn result_handling(&self) -> Vec<&'static str> {
        vec![
            "直接任意球进球有效",
            "间接任意球需他人触球",
            "罚球者重复触球判犯规",
            "犯规后重罚任意球",
            "禁区内犯规特殊处理",
        ]
    }

    /// 判定任意球类型
    pub fn determine_kick_type(&self, foul_description: &str) -> FreeKickType {
        // 简化的判定逻辑
        if self
            .direct_free_kick_conditions()
            .iter()
            .any(|c| foul_description.contains(c))
        {
            FreeKickType::Direct
        } else {
            FreeKickType::Indirect
        }
    }

    /// 计算任意球进球概率(模拟)
    pub fn calculate_goal_probability(&self, kick_type: FreeKickType, distance: f32) -> f32 {
        match kick_type {
            FreeKickType::Direct => {
                if distance < 20.0 {
                    0.25 // 20米内直接任意球进球概率较高
                } else if distance < 30.0 {
                    0.15
                } else {
                    0.05
                }
            }
            FreeKickType::Indirect => 0.02, // 间接任意球进球概率很低
        }
    }
}

impl Default for FootballFreeKickRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballFreeKickRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_free_kick")
    }

    fn explain(&self) -> String {
        format!(
            "【任意球规则详解】\n\n\
            直接任意球判罚条件:\n{}\n\n\
            间接任意球判罚条件:\n{}\n\n\
            执行程序:\n{}\n\n\
            人墙规定:\n{}\n\n\
            结果处理:\n{}\n",
            self.direct_free_kick_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.indirect_free_kick_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.execution_procedure()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wall_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.result_handling()
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
    fn test_direct_free_kick_conditions() {
        let rules = FootballFreeKickRules::new();
        let conditions = rules.direct_free_kick_conditions();
        assert!(conditions.contains(&"踢或企图踢对方球员"));
        assert!(conditions.contains(&"故意手球（门将除外）"));
    }

    #[test]
    fn test_indirect_free_kick_conditions() {
        let rules = FootballFreeKickRules::new();
        let indirect = rules.indirect_free_kick_conditions();
        assert!(indirect.contains(&"门将持球超过6秒"));
        assert!(indirect.contains(&"越位犯规"));
    }

    #[test]
    fn test_kick_type_determination() {
        let rules = FootballFreeKickRules::new();

        // 直接任意球犯规
        let direct_foul = "踢或企图踢对方球员";
        assert_eq!(rules.determine_kick_type(direct_foul), FreeKickType::Direct);

        // 间接任意球犯规
        let indirect_foul = "危险动作犯规";
        assert_eq!(
            rules.determine_kick_type(indirect_foul),
            FreeKickType::Indirect
        );
    }

    #[test]
    fn test_goal_probability() {
        let rules = FootballFreeKickRules::new();

        // 近距离直接任意球概率高
        let close_prob = rules.calculate_goal_probability(FreeKickType::Direct, 18.0);
        assert!(close_prob > 0.2);

        // 远距离直接任意球概率低
        let far_prob = rules.calculate_goal_probability(FreeKickType::Direct, 35.0);
        assert!(far_prob < 0.1);

        // 间接任意球概率很低
        let indirect_prob = rules.calculate_goal_probability(FreeKickType::Indirect, 20.0);
        assert!(indirect_prob < 0.05);
    }

    #[test]
    fn test_metadata() {
        let rules = FootballFreeKickRules::new();
        assert_eq!(rules.metadata().name, "任意球规则详解");
        assert_eq!(rules.category(), RuleCategory::sports("football_free_kick"));
    }
}
