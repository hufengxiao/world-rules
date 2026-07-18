//! 微缩模型游戏通用规则
//!
//! 提供微缩模型桌面游戏的通用规则框架。
//!
//! # 游戏要素
//!
//! - 模型规格（比例、尺寸）
//! - 地形和桌游垫
//! - 测量和移动
//! - 骰子和随机性
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::wargames::miniature_games::MiniatureGameRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = MiniatureGameRules::new();
//! assert!(rules.scale_types().len() >= 4);
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 模型比例
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelScale {
    /// 6mm（微缩战棋）
    SixMM,
    /// 15mm（战术战棋）
    FifteenMM,
    /// 28mm（桌面战棋）
    TwentyEightMM,
    /// 32mm（角色模型）
    ThirtyTwoMM,
    /// 54mm（收藏模型）
    FiftyFourMM,
}

impl ModelScale {
    /// 获取比例名称
    pub fn name(&self) -> &'static str {
        match self {
            ModelScale::SixMM => "6mm",
            ModelScale::FifteenMM => "15mm",
            ModelScale::TwentyEightMM => "28mm",
            ModelScale::ThirtyTwoMM => "32mm",
            ModelScale::FiftyFourMM => "54mm",
        }
    }

    /// 获取推荐桌面尺寸（英寸）
    pub fn recommended_table_size(&self) -> (u32, u32) {
        match self {
            ModelScale::SixMM => (36, 24),
            ModelScale::FifteenMM => (48, 36),
            ModelScale::TwentyEightMM => (72, 48),
            ModelScale::ThirtyTwoMM => (72, 48),
            ModelScale::FiftyFourMM => (36, 36),
        }
    }

    /// 获取测量单位转换（英寸/厘米）
    pub fn measurement_conversion(&self) -> &'static str {
        match self {
            ModelScale::SixMM => "1英寸 = 10cm",
            ModelScale::FifteenMM => "1英寸 = 5cm",
            ModelScale::TwentyEightMM => "1英寸 = 2cm",
            ModelScale::ThirtyTwoMM => "1英寸 = 2cm",
            ModelScale::FiftyFourMM => "1英寸 = 1cm",
        }
    }
}

/// 骰子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiceType {
    /// D6（六面骰）
    D6,
    /// D8（八面骰）
    D8,
    /// D10（十面骰）
    D10,
    /// D12（十二面骰）
    D12,
    /// D20（二十面骰）
    D20,
}

impl DiceType {
    /// 获取骰子名称
    pub fn name(&self) -> &'static str {
        match self {
            DiceType::D6 => "D6",
            DiceType::D8 => "D8",
            DiceType::D10 => "D10",
            DiceType::D12 => "D12",
            DiceType::D20 => "D20",
        }
    }

    /// 获取最大值
    pub fn max_value(&self) -> u32 {
        match self {
            DiceType::D6 => 6,
            DiceType::D8 => 8,
            DiceType::D10 => 10,
            DiceType::D12 => 12,
            DiceType::D20 => 20,
        }
    }

    /// 计算平均值
    pub fn average(&self) -> f64 {
        (self.max_value() + 1) as f64 / 2.0
    }
}

/// 地形元素类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainElement {
    /// 建筑物
    Building,
    /// 树林
    Woods,
    /// 山丘
    Hill,
    /// 水域
    WaterFeature,
    /// 防御工事
    Bunker,
    /// 废墟
    Ruins,
    /// 围墙
    Wall,
    /// 道路
    Road,
}

impl TerrainElement {
    /// 获取元素名称
    pub fn name(&self) -> &'static str {
        match self {
            TerrainElement::Building => "建筑物",
            TerrainElement::Woods => "树林",
            TerrainElement::Hill => "山丘",
            TerrainElement::WaterFeature => "水域",
            TerrainElement::Bunker => "防御工事",
            TerrainElement::Ruins => "废墟",
            TerrainElement::Wall => "围墙",
            TerrainElement::Road => "道路",
        }
    }

    /// 是否提供掩护
    pub fn provides_cover(&self) -> bool {
        matches!(
            self,
            TerrainElement::Building
                | TerrainElement::Woods
                | TerrainElement::Bunker
                | TerrainElement::Ruins
                | TerrainElement::Wall
        )
    }

    /// 是否阻碍视线
    pub fn blocks_line_of_sight(&self) -> bool {
        matches!(
            self,
            TerrainElement::Building | TerrainElement::Hill | TerrainElement::Bunker
        )
    }

    /// 是否可进入
    pub fn is_traversable(&self) -> bool {
        !matches!(self, TerrainElement::WaterFeature)
    }
}

/// 游戏类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    /// 小队级战斗
    Skirmish,
    /// 排级战斗
    Platoon,
    /// 连级战斗
    Company,
    /// 营级战斗
    Battalion,
}

impl GameType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            GameType::Skirmish => "小队级",
            GameType::Platoon => "排级",
            GameType::Company => "连级",
            GameType::Battalion => "营级",
        }
    }

    /// 获取推荐模型数量
    pub fn recommended_model_count(&self) -> (u32, u32) {
        match self {
            GameType::Skirmish => (5, 20),
            GameType::Platoon => (20, 50),
            GameType::Company => (50, 150),
            GameType::Battalion => (150, 500),
        }
    }

    /// 获取预计游戏时长（分钟）
    pub fn estimated_duration(&self) -> u32 {
        match self {
            GameType::Skirmish => 60,
            GameType::Platoon => 90,
            GameType::Company => 180,
            GameType::Battalion => 300,
        }
    }
}

/// 微缩模型游戏规则
#[derive(Debug, Clone)]
pub struct MiniatureGameRules {
    /// 规则元数据
    metadata: RuleMetadata,
}

impl MiniatureGameRules {
    /// 创建新的规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("微缩模型游戏规则", "微缩模型桌面游戏通用框架"),
        }
    }

    /// 获取所有模型比例
    pub fn scale_types(&self) -> Vec<ModelScale> {
        vec![
            ModelScale::SixMM,
            ModelScale::FifteenMM,
            ModelScale::TwentyEightMM,
            ModelScale::ThirtyTwoMM,
            ModelScale::FiftyFourMM,
        ]
    }

    /// 获取所有骰子类型
    pub fn dice_types(&self) -> Vec<DiceType> {
        vec![
            DiceType::D6,
            DiceType::D8,
            DiceType::D10,
            DiceType::D12,
            DiceType::D20,
        ]
    }

    /// 获取所有地形元素
    pub fn terrain_elements(&self) -> Vec<TerrainElement> {
        vec![
            TerrainElement::Building,
            TerrainElement::Woods,
            TerrainElement::Hill,
            TerrainElement::WaterFeature,
            TerrainElement::Bunker,
            TerrainElement::Ruins,
            TerrainElement::Wall,
            TerrainElement::Road,
        ]
    }

    /// 获取所有游戏类型
    pub fn game_types(&self) -> Vec<GameType> {
        vec![
            GameType::Skirmish,
            GameType::Platoon,
            GameType::Company,
            GameType::Battalion,
        ]
    }

    /// 检查模型是否可进入地形
    pub fn can_enter_terrain(&self, terrain: TerrainElement) -> bool {
        terrain.is_traversable()
    }

    /// 计算距离修正（考虑地形）
    pub fn distance_modifier(&self, base_distance: u32, terrain: TerrainElement) -> u32 {
        match terrain {
            TerrainElement::Woods => base_distance / 2,
            TerrainElement::Hill => base_distance,
            _ => base_distance,
        }
    }

    /// 获取测量工具建议
    pub fn measurement_tools(&self, _scale: ModelScale) -> Vec<&'static str> {
        vec![
            "卷尺（英寸）",
            "直尺（英寸）",
            "测量棒",
            "模板（扇形/圆形）",
        ]
    }

    /// 获取基础装备清单
    pub fn essential_equipment(&self) -> Vec<&'static str> {
        vec![
            "卷尺",
            "骰子（D6/D8/D10/D12/D20）",
            "模型",
            "地形元素",
            "规则书",
            "数据卡",
            "标记物/代币",
            "桌游垫",
        ]
    }

    /// 计算成功概率
    pub fn success_probability(&self, dice: DiceType, target: u32) -> f64 {
        if target > dice.max_value() {
            return 0.0;
        }
        (dice.max_value() - target + 1) as f64 / dice.max_value() as f64
    }
}

impl Default for MiniatureGameRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MiniatureGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("wargames")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_types() {
        let rules = MiniatureGameRules::new();
        let scales = rules.scale_types();
        assert_eq!(scales.len(), 5);
        assert_eq!(ModelScale::TwentyEightMM.name(), "28mm");
    }

    #[test]
    fn test_dice_types() {
        let rules = MiniatureGameRules::new();
        let dice = rules.dice_types();
        assert_eq!(dice.len(), 5);
        assert_eq!(DiceType::D6.max_value(), 6);
        assert!((DiceType::D20.average() - 10.5).abs() < 0.01);
    }

    #[test]
    fn test_terrain_elements() {
        let rules = MiniatureGameRules::new();
        let terrain = rules.terrain_elements();
        assert_eq!(terrain.len(), 8);
        assert!(TerrainElement::Building.provides_cover());
        assert!(TerrainElement::Hill.blocks_line_of_sight());
        assert!(!TerrainElement::WaterFeature.is_traversable());
    }

    #[test]
    fn test_game_types() {
        let rules = MiniatureGameRules::new();
        let types = rules.game_types();
        assert_eq!(types.len(), 4);
        let (min, max) = GameType::Skirmish.recommended_model_count();
        assert!(min <= max);
    }

    #[test]
    fn test_success_probability() {
        let rules = MiniatureGameRules::new();
        // D6 需要 4+
        let prob = rules.success_probability(DiceType::D6, 4);
        assert!((prob - 0.5).abs() < 0.01);
        // D20 需要 11+
        let prob = rules.success_probability(DiceType::D20, 11);
        assert!((prob - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_distance_modifier() {
        let rules = MiniatureGameRules::new();
        let modified = rules.distance_modifier(10, TerrainElement::Woods);
        assert_eq!(modified, 5);
    }

    #[test]
    fn test_can_enter_terrain() {
        let rules = MiniatureGameRules::new();
        assert!(rules.can_enter_terrain(TerrainElement::Building));
        assert!(!rules.can_enter_terrain(TerrainElement::WaterFeature));
    }
}
