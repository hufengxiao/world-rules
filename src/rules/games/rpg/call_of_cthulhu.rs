//! 克苏鲁的呼唤规则（Call of Cthulhu Roleplaying Game）
//!
//! Chaosium 开发的洛夫克拉夫特式恐怖角色扮演游戏。

use crate::simple_rule;

simple_rule! {
    struct: CallOfCthulhuRules,
    name: "克苏鲁的呼唤规则",
    desc: "Call of Cthulhu 角色扮演游戏规则系统",
    origin: "美国",
    tags: ["游戏", "RPG", "克苏鲁", "恐怖", "BRP"],
}

impl CallOfCthulhuRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "Call of Cthulhu（克苏鲁的呼唤）",
            "1981 年由 Chaosium 发布",
            "基于 H.P. Lovecraft 的克苏鲁神话",
            "玩家扮演调查员探索神秘恐怖",
            "核心主题：宇宙恐怖、知识代价、人类渺小",
            "使用 BRP（Basic Roleplaying）系统",
            "第 7 版（2020）为最新版本",
        ]
    }

    /// 属性系统
    pub fn section_attributes(&self) -> Vec<&'static str> {
        vec![
            "力量（STR）：体能、近战伤害",
            "体质（CON）：耐力、生命值",
            "体型（SIZ）：体型、HP、伤害加成",
            "敏捷（DEX）：灵巧、技能基础",
            "外貌（APP）：魅力、第一印象",
            "智力（INT）：推理、知识技能",
            "意志（POW）：精神力量、魔法",
            "教育（EDU）：学识、职业技能",
            "属性范围：通常 15-90（3d6 × 5）",
            "HP = (CON + SIZ) ÷ 10",
            "SAN（理智）起始 = POW",
        ]
    }

    /// 技能系统
    pub fn section_skills(&self) -> Vec<&'static str> {
        vec![
            "技能范围：0-100%",
            "职业技能点：EDU × 4",
            "兴趣技能点：INT × 2",
            "核心调查技能：",
            "- 图书馆使用（Library Use）：查阅资料",
            "- 察觉（Spot Hidden）：发现线索",
            "- 心理学（Psychology）：读懂他人",
            "战斗技能：",
            "- 斗殴（Brawl）：徒手战斗 25%",
            "- 手枪（Handgun）：枪械使用 20%",
            "- 斗殴（Melee）：近战武器 20%",
            "社交技能：",
            "- 劝说（Persuade）：说服他人",
            "- 快速交谈（Fast Talk）：误导",
            "- 欺瞒（Charm）：魅力影响",
            "理智风险技能：Cthulhu Mythos（代价高昂）",
        ]
    }

    /// 技能检定系统（BRP）
    pub fn section_skill_checks(&self) -> Vec<&'static str> {
        vec![
            "百分比检定：d100 ≤ 技能值即成功",
            "成功等级：",
            "- 常规成功：≤ 技能值",
            "- 困难成功：≤ 技能值 ÷ 2",
            "- 极难成功：≤ 技能值 ÷ 5",
            "- 大失败：≥ 96（技能 < 50）或 ≥ 100",
            "推奖（Pushing the roll）：",
            "- 失败后可再尝试一次",
            "- 失败后果更严重",
            "- 必须有合理理由",
            "奖励/惩罚骰：",
            "- 奖励骰：掷两次 d10，取较低十位",
            "- 惩罚骰：掷两次 d10，取较高十位",
        ]
    }

    /// 理智系统（Sanity）
    pub fn section_sanity(&self) -> Vec<&'static str> {
        vec![
            "SAN 是核心机制：衡量精神稳定性",
            "起始 SAN = POW",
            "SAN 损失触发：",
            "- 见证恐怖场景",
            "- 接触 Mythos 生物",
            "- 阅读禁忌知识",
            "理智检定（SAN check）：",
            "- 见到恐怖后立即进行",
            "- 失败：损失 SAN",
            "- 成功：损失较少 SAN",
            "理智损失后果：",
            "- 临时疯狂（Temporary Insanity）",
            "- 不定疯狂（Indefinite Insanity）",
            "- 永久疯狂（Permanent Insanity）",
            "SAN 恢复：心理治疗、成功完成冒险",
            "Cthulhu Mythos 技能增加会降低最大 SAN",
            "最大 SAN = 99 - Cthulhu Mythos",
        ]
    }

    /// 疯狂系统
    pub fn section_insanity(&self) -> Vec<&'static str> {
        vec![
            "临时疯狂（1d10 小时）：",
            "- 强迫行为（Phobia/Mania）",
            "- 失忆",
            "- 暴力发作",
            "- 偏执",
            "- 逃避现实",
            "不定疯狂（恢复需心理治疗）：",
            "- 进入疗养院",
            "- 恢复时间：1d10 个月",
            "- 成功后恢复部分 SAN",
            "永久疯狂：",
            "- 角色不可恢复",
            "- 转为 NPC 或退役",
            "疯狂表（Insanity Table）：随机决定症状",
        ]
    }

    /// 战斗系统
    pub fn section_combat(&self) -> Vec<&'static str> {
        vec![
            "战斗回合：每个角色按 DEX 排序行动",
            "战斗动作：",
            "- 攻击：技能检定",
            "- 闪避：战斗轮可选闪避（DEX × 5）",
            "- 格斗：STR 对抗",
            "伤害计算：",
            "- 武器伤害骰 + 伤害加成（DB）",
            "- DB 基于 STR + SIZ",
            "- 伤害减免（Armor）",
            "HP 归零：昏迷",
            "HP 为负：死亡",
            "致命伤害：身体部位受伤表",
        ]
    }

    /// 武器与伤害
    pub fn section_weapons(&self) -> Vec<&'static str> {
        vec![
            "近战武器：",
            "- 刀：1d4 + DB",
            "- 基地棒：1d6 + DB",
            "- 手枪：1d10",
            "- 步枪：1d12",
            "- 散弹枪：2d6（近距离）",
            "火器特点：",
            "- 高伤害",
            "- 噪音引来注意",
            "- 弹药有限",
            "- 范围伤害",
            "伤害加成表（Damage Bonus）：",
            "- STR+SIZ 2-64：-2",
            "- STR+SIZ 65-84：-1",
            "- STR+SIZ 85-124：无",
            "- STR+SIZ 125-164：+1d4",
            "- STR+SIZ 165-204：+1d6",
        ]
    }

    /// 魔法系统
    pub fn section_magic(&self) -> Vec<&'static str> {
        vec![
            "Mythos 魔法代价高昂",
            "施法要求：",
            "- 学习咒文（需阅读禁忌书籍）",
            "- 消耗 POW（永久）",
            "- SAN 损失",
            "- 材料和时间",
            "咒文类型：",
            "- 召唤/束缚神话生物",
            "- 赋予力量",
            "- 预知未来",
            "- 接触外神",
            "魔法是双刃剑：",
            "- 提供力量",
            "- 加速疯狂",
            "- 吸引神话生物",
        ]
    }

    /// 时代设定
    pub fn section_eras(&self) -> Vec<&'static str> {
        vec![
            "经典时代（Classic Era，1920s）：",
            "- 最经典设定",
            "- 禁酒令时期",
            "- 黑帮、移民、神秘社团",
            "现代时代（Modern Era，2010s+）：",
            "- 当代科技",
            "- 网络调查",
            "- 全球恐怖主义",
            "煤气灯时代（Gaslight Era，1890s）：",
            "- 维多利亚伦敦",
            "- 伦敦雾",
            "- 开膛手杰克",
            "其他时代：",
            "- Down Darker Trails（美国西部）",
            "- Cthulhu: Dark Ages（中世纪）",
            "- Cthulhu Invictus（罗马时代）",
        ]
    }

    /// 调查员组织
    pub fn section_organizations(&self) -> Vec<&'static str> {
        vec![
            "著名组织：",
            "- 阿卡姆侦探社（Arkham Detective Agency）",
            "- 玫瑰十字会（Hermetic Order of the Silver Twilight）",
            "- 卡德玛斯基金会（Kadath Foundation）",
            "- 洛夫克拉夫特协会",
            "- 普罗维登斯博物馆",
            "组织作为调查员支持：",
            "- 提供资源和线索",
            "- 可能有隐藏动机",
            "- 部分被 Mythos 渗透",
        ]
    }

    /// 创造调查员
    pub fn section_character_creation(&self) -> Vec<&'static str> {
        vec![
            "步骤：",
            "1. 掷骰属性（快速生成）或分配点数",
            "2. 选择职业（Occupation）",
            "3. 分配职业技能点",
            "4. 分配兴趣技能点",
            "5. 确定背景故事",
            "6. 购买装备",
            "7. 填写调查员表格",
            "快速生成：约 30 分钟",
            "详细生成：约 1 小时",
            "调查员寿命通常较短",
        ]
    }

    /// 游戏风格建议
    pub fn section_gameplay_style(&self) -> Vec<&'static str> {
        vec![
            "调查员通常不是战斗专家",
            "避免直接对抗是明智选择",
            "潜行和智慧更有价值",
            "知识代价高昂",
            "理智是有限资源",
            "成功 ≠ 击败敌人",
            "成功 = 生存并揭露真相",
            "恐怖氛围营造：",
            "- 描述感官细节",
            "- 暗示多于展示",
            "- 缓慢揭示真相",
            "- 理智检定营造紧张感",
        ]
    }

    /// Keeper 建议
    pub fn section_keeper_tips(&self) -> Vec<&'static str> {
        vec![
            "Keeper（类似 DM）职责：",
            "- 营造恐怖氛围",
            "- 设计神秘事件",
            "- 平衡调查与恐怖",
            "- 管理理智和节奏",
            "场景设计：",
            "- 钩子（Hook）：引入调查",
            "- 线索链：逐步揭示",
            "- 高潮：直面恐怖",
            "- 结局：幸存或疯狂",
            "避免：",
            "- 过度战斗",
            "- 过多 Mythos 生物",
            "- 让调查员无敌",
        ]
    }
}

impl crate::rules::core::Rule for CallOfCthulhuRules {
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
            "克苏鲁的呼唤规则",
            &[
                ("概述", &self.section_overview()),
                ("属性", &self.section_attributes()),
                ("技能", &self.section_skills()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::Rule;

    #[test]
    fn test_coc_rules_creation() {
        let rules = CallOfCthulhuRules::new();
        assert_eq!(rules.metadata().name, "克苏鲁的呼唤规则");
    }

    #[test]
    fn test_attributes() {
        let rules = CallOfCthulhuRules::new();
        let attrs = rules.section_attributes();
        assert!(attrs.len() >= 8);
        assert!(attrs.iter().any(|a| a.contains("STR")));
        assert!(attrs.iter().any(|a| a.contains("POW")));
    }

    #[test]
    fn test_skills() {
        let rules = CallOfCthulhuRules::new();
        let skills = rules.section_skills();
        assert!(skills.len() >= 5);
        assert!(skills.iter().any(|s| s.contains("图书馆")));
    }

    #[test]
    fn test_skill_checks() {
        let rules = CallOfCthulhuRules::new();
        let checks = rules.section_skill_checks();
        assert!(checks.len() >= 5);
        assert!(checks.iter().any(|c| c.contains("困难成功")));
    }

    #[test]
    fn test_sanity_system() {
        let rules = CallOfCthulhuRules::new();
        let san = rules.section_sanity();
        assert!(san.len() >= 5);
        assert!(san.iter().any(|s| s.contains("SAN")));
    }

    #[test]
    fn test_insanity() {
        let rules = CallOfCthulhuRules::new();
        let insanity = rules.section_insanity();
        assert!(insanity.len() >= 5);
        assert!(insanity.iter().any(|i| i.contains("临时疯狂")));
    }

    #[test]
    fn test_combat() {
        let rules = CallOfCthulhuRules::new();
        let combat = rules.section_combat();
        assert!(combat.len() >= 5);
    }

    #[test]
    fn test_weapons() {
        let rules = CallOfCthulhuRules::new();
        let weapons = rules.section_weapons();
        assert!(weapons.len() >= 5);
        assert!(weapons.iter().any(|w| w.contains("手枪")));
    }

    #[test]
    fn test_magic() {
        let rules = CallOfCthulhuRules::new();
        let magic = rules.section_magic();
        assert!(magic.len() >= 5);
        assert!(magic.iter().any(|m| m.contains("Mythos")));
    }

    #[test]
    fn test_eras() {
        let rules = CallOfCthulhuRules::new();
        let eras = rules.section_eras();
        assert!(eras.len() >= 5);
        assert!(eras.iter().any(|e| e.contains("1920s")));
    }

    #[test]
    fn test_character_creation() {
        let rules = CallOfCthulhuRules::new();
        let creation = rules.section_character_creation();
        assert!(creation.len() >= 5);
    }

    #[test]
    fn test_gameplay_style() {
        let rules = CallOfCthulhuRules::new();
        let style = rules.section_gameplay_style();
        assert!(style.len() >= 5);
        assert!(style.iter().any(|s| s.contains("理智")));
    }
}
