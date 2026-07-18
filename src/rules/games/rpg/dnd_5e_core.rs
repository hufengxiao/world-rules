//! D&D 5e 核心规则（Dungeons & Dragons 5th Edition Core Rules）
//!
//! D&D 第五版（2014年发布）的核心规则扩展。
//! 包含更详细的战斗、施法和角色创建规则。

use crate::simple_rule;

simple_rule! {
    struct: Dnd5eCoreRules,
    name: "D&D 5e 核心规则",
    desc: "龙与地下城第五版核心规则系统",
    origin: "美国",
    tags: ["游戏", "RPG", "龙与地下城", "D&D5e"],
}

impl Dnd5eCoreRules {
    /// 扩展种族（包含 PHB 全部）
    pub fn section_extended_races(&self) -> Vec<&'static str> {
        vec![
            "矮人（Dwarf）：CON +2，黑暗视觉 60ft，抗毒",
            "- 山丘矮人：WIS +1，坚韧",
            "- 山地矮人：STR +1，护甲训练",
            "精灵（Elf）：DEX +2，黑暗视觉 60ft， Fey Ancestry",
            "- 高等精灵：INT +1，额外语言，精灵魔法",
            "- 木精灵：WIS +1，移动速度 35ft，面具自然",
            "- 暗精灵（Drow）：CHA +1， Superior Darkvision",
            "半身人（Halfling）：DEX +2，幸运，勇敢",
            "- 轻足半身人：CHA +1，隐形",
            "- 壮硕半身人：CON +1，耐力",
            "人类（Human）：所有属性 +1",
            "- 变体人类：两项属性 +1，技能，专长",
            "龙裔（Dragonborn）：STR +2, CHA +1",
            "侏儒（Gnome）：INT +2，黑暗视觉",
            "- 岩石侏儒：CON +1，修补术",
            "- 森林侏儒：DEX +1，幻觉术",
            "半精灵（Half-Elf）：CHA +2，两项属性 +1",
            "半兽人（Half-Orc）：STR +2, CON +1",
            "提夫林（Tiefling）：CHA +2, INT +1",
            "埃塞里克（Aasimar）（Volo's）：CHA +2",
            "菲尔博格（Firbolg）（Volo's）：WIS +2",
            "戈利安（Goliath）（Volo's）：STR +2",
        ]
    }

    /// 扩展职业（包含 PHB 全部）
    pub fn section_extended_classes(&self) -> Vec<&'static str> {
        vec![
            "野蛮人（Barbarian）：近战坦克，狂暴",
            "吟游诗人（Bard）：全能支持，魔法音乐",
            "牧师（Cleric）：神术施法者，治疗",
            "德鲁伊（Druid）：自然施法者，野性形态",
            "战士（Fighter）：战斗专家，多次攻击",
            "武僧（Monk）：徒手格斗，气系统",
            "圣武士（Paladin）：神圣战士，圣斩",
            "巡林客（Ranger）：野外生存，战斗风格",
            "游荡者（Rogue）：技能专家，偷袭",
            "术士（Sorcerer）：天生施法者， metamagic",
            "邪术师（Warlock）：契约施法者，Invocation",
            "法师（Wizard）：奥术学者，法术书",
            "PHB 共 12 个职业",
        ]
    }

    /// 职业子职业（Archetypes）
    pub fn section_archetypes(&self) -> Vec<&'static str> {
        vec![
            "野蛮人：狂野魔法、图腾武士、狂战士",
            "吟游诗人：洛蕾学院、英勇学院",
            "牧师：生命领域、光明领域、风暴领域等 7 个",
            "德鲁伊：月亮之环、大地之环",
            "战士：冠军勇士、战斗大师、Eldritch Knight",
            "武僧：Way of the Open Hand、Shadow、Four Elements",
            "圣武士：Oath of Devotion、Ancients、Vengeance",
            "巡林客：猎人、兽王",
            "游荡者：Thief、Assassin、Arcane Trickster",
            "术士：Draconic Bloodline、Wild Magic",
            "邪术师：Archfey、Fiend、Great Old One",
            "法师：8 大魔法学派",
        ]
    }

    /// 多职业系统
    pub fn section_multiclassing(&self) -> Vec<&'static str> {
        vec![
            "多职业：结合多个职业能力",
            "属性要求：需满足最低属性要求",
            "- 野蛮人：STR 13",
            "- 吟游诗人：CHA 13",
            "- 牧师：WIS 13",
            "- 德鲁伊：WIS 13",
            "- 战士：STR 13 或 DEX 13",
            "- 武僧：DEX 13 和 WIS 13",
            "- 圣武士：STR 13 和 CHA 13",
            "- 巡林客：DEX 13 和 WIS 13",
            "- 游荡者：DEX 13",
            "- 术士：CHA 13",
            "- 邪术师：CHA 13",
            "- 法师：INT 13",
            "熟练叠加：部分熟练可叠加",
            "施法：多职业施法者法术位合并",
        ]
    }

    /// 专长系统（Feats）
    pub fn section_feats(&self) -> Vec<&'static str> {
        vec![
            "专长替代属性值提升（ASI）",
            "可选规则，需 DM 允许",
            "攻击型专长：",
            "- Great Weapon Master：暴击后额外攻击",
            "- Sharpshooter：远程攻击 +10 伤害",
            "- Polearm Master：长柄武器额外攻击",
            "防御型专长：",
            "- Heavy Armor Master：非魔法伤害 -3",
            "- Shield Master：盾牌推进和敏捷豁免加成",
            "魔法型专长：",
            "- War Caster：专注优势，被包围可施法",
            "- Elemental Adept：忽略元素抗性",
            "技能型专长：",
            "- Skilled：获得 3 个技能熟练",
            "- Expertise：技能 Expertise",
            "移动型专长：",
            "- Mobile：移动速度 +10ft",
            "- Cunning Action：游荡者特性提前获得",
        ]
    }

    /// 战斗机动系统
    pub fn section_combat_maneuvers(&self) -> Vec<&'static str> {
        vec![
            "基本机动（所有角色）：",
            "- 协助（Help）：盟友攻击优势",
            "- 闪避（Dodge）：攻击劣势，敏捷豁免优势",
            "- 逃脱（Escape）：脱离擒抱",
            "- 隐匿（Hide）：进行隐匿检定",
            "- 准备（Ready）：设定触发条件行动",
            "- 搜索（Search）：进行察觉或调查检定",
            "- 使用物品（Use an Object）：使用物品动作",
            "战斗大师机动（战士子职业）：",
            "- Commander's Strike：盟友反应攻击",
            "- Disarming Attack：击落武器",
            "- Evasive Footwork：移动时增加 AC",
            "- Feinting Attack：攻击优势",
            "- Goading Attack：目标对你劣势",
            "- Lunging Attack：增加攻击距离",
            "- Menacing Attack：恐惧效果",
            "- Parry：反应减少伤害",
            "- Precision Attack：增加攻击结果",
            "- Pushing Attack：推移目标",
            "- Rally：给予临时 HP",
            "- Riposte：反应反击",
            "- Sweeping Attack：攻击第二个目标",
            "- Trip Attack：绊倒目标",
        ]
    }

    /// 伤害类型
    pub fn section_damage_types(&self) -> Vec<&'static str> {
        vec![
            "强酸（Acid）：腐蚀性物质",
            "钝击（Bludgeoning）：钝器重击",
            "寒冷（Cold）：冰冻伤害",
            "火焰（Fire）：燃烧伤害",
            "力场（Force）：纯粹魔法能量",
            "闪电（Lightning）：电击伤害",
            "死灵（Necrotic）：生命能量腐化",
            "穿刺（Piercing）：尖锐武器穿刺",
            "毒素（Poison）：毒物伤害",
            "心灵（Psychic）：精神伤害",
            " radiant：神圣能量",
            "挥砍（Slashing）：利刃切割",
            "雷鸣（Thunder）：声波冲击",
            "共 13 种伤害类型",
        ]
    }

    /// 状态效果（Conditions）
    pub fn section_conditions(&self) -> Vec<&'static str> {
        vec![
            "目盲（Blinded）：自动失败视觉检定，攻击劣势",
            "魅惑（Charmed）：不能攻击魅惑者",
            "耳聋（Deafened）：自动失败听觉检定",
            "恐惧（Frightened）：攻击劣势，不能接近恐惧源",
            "擒抱（Grappled）：速度降为 0",
            "失能（Incapacitated）：不能采取行动或反应",
            "隐形（Invisible）：攻击劣势，攻击者优势",
            "麻痹（Paralyzed）：失能 + 自动暴击 + 敏捷豁免失败",
            "石化（Petrified）：重量增加，失能",
            "中毒（Poisoned）：攻击劣势",
            "倒地（Prone）：爬起花费一半移动",
            "受制（Restrained）：速度 0，攻击劣势，攻击者优势",
            "震慑（Stunned）：失能 + 失败敏捷豁免",
            "昏迷（Unconscious）：失能 + 自动暴击",
        ]
    }

    /// 稀有装备系统
    pub fn section_magic_items(&self) -> Vec<&'static str> {
        vec![
            "魔法物品稀有度：",
            "- 普通（Common）：简单魔法效果",
            "- 非普通（Uncommon）：轻微战斗加成",
            "- 稀有（Rare）：显著战斗加成",
            "- 极稀有（Very Rare）：强大魔法效果",
            "- 传说（Legendary）：最强魔法物品",
            "- 神器（Artifact）：独特传奇物品",
            "鉴定：短休息鉴定，或魔法鉴定术",
            "使用：大部分需同调（Attunement）",
            "同调限制：同时最多 3 个物品",
        ]
    }

    /// 法术位表（Spell Slots by Level）
    pub fn section_spell_slots(&self) -> Vec<&'static str> {
        vec![
            "施法者等级决定法术位数量",
            "牧师/德鲁伊/法师/术士（满级施法者）：",
            "- 1级：2个1环",
            "- 2级：3个1环",
            "- 3级：4个1环，2个2环",
            "- 5级：4个1环，3个2环，2个3环",
            "- 11级：所有低环位 + 6环",
            "- 17级：所有低环位 + 1-9环",
            "巡林客/圣武士（半施法者）：",
            "- 法术位获得速度减半",
            "- 但施法者等级按总等级的一半计算",
            "邪术师（契约施法者）：",
            "- 所有法术位都是最高环位",
            "- 短休息恢复",
        ]
    }

    /// 法术专注系统
    pub fn section_concentration(&self) -> Vec<&'static str> {
        vec![
            "专注法术需要施法者集中注意力",
            "每次只能专注一个法术",
            "专注检定触发条件：",
            "- 受到伤害：DC = 10 或 伤害值一半（取较高）",
            "- 环境干扰：DM 决定 DC",
            "- 技能检定成功可免检定（如 Concentration）",
            "专注检定：CON 豁免",
            "专注中断后果：法术立即结束",
            "优势情况： War Caster 专长",
        ]
    }

    /// 经验值与里程碑
    pub fn section_experience(&self) -> Vec<&'static str> {
        vec![
            "标准 XP 系统：",
            "- 1级：0 XP",
            "- 2级：300 XP",
            "- 3级：900 XP",
            "- 4级：2,700 XP",
            "- 5级：6,500 XP",
            "- 10级：64,000 XP",
            "- 15级：195,000 XP",
            "- 20级：355,000 XP",
            "里程碑升级（可选）：",
            "- DM 根据剧情进度升级",
            "- 减少数值追踪",
            "- 鼓励故事导向游戏",
        ]
    }

    /// 团队合作与夹击
    pub fn section_flanking(&self) -> Vec<&'static str> {
        vec![
            "夹击（可选规则）：",
            "- 两个盟友对峙同一目标",
            "- 两者在对立位置",
            "- 攻击时获得优势",
            "协助动作：",
            "- 牺牲动作给予盟友优势",
            "- 技能检定或攻击检定",
            "掩护效果：",
            "- 半掩护：+2 AC 和 DEX 豁免",
            "- 3/4 掩护：+5 AC 和 DEX 豁免",
            "- 全掩护：无法被攻击",
        ]
    }

    /// 环境互动
    pub fn section_environment(&self) -> Vec<&'static str> {
        vec![
            "可互动物体：",
            "- 可攀爬（Climbable）：需要运动检定",
            "- 可破坏（Destructible）：HP 和 AC",
            "- 可躲藏（Hideable）：提供遮蔽",
            "困难地形：",
            "- 每尺移动花费 2 尺移动速度",
            "- 飞行不受影响",
            "危险地形：",
            "- 岩浆：每回合 5d10 火焰伤害",
            "- 深水：需要游泳检定",
            "- 薄冰：需要运动检定",
            "光照等级：",
            "- 明亮：正常视野",
            "- 微光：微光视觉可见，否则劣势",
            "- 黑暗：黑暗视觉可见，否则失明",
        ]
    }

    /// 社交互动规则
    pub fn section_social_interaction(&self) -> Vec<&'static str> {
        vec![
            "态度检定：",
            "- 敌对：可能攻击",
            "- 中立：可能提供有限帮助",
            "- 友善：可能提供帮助",
            "技能应用：",
            "- 威吓（Intimidation）：迫使配合",
            "- 劝说（Persuasion）：说服同意",
            "- 欺瞒（Deception）：误导和欺骗",
            "角色扮演因素：",
            "- DM 可根据角色扮演调整检定",
            "- 好的角色扮演可降低 DC",
            "- 差的角色扮演可提高 DC",
        ]
    }

    /// 探索规则
    pub fn section_exploration(&self) -> Vec<&'static str> {
        vec![
            "行进速度：",
            "- 正常：每小时 3 英里",
            "- 慢速：每小时 2 英里，可隐匿",
            "- 快速：每小时 4 英里，-5 察觉",
            "导航检定：",
            "- 生存检定避免迷路",
            "- DC 基于地形复杂度",
            "追踪：",
            "- 生存检定追踪足迹",
            "- DC 基于目标隐蔽能力",
            "惊动：",
            "- 察觉检定避免惊动",
            "- 受惊第一轮无法行动",
        ]
    }
}

impl crate::rules::core::Rule for Dnd5eCoreRules {
    fn metadata(&self) -> &crate::rules::core::RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> crate::rules::core::RuleCategory {
        crate::rules::core::RuleCategory::games("rpg")
    }
    fn validate(
        &self,
        _ctx: &crate::rules::core::ValidateContext,
    ) -> crate::rules::core::RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "D&D 5e 核心规则",
            &[
                ("专长", &self.section_feats()),
                ("战斗机巧", &self.section_combat_maneuvers()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::Rule;

    #[test]
    fn test_dnd5e_core_rules_creation() {
        let rules = Dnd5eCoreRules::new();
        assert_eq!(rules.metadata().name, "D&D 5e 核心规则");
    }

    #[test]
    fn test_extended_races() {
        let rules = Dnd5eCoreRules::new();
        let races = rules.section_extended_races();
        assert!(races.len() >= 9);
        assert!(races.iter().any(|r| r.contains("龙裔")));
        assert!(races.iter().any(|r| r.contains("提夫林")));
    }

    #[test]
    fn test_extended_classes() {
        let rules = Dnd5eCoreRules::new();
        let classes = rules.section_extended_classes();
        assert_eq!(classes.len(), 13);
    }

    #[test]
    fn test_archetypes() {
        let rules = Dnd5eCoreRules::new();
        let archetypes = rules.section_archetypes();
        assert!(archetypes.len() >= 12);
    }

    #[test]
    fn test_multiclassing_requirements() {
        let rules = Dnd5eCoreRules::new();
        let multiclass = rules.section_multiclassing();
        assert!(multiclass.len() >= 10);
        assert!(multiclass.iter().any(|m| m.contains("野蛮人")));
    }

    #[test]
    fn test_feats() {
        let rules = Dnd5eCoreRules::new();
        let feats = rules.section_feats();
        assert!(feats.len() >= 10);
        assert!(feats.iter().any(|f| f.contains("War Caster")));
    }

    #[test]
    fn test_damage_types() {
        let rules = Dnd5eCoreRules::new();
        let damage = rules.section_damage_types();
        assert_eq!(damage.len(), 14);
    }

    #[test]
    fn test_conditions() {
        let rules = Dnd5eCoreRules::new();
        let conditions = rules.section_conditions();
        assert_eq!(conditions.len(), 14);
    }

    #[test]
    fn test_magic_items() {
        let rules = Dnd5eCoreRules::new();
        let items = rules.section_magic_items();
        assert!(items.len() >= 5);
        assert!(items.iter().any(|i| i.contains("传说")));
    }

    #[test]
    fn test_concentration() {
        let rules = Dnd5eCoreRules::new();
        let conc = rules.section_concentration();
        assert!(conc.len() >= 5);
    }

    #[test]
    fn test_experience_system() {
        let rules = Dnd5eCoreRules::new();
        let xp = rules.section_experience();
        assert!(xp.len() >= 8);
    }

    #[test]
    fn test_environment() {
        let rules = Dnd5eCoreRules::new();
        let env = rules.section_environment();
        assert!(env.len() >= 10);
    }
}
