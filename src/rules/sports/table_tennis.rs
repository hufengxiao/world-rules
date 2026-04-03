//! 乒乓球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 乒乓球规则
pub struct TableTennisRules {
    metadata: RuleMetadata,
}

impl TableTennisRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "乒乓球规则",
                "国际乒乓球联合会 (ITTF) 标准规则"
            )
            .with_origin("ITTF")
            .with_tags(vec!["体育".into(), "乒乓球".into()]),
        }
    }

    /// 单局获胜分数
    pub fn game_win_score(&self) -> u8 {
        11
    }

    /// 需要领先分数
    pub fn lead_requirement(&self) -> u8 {
        2
    }

    /// 单场比赛局数
    pub fn match_games(&self) -> u8 {
        // 通常为5局3胜或7局4胜
        5
    }

    /// 换发球规则
    pub fn serve_change_interval(&self) -> u8 {
        2 // 每2分换发球
    }

    /// 球桌尺寸 (厘米)
    pub fn table_dimensions(&self) -> (u16, u16) {
        (274, 152) // 长274cm, 宽152cm
    }

    /// 网高 (厘米)
    pub fn net_height(&self) -> u16 {
        15
    }

    /// 检查是否获胜
    pub fn check_game_win(&self, score1: u8, score2: u8) -> Option<u8> {
        if score1 >= self.game_win_score() && score1 - score2 >= self.lead_requirement() {
            Some(1)
        } else if score2 >= self.game_win_score() && score2 - score1 >= self.lead_requirement() {
            Some(2)
        } else {
            None
        }
    }
}

impl Default for TableTennisRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TableTennisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("table_tennis")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【乒乓球规则】\n\n\
            单局获胜: {}分 (需领先{}分)\n\
            比赛局数: {}局{}胜\n\
            换发球: 每{}分换发球\n\n\
            球桌规格:\n\
            - 长度: {}厘米\n\
            - 宽度: {}厘米\n\
            - 网高: {}厘米\n\n\
            发球规则:\n\
            1. 发球必须抛球16厘米以上\n\
            2. 发球必须先触及己方半区再过网\n\
            3. 不能遮挡发球\n\n\
            比赛规则:\n\
            1. 球触网后落入对方有效区算好球\n\
            2. 球触网后出界算失误\n\
            3. 接发球方先得分后换发球",
            self.game_win_score(),
            self.lead_requirement(),
            self.match_games(),
            (self.match_games() + 1) / 2,
            self.serve_change_interval(),
            self.table_dimensions().0,
            self.table_dimensions().1,
            self.net_height()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_tennis_win() {
        let rules = TableTennisRules::new();
        assert_eq!(rules.check_game_win(11, 9), Some(1));
        assert_eq!(rules.check_game_win(10, 10), None);
        assert_eq!(rules.check_game_win(13, 11), Some(1));
    }
}