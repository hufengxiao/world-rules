//! 路径探路者规则（Pathfinder Roleplaying Game）
//!
//! Paizo Publishing 开发的 D&D 3.5 继承者，提供更深入的角色定制。

use crate::simple_rule;

simple_rule! {
    struct: PathfinderRules,
    name: "路径探路者规则",
    desc: "Pathfinder 角色扮演游戏规则系统",
    origin: "美国",
    tags: ["游戏", "RPG", "Pathfinder", "路径探路者"],
}

impl PathfinderRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "Pathfinder Roleplaying Game",
            "2009 年发布，继承 D&D 3.5 系统",
            "更深入的角色定制选项",
            "兼容 D&D 3.5 材料",
            "第二版（2019）简化规则",
            "GRD（Game Reference Document）免费在线",
        ]
    }

    /// 第一版核心种族
    pub fn section_pf1_races(&self) -> Vec<&'static str> {
        vec![
            "矮人（Dwarf）：CON +2, CHA -2，多项种族特性",
            "精灵（Elf）：DEX +2, CON -2，低光视觉",
            "侏儒（Gnome）：CON +2, STR -2，小型",
            "半精灵（Half-Elf）：灵活属性加成",
            "半兽人（Half-Orc）：STR +2, CHA -2，恐吓加成",
            "半身人（Halfling）：DEX +2, STR -2，幸运",
            "人类（Human）：灵活属性 +2，额外技能和专长",
            "核心规则共 7 个种族",
        ]
    }

    /// 第一版核心职业
    pub fn section_pf1_classes(&self) -> Vec<&'static str> {
        vec![
            "野蛮人（Barbarian）：狂暴战斗者",
            "吟游诗人（Bard）：全能表演者",
            "牧师（Cleric）：神术施法者",
            "德鲁伊（Druid）：自然施法者",
            "战士（Fighter）：战斗大师",
            "武僧（Monk）：徒手格斗者",
            "圣武士（Paladin）：神圣战士",
            "巡林客（Ranger）：野外生存者",
            "游荡者（Rogue）：技能专家",
            "术士（Sorcerer）：天生施法者",
            "法师（Wizard）：奥术学者",
            "核心规则共 11 个职业",
        ]
    }

    /// 基础职业变体（Archetypes）
    pub fn section_archetypes(&self) -> Vec<&'static str> {
        vec![
            "替代职业特性系统",
            "每个职业有多个变体选择",
            "野蛮人：Totem Warrior, Urban Barbarian",
            "吟游诗人：Arcane Duelist, Archaeologist",
            "战士：Archer, Armor Master, Two-Handed Fighter",
            "游荡者：Acrobat, Scout, Sniper",
            "可叠加多个变体",
            "Ultimate Magic 和 Ultimate Combat 扩展变体",
        ]
    }

    /// 专长系统
    pub fn section_feats(&self) -> Vec<&'static str> {
        vec![
            "更丰富的专长树系统",
            "战斗专长：",
            "- Power Attack：伤害加成，攻击减值",
            "- Weapon Focus：+1 武器攻击",
            "- Dazzling Display：恐吓整轮动作",
            "- Improved Critical：武器暴击范围 ×2",
            "魔法专长：",
            "- Improved Initiative：+4 先攻",
            "- Spell Focus：+1 法术 DC",
            "- Augment Summoning：召唤生物增强",
            "技能专长：",
            "- Skill Focus：+3 技能（+6 at 10 级）",
            "- Athletic：运动技能加成",
            "种族专长：种族特定专长树",
        ]
    }

    /// 技能系统
    pub fn section_skills(&self) -> Vec<&'static str> {
        vec![
            "技能点系统：每级 INT 调整值 + 职业技能点",
            "职业技能：+3 熟练加成（首级时）",
            "技能上限：等级 + 3",
            "核心技能：",
            "- 特技（Acrobatics）：DEX",
            "- 攀爬（Climb）：STR",
            "- 知识（Knowledge）：INT（多个子类别）",
            "- 察觉（Perception）：WIS（合并 Listen/Spot）",
            "- 隐匿（Stealth）：DEX（合并 Hide/Move Silently）",
            "技能解锁（Skill Unlocks，可选）：",
            "- 5/10/15/20 级解锁技能特殊能力",
        ]
    }

    /// 战斗系统
    pub fn section_combat(&self) -> Vec<&'static str> {
        vec![
            "攻击类型：",
            "- 标准攻击：单次攻击",
            "- 全回合攻击：多次攻击（-5 惩罚）",
            "- Cleave：攻击多个相邻目标",
            "- Spring Attack：移动中攻击",
            "战技（Combat Maneuvers）：",
            "- 擒抱（Grapple）：控制对手",
            "- 推撞（Bull Rush）：推移目标",
            "- 绊摔（Trip）：使目标倒地",
            "- 缴械（Disarm）：击落武器",
            "- 擒拿（Sunder）：破坏装备",
            "CMB = BAB + STR + 其他",
            "CMD = 10 + BAB + STR + DEX + 其他",
        ]
    }

    /// 法术系统
    pub fn section_spellcasting(&self) -> Vec<&'static str> {
        vec![
            "法术等级：0-9 环",
            "准备施法者：牧师、德鲁伊、法师、巡林客、圣武士",
            "自发施法者：吟游诗人、游荡者、术士",
            "专注检定：d20 + CL + 属性调整值",
            "法术抗力（SR）：抵抗法术的能力",
            "施法者等级检定（CL check）：d20 + CL vs SR",
            "Metamagic 专长：修改法术效果",
            "- Empower：变量效果 ×1.5",
            "- Maximize：变量效果取最大",
            "- Quicken：施法时间变为迅捷动作",
        ]
    }

    /// 物品与财富
    pub fn section_equipment(&self) -> Vec<&'static str> {
        vec![
            "1 级起始财富：职业决定（平均值）",
            "魔法物品市价：",
            "- +1 武器：2,315 gp",
            "- +1 护甲：1,000 gp + 护甲基础价",
            "- 一次性法术物品：750 gp × 法术环级",
            "负重规则：详细负重系统",
            "财富指南（Character Wealth by Level）：",
            "- 1 级：240 gp",
            "- 5 级：9,500 gp",
            "- 10 级：62,000 gp",
            "- 15 级：240,000 gp",
            "- 20 级：880,000 gp",
        ]
    }

    /// 经验值系统
    pub fn section_experience(&self) -> Vec<&'static str> {
        vec![
            "怪物 CR 提供 XP：",
            "- CR 1/8：50 XP",
            "- CR 1：400 XP",
            "- CR 5：1,600 XP",
            "- CR 10：9,600 XP",
            "- CR 15：65,000 XP",
            "- CR 20：307,200 XP",
            "升级所需 XP（慢速进程）：",
            "- 2 级：2,000 XP",
            "- 5 级：20,000 XP",
            "- 10 级：160,000 XP",
            "- 15 级：1,200,000 XP",
            "- 20 级：5,000,000 XP",
        ]
    }

    /// 第二版核心变更
    pub fn section_pf2_changes(&self) -> Vec<&'static str> {
        vec![
            "第二版（2019）主要变更：",
            "动作经济：每回合 3 个动作",
            "- 灵活行动分配",
            "- 专长解锁新动作",
            "属性生成： ancestries + backgrounds + classes",
            "等级加成：一切与等级绑定",
            "- 技能、攻击、豁免都加等级",
            "专长分类：",
            "- Ancestry Feats",
            "- Class Feats",
            "- Skill Feats",
            "- General Feats",
            "三行动施法：",
            "- 1 动作：小法术",
            "- 2 动作：标准法术",
            "- 3 动作：增强法术",
        ]
    }

    /// 第二版种族（Ancestries）
    pub fn section_pf2_ancestries(&self) -> Vec<&'static str> {
        vec![
            "第二版种族（Ancestries）：",
            "- 提供 HP 和属性加成",
            "- 提供 Heritage 和 Feats",
            "核心 Ancestries：",
            "- 矮人（Dwarf）：CON/CHA，黑暗视觉",
            "- 精灵（Elf）：DEX/INT，低光视觉",
            "- 侏儒（Gnome）：CON/CHA，小型",
            "- 哥布林（Goblin）：DEX/CHA，小型",
            "- 半精灵（Half-Elf）：CHA，Elf Heritage",
            "- 半兽人（Half-Orc）：STR/WIS，Orc Heritage",
            "- 半身人（Halfling）：DEX/WIS，小型",
            "- 人类（Human）：灵活属性",
            "扩展 Ancestries：Catfolk, Kobold, Orc, Tengu 等",
        ]
    }

    /// 第二版职业（Classes）
    pub fn section_pf2_classes(&self) -> Vec<&'static str> {
        vec![
            "第二版职业：每个职业有独特 Class Feats",
            "核心职业（Core Rulebook）：",
            "- Alchemist：药剂大师",
            "- Barbarian：狂暴战士",
            "- Bard：吟游诗人",
            "- Champion：圣骑士（Paladin 变体）",
            "- Cleric：牧师",
            "- Druid：德鲁伊",
            "- Fighter：战士",
            "- Monk：武僧",
            "- Ranger：巡林客",
            "- Rogue：游荡者",
            "- Sorcerer：术士",
            "- Wizard：法师",
            "扩展职业：Investigator, Oracle, Swashbuckler, Witch 等",
        ]
    }
}

impl crate::rules::core::Rule for PathfinderRules {
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
            "路径探路者规则",
            &[
                ("概述", &self.section_overview()),
                ("职业", &self.section_pf1_classes()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathfinder_rules_creation() {
        let rules = PathfinderRules::new();
        assert_eq!(rules.metadata().name, "路径探路者规则");
    }

    #[test]
    fn test_pf1_races() {
        let rules = PathfinderRules::new();
        let races = rules.section_pf1_races();
        assert_eq!(races.len(), 8);
    }

    #[test]
    fn test_pf1_classes() {
        let rules = PathfinderRules::new();
        let classes = rules.section_pf1_classes();
        assert_eq!(classes.len(), 12);
    }

    #[test]
    fn test_archetypes() {
        let rules = PathfinderRules::new();
        let archetypes = rules.section_archetypes();
        assert!(archetypes.len() >= 5);
    }

    #[test]
    fn test_feats() {
        let rules = PathfinderRules::new();
        let feats = rules.section_feats();
        assert!(feats.len() >= 5);
    }

    #[test]
    fn test_skills() {
        let rules = PathfinderRules::new();
        let skills = rules.section_skills();
        assert!(skills.len() >= 5);
    }

    #[test]
    fn test_combat() {
        let rules = PathfinderRules::new();
        let combat = rules.section_combat();
        assert!(combat.len() >= 5);
        assert!(combat.iter().any(|c| c.contains("CMB")));
    }

    #[test]
    fn test_spellcasting() {
        let rules = PathfinderRules::new();
        let spells = rules.section_spellcasting();
        assert!(spells.len() >= 5);
    }

    #[test]
    fn test_pf2_changes() {
        let rules = PathfinderRules::new();
        let changes = rules.section_pf2_changes();
        assert!(changes.len() >= 5);
        assert!(changes.iter().any(|c| c.contains("动作经济")));
    }

    #[test]
    fn test_pf2_ancestries() {
        let rules = PathfinderRules::new();
        let ancestries = rules.section_pf2_ancestries();
        assert!(ancestries.len() >= 5);
    }

    #[test]
    fn test_pf2_classes() {
        let rules = PathfinderRules::new();
        let classes = rules.section_pf2_classes();
        assert!(classes.len() >= 10);
    }
}
