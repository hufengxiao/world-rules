//! 扑克规则 - 德州扑克

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 德州扑克规则
pub struct TexasHoldemRules {
    metadata: RuleMetadata,
}

impl TexasHoldemRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "德州扑克规则",
                "Texas Hold'em扑克规则"
            )
            .with_origin("美国")
            .with_tags(vec!["游戏".into(), "扑克".into()]),
        }
    }

    /// 游戏流程
    pub fn game_flow(&self) -> Vec<&'static str> {
        vec![
            "每位玩家获得2张私有牌",
            "翻牌前: 第一轮下注",
            "翻牌: 公布3张公共牌",
            "翻牌圈: 第二轮下注",
            "转牌: 公布第4张公共牌",
            "转牌圈: 第三轮下注",
            "河牌: 公布第5张公共牌",
            "河牌圈: 最后一轮下注",
        ]
    }

    /// 牌型排名
    pub fn hand_rankings(&self) -> Vec<&'static str> {
        vec![
            "皇家同花顺: 同花色A-K-Q-J-10",
            "同花顺: 同花色连续五张",
            "四条: 四张相同",
            "满堂红: 三条加一对",
            "同花: 五张同花色",
            "顺子: 五张连续(不同花色)",
            "三条: 三张相同",
            "两对: 两组对子",
            "一对: 一组对子",
            "高牌: 最大的单张",
        ]
    }

    /// 下注行动
    pub fn betting_actions(&self) -> Vec<&'static str> {
        vec![
            "过牌: 不下注继续",
            "下注: 投入筹码",
            "跟注: 跟随之前下注金额",
            "加注: 提高下注金额",
            "弃牌: 放弃本局游戏",
            "全下: 投入所有筹码",
        ]
    }

    /// 位置优势
    pub fn position_advantage(&self) -> Vec<&'static str> {
        vec![
            "按钮位: 最后行动，优势最大",
            "截位: 按钮位前一位置",
            "枪口位: 首先行动，优势最小",
            "后位可观察前位行动再做决定",
            "位置轮换，每人轮流当按钮",
        ]
    }

    /// 底池计算
    pub fn pot_calculation(&self) -> Vec<&'static str> {
        vec![
            "底池 = 所有玩家下注总和",
            "边池: 部分玩家已全下时产生",
            "赢家获得对应底池",
            "多人获胜时平分底池",
        ]
    }

    /// 盲注结构
    pub fn blind_structure(&self) -> Vec<&'static str> {
        vec![
            "小盲注: 按钮位左边玩家",
            "大盲注: 小盲注左边玩家",
            "大盲注通常是小盲注的两倍",
            "盲注金额由比赛规则决定",
            "现金游戏盲注固定",
        ]
    }

    /// 比牌规则
    pub fn showdown_rules(&self) -> Vec<&'static str> {
        vec![
            "最后下注者先亮牌",
            "其他玩家可选择亮牌或弃牌",
            "最佳牌型者获胜",
            "可使用任意组合的公共牌和私有牌",
            "最多使用5张牌组成牌型",
        ]
    }
}

impl Default for TexasHoldemRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TexasHoldemRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("texas_holdem")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【德州扑克规则】\n\n\
            游戏流程:\n{}\n\n\
            牌型排名:\n{}\n\n\
            下注行动:\n{}\n\n\
            盲注结构:\n{}\n",
            self.game_flow().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.hand_rankings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.betting_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.blind_structure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texas_holdem_rules() {
        let rules = TexasHoldemRules::new();
        assert!(!rules.game_flow().is_empty());
        assert!(!rules.hand_rankings().is_empty());
    }
}