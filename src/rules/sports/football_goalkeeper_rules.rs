//! 门将规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 门将动作类型
#[derive(Debug, Clone, Copy)]
pub enum GoalkeeperAction {
    /// 正常扑救
    NormalSave,
    /// 手接回传球
    CatchBackPass,
    /// 持球超时
    HoldBallTimeout,
    /// 二次触球
    DoubleTouch,
    /// 出禁区
    OutOfBox,
}

/// 门将规则详解
pub struct FootballGoalkeeperRules {
    metadata: RuleMetadata,
}

impl FootballGoalkeeperRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("门将规则详解", "足球门将的完整规则和限制")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "门将".into()]),
        }
    }

    /// 门将基本权限
    pub fn basic_permissions(&self) -> Vec<&'static str> {
        vec![
            "禁区内用手触球",
            "扑救射门",
            "接高空球",
            "扑救传球",
            "发球开始比赛",
            "指挥防守",
        ]
    }

    /// 门将限制规定
    pub fn restrictions(&self) -> Vec<&'static str> {
        vec![
            "禁区内用手限制",
            "禁区内持球限制6秒",
            "不能用手接回传球",
            "不能用手接队友掷球",
            "二次触球限制",
            "出禁区后失去特权",
        ]
    }

    /// 持球规则
    pub fn ball_handling_rules(&self) -> Vec<&'static str> {
        vec![
            "持球最多6秒",
            "必须在禁区内",
            "不能用手接回传球",
            "可以接对方传球",
            "可以接对方射门",
            "队友无意回传球可用手",
        ]
    }

    /// 回传球规定
    pub fn back_pass_rules(&self) -> Vec<&'static str> {
        vec![
            "队友故意回传球不能用手",
            "队友无意回传球可用手",
            "头球回传球可用手",
            "胸部回传球可用手",
            "膝盖回传球可用脚",
            "违反判间接任意球",
        ]
    }

    /// 二次触球规定
    pub fn double_touch_rules(&self) -> Vec<&'static str> {
        vec![
            "发球后不能再次用手触球",
            "必须经他人触球才能再次用手",
            "脚下触球后不能用手",
            "违反判间接任意球",
            "比赛中执行",
            "在本方禁区内执行",
        ]
    }

    /// 点球防守规则
    pub fn penalty_defense_rules(&self) -> Vec<&'static str> {
        vec![
            "必须站在门线上",
            "哨响前不能移动",
            "可以横向移动",
            "扑救后可以继续防守",
            "违反重罚点球",
            "可以研究罚球习惯",
        ]
    }

    /// 门将犯规处罚
    pub fn foul_punishments(&self) -> Vec<&'static str> {
        vec![
            "回传球犯规:间接任意球",
            "持球超时:间接任意球",
            "二次触球:间接任意球",
            "禁区外手球:直接任意球",
            "禁区内严重犯规:点球",
            "暴力行为:红牌",
        ]
    }

    /// 门将参与进攻
    pub fn attacking_role(&self) -> Vec<&'static str> {
        vec![
            "角球进攻参与",
            "最后时刻上前",
            "头球攻门",
            "远射能力",
            "组织进攻",
            "参与点球大战",
        ]
    }

    /// 门将特殊装备
    pub fn special_equipment(&self) -> Vec<&'static str> {
        vec![
            "特殊颜色球衣",
            "手套",
            "护腿板",
            "与其他球员区分",
            "裁判确认",
            "替补门将规定",
        ]
    }

    /// 判定门将动作是否违规
    pub fn is_action_valid(&self, action: GoalkeeperAction) -> bool {
        match action {
            GoalkeeperAction::NormalSave => true,
            GoalkeeperAction::CatchBackPass => false, // 违规
            GoalkeeperAction::HoldBallTimeout => false, // 违规
            GoalkeeperAction::DoubleTouch => false,   // 违规
            GoalkeeperAction::OutOfBox => true,       // 出禁区本身不违规，但失去手球特权
        }
    }

    /// 计算门将扑救成功率(模拟)
    pub fn calculate_save_rate(&self, shot_type: &str) -> f32 {
        match shot_type {
            "近距离射门" => 0.35,
            "中距离射门" => 0.55,
            "远距离射门" => 0.75,
            "点球" => 0.25,
            _ => 0.50,
        }
    }
}

impl Default for FootballGoalkeeperRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballGoalkeeperRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_goalkeeper")
    }

    fn explain(&self) -> String {
        format!(
            "【门将规则详解】\n\n\
            基本权限:\n{}\n\n\
            限制规定:\n{}\n\n\
            持球规则:\n{}\n\n\
            回传球规定:\n{}\n\n\
            点球防守:\n{}\n",
            self.basic_permissions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.restrictions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ball_handling_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.back_pass_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.penalty_defense_rules()
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
    fn test_basic_permissions() {
        let rules = FootballGoalkeeperRules::new();
        let permissions = rules.basic_permissions();
        assert!(permissions.contains(&"禁区内用手触球"));
        assert!(permissions.contains(&"扑救射门"));
    }

    #[test]
    fn test_restrictions() {
        let rules = FootballGoalkeeperRules::new();
        let restrictions = rules.restrictions();
        assert!(restrictions.contains(&"禁区内持球限制6秒"));
        assert!(restrictions.contains(&"不能用手接回传球"));
    }

    #[test]
    fn test_action_validity() {
        let rules = FootballGoalkeeperRules::new();

        // 正常扑救:有效
        assert!(rules.is_action_valid(GoalkeeperAction::NormalSave));

        // 手接回传球:违规
        assert!(!rules.is_action_valid(GoalkeeperAction::CatchBackPass));

        // 持球超时:违规
        assert!(!rules.is_action_valid(GoalkeeperAction::HoldBallTimeout));

        // 二次触球:违规
        assert!(!rules.is_action_valid(GoalkeeperAction::DoubleTouch));
    }

    #[test]
    fn test_save_rate() {
        let rules = FootballGoalkeeperRules::new();

        // 点球扑救率较低
        let penalty_rate = rules.calculate_save_rate("点球");
        assert!(penalty_rate < 0.30);

        // 远射扑救率较高
        let long_shot_rate = rules.calculate_save_rate("远距离射门");
        assert!(long_shot_rate > 0.70);
    }

    #[test]
    fn test_penalty_defense() {
        let rules = FootballGoalkeeperRules::new();
        let defense = rules.penalty_defense_rules();
        assert!(defense.contains(&"必须站在门线上"));
        assert!(defense.contains(&"哨响前不能移动"));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballGoalkeeperRules::new();
        assert_eq!(rules.metadata().name, "门将规则详解");
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_goalkeeper")
        );
    }
}
