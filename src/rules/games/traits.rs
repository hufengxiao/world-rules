//! 游戏规则通用 trait 设计
//!
//! 本模块定义了游戏规则的通用 trait 层次结构，为各类游戏提供统一的接口抽象。
//!
//! # 设计目标
//!
//! - **可扩展性**: 支持棋类、卡牌、桌游等各类游戏
//! - **类型安全**: 使用泛型和关联类型确保类型安全
//! - **可组合性**: 通过 trait 组合实现不同游戏特性
//!
//! # Trait 层次结构
//!
//! ```text
//! Game (基础游戏 trait)
//! ├── TurnBased (回合制游戏)
//! ├── Scoreable (可计分游戏)
//! ├── BoardGame (棋盘游戏)
//! │   └── BoardGameExt (扩展棋盘功能)
//! └── CardGame (卡牌游戏)
//!     └── CardGameExt (扩展卡牌功能)
//! ```
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::traits::*;
//!
//! // 检查游戏是否支持回合制
//! fn play_game<G: Game + TurnBased>(game: &mut G) {
//!     while !game.is_game_over() {
//!         let player = game.current_player();
//!         println!("当前玩家: {:?}", player);
//!         // 执行回合逻辑...
//!         game.end_turn();
//!     }
//! }
//! ```

use std::fmt::Debug;
use std::hash::Hash;

/// 游戏状态
///
/// 表示游戏的当前运行状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// 游戏尚未开始
    #[default]
    NotStarted,
    /// 游戏进行中
    InProgress,
    /// 游戏已结束
    Finished,
    /// 游戏暂停
    Paused,
}

/// 游戏难度等级
///
/// 用于游戏规则难度分级系统。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Difficulty {
    /// 入门级 - 适合新手学习基本规则
    Beginner,
    /// 简单级 - 掌握基本策略即可参与
    Easy,
    /// 普通级 - 需要一定经验和策略
    #[default]
    Normal,
    /// 困难级 - 需要深入理解和高级策略
    Hard,
    /// 专家级 - 需要精通规则和复杂策略
    Expert,
    /// 大师级 - 最高难度，竞技级别
    Master,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Beginner => write!(f, "入门"),
            Self::Easy => write!(f, "简单"),
            Self::Normal => write!(f, "普通"),
            Self::Hard => write!(f, "困难"),
            Self::Expert => write!(f, "专家"),
            Self::Master => write!(f, "大师"),
        }
    }
}

/// 游戏类型分类
///
/// 用于对游戏进行大类划分。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    /// 棋类游戏（象棋、围棋等）
    Board,
    /// 卡牌游戏（扑克、麻将等）
    Card,
    /// 桌面游戏（大富翁、卡坦岛等）
    Tabletop,
    /// 策略游戏（文明、全面战争等）
    Strategy,
    /// 竞技游戏（电子竞技等）
    Competitive,
    /// 休闲游戏（聚会游戏等）
    Party,
    /// 益智游戏（数独、魔方等）
    Puzzle,
    /// 角色扮演游戏（D&D等）
    RolePlaying,
    /// 其他类型
    Other,
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Board => write!(f, "棋类"),
            Self::Card => write!(f, "卡牌"),
            Self::Tabletop => write!(f, "桌游"),
            Self::Strategy => write!(f, "策略"),
            Self::Competitive => write!(f, "竞技"),
            Self::Party => write!(f, "聚会"),
            Self::Puzzle => write!(f, "益智"),
            Self::RolePlaying => write!(f, "角色扮演"),
            Self::Other => write!(f, "其他"),
        }
    }
}

/// 游戏信息
///
/// 包含游戏的基本元数据信息。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameInfo {
    /// 游戏名称
    pub name: String,
    /// 游戏描述
    pub description: String,
    /// 游戏类型
    pub game_type: GameType,
    /// 游戏难度
    pub difficulty: Difficulty,
    /// 最少玩家数
    pub min_players: u8,
    /// 最多玩家数
    pub max_players: u8,
    /// 平均游戏时长（分钟）
    pub average_duration: Option<u16>,
    /// 游戏来源/地区
    pub origin: Option<String>,
    /// 游戏版本
    pub version: String,
}

impl GameInfo {
    /// 创建游戏信息
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::games::traits::{GameInfo, GameType, Difficulty};
    ///
    /// let info = GameInfo::new("围棋", "古老的棋类游戏")
    ///     .with_type(GameType::Board)
    ///     .with_difficulty(Difficulty::Expert)
    ///     .with_players(2, 2);
    ///
    /// assert_eq!(info.name, "围棋");
    /// assert_eq!(info.min_players, 2);
    /// ```
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            game_type: GameType::Other,
            difficulty: Difficulty::Normal,
            min_players: 1,
            max_players: 1,
            average_duration: None,
            origin: None,
            version: "1.0.0".to_string(),
        }
    }

    /// 设置游戏类型
    pub fn with_type(mut self, game_type: GameType) -> Self {
        self.game_type = game_type;
        self
    }

    /// 设置游戏难度
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    /// 设置玩家数量范围
    pub fn with_players(mut self, min: u8, max: u8) -> Self {
        self.min_players = min;
        self.max_players = max;
        self
    }

    /// 设置平均游戏时长
    pub fn with_duration(mut self, minutes: u16) -> Self {
        self.average_duration = Some(minutes);
        self
    }

    /// 设置游戏来源
    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    /// 设置游戏版本
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }
}

/// 游戏基础 trait
///
/// 所有游戏都必须实现的基础接口。
///
/// # 实现要求
///
/// - `Info`: 游戏信息类型
/// - `State`: 游戏状态类型
/// - `Action`: 玩家行动类型
///
/// # Examples
///
/// ```rust
/// use world_rules::rules::games::traits::*;
///
/// struct MyGame {
///     info: GameInfo,
///     state: GameState,
/// }
///
/// impl Game for MyGame {
///     type Info = GameInfo;
///     type State = GameState;
///     type Action = String;
///
///     fn info(&self) -> &Self::Info {
///         &self.info
///     }
///
///     fn state(&self) -> &Self::State {
///         &self.state
///     }
///
///     fn is_game_over(&self) -> bool {
///         matches!(self.state, GameState::Finished)
///     }
///
///     fn reset(&mut self) {
///         self.state = GameState::NotStarted;
///     }
/// }
/// ```
pub trait Game: Send + Sync {
    /// 游戏信息类型
    type Info: Debug + Send + Sync;
    /// 游戏状态类型
    type State: Debug + Send + Sync;
    /// 玩家行动类型
    type Action: Debug + Send + Sync;

    /// 获取游戏信息
    fn info(&self) -> &Self::Info;

    /// 获取当前游戏状态
    fn state(&self) -> &Self::State;

    /// 检查游戏是否结束
    fn is_game_over(&self) -> bool;

    /// 重置游戏
    fn reset(&mut self);

    /// 开始游戏
    fn start(&mut self) {
        // 默认实现：空操作
    }

    /// 获取游戏难度
    fn difficulty(&self) -> Difficulty {
        Difficulty::Normal
    }

    /// 获取游戏类型
    fn game_type(&self) -> GameType {
        GameType::Other
    }
}

/// 回合制游戏 trait
///
/// 为回合制游戏提供回合管理功能。
///
/// # Examples
///
/// ```rust
/// use world_rules::rules::games::traits::*;
///
/// struct ChessGame {
///     current_player: u8,
///     turn_count: u32,
/// }
///
/// impl TurnBased for ChessGame {
///     type Player = u8;
///
///     fn current_player(&self) -> Self::Player {
///         self.current_player
///     }
///
///     fn advance_turn(&mut self) {
///         self.current_player = 1 - self.current_player;
///         self.turn_count += 1;
///     }
///
///     fn turn_number(&self) -> u32 {
///         self.turn_count
///     }
/// }
/// ```
pub trait TurnBased: Game {
    /// 玩家标识类型
    type Player: Debug + Clone + Eq + Hash + Send + Sync;

    /// 获取当前玩家
    fn current_player(&self) -> Self::Player;

    /// 推进到下一回合
    fn advance_turn(&mut self);

    /// 获取当前回合数
    fn turn_number(&self) -> u32;

    /// 结束当前回合
    ///
    /// 默认实现调用 `advance_turn`。
    fn end_turn(&mut self) {
        self.advance_turn();
    }

    /// 检查是否轮到指定玩家
    fn is_player_turn(&self, player: &Self::Player) -> bool {
        &self.current_player() == player
    }

    /// 获取所有玩家列表
    fn players(&self) -> Vec<Self::Player>;

    /// 获取玩家数量
    fn player_count(&self) -> usize {
        self.players().len()
    }
}

/// 可计分游戏 trait
///
/// 为有分数系统的游戏提供计分功能。
///
/// # Examples
///
/// ```rust
/// use world_rules::rules::games::traits::*;
/// use std::collections::HashMap;
///
/// struct CardGame {
///     scores: HashMap<u8, i32>,
/// }
///
/// impl Scoreable for CardGame {
///     type Player = u8;
///     type Score = i32;
///
///     fn score(&self, player: &Self::Player) -> Self::Score {
///         *self.scores.get(player).unwrap_or(&0)
///     }
///
///     fn add_score(&mut self, player: Self::Player, points: Self::Score) {
///         *self.scores.entry(player).or_insert(0) += points;
///     }
///
///     fn scores(&self) -> Vec<(Self::Player, Self::Score)> {
///         self.scores.iter().map(|(p, s)| (*p, *s)).collect()
///     }
///
///     fn set_score(&mut self, player: Self::Player, score: Self::Score) {
///         *self.scores.entry(player).or_insert(0) = score;
///     }
///     
///     fn reset_scores(&mut self) {
///         self.scores.clear();
///     }
/// }
/// ```
pub trait Scoreable: Game {
    /// 玩家标识类型
    type Player: Debug + Clone + Eq + Hash + Send + Sync;
    /// 分数类型
    type Score: Debug + Clone + Copy + PartialOrd + Default + Send + Sync;

    /// 获取指定玩家的分数
    fn score(&self, player: &Self::Player) -> Self::Score;

    /// 为指定玩家增加分数
    fn add_score(&mut self, player: Self::Player, points: Self::Score);

    /// 获取所有玩家的分数列表
    fn scores(&self) -> Vec<(Self::Player, Self::Score)>;

    /// 获取当前领先玩家
    fn leader(&self) -> Option<Self::Player> {
        self.scores()
            .into_iter()
            .max_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(p, _)| p)
    }

    /// 设置指定玩家的分数
    fn set_score(&mut self, player: Self::Player, score: Self::Score);

    /// 重置所有分数
    fn reset_scores(&mut self);
}

/// 棋盘位置 trait
///
/// 定义棋盘位置的基本接口。
pub trait Position: Debug + Clone + Eq + Hash + Send + Sync {
    /// 检查位置是否有效
    fn is_valid(&self) -> bool;
}

/// 棋盘游戏 trait
///
/// 为棋盘类游戏提供棋盘状态管理功能。
///
/// # Examples
///
/// ```rust
/// use world_rules::rules::games::traits::*;
///
/// struct GoBoard {
///     size: usize,
///     stones: Vec<Vec<Option<bool>>>, // Some(true) = 黑, Some(false) = 白
/// }
///
/// impl BoardGame for GoBoard {
///     type Position = (usize, usize);
///     type Piece = bool; // true = 黑, false = 白
///     type BoardState = Vec<Vec<Option<bool>>>;
///
///     fn board_size(&self) -> (usize, usize) {
///         (self.size, self.size)
///     }
///
///     fn get_piece(&self, pos: &Self::Position) -> Option<Self::Piece> {
///         self.stones.get(pos.0)?.get(pos.1)?.clone()
///     }
///
///     fn place_piece(&mut self, pos: Self::Position, piece: Self::Piece) -> Result<(), String> {
///         if pos.0 >= self.size || pos.1 >= self.size {
///             return Err("位置越界".to_string());
///         }
///         self.stones[pos.0][pos.1] = Some(piece);
///         Ok(())
///     }
///
///     fn remove_piece(&mut self, pos: &Self::Position) -> Option<Self::Piece> {
///         self.stones.get_mut(pos.0)?.get_mut(pos.1)?.take()
///     }
///
///     fn board_state(&self) -> &Self::BoardState {
///         &self.stones
///     }
///
///     fn empty_positions(&self) -> Vec<Self::Position> {
///         let mut empty = Vec::new();
///         for i in 0..self.size {
///             for j in 0..self.size {
///                 if self.stones[i][j].is_none() {
///                     empty.push((i, j));
///                 }
///             }
///         }
///         empty
///     }
/// }
/// ```
pub trait BoardGame: Game {
    /// 棋盘位置类型
    type Position: Position;
    /// 棋子类型
    type Piece: Debug + Clone + Send + Sync;
    /// 棋盘状态类型
    type BoardState: Debug + Send + Sync;

    /// 获取棋盘大小 (行, 列)
    fn board_size(&self) -> (usize, usize);

    /// 获取指定位置的棋子
    fn get_piece(&self, pos: &Self::Position) -> Option<Self::Piece>;

    /// 在指定位置放置棋子
    fn place_piece(&mut self, pos: Self::Position, piece: Self::Piece) -> Result<(), String>;

    /// 移除指定位置的棋子
    fn remove_piece(&mut self, pos: &Self::Position) -> Option<Self::Piece>;

    /// 移动棋子
    fn move_piece(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> Result<Self::Piece, String> {
        let piece = self
            .remove_piece(&from)
            .ok_or_else(|| "起始位置无棋子".to_string())?;
        self.place_piece(to, piece.clone())?;
        Ok(piece)
    }

    /// 获取棋盘状态
    fn board_state(&self) -> &Self::BoardState;

    /// 获取所有空位
    fn empty_positions(&self) -> Vec<Self::Position>;

    /// 检查位置是否为空
    fn is_empty(&self, pos: &Self::Position) -> bool {
        self.get_piece(pos).is_none()
    }

    /// 获取棋盘上的棋子总数
    fn piece_count(&self) -> usize {
        let empty_count = self.empty_positions().len();
        let (rows, cols) = self.board_size();
        rows * cols - empty_count
    }

    /// 清空棋盘
    fn clear_board(&mut self);
}

/// 棋盘游戏扩展 trait
///
/// 提供额外的棋盘分析功能。
pub trait BoardGameExt: BoardGame {
    /// 获取指定棋子的所有位置
    fn find_pieces(&self, piece: &Self::Piece) -> Vec<Self::Position>;

    /// 统计指定棋子的数量
    fn count_piece(&self, piece: &Self::Piece) -> usize {
        self.find_pieces(piece).len()
    }

    /// 检查位置是否在棋盘范围内
    fn is_valid_position(&self, pos: &Self::Position) -> bool;

    /// 获取棋盘的字符串表示（用于显示）
    fn board_to_string(&self) -> String;
}

/// 卡牌游戏 trait
///
/// 为卡牌类游戏提供手牌和牌堆管理功能。
///
/// # Examples
///
/// ```rust
/// use world_rules::rules::games::traits::*;
/// use std::collections::VecDeque;
///
/// struct PokerGame {
///     deck: VecDeque<u8>,
///     hands: Vec<Vec<u8>>,
///     discard_pile: Vec<u8>,
/// }
///
/// impl CardGame for PokerGame {
///     type Card = u8;
///     type Hand = Vec<u8>;
///     type Player = usize;
///
///     fn hand(&self, player: Self::Player) -> &Self::Hand {
///         &self.hands[player]
///     }
///
///     fn hand_mut(&mut self, player: Self::Player) -> &mut Self::Hand {
///         &mut self.hands[player]
///     }
///
///     fn draw_card(&mut self, player: Self::Player) -> Option<Self::Card> {
///         let card = self.deck.pop_front()?;
///         self.hands[player].push(card);
///         Some(card)
///     }
///
///     fn play_card(&mut self, player: Self::Player, card: Self::Card) -> Result<(), String> {
///         let hand = &mut self.hands[player];
///         if let Some(pos) = hand.iter().position(|c| *c == card) {
///             hand.remove(pos);
///             self.discard_pile.push(card);
///             Ok(())
///         } else {
///             Err("手牌中不存在该牌".to_string())
///         }
///     }
///
///     fn deck_count(&self) -> usize {
///         self.deck.len()
///     }
///
///     fn discard_pile_count(&self) -> usize {
///         self.discard_pile.len()
///     }
/// }
/// ```
pub trait CardGame: Game {
    /// 卡牌类型
    type Card: Debug + Clone + Eq + Send + Sync;
    /// 手牌类型
    type Hand: Debug + Send + Sync;
    /// 玩家标识类型
    type Player: Debug + Clone + Eq + Hash + Send + Sync;

    /// 获取指定玩家的手牌
    fn hand(&self, player: Self::Player) -> &Self::Hand;

    /// 获取指定玩家的手牌（可变引用）
    fn hand_mut(&mut self, player: Self::Player) -> &mut Self::Hand;

    /// 从牌堆抽牌到指定玩家的手牌
    fn draw_card(&mut self, player: Self::Player) -> Option<Self::Card>;

    /// 指定玩家打出一张牌
    fn play_card(&mut self, player: Self::Player, card: Self::Card) -> Result<(), String>;

    /// 获取牌堆剩余数量
    fn deck_count(&self) -> usize;

    /// 获取弃牌堆数量
    fn discard_pile_count(&self) -> usize;

    /// 洗牌
    fn shuffle_deck(&mut self);

    /// 手牌数量
    fn hand_count(&self, player: Self::Player) -> usize;

    /// 检查牌堆是否为空
    fn is_deck_empty(&self) -> bool {
        self.deck_count() == 0
    }
}

/// 卡牌游戏扩展 trait
///
/// 提供额外的卡牌操作功能。
pub trait CardGameExt: CardGame {
    /// 从其他玩家获取卡牌
    fn take_card_from(
        &mut self,
        from: Self::Player,
        to: Self::Player,
        card: Self::Card,
    ) -> Result<(), String>;

    /// 查看牌堆顶的牌（不抽取）
    fn peek_deck(&self) -> Option<Self::Card>;

    /// 从指定玩家的手牌中搜索卡牌
    fn search_hand(&self, player: Self::Player, predicate: impl Fn(&Self::Card) -> bool)
        -> Vec<Self::Card>;

    /// 将牌从弃牌堆洗回牌堆
    fn reshuffle_discard(&mut self);

    /// 发牌给所有玩家
    fn deal_cards(&mut self, count: usize) {
        for _ in 0..count {
            for player in self.all_players() {
                self.draw_card(player);
            }
        }
    }

    /// 获取所有玩家列表
    fn all_players(&self) -> Vec<Self::Player>;
}

/// 游戏规则验证器 trait
///
/// 提供游戏状态和行动的验证功能。
pub trait GameValidator: Game {
    /// 验证行动是否合法
    fn is_valid_action(&self, action: &Self::Action) -> bool;

    /// 验证并执行行动
    fn execute_action(&mut self, action: Self::Action) -> Result<(), String> {
        if !self.is_valid_action(&action) {
            return Err("非法行动".to_string());
        }
        self.apply_action(action);
        Ok(())
    }

    /// 应用行动（内部实现）
    fn apply_action(&mut self, action: Self::Action);

    /// 获取所有合法行动
    fn legal_actions(&self) -> Vec<Self::Action>;
}

/// 游戏序列化 trait
///
/// 提供游戏状态的序列化和反序列化功能。
#[cfg(feature = "serde_json")]
pub trait GameSerializable: Game {
    /// 序列化游戏状态
    fn serialize(&self) -> Result<String, String>;

    /// 从字符串反序列化
    fn deserialize(&self, data: &str) -> Result<Self, String>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_ordering() {
        assert!(Difficulty::Beginner < Difficulty::Easy);
        assert!(Difficulty::Easy < Difficulty::Normal);
        assert!(Difficulty::Normal < Difficulty::Hard);
        assert!(Difficulty::Hard < Difficulty::Expert);
        assert!(Difficulty::Expert < Difficulty::Master);
    }

    #[test]
    fn test_game_info_builder() {
        let info = GameInfo::new("测试游戏", "这是一个测试")
            .with_type(GameType::Board)
            .with_difficulty(Difficulty::Hard)
            .with_players(2, 4)
            .with_duration(60)
            .with_origin("中国")
            .with_version("1.0.0");

        assert_eq!(info.name, "测试游戏");
        assert_eq!(info.game_type, GameType::Board);
        assert_eq!(info.difficulty, Difficulty::Hard);
        assert_eq!(info.min_players, 2);
        assert_eq!(info.max_players, 4);
        assert_eq!(info.average_duration, Some(60));
        assert_eq!(info.origin, Some("中国".to_string()));
    }

    #[test]
    fn test_game_state_default() {
        let state = GameState::default();
        assert_eq!(state, GameState::NotStarted);
    }

    #[test]
    fn test_difficulty_display() {
        assert_eq!(format!("{}", Difficulty::Beginner), "入门");
        assert_eq!(format!("{}", Difficulty::Easy), "简单");
        assert_eq!(format!("{}", Difficulty::Normal), "普通");
        assert_eq!(format!("{}", Difficulty::Hard), "困难");
        assert_eq!(format!("{}", Difficulty::Expert), "专家");
        assert_eq!(format!("{}", Difficulty::Master), "大师");
    }

    #[test]
    fn test_game_type_display() {
        assert_eq!(format!("{}", GameType::Board), "棋类");
        assert_eq!(format!("{}", GameType::Card), "卡牌");
        assert_eq!(format!("{}", GameType::Tabletop), "桌游");
        assert_eq!(format!("{}", GameType::Strategy), "策略");
    }

    #[test]
    fn test_game_info_defaults() {
        let info = GameInfo::new("简单游戏", "描述");
        assert_eq!(info.game_type, GameType::Other);
        assert_eq!(info.difficulty, Difficulty::Normal);
        assert_eq!(info.min_players, 1);
        assert_eq!(info.max_players, 1);
        assert!(info.average_duration.is_none());
        assert!(info.origin.is_none());
        assert_eq!(info.version, "1.0.0");
    }
}