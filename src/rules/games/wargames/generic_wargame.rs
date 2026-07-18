//! 通用战棋游戏规则
//!
//! 提供回合制战棋游戏的通用规则框架。
//!
//! # 游戏类型
//!
//! - 六角格战棋
//! - 区域控制战棋
//! - 大战略战棋
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::wargames::generic_wargame::GenericWargameRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = GenericWargameRules::new();
//! assert!(rules.terrain_types().len() >= 4);
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 地形类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainType {
    /// 平原
    Plain,
    /// 森林
    Forest,
    /// 山地
    Mountain,
    /// 水域
    Water,
    /// 沼泽
    Swamp,
    /// 沙漠
    Desert,
    /// 城市
    Urban,
    /// 要塞
    Fortress,
}

impl TerrainType {
    /// 获取地形名称
    pub fn name(&self) -> &'static str {
        match self {
            TerrainType::Plain => "平原",
            TerrainType::Forest => "森林",
            TerrainType::Mountain => "山地",
            TerrainType::Water => "水域",
            TerrainType::Swamp => "沼泽",
            TerrainType::Desert => "沙漠",
            TerrainType::Urban => "城市",
            TerrainType::Fortress => "要塞",
        }
    }

    /// 获取移动修正
    pub fn movement_modifier(&self) -> i32 {
        match self {
            TerrainType::Plain => 0,
            TerrainType::Forest => -1,
            TerrainType::Mountain => -2,
            TerrainType::Water => -3,
            TerrainType::Swamp => -2,
            TerrainType::Desert => -1,
            TerrainType::Urban => 0,
            TerrainType::Fortress => 0,
        }
    }

    /// 获取防御加值
    pub fn defense_bonus(&self) -> i32 {
        match self {
            TerrainType::Plain => 0,
            TerrainType::Forest => 1,
            TerrainType::Mountain => 2,
            TerrainType::Water => 0,
            TerrainType::Swamp => 0,
            TerrainType::Desert => 0,
            TerrainType::Urban => 1,
            TerrainType::Fortress => 3,
        }
    }
}

/// 单位类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitCategory {
    /// 步兵
    Infantry,
    /// 骑兵
    Cavalry,
    /// 远程单位
    Ranged,
    /// 炮兵
    Artillery,
    /// 车辆
    Vehicle,
    /// 空中单位
    AirUnit,
    /// 海军单位
    NavalUnit,
    /// 指挥单位
    Command,
}

impl UnitCategory {
    /// 获取类别名称
    pub fn name(&self) -> &'static str {
        match self {
            UnitCategory::Infantry => "步兵",
            UnitCategory::Cavalry => "骑兵",
            UnitCategory::Ranged => "远程单位",
            UnitCategory::Artillery => "炮兵",
            UnitCategory::Vehicle => "车辆",
            UnitCategory::AirUnit => "空中单位",
            UnitCategory::NavalUnit => "海军单位",
            UnitCategory::Command => "指挥单位",
        }
    }

    /// 是否可进入水域
    pub fn can_enter_water(&self) -> bool {
        matches!(self, UnitCategory::NavalUnit | UnitCategory::AirUnit)
    }

    /// 是否有远程攻击能力
    pub fn has_ranged_attack(&self) -> bool {
        matches!(
            self,
            UnitCategory::Ranged | UnitCategory::Artillery | UnitCategory::AirUnit
        )
    }
}

/// 战斗结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatResult {
    /// 完全胜利
    DecisiveVictory,
    /// 胜利
    Victory,
    /// 惨胜
    MarginalVictory,
    /// 平局
    Draw,
    /// 失败
    Defeat,
}

impl CombatResult {
    /// 获取结果名称
    pub fn name(&self) -> &'static str {
        match self {
            CombatResult::DecisiveVictory => "完全胜利",
            CombatResult::Victory => "胜利",
            CombatResult::MarginalVictory => "惨胜",
            CombatResult::Draw => "平局",
            CombatResult::Defeat => "失败",
        }
    }

    /// 是否为胜利
    pub fn is_victory(&self) -> bool {
        matches!(
            self,
            CombatResult::DecisiveVictory | CombatResult::Victory | CombatResult::MarginalVictory
        )
    }
}

/// 回合阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    /// 初始阶段
    Initiative,
    /// 命令阶段
    Command,
    /// 移动阶段
    Movement,
    /// 战斗阶段
    Combat,
    /// 结算阶段
    Resolution,
}

impl TurnPhase {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            TurnPhase::Initiative => "初始阶段",
            TurnPhase::Command => "命令阶段",
            TurnPhase::Movement => "移动阶段",
            TurnPhase::Combat => "战斗阶段",
            TurnPhase::Resolution => "结算阶段",
        }
    }
}

/// 通用战棋规则
#[derive(Debug, Clone)]
pub struct GenericWargameRules {
    /// 规则元数据
    metadata: RuleMetadata,
}

impl GenericWargameRules {
    /// 创建新的规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("通用战棋规则", "回合制战棋游戏通用框架"),
        }
    }

    /// 获取所有地形类型
    pub fn terrain_types(&self) -> Vec<TerrainType> {
        vec![
            TerrainType::Plain,
            TerrainType::Forest,
            TerrainType::Mountain,
            TerrainType::Water,
            TerrainType::Swamp,
            TerrainType::Desert,
            TerrainType::Urban,
            TerrainType::Fortress,
        ]
    }

    /// 获取所有单位类别
    pub fn unit_categories(&self) -> Vec<UnitCategory> {
        vec![
            UnitCategory::Infantry,
            UnitCategory::Cavalry,
            UnitCategory::Ranged,
            UnitCategory::Artillery,
            UnitCategory::Vehicle,
            UnitCategory::AirUnit,
            UnitCategory::NavalUnit,
            UnitCategory::Command,
        ]
    }

    /// 获取所有回合阶段
    pub fn turn_phases(&self) -> Vec<TurnPhase> {
        vec![
            TurnPhase::Initiative,
            TurnPhase::Command,
            TurnPhase::Movement,
            TurnPhase::Combat,
            TurnPhase::Resolution,
        ]
    }

    /// 计算战斗优势
    pub fn calculate_combat_odds(&self, attacker_strength: u32, defender_strength: u32) -> f64 {
        attacker_strength as f64 / defender_strength.max(1) as f64
    }

    /// 确定战斗结果
    pub fn determine_combat_result(&self, odds_ratio: f64, die_roll: u32) -> CombatResult {
        // 简化的战斗结果表
        let adjusted = die_roll as f64 + (odds_ratio - 1.0) * 2.0;
        if adjusted >= 10.0 {
            CombatResult::DecisiveVictory
        } else if adjusted >= 7.0 {
            CombatResult::Victory
        } else if adjusted >= 5.0 {
            CombatResult::MarginalVictory
        } else if adjusted >= 3.0 {
            CombatResult::Draw
        } else {
            CombatResult::Defeat
        }
    }

    /// 获取默认回合数
    pub fn default_rounds(&self) -> u32 {
        10
    }

    /// 获取胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "消灭敌方所有单位",
            "控制关键目标点",
            "达成战略目标",
            "敌方投降",
        ]
    }

    /// 检查单位是否可移动到目标地形
    pub fn can_enter_terrain(&self, unit: UnitCategory, terrain: TerrainType) -> bool {
        if terrain == TerrainType::Water && !unit.can_enter_water() {
            return false;
        }
        true
    }
}

impl Default for GenericWargameRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GenericWargameRules {
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
    fn test_terrain_types() {
        let rules = GenericWargameRules::new();
        let terrains = rules.terrain_types();
        assert_eq!(terrains.len(), 8);
        assert_eq!(TerrainType::Forest.movement_modifier(), -1);
        assert_eq!(TerrainType::Fortress.defense_bonus(), 3);
    }

    #[test]
    fn test_unit_categories() {
        let rules = GenericWargameRules::new();
        let units = rules.unit_categories();
        assert_eq!(units.len(), 8);
        assert!(!UnitCategory::Infantry.can_enter_water());
        assert!(UnitCategory::Artillery.has_ranged_attack());
    }

    #[test]
    fn test_combat_odds() {
        let rules = GenericWargameRules::new();
        let odds = rules.calculate_combat_odds(10, 5);
        assert!((odds - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_combat_result() {
        let rules = GenericWargameRules::new();
        let result = rules.determine_combat_result(2.0, 6);
        assert!(result.is_victory());
    }

    #[test]
    fn test_turn_phases() {
        let rules = GenericWargameRules::new();
        let phases = rules.turn_phases();
        assert_eq!(phases.len(), 5);
    }

    #[test]
    fn test_can_enter_terrain() {
        let rules = GenericWargameRules::new();
        assert!(!rules.can_enter_terrain(UnitCategory::Infantry, TerrainType::Water));
        assert!(rules.can_enter_terrain(UnitCategory::NavalUnit, TerrainType::Water));
    }
}