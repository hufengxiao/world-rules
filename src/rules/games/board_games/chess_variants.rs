//! 象棋变体规则
//!
//! 支持多种象棋类游戏规则：
//! - 国际象棋（Standard Chess）
//! - 日本将棋（Shogi）
//! - 韩国象棋（Janggi）

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 象棋变体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChessVariant {
    /// 国际象棋（标准 FIDE 规则）
    /// - 8x8 棋盘
    /// - 64 格、32 枚棋子
    /// - 王车易位、吃过路兵、兵升变
    International,
    /// 日本将棋（Shogi）
    /// - 9x9 棋盘
    /// - 40 枚棋子
    /// - 打入规则、升变
    Shogi,
    /// 韩国象棋（Janggi）
    /// - 9x10 棋盘
    /// - 32 枚棋子
    /// - 宫殿斜线、开局选择象马位置
    Janggi,
}

impl ChessVariant {
    /// 获取变体名称
    pub fn name(&self) -> &'static str {
        match self {
            ChessVariant::International => "国际象棋",
            ChessVariant::Shogi => "日本将棋",
            ChessVariant::Janggi => "韩国象棋",
        }
    }

    /// 获取英文名称
    pub fn english_name(&self) -> &'static str {
        match self {
            ChessVariant::International => "Chess",
            ChessVariant::Shogi => "Shogi",
            ChessVariant::Janggi => "Janggi",
        }
    }

    /// 获取棋盘尺寸
    pub fn board_size(&self) -> (u8, u8) {
        match self {
            ChessVariant::International => (8, 8),
            ChessVariant::Shogi => (9, 9),
            ChessVariant::Janggi => (9, 10),
        }
    }

    /// 获取起源地
    pub fn origin(&self) -> &'static str {
        match self {
            ChessVariant::International => "印度/波斯",
            ChessVariant::Shogi => "日本",
            ChessVariant::Janggi => "朝鲜",
        }
    }
}

/// 象棋变体规则集合
pub struct ChessVariantsRules {
    metadata: RuleMetadata,
}

impl ChessVariantsRules {
    /// 创建新实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("象棋变体规则", "国际象棋、日本将棋、韩国象棋规则合集")
                .with_origin("国际")
                .with_tags(vec!["游戏".into(), "棋类".into(), "象棋".into()]),
        }
    }

    /// 国际象棋棋子类型
    pub fn international_pieces(&self) -> Vec<&'static str> {
        vec![
            "王(King) - 核心棋子，被将死即输",
            "后(Queen) - 最强棋子，横竖斜任意距离",
            "车(Rook) - 直线移动任意距离",
            "象(Bishop) - 斜线移动任意距离",
            "马(Knight) - L形跳跃，可越子",
            "兵(Pawn) - 前进一格，首步可两格，斜吃",
        ]
    }

    /// 国际象棋特殊规则
    pub fn international_special(&self) -> Vec<&'static str> {
        vec![
            "王车易位(Castling): 王向车移动两格，车跳到王旁",
            "条件: 王和车未移动，中间无子，王不被将军",
            "吃过路兵(En Passant): 兵首步两格时可被斜吃",
            "兵升变(Promotion): 兵到底线升为后/车/象/马",
            "将死(Checkmate): 王被将军且无法逃脱",
            "逼和(Stalemate): 无合法走法但未被将军",
        ]
    }

    /// 日本将棋棋子类型
    pub fn shogi_pieces(&self) -> Vec<&'static str> {
        vec![
            "王将/玉将(King) - 核心棋子",
            "飞车(Rook) - 横竖任意距离，升变为龙",
            "角行(Bishop) - 斜向任意距离，升变为马",
            "金将(Gold) - 周围6格",
            "银将(Silver) - 前方和斜向5格",
            "桂马(Knight) - 前方跳跃",
            "香车(Lance) - 前方任意距离",
            "步兵(Pawn) - 前方一格",
        ]
    }

    /// 日本将棋特殊规则
    pub fn shogi_special(&self) -> Vec<&'static str> {
        vec![
            "打入(Drop): 吃掉的棋子可放回棋盘",
            "升变(Promotion): 进入敌方最后三排可升变",
            "禁止打入步兵直接将死",
            "二步禁止: 同一列不能有两个未升变的步兵",
            "千日手: 同一局面重复4次判和",
        ]
    }

    /// 韩国象棋棋子类型
    pub fn janggi_pieces(&self) -> Vec<&'static str> {
        vec![
            "将/楚(King) - 宫内移动，可斜走",
            "士(Advisor) - 宫内移动，可斜走",
            "象(Elephant) - 先直后斜走两格",
            "车(Chariot) - 直线任意距离",
            "马(Horse) - L形跳跃",
            "炮(Cannon) - 跳过棋子移动或吃子",
            "兵/卒(Soldier) - 前进或横走一格",
        ]
    }

    /// 韩国象棋特殊规则
    pub fn janggi_special(&self) -> Vec<&'static str> {
        vec![
            "无河流限制(象可过河)",
            "炮可跳过棋子移动(不一定吃子)",
            "将可在宫内斜走",
            "兵可横向移动",
            "开局可选择象马布局位置",
            "宫殿有斜线连接(允许斜走)",
        ]
    }

    /// 变体对比
    pub fn comparison(&self) -> Vec<&'static str> {
        vec![
            "棋盘大小: 国际象棋8x8, 将棋9x9, 韩国象棋9x10",
            "棋子数量: 国际象棋32枚, 将棋40枚, 韩国象棋32枚",
            "打入规则: 仅将棋有打入机制",
            "升变: 国际象棋仅兵可升变, 将棋多子可升变",
            "宫殿: 仅韩国象棋有宫殿斜线",
            "河流: 仅中国象棋有河流限制",
        ]
    }
}

impl Default for ChessVariantsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChessVariantsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess_variants")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "象棋变体规则",
            &[
                ("国际象棋棋子", &self.international_pieces()),
                ("国际象棋特殊规则", &self.international_special()),
                ("日本将棋棋子", &self.shogi_pieces()),
                ("日本将棋特殊规则", &self.shogi_special()),
                ("韩国象棋棋子", &self.janggi_pieces()),
                ("韩国象棋特殊规则", &self.janggi_special()),
                ("变体对比", &self.comparison()),
            ],
        )
    }
}

/// 国际象棋变体规则
pub struct InternationalChessRules {
    metadata: RuleMetadata,
}

impl InternationalChessRules {
    /// 创建国际象棋规则实例
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::games::board_games::chess_variants::InternationalChessRules;
    /// use world_rules::rules::core::Rule;
    ///
    /// let rules = InternationalChessRules::new();
    /// assert_eq!(rules.metadata().name, "国际象棋规则");
    /// ```
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("国际象棋规则", "FIDE标准国际象棋规则")
                .with_origin("国际")
                .with_tags(vec!["游戏".into(), "棋类".into(), "国际象棋".into()]),
        }
    }
}

impl Default for InternationalChessRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for InternationalChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("international_chess")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        let variants = ChessVariantsRules::new();
        crate::rules::core::format_rule_sections(
            "国际象棋规则",
            &[
                ("棋盘", &vec!["8x8棋盘，64格", "白方先行", "交替走棋"]),
                ("棋子走法", &variants.international_pieces()),
                ("特殊规则", &variants.international_special()),
            ],
        )
    }
}

/// 日本将棋变体规则
pub struct ShogiVariantRules {
    metadata: RuleMetadata,
}

impl ShogiVariantRules {
    /// 创建日本将棋规则实例
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::games::board_games::chess_variants::ShogiVariantRules;
    /// use world_rules::rules::core::Rule;
    ///
    /// let rules = ShogiVariantRules::new();
    /// assert_eq!(rules.metadata().name, "日本将棋规则");
    /// ```
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("日本将棋规则", "日本将棋完整规则")
                .with_origin("日本")
                .with_tags(vec!["游戏".into(), "棋类".into(), "将棋".into()]),
        }
    }
}

impl Default for ShogiVariantRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShogiVariantRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("shogi_variant")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        let variants = ChessVariantsRules::new();
        crate::rules::core::format_rule_sections(
            "日本将棋规则",
            &[
                (
                    "棋盘",
                    &vec!["9x9棋盘", "双方各20枚棋子", "棋子初始位置固定"],
                ),
                ("棋子", &variants.shogi_pieces()),
                ("特殊规则", &variants.shogi_special()),
            ],
        )
    }
}

/// 韩国象棋变体规则
pub struct JanggiVariantRules {
    metadata: RuleMetadata,
}

impl JanggiVariantRules {
    /// 创建韩国象棋规则实例
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::games::board_games::chess_variants::JanggiVariantRules;
    /// use world_rules::rules::core::Rule;
    ///
    /// let rules = JanggiVariantRules::new();
    /// assert_eq!(rules.metadata().name, "韩国象棋规则");
    /// ```
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("韩国象棋规则", "韩国象棋(Janggi)完整规则")
                .with_origin("朝鲜")
                .with_tags(vec!["游戏".into(), "棋类".into(), "韩国象棋".into()]),
        }
    }
}

impl Default for JanggiVariantRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for JanggiVariantRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("janggi_variant")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        let variants = ChessVariantsRules::new();
        crate::rules::core::format_rule_sections(
            "韩国象棋规则",
            &[
                ("棋盘", &vec!["9x10棋盘", "宫殿3x3有斜线", "无河流分隔"]),
                ("棋子", &variants.janggi_pieces()),
                ("特殊规则", &variants.janggi_special()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chess_variants() {
        let rules = ChessVariantsRules::new();
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("国际象棋"));
        assert!(rules.explain().contains("将棋"));
        assert!(rules.explain().contains("韩国象棋"));
    }

    #[test]
    fn test_variant_types() {
        assert_eq!(ChessVariant::International.name(), "国际象棋");
        assert_eq!(ChessVariant::Shogi.name(), "日本将棋");
        assert_eq!(ChessVariant::Janggi.name(), "韩国象棋");
    }

    #[test]
    fn test_board_sizes() {
        assert_eq!(ChessVariant::International.board_size(), (8, 8));
        assert_eq!(ChessVariant::Shogi.board_size(), (9, 9));
        assert_eq!(ChessVariant::Janggi.board_size(), (9, 10));
    }

    #[test]
    fn test_international_chess() {
        let rules = InternationalChessRules::new();
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("王车易位"));
    }

    #[test]
    fn test_shogi_variant() {
        let rules = ShogiVariantRules::new();
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("打入"));
    }

    #[test]
    fn test_janggi_variant() {
        let rules = JanggiVariantRules::new();
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("宫殿") || rules.explain().contains("无河流"));
    }
}
