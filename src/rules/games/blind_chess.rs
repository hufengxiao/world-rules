//! 盲棋规则 (Blind Chess / Blindfold Chess)
//!
//! 盲人对弈象棋，不使用实体棋盘，全靠记忆和口述。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BlindChessRules,
    name: "盲棋规则",
    desc: "盲棋(Blindfold Chess)无棋盘对弈规则，考验记忆与计算",
    origin: "国际",
    tags: ["游戏", "棋类", "象棋", "盲棋", "记忆"],
}

impl BlindChessRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "盲棋是高水平棋手展示记忆和计算能力的对弈方式",
            "不使用实体棋盘，全凭记忆",
            "棋手口头报出棋步，助手在棋盘上执行",
            "最初用于盲人棋手，现为棋手能力展示",
            "世界顶级棋手可同时对弈多盘盲棋",
        ]
    }

    /// 棋盘记忆
    pub fn section_board_memory(&self) -> Vec<&'static str> {
        vec![
            "棋手需在脑中维护完整的棋盘状态",
            "记住所有棋子的位置和类型",
            "记住双方的走棋历史",
            "计算时要考虑所有可能的走法",
            "极强的空间记忆和抽象思维能力",
        ]
    }

    /// 走棋规则
    pub fn section_moves(&self) -> Vec<&'static str> {
        vec![
            "使用标准象棋坐标表示法",
            "白方走棋：\"e2-e4\"（e2到e4）",
            "吃棋：\"e5xd6\"（e5吃d6的棋子）",
            "特殊走法需明确说明（王车易位等）",
            "助手或对手确认走棋合法性",
        ]
    }

    /// 对弈流程
    pub fn section_process(&self) -> Vec<&'static str> {
        vec![
            "一方报出走棋（如\"马f3\"）",
            "对手听到后思考并回应",
            "助手（或裁判）在实体棋盘上执行",
            "双方不可看到棋盘",
            "结束时裁判宣布结果",
        ]
    }

    /// 记录要求
    pub fn section_recording(&self) -> Vec<&'static str> {
        vec![
            "裁判完整记录走棋历史",
            "记录格式使用标准象棋记谱法",
            "棋手不可查看记录",
            "可通过裁判询问当前局面（限制次数）",
            "裁判可提示非法走棋",
        ]
    }

    /// 技能要求
    pub fn section_skills(&self) -> Vec<&'static str> {
        vec![
            "极强的棋盘空间记忆力",
            "快速准确的局面计算能力",
            "熟悉标准象棋记谱法",
            "高度专注力（长时间无视觉辅助）",
            "顶级棋手：同时10+盘盲棋",
        ]
    }

    /// 世界纪录
    pub fn section_records(&self) -> Vec<&'static str> {
        vec![
            "世界纪录：同时盲棋对弈46盘",
            "保持者：Marc Lang（2011年）",
            "著名盲棋大师：Alekhine, Koltanowski",
            "盲棋锦标赛在全球举行",
            "中国棋手也有盲棋表演传统",
        ]
    }
}

impl Rule for BlindChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("blind_chess")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "盲棋规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("棋盘记忆", &self.section_board_memory()),
                ("走棋规则", &self.section_moves()),
                ("对弈流程", &self.section_process()),
                ("记录要求", &self.section_recording()),
                ("技能要求", &self.section_skills()),
                ("世界纪录", &self.section_records()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blind_chess_rules_basic() {
        let rules = BlindChessRules::new();
        assert_eq!(rules.metadata().name, "盲棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn blind_chess_has_memory_info() {
        let rules = BlindChessRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("记忆"));
    }
}
