//! 游戏设计文档模板
//!
//! 提供标准化的游戏设计文档模板，帮助游戏设计师快速创建专业的设计文档。
//!
//! # 示例
//!
//! ```rust
//! use world_rules::rules::game_design_tools::template::*;
//!
//! let template = GameDesignTemplate::new("我的游戏")
//!     .with_genre("策略")
//!     .with_platform("PC")
//!     .with_target_audience("青少年");
//!
//! let doc = template.generate_document();
//! assert!(doc.contains("我的游戏"));
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 游戏设计文档模板
///
/// 提供标准化的游戏设计文档框架，包含游戏概述、玩法机制、角色设定、关卡设计等核心模块。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
///
/// let template = GameDesignTemplate::new("围棋")
///     .with_genre("棋类")
///     .with_platform("桌面");
///
/// assert_eq!(template.game_name, "围棋");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDesignTemplate {
    /// 游戏名称
    pub game_name: String,
    /// 游戏类型
    pub genre: Option<String>,
    /// 目标平台
    pub platform: Option<String>,
    /// 目标受众
    pub target_audience: Option<String>,
    /// 游戏概述
    pub overview: GameOverview,
    /// 核心玩法
    pub core_mechanics: Vec<Mechanic>,
    /// 角色设定
    pub characters: Vec<Character>,
    /// 关卡设计
    pub levels: Vec<Level>,
    /// 物品系统
    pub items: Vec<Item>,
    /// 成就系统
    pub achievements: Vec<Achievement>,
    /// 规则列表
    pub rules: Vec<RuleDefinition>,
    /// 自定义字段
    pub custom_fields: HashMap<String, String>,
}

/// 游戏概述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameOverview {
    /// 游戏简介
    pub description: String,
    /// 游戏目标
    pub objective: String,
    /// 游戏背景故事
    pub backstory: Option<String>,
    /// 艺术风格
    pub art_style: Option<String>,
    /// 音乐风格
    pub music_style: Option<String>,
}

/// 游戏机制定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mechanic {
    /// 机制名称
    pub name: String,
    /// 机制描述
    pub description: String,
    /// 触发条件
    pub trigger: String,
    /// 效果
    pub effect: String,
    /// 相关参数
    pub parameters: HashMap<String, f64>,
}

/// 角色定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// 角色名称
    pub name: String,
    /// 角色类型
    pub character_type: CharacterType,
    /// 角色描述
    pub description: String,
    /// 属性值
    pub attributes: HashMap<String, f64>,
    /// 特殊能力
    pub abilities: Vec<String>,
}

/// 角色类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterType {
    /// 玩家角色
    Player,
    /// 非玩家角色
    NPC,
    /// 敌人
    Enemy,
    /// Boss
    Boss,
    /// 召唤物
    Summon,
}

/// 关卡定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    /// 关卡名称
    pub name: String,
    /// 关卡描述
    pub description: String,
    /// 难度等级
    pub difficulty: u8,
    /// 胜利条件
    pub win_condition: String,
    /// 失败条件
    pub lose_condition: String,
    /// 环境设置
    pub environment: HashMap<String, String>,
}

/// 物品定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// 物品名称
    pub name: String,
    /// 物品类型
    pub item_type: ItemType,
    /// 物品描述
    pub description: String,
    /// 效果
    pub effects: Vec<Effect>,
    /// 稀有度
    pub rarity: Rarity,
}

/// 物品类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    /// 武器
    Weapon,
    /// 防具
    Armor,
    /// 消耗品
    Consumable,
    /// 任务物品
    Quest,
    /// 材料
    Material,
}

/// 效果定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    /// 效果名称
    pub name: String,
    /// 效果数值
    pub value: f64,
    /// 持续时间（秒）
    pub duration: Option<f64>,
}

/// 稀有度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    /// 普通
    Common,
    /// 稀有
    Rare,
    /// 史诗
    Epic,
    /// 传说
    Legendary,
}

/// 成就定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// 成就名称
    pub name: String,
    /// 成就描述
    pub description: String,
    /// 解锁条件
    pub unlock_condition: String,
    /// 奖励
    pub reward: String,
    /// 隐藏成就
    pub hidden: bool,
}

/// 规则定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 规则类型
    pub rule_type: RuleType,
    /// 规则内容
    pub content: String,
}

/// 规则类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    /// 核心规则
    Core,
    /// 玩法规则
    Gameplay,
    /// 计分规则
    Scoring,
    /// 时间规则
    Timing,
    /// 胜负规则
    Victory,
    /// 特殊规则
    Special,
}

impl Default for GameDesignTemplate {
    fn default() -> Self {
        Self::new("未命名游戏")
    }
}

impl GameDesignTemplate {
    /// 创建新的游戏设计模板
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("我的游戏");
    /// assert_eq!(template.game_name, "我的游戏");
    /// ```
    pub fn new(game_name: impl Into<String>) -> Self {
        Self {
            game_name: game_name.into(),
            genre: None,
            platform: None,
            target_audience: None,
            overview: GameOverview {
                description: String::new(),
                objective: String::new(),
                backstory: None,
                art_style: None,
                music_style: None,
            },
            core_mechanics: Vec::new(),
            characters: Vec::new(),
            levels: Vec::new(),
            items: Vec::new(),
            achievements: Vec::new(),
            rules: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }

    /// 设置游戏类型
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("游戏").with_genre("策略");
    /// assert_eq!(template.genre, Some("策略".to_string()));
    /// ```
    pub fn with_genre(mut self, genre: impl Into<String>) -> Self {
        self.genre = Some(genre.into());
        self
    }

    /// 设置目标平台
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("游戏").with_platform("PC");
    /// assert_eq!(template.platform, Some("PC".to_string()));
    /// ```
    pub fn with_platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    /// 设置目标受众
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("游戏").with_target_audience("青少年");
    /// assert_eq!(template.target_audience, Some("青少年".to_string()));
    /// ```
    pub fn with_target_audience(mut self, audience: impl Into<String>) -> Self {
        self.target_audience = Some(audience.into());
        self
    }

    /// 设置游戏概述
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("游戏")
    ///     .with_overview("这是一款策略游戏", "击败所有敌人");
    /// assert_eq!(template.overview.description, "这是一款策略游戏");
    /// ```
    pub fn with_overview(
        mut self,
        description: impl Into<String>,
        objective: impl Into<String>,
    ) -> Self {
        self.overview.description = description.into();
        self.overview.objective = objective.into();
        self
    }

    /// 添加核心机制
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("游戏")
    ///     .add_mechanic("回合制", "玩家轮流行动", "回合开始", "执行行动");
    /// assert_eq!(template.core_mechanics.len(), 1);
    /// ```
    pub fn add_mechanic(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        trigger: impl Into<String>,
        effect: impl Into<String>,
    ) -> Self {
        self.core_mechanics.push(Mechanic {
            name: name.into(),
            description: description.into(),
            trigger: trigger.into(),
            effect: effect.into(),
            parameters: HashMap::new(),
        });
        self
    }

    /// 添加角色
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::{GameDesignTemplate, CharacterType};
    ///
    /// let template = GameDesignTemplate::new("游戏")
    ///     .add_character("战士", CharacterType::Player, "近战角色");
    /// assert_eq!(template.characters.len(), 1);
    /// ```
    pub fn add_character(
        mut self,
        name: impl Into<String>,
        character_type: CharacterType,
        description: impl Into<String>,
    ) -> Self {
        self.characters.push(Character {
            name: name.into(),
            character_type,
            description: description.into(),
            attributes: HashMap::new(),
            abilities: Vec::new(),
        });
        self
    }

    /// 添加关卡
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("游戏")
    ///     .add_level("第一关", "新手教程", 1, "击败所有敌人", "角色死亡");
    /// assert_eq!(template.levels.len(), 1);
    /// ```
    pub fn add_level(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        difficulty: u8,
        win_condition: impl Into<String>,
        lose_condition: impl Into<String>,
    ) -> Self {
        self.levels.push(Level {
            name: name.into(),
            description: description.into(),
            difficulty,
            win_condition: win_condition.into(),
            lose_condition: lose_condition.into(),
            environment: HashMap::new(),
        });
        self
    }

    /// 添加规则
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::{GameDesignTemplate, RuleType};
    ///
    /// let template = GameDesignTemplate::new("游戏")
    ///     .add_rule("回合时间", "每回合限时60秒", RuleType::Timing, "超时判负");
    /// assert_eq!(template.rules.len(), 1);
    /// ```
    pub fn add_rule(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        rule_type: RuleType,
        content: impl Into<String>,
    ) -> Self {
        self.rules.push(RuleDefinition {
            name: name.into(),
            description: description.into(),
            rule_type,
            content: content.into(),
        });
        self
    }

    /// 生成设计文档（Markdown格式）
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("围棋");
    /// let doc = template.generate_document();
    /// assert!(doc.contains("围棋"));
    /// ```
    pub fn generate_document(&self) -> String {
        let mut doc = String::new();

        // 标题
        doc.push_str(&format!("# {} 设计文档\n\n", self.game_name));

        // 基本信息
        doc.push_str("## 基本信息\n\n");
        doc.push_str(&format!("- **游戏名称**: {}\n", self.game_name));
        if let Some(ref genre) = self.genre {
            doc.push_str(&format!("- **游戏类型**: {}\n", genre));
        }
        if let Some(ref platform) = self.platform {
            doc.push_str(&format!("- **目标平台**: {}\n", platform));
        }
        if let Some(ref audience) = self.target_audience {
            doc.push_str(&format!("- **目标受众**: {}\n", audience));
        }
        doc.push('\n');

        // 游戏概述
        if !self.overview.description.is_empty() {
            doc.push_str("## 游戏概述\n\n");
            doc.push_str(&format!("{}\n\n", self.overview.description));
            doc.push_str(&format!("**游戏目标**: {}\n\n", self.overview.objective));
            if let Some(ref backstory) = self.overview.backstory {
                doc.push_str(&format!("**背景故事**: {}\n\n", backstory));
            }
        }

        // 核心玩法
        if !self.core_mechanics.is_empty() {
            doc.push_str("## 核心玩法\n\n");
            for mechanic in &self.core_mechanics {
                doc.push_str(&format!("### {}\n\n", mechanic.name));
                doc.push_str(&format!("{}\n\n", mechanic.description));
                doc.push_str(&format!("- **触发条件**: {}\n", mechanic.trigger));
                doc.push_str(&format!("- **效果**: {}\n\n", mechanic.effect));
            }
        }

        // 角色设定
        if !self.characters.is_empty() {
            doc.push_str("## 角色设定\n\n");
            for character in &self.characters {
                doc.push_str(&format!(
                    "### {} ({})\n\n",
                    character.name,
                    character.character_type_name()
                ));
                doc.push_str(&format!("{}\n\n", character.description));
                if !character.attributes.is_empty() {
                    doc.push_str("**属性**:\n");
                    for (key, value) in &character.attributes {
                        doc.push_str(&format!("- {}: {}\n", key, value));
                    }
                    doc.push('\n');
                }
                if !character.abilities.is_empty() {
                    doc.push_str(&format!(
                        "**特殊能力**: {}\n\n",
                        character.abilities.join(", ")
                    ));
                }
            }
        }

        // 关卡设计
        if !self.levels.is_empty() {
            doc.push_str("## 关卡设计\n\n");
            for level in &self.levels {
                doc.push_str(&format!(
                    "### {} (难度: {})\n\n",
                    level.name, level.difficulty
                ));
                doc.push_str(&format!("{}\n\n", level.description));
                doc.push_str(&format!("- **胜利条件**: {}\n", level.win_condition));
                doc.push_str(&format!("- **失败条件**: {}\n\n", level.lose_condition));
            }
        }

        // 物品系统
        if !self.items.is_empty() {
            doc.push_str("## 物品系统\n\n");
            for item in &self.items {
                doc.push_str(&format!("### {} ({})\n\n", item.name, item.rarity_name()));
                doc.push_str(&format!("{}\n\n", item.description));
            }
        }

        // 成就系统
        if !self.achievements.is_empty() {
            doc.push_str("## 成就系统\n\n");
            for achievement in &self.achievements {
                doc.push_str(&format!("### {}\n\n", achievement.name));
                doc.push_str(&format!("{}\n\n", achievement.description));
                doc.push_str(&format!(
                    "- **解锁条件**: {}\n",
                    achievement.unlock_condition
                ));
                doc.push_str(&format!("- **奖励**: {}\n", achievement.reward));
                if achievement.hidden {
                    doc.push_str("- **隐藏成就**: 是\n");
                }
                doc.push('\n');
            }
        }

        // 规则列表
        if !self.rules.is_empty() {
            doc.push_str("## 游戏规则\n\n");
            for rule in &self.rules {
                doc.push_str(&format!(
                    "### {} ({})\n\n",
                    rule.name,
                    rule.rule_type_name()
                ));
                doc.push_str(&format!("{}\n\n", rule.description));
                doc.push_str(&format!("{}\n\n", rule.content));
            }
        }

        // 自定义字段
        if !self.custom_fields.is_empty() {
            doc.push_str("## 其他信息\n\n");
            for (key, value) in &self.custom_fields {
                doc.push_str(&format!("- **{}**: {}\n", key, value));
            }
            doc.push('\n');
        }

        doc
    }

    /// 导出为JSON格式
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("围棋");
    /// let json = template.to_json();
    /// assert!(json.contains("围棋"));
    /// ```
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("序列化错误: {}", e))
    }

    /// 从JSON导入
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::GameDesignTemplate;
    ///
    /// let template = GameDesignTemplate::new("围棋");
    /// let json = template.to_json();
    /// let loaded = GameDesignTemplate::from_json(&json).unwrap();
    /// assert_eq!(loaded.game_name, "围棋");
    /// ```
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Character {
    /// 获取角色类型名称
    fn character_type_name(&self) -> &'static str {
        match self.character_type {
            CharacterType::Player => "玩家",
            CharacterType::NPC => "NPC",
            CharacterType::Enemy => "敌人",
            CharacterType::Boss => "Boss",
            CharacterType::Summon => "召唤物",
        }
    }
}

impl Item {
    /// 获取稀有度名称
    fn rarity_name(&self) -> &'static str {
        match self.rarity {
            Rarity::Common => "普通",
            Rarity::Rare => "稀有",
            Rarity::Epic => "史诗",
            Rarity::Legendary => "传说",
        }
    }
}

impl RuleDefinition {
    /// 获取规则类型名称
    fn rule_type_name(&self) -> &'static str {
        match self.rule_type {
            RuleType::Core => "核心规则",
            RuleType::Gameplay => "玩法规则",
            RuleType::Scoring => "计分规则",
            RuleType::Timing => "时间规则",
            RuleType::Victory => "胜负规则",
            RuleType::Special => "特殊规则",
        }
    }
}

/// 游戏文档（已生成的完整文档）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDocument {
    /// 文档标题
    pub title: String,
    /// 文档版本
    pub version: String,
    /// 创建日期
    pub created_at: String,
    /// 最后修改日期
    pub modified_at: String,
    /// 作者
    pub author: Option<String>,
    /// 文档内容（Markdown格式）
    pub content: String,
    /// 文档标签
    pub tags: Vec<String>,
}

impl GameDocument {
    /// 从模板创建文档
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::{GameDesignTemplate, GameDocument};
    ///
    /// let template = GameDesignTemplate::new("围棋");
    /// let doc = GameDocument::from_template(&template);
    /// assert_eq!(doc.title, "围棋");
    /// ```
    pub fn from_template(template: &GameDesignTemplate) -> Self {
        Self {
            title: template.game_name.clone(),
            version: "1.0.0".to_string(),
            created_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
            modified_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
            author: None,
            content: template.generate_document(),
            tags: Vec::new(),
        }
    }

    /// 设置作者
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::{GameDesignTemplate, GameDocument};
    ///
    /// let template = GameDesignTemplate::new("围棋");
    /// let doc = GameDocument::from_template(&template).with_author("设计师");
    /// assert_eq!(doc.author, Some("设计师".to_string()));
    /// ```
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// 添加标签
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::{GameDesignTemplate, GameDocument};
    ///
    /// let template = GameDesignTemplate::new("围棋");
    /// let doc = GameDocument::from_template(&template).with_tag("棋类");
    /// assert!(doc.tags.contains(&"棋类".to_string()));
    /// ```
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// 保存到文件
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::template::{GameDesignTemplate, GameDocument};
    ///
    /// let template = GameDesignTemplate::new("围棋");
    /// let doc = GameDocument::from_template(&template);
    /// // let result = doc.save_to_file("design.md");
    /// // assert!(result.is_ok());
    /// ```
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        std::fs::write(path.as_ref(), &self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = GameDesignTemplate::new("测试游戏")
            .with_genre("策略")
            .with_platform("PC");

        assert_eq!(template.game_name, "测试游戏");
        assert_eq!(template.genre, Some("策略".to_string()));
        assert_eq!(template.platform, Some("PC".to_string()));
    }

    #[test]
    fn test_generate_document() {
        let template = GameDesignTemplate::new("围棋")
            .with_genre("棋类")
            .with_overview("古老的棋类游戏", "围地多者胜")
            .add_rule("落子", "轮流在棋盘上放置棋子", RuleType::Core, "黑先白后");

        let doc = template.generate_document();
        assert!(doc.contains("围棋"));
        assert!(doc.contains("棋类"));
        assert!(doc.contains("古老的棋类游戏"));
    }

    #[test]
    fn test_json_serialization() {
        let template = GameDesignTemplate::new("围棋").with_genre("棋类");

        let json = template.to_json();
        assert!(json.contains("围棋"));

        let loaded = GameDesignTemplate::from_json(&json).unwrap();
        assert_eq!(loaded.game_name, "围棋");
    }

    #[test]
    fn test_add_mechanics() {
        let template = GameDesignTemplate::new("游戏")
            .add_mechanic("回合制", "轮流行动", "回合开始", "执行行动")
            .add_mechanic("资源收集", "收集资源", "回合开始", "增加资源");

        assert_eq!(template.core_mechanics.len(), 2);
        assert_eq!(template.core_mechanics[0].name, "回合制");
    }

    #[test]
    fn test_add_characters() {
        let template = GameDesignTemplate::new("游戏")
            .add_character("战士", CharacterType::Player, "近战角色")
            .add_character("Boss", CharacterType::Boss, "最终Boss");

        assert_eq!(template.characters.len(), 2);
        assert_eq!(template.characters[0].character_type, CharacterType::Player);
    }

    #[test]
    fn test_add_levels() {
        let template = GameDesignTemplate::new("游戏").add_level(
            "第一关",
            "新手教程",
            1,
            "击败敌人",
            "角色死亡",
        );

        assert_eq!(template.levels.len(), 1);
        assert_eq!(template.levels[0].difficulty, 1);
    }

    #[test]
    fn test_game_document() {
        let template = GameDesignTemplate::new("围棋");
        let doc = GameDocument::from_template(&template)
            .with_author("设计师")
            .with_tag("棋类");

        assert_eq!(doc.title, "围棋");
        assert_eq!(doc.author, Some("设计师".to_string()));
        assert!(doc.tags.contains(&"棋类".to_string()));
    }
}
