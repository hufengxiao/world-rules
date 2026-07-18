//! 战锤：西格玛时代 (Warhammer Age of Sigmar) 规则
//!
//! 由 Games Workshop 出品的奇幻桌面战争游戏，设定在凡间八域。
//!
//! # 游戏概述
//!
//! - **类型**：回合制微缩模型战争游戏
//! - **规模**：小队到军团级战斗
//! - **核心机制**：交替激活、英雄阶段、战斗震慑
//!
//! # 主要势力
//!
//! - 秩序势力：风暴守望、西格玛之子、精灵
//! - 混沌势力：混沌战士、斯拉阿什、纳垢
//! - 死亡势力：死灵法师、墓穴王
//! - 破坏势力：兽人、食人魔、史兰天
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::wargames::age_of_sigmar::AgeOfSigmarRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = AgeOfSigmarRules::new();
//! assert!(rules.point_limit_default() > 0);
//! assert_eq!(rules.battle_phases().len(), 5);
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 战斗阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BattlePhase {
    /// 英雄阶段
    Hero,
    /// 移动阶段
    Movement,
    /// 射击阶段
    Shooting,
    /// 冲锋阶段
    Charge,
    /// 战斗阶段
    Combat,
}

impl BattlePhase {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            BattlePhase::Hero => "英雄阶段",
            BattlePhase::Movement => "移动阶段",
            BattlePhase::Shooting => "射击阶段",
            BattlePhase::Charge => "冲锋阶段",
            BattlePhase::Combat => "战斗阶段",
        }
    }
}

/// 大阵营（凡间八域势力）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrandAlliance {
    /// 秩序
    Order,
    /// 混沌
    Chaos,
    /// 死亡
    Death,
    /// 破坏
    Destruction,
}

impl GrandAlliance {
    /// 获取阵营名称
    pub fn name(&self) -> &'static str {
        match self {
            GrandAlliance::Order => "秩序",
            GrandAlliance::Chaos => "混沌",
            GrandAlliance::Death => "死亡",
            GrandAlliance::Destruction => "破坏",
        }
    }
}

/// 具体势力
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Faction {
    /// 风暴守望
    StormcastEternals,
    /// 西格玛之城
    CitiesOfSigmar,
    /// 精灵（各类）
    LuminethRealmLords,
    /// 混沌战士
    SlavesToDarkness,
    /// 纳垢
    MaggotkinOfNurgle,
    /// 斯拉阿什
    HedonitesOfSlaanesh,
    /// 死灵法师
    SoulblightGravelords,
    /// 墓穴王
    OssiarchBonereapers,
    /// 铁颚兽人
    Ironjawz,
    /// 史兰天
    Seraphon,
}

impl Faction {
    /// 获取势力名称
    pub fn name(&self) -> &'static str {
        match self {
            Faction::StormcastEternals => "风暴守望",
            Faction::CitiesOfSigmar => "西格玛之城",
            Faction::LuminethRealmLords => "光辉领域领主",
            Faction::SlavesToDarkness => "黑暗奴隶",
            Faction::MaggotkinOfNurgle => "纳垢腐化者",
            Faction::HedonitesOfSlaanesh => "斯拉阿什享乐者",
            Faction::SoulblightGravelords => "血墓领主",
            Faction::OssiarchBonereapers => "白骨收割者",
            Faction::Ironjawz => "铁颚兽人",
            Faction::Seraphon => "史兰天",
        }
    }

    /// 获取所属大阵营
    pub fn grand_alliance(&self) -> GrandAlliance {
        match self {
            Faction::StormcastEternals
            | Faction::CitiesOfSigmar
            | Faction::LuminethRealmLords
            | Faction::Seraphon => GrandAlliance::Order,
            Faction::SlavesToDarkness
            | Faction::MaggotkinOfNurgle
            | Faction::HedonitesOfSlaanesh => GrandAlliance::Chaos,
            Faction::SoulblightGravelords | Faction::OssiarchBonereapers => GrandAlliance::Death,
            Faction::Ironjawz => GrandAlliance::Destruction,
        }
    }
}

/// 战斗目标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BattleTactic {
    /// 守护目标
    HoldObjective,
    /// 突袭
    Attack,
    /// 防御
    Defense,
    /// 搜寻战利品
    SeekTreasure,
    /// 破坏任务
    Sabotage,
}

impl BattleTactic {
    /// 获取目标名称
    pub fn name(&self) -> &'static str {
        match self {
            BattleTactic::HoldObjective => "守护目标",
            BattleTactic::Attack => "突袭",
            BattleTactic::Defense => "防御",
            BattleTactic::SeekTreasure => "搜寻战利品",
            BattleTactic::Sabotage => "破坏任务",
        }
    }

    /// 获取胜利点数
    pub fn victory_points(&self) -> u32 {
        match self {
            BattleTactic::HoldObjective => 3,
            BattleTactic::Attack => 2,
            BattleTactic::Defense => 2,
            BattleTactic::SeekTreasure => 3,
            BattleTactic::Sabotage => 3,
        }
    }
}

/// 游戏规模
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSize {
    /// 先锋战（1000点）
    Vanguard,
    /// 突袭战（2000点）
    Warhost,
}

impl GameSize {
    /// 获取规模名称
    pub fn name(&self) -> &'static str {
        match self {
            GameSize::Vanguard => "先锋战",
            GameSize::Warhost => "突袭战",
        }
    }

    /// 获取点数上限
    pub fn points_limit(&self) -> u32 {
        match self {
            GameSize::Vanguard => 1000,
            GameSize::Warhost => 2000,
        }
    }
}

/// 西格玛时代规则
#[derive(Debug, Clone)]
pub struct AgeOfSigmarRules {
    /// 规则元数据
    metadata: RuleMetadata,
}

impl AgeOfSigmarRules {
    /// 创建新的规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("战锤西格玛时代规则", "Warhammer Age of Sigmar 第四版规则"),
        }
    }

    /// 获取所有战斗阶段
    pub fn battle_phases(&self) -> Vec<BattlePhase> {
        vec![
            BattlePhase::Hero,
            BattlePhase::Movement,
            BattlePhase::Shooting,
            BattlePhase::Charge,
            BattlePhase::Combat,
        ]
    }

    /// 获取所有大阵营
    pub fn grand_alliances(&self) -> Vec<GrandAlliance> {
        vec![
            GrandAlliance::Order,
            GrandAlliance::Chaos,
            GrandAlliance::Death,
            GrandAlliance::Destruction,
        ]
    }

    /// 获取所有势力
    pub fn factions(&self) -> Vec<Faction> {
        vec![
            Faction::StormcastEternals,
            Faction::CitiesOfSigmar,
            Faction::LuminethRealmLords,
            Faction::SlavesToDarkness,
            Faction::MaggotkinOfNurgle,
            Faction::HedonitesOfSlaanesh,
            Faction::SoulblightGravelords,
            Faction::OssiarchBonereapers,
            Faction::Ironjawz,
            Faction::Seraphon,
        ]
    }

    /// 获取所有战斗目标
    pub fn battle_tactics(&self) -> Vec<BattleTactic> {
        vec![
            BattleTactic::HoldObjective,
            BattleTactic::Attack,
            BattleTactic::Defense,
            BattleTactic::SeekTreasure,
            BattleTactic::Sabotage,
        ]
    }

    /// 获取游戏规模
    pub fn game_sizes(&self) -> Vec<GameSize> {
        vec![GameSize::Vanguard, GameSize::Warhost]
    }

    /// 获取默认点数上限
    pub fn point_limit_default(&self) -> u32 {
        2000
    }

    /// 计算命中成功率
    pub fn hit_probability(&self, to_hit: u32) -> f64 {
        // to_hit 为需要掷出的最小值（如3+）
        let needed = 7 - to_hit.clamp(2, 6);
        needed as f64 / 6.0
    }

    /// 计算造伤成功率
    pub fn wound_probability(&self, to_wound: u32) -> f64 {
        let needed = 7 - to_wound.clamp(2, 6);
        needed as f64 / 6.0
    }

    /// 计算护甲检定成功率
    pub fn save_probability(&self, save: u32, rend: i32) -> f64 {
        // rend 为护甲穿透修正（负数表示降低护甲值）
        let modified_save = (save as i32 - rend).clamp(2, 6) as u32;
        let needed = 7 - modified_save;
        needed as f64 / 6.0
    }

    /// 检查战斗震慑
    pub fn battleshock_test(&self, models_lost: u32, bravery: u32) -> bool {
        bravery > models_lost
    }

    /// 获取胜利点数上限
    pub fn victory_points_limit(&self) -> u32 {
        30
    }

    /// 获取回合数上限
    pub fn max_rounds(&self) -> u32 {
        5
    }

    /// 获取目标点控制规则
    pub fn objective_control_rules(&self) -> Vec<&'static str> {
        vec![
            "单位必须完全在目标点范围内才能控制",
            "控制范围为目标点周围6英寸",
            "多数模型控制目标点",
            "所有单位类型都可以控制目标点",
        ]
    }
}

impl Default for AgeOfSigmarRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AgeOfSigmarRules {
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
    fn test_battle_phases() {
        let rules = AgeOfSigmarRules::new();
        let phases = rules.battle_phases();
        assert_eq!(phases.len(), 5);
    }

    #[test]
    fn test_grand_alliances() {
        let rules = AgeOfSigmarRules::new();
        let alliances = rules.grand_alliances();
        assert_eq!(alliances.len(), 4);
    }

    #[test]
    fn test_factions() {
        let rules = AgeOfSigmarRules::new();
        let factions = rules.factions();
        assert_eq!(factions.len(), 10);
        assert_eq!(
            Faction::StormcastEternals.grand_alliance(),
            GrandAlliance::Order
        );
        assert_eq!(
            Faction::SlavesToDarkness.grand_alliance(),
            GrandAlliance::Chaos
        );
        assert_eq!(
            Faction::SoulblightGravelords.grand_alliance(),
            GrandAlliance::Death
        );
    }

    #[test]
    fn test_hit_probability() {
        let rules = AgeOfSigmarRules::new();
        let prob = rules.hit_probability(3);
        assert!((prob - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_save_probability() {
        let rules = AgeOfSigmarRules::new();
        // 护甲4+，穿透-1，需要5+
        let prob = rules.save_probability(4, -1);
        assert!((prob - 0.3333).abs() < 0.01);
    }

    #[test]
    fn test_battleshock() {
        let rules = AgeOfSigmarRules::new();
        assert!(rules.battleshock_test(3, 6));
        assert!(!rules.battleshock_test(7, 6));
    }

    #[test]
    fn test_game_sizes() {
        let rules = AgeOfSigmarRules::new();
        let sizes = rules.game_sizes();
        assert_eq!(sizes.len(), 2);
        assert_eq!(GameSize::Vanguard.points_limit(), 1000);
    }

    #[test]
    fn test_battle_tactics() {
        let rules = AgeOfSigmarRules::new();
        let tactics = rules.battle_tactics();
        assert_eq!(tactics.len(), 5);
        assert_eq!(BattleTactic::HoldObjective.victory_points(), 3);
    }
}
