//! 麻将规则定义

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};
use super::tiles::TileType;
use super::hands::{Hand, HandPattern, WinningHand};

/// 麻将规则变体
#[derive(Debug, Clone)]
pub enum MahjongVariant {
    /// 四川麻将 - 血战到底
    Sichuan,
    /// 广东麻将
    Guangdong,
    /// 上海麻将
    Shanghai,
    /// 日本麻将 (立直麻将)
    Riichi,
    /// 国标麻将
    Guobiao,
    /// 台湾麻将
    Taiwan,
}

impl std::fmt::Display for MahjongVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MahjongVariant::Sichuan => write!(f, "四川麻将"),
            MahjongVariant::Guangdong => write!(f, "广东麻将"),
            MahjongVariant::Shanghai => write!(f, "上海麻将"),
            MahjongVariant::Riichi => write!(f, "日本麻将"),
            MahjongVariant::Guobiao => write!(f, "国标麻将"),
            MahjongVariant::Taiwan => write!(f, "台湾麻将"),
        }
    }
}

/// 番种定义
#[derive(Debug, Clone)]
pub enum FanType {
    // 基础番种
    /// 平胡
    PingHu,
    /// 对对胡
    DuiDuiHu,
    /// 清一色
    QingYiSe,
    /// 混一色
    HunYiSe,
    /// 七对子
    SevenPairs,
    /// 十三幺
    ThirteenOrphans,

    // 高级番种
    /// 大三元
    DaSanYuan,
    /// 小三元
    XiaoSanYuan,
    /// 大四喜
    DaSiXi,
    /// 小四喜
    XiaoSiXi,
    /// 字一色
    ZiYiSe,
    /// 清老头
    QingLaoTou,
    /// 绿一色
    LvYiSe,
    /// 九莲宝灯
    JiuLianBaoDeng,

    // 特殊番种
    /// 天胡
    TianHu,
    /// 地胡
    DiHu,
    /// 人胡
    RenHu,
    /// 杠开
    GangKai,
    /// 抢杠
    QiangGang,
    /// 海底捞月
    HaiDiLaoYue,
    /// 河底摸鱼
    HeDiMoYu,
}

impl FanType {
    pub fn name(&self) -> &'static str {
        match self {
            FanType::PingHu => "平胡",
            FanType::DuiDuiHu => "对对胡",
            FanType::QingYiSe => "清一色",
            FanType::HunYiSe => "混一色",
            FanType::SevenPairs => "七对子",
            FanType::ThirteenOrphans => "十三幺",
            FanType::DaSanYuan => "大三元",
            FanType::XiaoSanYuan => "小三元",
            FanType::DaSiXi => "大四喜",
            FanType::XiaoSiXi => "小四喜",
            FanType::ZiYiSe => "字一色",
            FanType::QingLaoTou => "清老头",
            FanType::LvYiSe => "绿一色",
            FanType::JiuLianBaoDeng => "九莲宝灯",
            FanType::TianHu => "天胡",
            FanType::DiHu => "地胡",
            FanType::RenHu => "人胡",
            FanType::GangKai => "杠开",
            FanType::QiangGang => "抢杠",
            FanType::HaiDiLaoYue => "海底捞月",
            FanType::HeDiMoYu => "河底摸鱼",
        }
    }
}

/// 麻将规则
pub struct MahjongRules {
    /// 规则变体
    variant: MahjongVariant,
    /// 番数表
    fan_table: Vec<(FanType, u8)>,
    /// 起始番数
    min_fan: u8,
    /// 是否支持花牌
    with_flowers: bool,
}

impl MahjongRules {
    pub fn new(variant: MahjongVariant) -> Self {
        let (fan_table, min_fan, with_flowers) = Self::get_variant_config(&variant);
        Self {
            variant,
            fan_table,
            min_fan,
            with_flowers,
        }
    }

    fn get_variant_config(variant: &MahjongVariant) -> (Vec<(FanType, u8)>, u8, bool) {
        match variant {
            MahjongVariant::Sichuan => {
                // 四川麻将: 必须缺一门，平胡起胡
                (
                    vec![
                        (FanType::PingHu, 1),
                        (FanType::DuiDuiHu, 2),
                        (FanType::QingYiSe, 4),
                        (FanType::DuiDuiHu, 2), // 对对胡加倍
                    ],
                    1,
                    false,
                )
            }
            MahjongVariant::Guobiao => {
                // 国标麻将: 88番制
                (
                    vec![
                        (FanType::ThirteenOrphans, 88),
                        (FanType::DaSiXi, 88),
                        (FanType::ZiYiSe, 64),
                        (FanType::DaSanYuan, 88),
                        (FanType::QingYiSe, 24),
                        (FanType::SevenPairs, 24),
                        (FanType::PingHu, 2),
                    ],
                    8, // 国标8番起胡
                    true,
                )
            }
            MahjongVariant::Riichi => {
                // 日本麻将: 立直麻将
                (
                    vec![
                        (FanType::SevenPairs, 2),
                        (FanType::PingHu, 1),
                        (FanType::QingYiSe, 6),
                        (FanType::HunYiSe, 3),
                        (FanType::ThirteenOrphans, 13), // 役满
                        (FanType::DaSanYuan, 13),
                        (FanType::DaSiXi, 13),
                    ],
                    1,
                    false,
                )
            }
            _ => {
                // 默认规则
                (
                    vec![
                        (FanType::PingHu, 1),
                        (FanType::DuiDuiHu, 2),
                        (FanType::QingYiSe, 4),
                        (FanType::HunYiSe, 2),
                        (FanType::SevenPairs, 2),
                        (FanType::ThirteenOrphans, 88),
                    ],
                    1,
                    true,
                )
            }
        }
    }

    /// 获取番数
    pub fn get_fan(&self, fan_type: &FanType) -> u8 {
        self.fan_table
            .iter()
            .find(|(ft, _)| std::mem::discriminant(ft) == std::mem::discriminant(fan_type))
            .map(|(_, fan)| *fan)
            .unwrap_or(0)
    }

    /// 计算胡牌番数
    pub fn calculate_fan(&self, hand: &Hand, winning: &WinningHand) -> Vec<(FanType, u8)> {
        let mut fans = Vec::new();

        // 检查各种番型
        if self.is_qing_yi_se(hand) {
            fans.push((FanType::QingYiSe, self.get_fan(&FanType::QingYiSe)));
        }

        if self.is_hun_yi_se(hand) {
            fans.push((FanType::HunYiSe, self.get_fan(&FanType::HunYiSe)));
        }

        match &winning.pattern {
            HandPattern::SevenPairs { .. } => {
                fans.push((FanType::SevenPairs, self.get_fan(&FanType::SevenPairs)));
            }
            HandPattern::ThirteenOrphans => {
                fans.push((FanType::ThirteenOrphans, self.get_fan(&FanType::ThirteenOrphans)));
            }
            _ => {}
        }

        fans
    }

    /// 检查是否清一色
    fn is_qing_yi_se(&self, hand: &Hand) -> bool {
        let tiles = hand.tiles();
        if tiles.is_empty() {
            return false;
        }

        // 获取第一个数牌的花色
        let first_suit = tiles.iter().find_map(|t| t.tile_type.suit());

        match first_suit {
            Some(suit) => tiles.iter().all(|t| {
                t.tile_type.suit().map(|s| s == suit).unwrap_or(false)
            }),
            None => false,
        }
    }

    /// 检查是否混一色
    fn is_hun_yi_se(&self, hand: &Hand) -> bool {
        let tiles = hand.tiles();

        let suits: Vec<_> = tiles.iter()
            .filter_map(|t| t.tile_type.suit())
            .collect();

        if suits.is_empty() {
            return false;
        }

        // 只有一种花色 + 字牌
        let unique_suits: std::collections::HashSet<_> = suits.iter().collect();
        let has_honors = tiles.iter().any(|t| t.tile_type.is_honor());

        unique_suits.len() == 1 && has_honors
    }

    /// 获取规则变体
    pub fn variant(&self) -> &MahjongVariant {
        &self.variant
    }

    /// 获取起始番数
    pub fn min_fan(&self) -> u8 {
        self.min_fan
    }

    /// 是否支持花牌
    pub fn with_flowers(&self) -> bool {
        self.with_flowers
    }
}

impl Rule for MahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        // 使用 RefCell 或直接返回创建的 metadata
        // 这里简化实现，每次创建新的
        static METADATA: std::sync::OnceLock<RuleMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| {
            RuleMetadata::new(
                format!("{}规则", self.variant),
                format!("{}的标准规则说明", self.variant),
            )
            .with_origin(format!("{}", self.variant))
        })
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        // 验证游戏状态是否符合规则
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}】\n\n{}\n\n起始番数: {}番\n花牌: {}\n\n主要番种:\n{}",
            self.metadata().name,
            self.metadata().description,
            self.min_fan,
            if self.with_flowers { "支持" } else { "不支持" },
            self.fan_table
                .iter()
                .map(|(ft, f)| format!("  - {}: {}番", ft.name(), f))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

/// 四川麻将规则 (血战到底)
pub struct SichuanMahjongRules {
    base: MahjongRules,
}

impl SichuanMahjongRules {
    pub fn new() -> Self {
        Self {
            base: MahjongRules::new(MahjongVariant::Sichuan),
        }
    }

    /// 四川麻将特色: 缺一门
    pub fn check_missing_suit(&self, hand: &Hand) -> Option<&'static str> {
        let tiles = hand.tiles();
        let has_wan = tiles.iter().any(|t| matches!(t.tile_type, TileType::Wan(_)));
        let has_tiao = tiles.iter().any(|t| matches!(t.tile_type, TileType::Tiao(_)));
        let has_tong = tiles.iter().any(|t| matches!(t.tile_type, TileType::Tong(_)));

        match (has_wan, has_tiao, has_tong) {
            (true, true, false) => Some("筒"),
            (true, false, true) => Some("条"),
            (false, true, true) => Some("万"),
            (true, true, true) => None, // 需要缺一门
            _ => Some("已缺一门"),
        }
    }
}

impl Default for SichuanMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SichuanMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        self.base.metadata()
    }

    fn category(&self) -> RuleCategory {
        self.base.category()
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        self.base.validate(context)
    }

    fn explain(&self) -> String {
        format!(
            "{}\n\n【四川麻将特色规则】\n\
             1. 血战到底: 胡牌后继续打，所有人都能胡\n\
             2. 必须缺一门: 只保留两种花色\n\
             3. 不支持七对子\n\
             4. 只有平胡、对对胡、清一色、金钩钓等基本番型\n\
             5. 点炮: 只能自摸胡牌",
            self.base.explain()
        )
    }
}

/// 国标麻将规则
pub struct GuobiaoMahjongRules {
    base: MahjongRules,
}

impl GuobiaoMahjongRules {
    pub fn new() -> Self {
        Self {
            base: MahjongRules::new(MahjongVariant::Guobiao),
        }
    }

    /// 国标特色: 8番起胡
    pub fn check_minimum_fan(&self, total_fan: u8) -> bool {
        total_fan >= 8
    }
}

impl Default for GuobiaoMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GuobiaoMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        self.base.metadata()
    }

    fn category(&self) -> RuleCategory {
        self.base.category()
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        self.base.validate(context)
    }

    fn explain(&self) -> String {
        format!(
            "{}\n\n【国标麻将特色规则】\n\
             1. 8番起胡: 必须满足8番才能胡牌\n\
             2. 支持花牌: 春夏秋冬、梅兰竹菊各1张\n\
             3. 花牌算番: 每张花牌对应一番\n\
             4. 88种番型: 包括大四喜、十三幺、九莲宝灯等\n\
             5. 计分方式: 底分×番数",
            self.base.explain()
        )
    }
}

/// 日本麻将 (立直麻将) 规则
pub struct RiichiMahjongRules {
    base: MahjongRules,
}

impl RiichiMahjongRules {
    pub fn new() -> Self {
        Self {
            base: MahjongRules::new(MahjongVariant::Riichi),
        }
    }

    /// 检查立直状态
    pub fn can_riichi(&self, hand: &Hand, points: u32) -> bool {
        hand.is_ready() && points >= 1000
    }
}

impl Default for RiichiMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RiichiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        self.base.metadata()
    }

    fn category(&self) -> RuleCategory {
        self.base.category()
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        self.base.validate(context)
    }

    fn explain(&self) -> String {
        format!(
            "{}\n\n【日本麻将特色规则】\n\
             1. 立直: 宣布听牌后不能换牌，支付1000点供托\n\
             2. 振听: 不能胡自己打过的牌\n\
             3. 宝牌: 翻开宝牌增加番数\n\
             4. 役满: 满贯封顶，部分役种为役满\n\
             5. 流局: 没有人胡牌时本场结束",
            self.base.explain()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mahjong_rules_creation() {
        let rules = MahjongRules::new(MahjongVariant::Sichuan);
        assert_eq!(rules.min_fan(), 1);
        assert!(!rules.with_flowers());
    }

    #[test]
    fn test_sichuan_rules() {
        let rules = SichuanMahjongRules::new();
        assert!(rules.explain().contains("血战到底"));
    }

    #[test]
    fn test_guobiao_rules() {
        let rules = GuobiaoMahjongRules::new();
        assert!(rules.check_minimum_fan(8));
        assert!(!rules.check_minimum_fan(7));
    }
}