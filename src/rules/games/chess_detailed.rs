//! 国际象棋详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChessDetailedRules, name: "国际象棋详细规则", desc: "国际象棋详细规则", origin: "国际", tags: ["游戏", "棋类"] }
impl ChessDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "王(King)周围8格每步一格",
            "后(Queen)横竖斜任意距离",
            "车(Rook)横竖任意距离",
            "象(Bishop)斜向任意距离",
            "马(Knight)L形2+1可跳过棋子",
            "兵(Pawn)前进一格首步可两格斜吃",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "王车易位:王向车方向移动两格车跳到王另一侧",
            "条件:王和车未移动过中间无棋子王未被将军",
            "吃过路兵:敌方兵首步走两格时可斜吃它",
            "兵升变:兵到达底线必须升级为后/车/象/马",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "将死(Checkmate):王被将军且无法逃脱",
            "逼和:无合法走法但未被将军",
            "三次重复局面和棋",
            "50步规则:50回合内无吃子无兵移动判和",
            "双方同意和棋",
            "时间耗尽判负",
        ]
    }
}
impl Rule for ChessDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际象棋详细规则",
            &[
                ("棋子走法", &self.section_0()),
                ("特殊走法", &self.section_1()),
                ("胜负规则", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChessDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
