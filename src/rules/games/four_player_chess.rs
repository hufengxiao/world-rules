//! 四国象棋规则 (Four-Player Chess)
//!
//! 四人同时对弈的国际象棋变体，每人占据棋盘一个边。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: FourPlayerChessRules,
    name: "四国象棋规则",
    desc: "四国象棋(Four-Player Chess)四人同时对弈规则",
    origin: "国际",
    tags: ["游戏", "棋类", "象棋", "四人"],
}

impl FourPlayerChessRules {
    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为标准8x8格加四个延伸区域",
            "每个延伸区域3x8格，连接主棋盘四边",
            "总棋盘约14x14格（十字形）",
            "四人分坐四个方向（北、东、南、西）",
        ]
    }

    /// 棋子配置
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "每方拥有完整标准象棋棋子（16枚）",
            "四方棋子颜色不同（通常红/蓝/黄/绿）",
            "王、后、车、象、马、兵各司其职",
            "棋子初始排列在延伸区域",
        ]
    }

    /// 轮次顺序
    pub fn section_turns(&self) -> Vec<&'static str> {
        vec![
            "四人轮流走棋，顺序为北→东→南→西",
            "北方先手（可协商或随机决定）",
            "每人每回合走一步",
            "不可跳过自己的回合",
            "必须在自己回合内完成走棋",
        ]
    }

    /// 将死规则
    pub fn section_capture(&self) -> Vec<&'static str> {
        vec![
            "将死任何一方的王，该方出局",
            "出局方的棋子从棋盘移除",
            "其余三方继续对弈",
            "将死最后一方的王即获胜",
            "同时将死两方的情况需判定胜负",
        ]
    }

    /// 和棋与联盟
    pub fn section_special(&self) -> Vec<&'static str> {
        vec![
            "允许临时联盟（两人合作对付另外两人）",
            "联盟可随时解散",
            "三方僵局可判和棋",
            "只剩两方时按标准象棋规则判定",
            "循环僵局判和（三方都无法获胜）",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "注意防守三个方向可能的攻击",
            "联盟策略很重要",
            "避免过早暴露王的位置",
            "利用地理位置优势（边角相对安全）",
            "观察三方动态，把握时机出击",
        ]
    }
}

impl Rule for FourPlayerChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("four_player_chess")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "四国象棋规则",
            &[
                ("棋盘设置", &self.section_board()),
                ("棋子配置", &self.section_pieces()),
                ("轮次顺序", &self.section_turns()),
                ("将死规则", &self.section_capture()),
                ("和棋与联盟", &self.section_special()),
                ("策略要点", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_player_chess_rules_basic() {
        let rules = FourPlayerChessRules::new();
        assert_eq!(rules.metadata().name, "四国象棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn four_player_chess_has_four_players() {
        let rules = FourPlayerChessRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("四人"));
    }
}
