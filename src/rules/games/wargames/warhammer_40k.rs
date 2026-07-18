//! 战锤40,000 (Warhammer 40,000) 规则
//!
//! 由 Games Workshop 出品的科幻桌面战争游戏，设定在第41个千年的黑暗宇宙。
//!
//! # 游戏概述
//!
//! - **类型**：回合制微缩模型战争游戏
//! - **规模**：小队到军团级战斗
//! - **核心机制**：交替回合、骰子检定、点数系统
//!
//! # 主要势力
//!
//! - 帝国势力：星际战士、帝国卫队、机械教
//! - 异形势力：兽人、艾达灵族、泰伦虫族
//! - 混沌势力：混沌星际战士、混沌恶魔
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::wargames::warhammer_40k::Warhammer40KRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = Warhammer40KRules::new();
//! assert_eq!(rules.game_phases().len(), 5);
//! assert!(rules.default_points_limit() > 0);
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 游戏阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamePhase {
    /// 指挥阶段
    Command,
    /// 移动阶段
    Movement,
    /// 射击阶段
    Shooting,
    /// 冲锋阶段
    Charge,
    /// 战斗阶段
    Fight,
}

impl GamePhase {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            GamePhase::Command => "指挥阶段",
            GamePhase::Movement => "移动阶段",
            GamePhase::Shooting => "射击阶段",
            GamePhase::Charge => "冲锋阶段",
            GamePhase::Fight => "战斗阶段",
        }
    }

    /// 获取阶段顺序
    pub fn order(&self) -> u32 {
        match self {
            GamePhase::Command => 1,
            GamePhase::Movement => 2,
            GamePhase::Shooting => 3,
            GamePhase::Charge => 4,
            GamePhase::Fight => 5,
        }
    }
}

/// 阵营类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Faction {
    /// 星际战士
    SpaceMarines,
    /// 帝国卫队
    AstraMilitarum,
    /// 机械教
    AdeptusMechanicus,
    /// 灵族
    Aeldari,
    /// 兽人
    Orks,
    /// 泰伦虫族
    Tyranids,
    /// 混沌星际战士
    ChaosSpaceMarines,
    /// 死灵族
    Necrons,
    /// 泰'乌帝国
    TauEmpire,
    /// 混沌恶魔
    ChaosDaemons,
}

impl Faction {
    /// 获取阵营名称
    pub fn name(&self) -> &'static str {
        match self {
            Faction::SpaceMarines => "星际战士",
            Faction::AstraMilitarum => "帝国卫队",
            Faction::AdeptusMechanicus => "机械教",
            Faction::Aeldari => "灵族",
            Faction::Orks => "兽人",
            Faction::Tyranids => "泰伦虫族",
            Faction::ChaosSpaceMarines => "混沌星际战士",
            Faction::Necrons => "死灵族",
            Faction::TauEmpire => "泰'乌帝国",
            Faction::ChaosDaemons => "混沌恶魔",
        }
    }

    /// 是否为帝国势力
    pub fn is_imperium(&self) -> bool {
        matches!(
            self,
            Faction::SpaceMarines | Faction::AstraMilitarum | Faction::AdeptusMechanicus
        )
    }

    /// 是否为混沌势力
    pub fn is_chaos(&self) -> bool {
        matches!(self, Faction::ChaosSpaceMarines | Faction::ChaosDaemons)
    }

    /// 是否为异形势力
    pub fn is_xenos(&self) -> bool {
        matches!(
            self,
            Faction::Aeldari
                | Faction::Orks
                | Faction::Tyranids
                | Faction::Necrons
                | Faction::TauEmpire
        )
    }
}

/// 单位类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    /// 人物（英雄/指挥官）
    Character,
    /// 步兵
    Infantry,
    /// 骑兵/骑兵单位
    Mounted,
    /// 车辆
    Vehicle,
    /// 巨型单位（泰坦/巨型生物）
    Monster,
    /// 飞行器
    Aircraft,
    /// 建筑/防御工事
    Fortification,
}

impl UnitType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            UnitType::Character => "人物",
            UnitType::Infantry => "步兵",
            UnitType::Mounted => "骑兵",
            UnitType::Vehicle => "车辆",
            UnitType::Monster => "巨型单位",
            UnitType::Aircraft => "飞行器",
            UnitType::Fortification => "建筑",
        }
    }

    /// 是否可被运输
    pub fn can_be_transported(&self) -> bool {
        matches!(self, UnitType::Character | UnitType::Infantry)
    }

    /// 是否有压制值
    pub fn has_morale(&self) -> bool {
        matches!(
            self,
            UnitType::Character | UnitType::Infantry | UnitType::Mounted
        )
    }
}

/// 武器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponType {
    /// 射击武器（突击）
    Assault,
    /// 射击武器（重型）
    Heavy,
    /// 射击武器（手枪）
    Pistol,
    /// 射击武器（速射）
    RapidFire,
    /// 射击武器（榴弹）
    Grenade,
    /// 近战武器
    Melee,
}

impl WeaponType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            WeaponType::Assault => "突击武器",
            WeaponType::Heavy => "重型武器",
            WeaponType::Pistol => "手枪",
            WeaponType::RapidFire => "速射武器",
            WeaponType::Grenade => "榴弹",
            WeaponType::Melee => "近战武器",
        }
    }

    /// 是否为射击武器
    pub fn is_ranged(&self) -> bool {
        !matches!(self, WeaponType::Melee)
    }

    /// 是否可在移动后射击
    pub fn can_fire_after_moving(&self) -> bool {
        matches!(self, WeaponType::Assault | WeaponType::Pistol)
    }
}

/// 战术目标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TacticalObjective {
    /// 夺取目标点
    CaptureObjective,
    /// 消灭敌方单位
    DestroyUnits,
    /// 保卫己方区域
    DefendTerritory,
    /// 搜寻战利品
    RetrieveObjective,
    /// 阻击敌军
    Interdiction,
}

impl TacticalObjective {
    /// 获取目标名称
    pub fn name(&self) -> &'static str {
        match self {
            TacticalObjective::CaptureObjective => "夺取目标点",
            TacticalObjective::DestroyUnits => "消灭敌方",
            TacticalObjective::DefendTerritory => "保卫区域",
            TacticalObjective::RetrieveObjective => "搜寻战利品",
            TacticalObjective::Interdiction => "阻击敌军",
        }
    }

    /// 获取胜利点数
    pub fn victory_points(&self) -> u32 {
        match self {
            TacticalObjective::CaptureObjective => 4,
            TacticalObjective::DestroyUnits => 3,
            TacticalObjective::DefendTerritory => 4,
            TacticalObjective::RetrieveObjective => 4,
            TacticalObjective::Interdiction => 3,
        }
    }
}

/// 游戏规模
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSize {
    /// 巡逻战（500点）
    CombatPatrol,
    /// 先锋战（1000点）
    Incursion,
    /// 突袭战（2000点）
    StrikeForce,
    /// 远征战（3000点以上）
    Onslaught,
}

impl GameSize {
    /// 获取规模名称
    pub fn name(&self) -> &'static str {
        match self {
            GameSize::CombatPatrol => "巡逻战",
            GameSize::Incursion => "先锋战",
            GameSize::StrikeForce => "突袭战",
            GameSize::Onslaught => "远征战",
        }
    }

    /// 获取默认点数上限
    pub fn points_limit(&self) -> u32 {
        match self {
            GameSize::CombatPatrol => 500,
            GameSize::Incursion => 1000,
            GameSize::StrikeForce => 2000,
            GameSize::Onslaught => 3000,
        }
    }

    /// 获取推荐回合数
    pub fn recommended_rounds(&self) -> u32 {
        match self {
            GameSize::CombatPatrol => 4,
            GameSize::Incursion => 5,
            GameSize::StrikeForce => 5,
            GameSize::Onslaught => 6,
        }
    }
}

/// 战锤40K规则
#[derive(Debug, Clone)]
pub struct Warhammer40KRules {
    /// 规则元数据
    metadata: RuleMetadata,
}

impl Warhammer40KRules {
    /// 创建新的战锤40K规则
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("战锤40K规则", "Warhammer 40,000 第十版规则"),
        }
    }

    /// 获取所有游戏阶段
    pub fn game_phases(&self) -> Vec<GamePhase> {
        vec![
            GamePhase::Command,
            GamePhase::Movement,
            GamePhase::Shooting,
            GamePhase::Charge,
            GamePhase::Fight,
        ]
    }

    /// 获取所有阵营
    pub fn factions(&self) -> Vec<Faction> {
        vec![
            Faction::SpaceMarines,
            Faction::AstraMilitarum,
            Faction::AdeptusMechanicus,
            Faction::Aeldari,
            Faction::Orks,
            Faction::Tyranids,
            Faction::ChaosSpaceMarines,
            Faction::Necrons,
            Faction::TauEmpire,
            Faction::ChaosDaemons,
        ]
    }

    /// 获取所有单位类型
    pub fn unit_types(&self) -> Vec<UnitType> {
        vec![
            UnitType::Character,
            UnitType::Infantry,
            UnitType::Mounted,
            UnitType::Vehicle,
            UnitType::Monster,
            UnitType::Aircraft,
            UnitType::Fortification,
        ]
    }

    /// 获取所有武器类型
    pub fn weapon_types(&self) -> Vec<WeaponType> {
        vec![
            WeaponType::Assault,
            WeaponType::Heavy,
            WeaponType::Pistol,
            WeaponType::RapidFire,
            WeaponType::Grenade,
            WeaponType::Melee,
        ]
    }

    /// 获取所有战术目标
    pub fn tactical_objectives(&self) -> Vec<TacticalObjective> {
        vec![
            TacticalObjective::CaptureObjective,
            TacticalObjective::DestroyUnits,
            TacticalObjective::DefendTerritory,
            TacticalObjective::RetrieveObjective,
            TacticalObjective::Interdiction,
        ]
    }

    /// 获取所有游戏规模
    pub fn game_sizes(&self) -> Vec<GameSize> {
        vec![
            GameSize::CombatPatrol,
            GameSize::Incursion,
            GameSize::StrikeForce,
            GameSize::Onslaught,
        ]
    }

    /// 获取默认点数上限
    pub fn default_points_limit(&self) -> u32 {
        2000
    }

    /// 计算命中成功率（基于技能值）
    pub fn hit_probability(&self, ballistic_skill: u32) -> f64 {
        // BS 为3+表示需要3+命中，即 (7-BS)/6
        let needed = 7 - ballistic_skill.min(6).max(2);
        needed as f64 / 6.0
    }

    /// 计算造伤成功率（基于强度和韧性）
    pub fn wound_probability(&self, strength: u32, toughness: u32) -> f64 {
        let needed = if strength >= toughness * 2 {
            2
        } else if strength > toughness {
            3
        } else if strength == toughness {
            4
        } else if strength * 2 <= toughness {
            6
        } else {
            5
        };
        needed as f64 / 6.0
    }

    /// 计算护甲穿透修正
    pub fn armor_save_modifier(&self, ap: i32) -> i32 {
        -ap // AP 为负数，表示护甲值需要加多少
    }

    /// 检查士气检定
    pub fn morale_test(&self, models_lost: u32, leadership: u32) -> bool {
        // 骰子结果 + 损失数 <= 领导力 则通过
        // 这里返回是否可能通过
        leadership >= models_lost + 1
    }

    /// 获取指挥点数上限
    pub fn max_command_points(&self) -> u32 {
        12
    }

    /// 获取初始指挥点数
    pub fn starting_command_points(&self, game_size: GameSize) -> u32 {
        match game_size {
            GameSize::CombatPatrol => 0,
            GameSize::Incursion => 3,
            GameSize::StrikeForce => 6,
            GameSize::Onslaught => 9,
        }
    }

    /// 检查单位是否可执行动作
    pub fn can_perform_action(&self, unit_type: UnitType, advanced: bool) -> bool {
        if advanced {
            matches!(unit_type, UnitType::Infantry | UnitType::Character)
        } else {
            true
        }
    }

    /// 获取目标点控制规则
    pub fn objective_control_rules(&self) -> Vec<&'static str> {
        vec![
            "单位必须在目标点范围内才能控制",
            "控制范围为目标点周围3英寸",
            "多个单位可争夺同一目标点",
            "只有步兵和骑兵可以控制目标点",
        ]
    }
}

impl Default for Warhammer40KRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for Warhammer40KRules {
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
    fn test_game_phases() {
        let rules = Warhammer40KRules::new();
        let phases = rules.game_phases();
        assert_eq!(phases.len(), 5);
        assert_eq!(GamePhase::Command.order(), 1);
    }

    #[test]
    fn test_factions() {
        let rules = Warhammer40KRules::new();
        let factions = rules.factions();
        assert_eq!(factions.len(), 10);
        assert!(Faction::SpaceMarines.is_imperium());
        assert!(Faction::ChaosSpaceMarines.is_chaos());
        assert!(Faction::Orks.is_xenos());
    }

    #[test]
    fn test_unit_types() {
        let rules = Warhammer40KRules::new();
        let types = rules.unit_types();
        assert_eq!(types.len(), 7);
        assert!(UnitType::Infantry.can_be_transported());
        assert!(!UnitType::Vehicle.can_be_transported());
    }

    #[test]
    fn test_weapon_types() {
        let rules = Warhammer40KRules::new();
        let weapons = rules.weapon_types();
        assert_eq!(weapons.len(), 6);
        assert!(WeaponType::Assault.is_ranged());
        assert!(!WeaponType::Melee.is_ranged());
        assert!(WeaponType::Assault.can_fire_after_moving());
        assert!(!WeaponType::Heavy.can_fire_after_moving());
    }

    #[test]
    fn test_hit_probability() {
        let rules = Warhammer40KRules::new();
        // BS 3+ 表示需要3+，即4/6 = 66.7%
        let prob = rules.hit_probability(3);
        assert!((prob - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_wound_probability() {
        let rules = Warhammer40KRules::new();
        // S >= 2T 造伤需要 2+
        let prob = rules.wound_probability(8, 4);
        assert!((prob - 0.8333).abs() < 0.01);
        // S > T 造伤需要 3+
        let prob = rules.wound_probability(5, 4);
        assert!((prob - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_game_sizes() {
        let rules = Warhammer40KRules::new();
        let sizes = rules.game_sizes();
        assert_eq!(sizes.len(), 4);
        assert_eq!(GameSize::CombatPatrol.points_limit(), 500);
        assert_eq!(GameSize::StrikeForce.points_limit(), 2000);
    }

    #[test]
    fn test_command_points() {
        let rules = Warhammer40KRules::new();
        assert_eq!(rules.max_command_points(), 12);
        assert_eq!(rules.starting_command_points(GameSize::StrikeForce), 6);
    }

    #[test]
    fn test_tactical_objectives() {
        let rules = Warhammer40KRules::new();
        let objectives = rules.tactical_objectives();
        assert_eq!(objectives.len(), 5);
        assert_eq!(TacticalObjective::CaptureObjective.victory_points(), 4);
    }

    #[test]
    fn test_morale() {
        let rules = Warhammer40KRules::new();
        // 领导力6，损失3个模型，可能通过
        assert!(rules.morale_test(3, 6));
        // 领导力6，损失7个模型，不可能通过
        assert!(!rules.morale_test(7, 6));
    }
}