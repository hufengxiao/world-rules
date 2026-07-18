//! D&D 基础规则（Dungeons & Dragons Basic Rules）
//!
//! 龙与地下城（D&D）是世界上第一个也是最著名的桌面角色扮演游戏。
//! 本模块实现 D&D 的基础规则系统。

use crate::simple_rule;

simple_rule! {
    struct: DndBasicRules,
    name: "D&D 基础规则",
    desc: "龙与地下城（Dungeons & Dragons）基础规则系统",
    origin: "美国",
    tags: ["游戏", "RPG", "龙与地下城", "桌面角色扮演"],
}

impl DndBasicRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "Dungeons & Dragons（龙与地下城）",
            "1974 年由 Gary Gygax 和 Dave Arneson 创建",
            "世界第一个桌面角色扮演游戏",
            "玩家扮演冒险者探索奇幻世界",
            "由 Dungeon Master（DM）主持游戏",
            "核心机制：d20 系统 + 三大支柱",
        ]
    }

    /// 核心机制 - d20 系统
    pub fn section_d20_system(&self) -> Vec<&'static str> {
        vec![
            "核心骰子：20 面骰（d20）",
            "检定公式：d20 + 属性调整值 + 熟练加值",
            "对抗 DC（难度等级）或对手检定",
            "优势（Advantage）：掷两次 d20，取较高值",
            "劣势（Disadvantage）：掷两次 d20，取较低值",
            "自然 20：攻击时为暴击，检定时为最佳结果",
            "自然 1：攻击时为失误，检定时为最差结果",
        ]
    }

    /// 三大支柱
    pub fn section_three_pillars(&self) -> Vec<&'static str> {
        vec![
            "1. 战斗（Combat）：回合制战术战斗",
            "2. 探索（Exploration）：地牢探索和世界发现",
            "3. 社交（Social Interaction）：角色扮演和 NPC 互动",
            "三支柱平衡构成完整游戏体验",
            "不同游戏风格可侧重不同支柱",
        ]
    }

    /// 六大属性
    pub fn section_ability_scores(&self) -> Vec<&'static str> {
        vec![
            "力量（Strength, STR）：体能、近战攻击、负重",
            "敏捷（Dexterity, DEX）：灵巧、反射、远程攻击",
            "体质（Constitution, CON）：耐力、生命值、集中",
            "智力（Intelligence, INT）：推理、知识、记忆",
            "感知（Wisdom, WIS）：洞察、直觉、意志",
            "魅力（Charisma, CHA）：影响力、领导力、魅力",
            "属性范围：通常 1-20，人类平均为 10",
            "调整值计算：（属性 - 10）÷ 2（向下取整）",
        ]
    }

    /// 核心种族（Basic Rules 包含）
    pub fn section_races(&self) -> Vec<&'static str> {
        vec![
            "矮人（Dwarf）：CON +2，黑暗视觉，抗毒",
            "精灵（Elf）：DEX +2，黑暗视觉，精灵魔法",
            "半身人（Halfling）：DEX +2，幸运，勇敢",
            "人类（Human）：所有属性 +1（或变体：两项 +1）",
            "基础规则仅包含 PHB 核心种族",
        ]
    }

    /// 核心职业（Basic Rules 包含）
    pub fn section_classes(&self) -> Vec<&'static str> {
        vec![
            "牧师（Cleric）：神术施法者，治疗和支持",
            "战士（Fighter）：战斗专家，多种战斗风格",
            "游荡者（Rogue）：技能专家，偷袭伤害",
            "法师（Wizard）：奥术施法者，法术书管理",
            "基础规则仅包含四个核心职业",
            "每个职业有独特的职业特性（Class Features）",
        ]
    }

    /// 背景系统
    pub fn section_backgrounds(&self) -> Vec<&'static str> {
        vec![
            "背景定义角色的过往经历",
            "提供技能熟练和工具熟练",
            "提供特性（Feature）和语言",
            "基础背景：侍僧、folk hero、贵族、学者等",
            "背景塑造角色性格和动机",
        ]
    }

    /// 等级系统
    pub fn section_levels(&self) -> Vec<&'static str> {
        vec![
            "角色等级范围：1-20 级",
            "经验值（XP）累积提升等级",
            "每级获得职业特性",
            "偶数等级提升熟练加值（某些版本）",
            "4 级及之后每 4 级获得属性值提升（ASI）",
            "里程碑升级：DM 根据剧情决定升级",
        ]
    }

    /// 熟练系统
    pub fn section_proficiency(&self) -> Vec<&'static str> {
        vec![
            "熟练加值：基于等级（+2 到 +6）",
            "等级 1-4：+2",
            "等级 5-8：+3",
            "等级 9-12：+4",
            "等级 13-16：+5",
            "等级 17-20：+6",
            "熟练应用于：攻击、技能、豁免、法术 DC",
        ]
    }

    /// 技能系统
    pub fn section_skills(&self) -> Vec<&'static str> {
        vec![
            "特技（Acrobatics）：DEX - 平衡和翻滚",
            "驯兽（Animal Handling）：WIS - 驾驭动物",
            "运动（Athletics）：STR - 攀爬、跳跃、游泳",
            "欺瞒（Deception）：CHA - 欺骗和误导",
            "历史（History）：INT - 历史知识",
            "洞察（Insight）：WIS - 读懂他人",
            "威吓（Intimidation）：CHA - 威慑",
            "调查（Investigation）：INT - 推理和调查",
            "医药（Medicine）：WIS - 医疗和诊断",
            "自然（Nature）：INT - 自然知识",
            "察觉（Perception）：WIS - 警觉和观察",
            "表演（Performance）：CHA - 艺术表演",
            "劝说（Persuasion）：CHA - 说服和影响",
            "宗教（Religion）：INT - 宗教知识",
            "巧手（Sleight of Hand）：DEX - 偷窃和戏法",
            "隐匿（Stealth）：DEX - 悄悄行动",
            "生存（Survival）：WIS - 野外生存",
            "共 18 种技能",
        ]
    }

    /// 豁免系统
    pub fn section_saving_throws(&self) -> Vec<&'static str> {
        vec![
            "力量豁免：对抗物理力量和身体压制",
            "敏捷豁免：躲避危险和区域效果",
            "体质豁免：抵抗疾病、毒素和疲劳",
            "智力豁免：抵抗精神攻击和记忆修改",
            "感知豁免：抵抗心灵控制和幻觉",
            "魅力豁免：抵抗附身和强迫",
            "职业提供特定豁免熟练",
        ]
    }

    /// 生命值系统
    pub fn section_hit_points(&self) -> Vec<&'static str> {
        vec![
            "HP = 基础 HP + 等级 × (职业 HD + CON 调整值)",
            "生命骰（Hit Dice）因职业不同",
            "战士：d10，牧师：d8，游荡者：d8，法师：d6",
            "短休息时可用 HD 恢复 HP",
            "长休息后恢复所有 HP 和一半 HD",
            "生命值归零时进入死亡豁免",
        ]
    }

    /// 护甲等级系统
    pub fn section_armor_class(&self) -> Vec<&'static str> {
        vec![
            "AC 决定被攻击的难度",
            "基础 AC = 10 + DEX 调整值",
            "轻甲：AC = 基础 + 护甲加值",
            "中甲：AC = 护甲基础 + DEX（最多 +2）",
            "重甲：固定 AC，不计算 DEX",
            "盾牌：+2 AC（需要装备动作）",
            "无甲防御：某些职业特性提供额外 AC",
        ]
    }

    /// 战斗系统
    pub fn section_combat(&self) -> Vec<&'static str> {
        vec![
            "回合制战斗，每轮所有角色行动一次",
            "先攻检定：d20 + DEX 调整值",
            "行动（Action）：攻击、施法、协助等",
            "附赠动作（Bonus Action）：特定能力",
            "反应（Reaction）：借机攻击等",
            "移动（Movement）：可达距离内自由移动",
            "动作经济：每回合有限行动次数",
        ]
    }

    /// 攻击系统
    pub fn section_attacks(&self) -> Vec<&'static str> {
        vec![
            "攻击检定：d20 + 熟练 + 属性调整值",
            "对抗目标 AC",
            "近战攻击：使用 STR（或 DEX - 敏捷武器）",
            "远程攻击：使用 DEX（或 STR - 投掷武器）",
            "自然 20：暴击（Critical Hit），伤害骰翻倍",
            "自然 1：失误（Critical Miss）",
            "伤害 = 武器骰 + 属性调整值",
        ]
    }

    /// 法术系统基础
    pub fn section_spellcasting_basic(&self) -> Vec<&'static str> {
        vec![
            "法术位（Spell Slots）：施法资源",
            "法术等级：1-9 环",
            "准备法术：每日准备可用法术",
            "自发施法：某些职业已知法术直接施放",
            "法术攻击：d20 + 熟练 + 施法属性",
            "法术 DC：8 + 熟练 + 施法属性调整值",
            "专注：部分法术需要专注维持",
        ]
    }

    /// 休息系统
    pub fn section_rests(&self) -> Vec<&'static str> {
        vec![
            "短休息（Short Rest）：至少 1 小时",
            "- 可花费生命骰恢复 HP",
            "- 某些职业特性需要短休息恢复",
            "长休息（Long Rest）：至少 8 小时",
            "- 恢复所有 HP",
            "- 恢复一半已用生命骰",
            "- 恢复所有法术位和职业特性",
            "- 每天最多一次长休息",
        ]
    }

    /// 装备系统
    pub fn section_equipment(&self) -> Vec<&'static str> {
        vec![
            "起始装备：由职业和背景决定",
            "武器：简单武器、军用武器",
            "护甲：轻甲、中甲、重甲",
            "工具：工具熟练提供检定优势",
            "负重：STR × 15 磅",
            "金币系统：gp、sp、cp",
        ]
    }

    /// 阵营系统
    pub fn section_alignment(&self) -> Vec<&'static str> {
        vec![
            "守序善良（Lawful Good）",
            "中立善良（Neutral Good）",
            "混乱善良（Chaotic Good）",
            "守序中立（Lawful Neutral）",
            "绝对中立（True Neutral）",
            "混乱中立（Chaotic Neutral）",
            "守序邪恶（Lawful Evil）",
            "中立邪恶（Neutral Evil）",
            "混乱邪恶（Chaotic Evil）",
            "阵营指导角色行为准则",
        ]
    }

    /// 死亡与复活
    pub fn section_death(&self) -> Vec<&'static str> {
        vec![
            "HP 降为 0 时进入昏迷状态",
            "死亡豁免检定：每回合开始时",
            "- 成功（10+）：累计一个成功",
            "- 失败（<10）：累计一个失败",
            "- 自然 20：恢复 1 HP",
            "- 自然 1：累计两个失败",
            "累计三次成功：伤势稳定",
            "累计三次失败：角色死亡",
            "复活法术：复活术、真死术等",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dnd_basic_rules_creation() {
        let rules = DndBasicRules::new();
        assert_eq!(rules.metadata().name, "D&D 基础规则");
        assert!(!rules.metadata().description.is_empty());
    }

    #[test]
    fn test_d20_system() {
        let rules = DndBasicRules::new();
        let d20_rules = rules.section_d20_system();
        assert!(d20_rules.len() >= 5);
        assert!(d20_rules.iter().any(|r| r.contains("d20")));
    }

    #[test]
    fn test_ability_scores() {
        let rules = DndBasicRules::new();
        let abilities = rules.section_ability_scores();
        assert!(abilities.len() >= 6);
        assert!(abilities.iter().any(|a| a.contains("力量")));
        assert!(abilities.iter().any(|a| a.contains("敏捷")));
    }

    #[test]
    fn test_three_pillars() {
        let rules = DndBasicRules::new();
        let pillars = rules.section_three_pillars();
        assert_eq!(pillars.len(), 5);
        assert!(pillars.iter().any(|p| p.contains("战斗")));
        assert!(pillars.iter().any(|p| p.contains("探索")));
        assert!(pillars.iter().any(|p| p.contains("社交")));
    }

    #[test]
    fn test_races() {
        let rules = DndBasicRules::new();
        let races = rules.section_races();
        assert!(races.len() >= 4);
        assert!(races.iter().any(|r| r.contains("矮人")));
        assert!(races.iter().any(|r| r.contains("精灵")));
    }

    #[test]
    fn test_classes() {
        let rules = DndBasicRules::new();
        let classes = rules.section_classes();
        assert!(classes.len() >= 4);
        assert!(classes.iter().any(|c| c.contains("战士")));
        assert!(classes.iter().any(|c| c.contains("法师")));
    }

    #[test]
    fn test_proficiency_bonus() {
        let rules = DndBasicRules::new();
        let proficiency = rules.section_proficiency();
        assert!(proficiency.len() >= 5);
        assert!(proficiency.iter().any(|p| p.contains("+2")));
        assert!(proficiency.iter().any(|p| p.contains("+6")));
    }

    #[test]
    fn test_skills() {
        let rules = DndBasicRules::new();
        let skills = rules.section_skills();
        assert_eq!(skills.len(), 18);
    }

    #[test]
    fn test_combat_system() {
        let rules = DndBasicRules::new();
        let combat = rules.section_combat();
        assert!(combat.len() >= 5);
        assert!(combat.iter().any(|c| c.contains("先攻")));
    }

    #[test]
    fn test_rests() {
        let rules = DndBasicRules::new();
        let rests = rules.section_rests();
        assert!(rests.len() >= 5);
        assert!(rests.iter().any(|r| r.contains("短休息")));
        assert!(rests.iter().any(|r| r.contains("长休息")));
    }

    #[test]
    fn test_death_mechanics() {
        let rules = DndBasicRules::new();
        let death = rules.section_death();
        assert!(death.len() >= 5);
        assert!(death.iter().any(|d| d.contains("死亡豁免")));
    }
}
