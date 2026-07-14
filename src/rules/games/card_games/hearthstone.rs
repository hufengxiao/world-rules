//! 炉石传说（Hearthstone）基础规则
//! 
//! 炉石传说是由暴雪娱乐开发的免费数字集换式卡牌游戏，
//! 基于《魔兽世界》的背景设定。

use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: HearthstoneBasicRules,
    name: "炉石传说基础规则",
    desc: "Hearthstone 基础游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌", "CCG", "数字卡牌"],
    category: RuleCategory::games("hearthstone"),
    sections: [
        ("游戏概述", section_overview),
        ("胜利条件", section_victory),
        ("英雄系统", section_heroes),
        ("卡牌类型", section_card_types),
        ("费用系统", section_mana),
        ("游戏区域", section_zones),
        ("回合流程", section_turn),
        ("战斗系统", section_combat),
        ("关键词", section_keywords),
        ("卡牌稀有度", section_rarity),
        ("游戏模式", section_modes),
        ("构筑规则", section_deck_building)
    ]
}

impl HearthstoneBasicRules {
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "炉石传说是由暴雪娱乐开发的数字CCG",
            "发行时间：2014年3月",
            "平台：PC、iOS、Android",
            "玩家：2人对战",
            "游戏时长：5-15 分钟",
            "核心机制：法力系统 + 随从攻击",
            "背景设定：魔兽世界",
            "游戏风格：快节奏、易上手",
        ]
    }
    
    pub fn section_victory(&self) -> Vec<&'static str> {
        vec![
            "【胜利方式】",
            "  1. 将对手英雄生命值降至 0（主要方式）",
            "  2. 对手疲劳死亡（牌库耗尽）",
            "  3. 特殊胜利条件（如 Raza 牧师）",
            "  4. 对手认输",
            "",
            "【初始生命值】",
            "  - 英雄生命值：30 点",
            "  - 护甲：额外生命值（战士/德鲁伊）",
            "",
            "【疲劳机制】",
            "  - 牌库耗尽后每次抽牌受到伤害",
            "  - 第一次疲劳：1 点伤害",
            "  - 第二次疲劳：2 点伤害",
            "  - 依此类推",
        ]
    }
    
    pub fn section_heroes(&self) -> Vec<&'static str> {
        vec![
            "【英雄职业】",
            "",
            "【战士 Warrior - 加尔鲁什】",
            "  - 技能：护甲升！获得 2 点护甲",
            "  - 特点：护甲、武器、控制",
            "",
            "【萨满 Shaman - 萨尔】",
            "  - 技能：图腾召唤！随机召唤图腾",
            "  - 特点：过载、图腾、元素",
            "",
            "【盗贼 Rogue - 瓦莉拉】",
            "  - 技能：匕首精通！装备 1/2 匕首",
            "  - 特点：连击、武器、抽牌",
            "",
            "【法师 Mage - 吉安娜】",
            "  - 技能：火冲！造成 1 点伤害",
            "  - 特点：法术、控制、奥秘",
            "",
            "【猎人 Hunter - 雷克萨】",
            "  - 技能：稳固射击！对敌人造成 2 点伤害",
            "  - 特点：快攻、野兽、奥秘",
            "",
            "【圣骑士 Paladin - 乌瑟尔】",
            "  - 技能：强化！召唤 1/1 白银之手新兵",
            "  - 特点：报告、Buff、回复",
            "",
            "【牧师 Priest - 安杜因】",
            "  - 技能：次级治疗！恢复 2 点生命",
            "  - 特点：治疗、控制、复制",
            "",
            "【术士 Warlock - 古尔丹】",
            "  - 技能：生命分流！抽 1 张牌，受到 2 点伤害",
            "  - 特点：自残、恶魔、弃牌",
            "",
            "【德鲁伊 Druid - 玛法里奥】",
            "  - 技能：变形！本回合获得 1 点攻击力，获得 1 点护甲",
            "  - 特点：法力、野兽、抉择",
            "",
            "【恶魔猎手 Demon Hunter - 伊利丹】",
            "  - 技能：恶魔之咬！本回合获得 1 点攻击力",
            "  - 特点：快攻、流放、武器",
        ]
    }
    
    pub fn section_card_types(&self) -> Vec<&'static str> {
        vec![
            "【卡牌类型】",
            "",
            "【随从 Minion】",
            "  - 可攻击的战斗单位",
            "  - 有攻击力、生命值、费用",
            "  - 可能有种族和关键字",
            "",
            "【法术 Spell】",
            "  - 产生各种效果",
            "  - 使用后立即生效",
            "  - 可能有奥秘或任务",
            "",
            "【武器 Weapon】",
            "  - 装备后英雄可攻击",
            "  - 有攻击力和耐久度",
            "  - 攻击后耐久度 -1",
            "",
            "【英雄牌 Hero Card】",
            "  - 替换当前英雄",
            "  - 提供新英雄技能",
            "  - 通常有战吼效果",
            "",
            "【地标 Location】",
            "  - 有耐久度的永久物",
            "  - 激活后产生效果",
            "  - 需要冷却时间",
            "",
            "【任务 Quest】",
            "  - 完成条件后获得奖励",
            "  - 占用 1 法力水晶",
            "  - 任务线有多个阶段",
        ]
    }
    
    pub fn section_mana(&self) -> Vec<&'static str> {
        vec![
            "【法力系统】",
            "",
            "【法力水晶】",
            "  - 每回合自动获得 1 个",
            "  - 上限：10 个",
            "  - 未使用的法力不累积",
            "",
            "【法力消耗】",
            "  - 卡牌左上角的费用",
            "  - 使用卡牌消耗相应法力",
            "  - 不能使用费用不足的牌",
            "",
            "【法力增长】",
            "  - 德鲁伊可加速法力增长",
            "  - 某些卡牌可增加临时法力",
            "  - 萨满的过载占用下回合法力",
            "",
            "【法力修正】",
            "  - 减费：降低卡牌费用",
            "  - 增费：提高卡牌费用",
            "  - 法力恢复：回复法力水晶",
        ]
    }
    
    pub fn section_zones(&self) -> Vec<&'static str> {
        vec![
            "【游戏区域】",
            "",
            "【手牌 Hand】",
            "  - 最多 10 张",
            "  - 超出部分被销毁",
            "  - 私有区域，对手不可见",
            "",
            "【牌库 Deck】",
            "  - 初始卡组（标准：30 张）",
            "  - 抽牌从顶端进行",
            "  - 牌库耗尽进入疲劳",
            "",
            "【战场 Battlefield】",
            "  - 随从放置区域",
            "  - 最多 7 个随从",
            "  - 双方各占半场",
            "",
            "【墓地 Graveyard】",
            "  - 死亡的随从和使用的法术",
            "  - 可被复活或检索",
            "  - 牧师和术士常用",
            "",
            "【除外 Removed from Game】",
            "  - 被完全移除的卡牌",
            "  - 通常无法返回",
            "  - 某些效果可以回收",
        ]
    }
    
    pub fn section_turn(&self) -> Vec<&'static str> {
        vec![
            "【回合流程】",
            "",
            "【1. 开始阶段 Start Phase】",
            "  - 增加法力水晶",
            "  - 解冻冻结的随从",
            "  - 触发回合开始效果",
            "",
            "【2. 抽牌阶段 Draw Phase】",
            "  - 从牌库抽 1 张牌",
            "  - 先手玩家第一回合不抽牌",
            "",
            "【3. 主阶段 Main Phase】",
            "  - 使用卡牌",
            "  - 攻击（随从需等待一回合）",
            "  - 激活英雄技能",
            "",
            "【4. 结束阶段 End Phase】",
            "  - 触发回合结束效果",
            "  - 手牌超过 10 张时销毁多余牌",
            "",
            "【回合限制】",
            "  - 时间限制：75 秒（标准）",
            "  - 超时自动结束回合",
            "  - 烧绳提示（15 秒）",
        ]
    }
    
    pub fn section_combat(&self) -> Vec<&'static str> {
        vec![
            "【战斗系统】",
            "",
            "【攻击规则】",
            "  - 攻击力 vs 生命值",
            "  - 同时造成伤害",
            "  - 生命值 ≤ 0 的随从死亡",
            "",
            "【攻击限制】",
            "  - 随从首回合不能攻击（冲锋除外）",
            "  - 冻结的随从不能攻击",
            "  - 0 攻击力的随从不能攻击",
            "",
            "【嘲讽 Taunt】",
            "  - 必须先消灭嘲讽随从",
            "  - 阻止直接攻击英雄",
            "  - 可被沉默移除",
            "",
            "【风怒 Windfury】",
            "  - 可攻击两次",
            "  - 每次攻击消耗攻击次数",
            "",
            "【攻击目标】",
            "  - 可攻击对手随从",
            "  - 可攻击对手英雄",
            "  - 嘲讽存在时优先攻击嘲讽",
        ]
    }
    
    pub fn section_keywords(&self) -> Vec<&'static str> {
        vec![
            "【常见关键词】",
            "",
            "【嘲讽 Taunt】",
            "  - 强制对手攻击此随从",
            "",
            "【冲锋 Charge】",
            "  - 首回合可攻击",
            "  - 现在较少见（改为突袭）",
            "",
            "【突袭 Rush】",
            "  - 首回合可攻击随从",
            "  - 不能攻击英雄",
            "",
            "【圣盾 Divine Shield】",
            "  - 免疫第一次伤害",
            "  - 受到伤害后消失",
            "",
            "【风怒 Windfury】",
            "  - 每回合可攻击两次",
            "",
            "【吸血 Lifesteal】",
            "  - 造成伤害时回复等量生命",
            "",
            "【剧毒 Poisonous】",
            "  - 对随从造成伤害时立即消灭",
            "",
            "【法术伤害 Spell Damage】",
            "  - 增强法术伤害",
            "  - 通常 +1 点伤害",
            "",
            "【战吼 Battlecry】",
            "  - 使用时触发的效果",
            "",
            "【亡语 Deathrattle】",
            "  - 死亡时触发的效果",
            "",
            "【嘲讽 Divine Shield + Taunt】",
            "  - 圣盾嘲讽，强力防守",
            "",
            "【磁力 Magnetic】",
            "  - 可与机械随从合并",
            "",
            "【复生 Reborn】",
            "  - 死亡后复活一次（1点生命）",
        ]
    }
    
    pub fn section_rarity(&self) -> Vec<&'static str> {
        vec![
            "【卡牌稀有度】",
            "",
            "【免费 Free】",
            "  - 基础卡牌",
            "  - 无需制作",
            "  - 灰色边框",
            "",
            "【普通 Common】",
            "  - 最常见的卡牌",
            "  - 制作：40 尘埃",
            "  - 分解：5 尘埃",
            "  - 白色边框",
            "",
            "【稀有 Rare】",
            "  - 较少见的卡牌",
            "  - 制作：100 尘埃",
            "  - 分解：20 尘埃",
            "  - 蓝色边框",
            "",
            "【史诗 Epic】",
            "  - 稀有卡牌",
            "  - 制作：400 尘埃",
            "  - 分解：100 尘埃",
            "  - 紫色边框",
            "",
            "【传说 Legendary】",
            "  - 最稀有的卡牌",
            "  - 制作：1600 尘埃",
            "  - 分解：400 尘埃",
            "  - 橙色边框",
            "  - 每个卡组只能有 1 张",
        ]
    }
    
    pub fn section_modes(&self) -> Vec<&'static str> {
        vec![
            "【游戏模式】",
            "",
            "【标准模式 Standard】",
            "  - 使用最新扩展包",
            "  - 卡组限制为特定年份的卡牌",
            "  - 每年轮换",
            "",
            "【狂野模式 Wild】",
            "  - 可使用所有卡牌",
            "  - 无轮换限制",
            "  - 更强的卡组",
            "",
            "【竞技场 Arena】",
            "  - 随机构建卡组",
            "  - 从三张牌中选一",
            "  - 最多 12 胜或 3 负",
            "",
            "【酒馆战棋 Battlegrounds】",
            "  - 自动战斗棋类模式",
            "  - 8 人对战",
            "  - 购买和升级随从",
            "",
            "【对决模式 Duels】",
            "  - 随机奖励构建卡组",
            "  - 类似地下城冒险",
            "  - 淘汰赛制",
            "",
            "【乱斗模式 Tavern Brawl】",
            "  - 每周特殊规则",
            "  - 通常只需 1 胜",
            "  - 首胜奖励经典卡包",
        ]
    }
    
    pub fn section_deck_building(&self) -> Vec<&'static str> {
        vec![
            "【构筑规则】",
            "",
            "【卡组大小】",
            "  - 标准：30 张卡牌",
            "  - 可添加额外卡牌（如雷诺）",
            "",
            "【职业限制】",
            "  - 只能使用本职业和中立卡牌",
            "  - 标准模式受年份限制",
            "",
            "【传说限制】",
            "  - 每种传说卡只能有 1 张",
            "  - 其他稀有度最多 2 张",
            "",
            "【卡组代码】",
            "  - 可分享卡组代码",
            "  - 导入导出卡组",
            "  - 方便复制卡组",
            "",
            "【粉尘系统】",
            "  - 分解卡牌获得尘埃",
            "  - 用尘埃制作卡牌",
            "  - 传说最贵（1600 尘埃）",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hearthstone_rules_creation() {
        let rules = HearthstoneBasicRules::new();
        assert!(rules.explain().contains("炉石传说"));
        assert!(rules.explain().contains("暴雪"));
    }
    
    #[test]
    fn test_heroes() {
        let rules = HearthstoneBasicRules::new();
        let heroes = rules.section_heroes();
        assert!(heroes.iter().any(|s| s.contains("战士")));
        assert!(heroes.iter().any(|s| s.contains("法师")));
    }
    
    #[test]
    fn test_keywords() {
        let rules = HearthstoneBasicRules::new();
        let keywords = rules.section_keywords();
        assert!(keywords.iter().any(|s| s.contains("嘲讽")));
        assert!(keywords.iter().any(|s| s.contains("圣盾")));
    }
    
    #[test]
    fn test_mana_system() {
        let rules = HearthstoneBasicRules::new();
        let mana = rules.section_mana();
        assert!(mana.iter().any(|s| s.contains("法力水晶")));
        assert!(mana.iter().any(|s| s.contains("10")));
    }
}