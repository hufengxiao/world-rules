//! 万智牌（Magic: The Gathering）基础规则
//!
//! 万智牌是由 Richard Garfield 设计的集换式卡牌游戏（TCG），
//! 于 1993 年由威世智（Wizards of the Coast）发行。
//! 是世界上第一个集换式卡牌游戏。

use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MagicTheGatheringBasicRules,
    name: "万智牌基础规则",
    desc: "Magic: The Gathering 基础游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌", "TCG", "集换式卡牌"],
    category: RuleCategory::games("mtg"),
    sections: [
        ("游戏概述", section_overview),
        ("胜利条件", section_victory),
        ("卡牌类型", section_card_types),
        ("颜色系统", section_colors),
        ("区域概念", section_zones),
        ("游戏流程", section_gameplay),
        ("回合结构", section_turn_structure),
        ("堆叠系统", section_stack),
        ("费用支付", section_costs),
        ("战斗系统", section_combat),
        ("关键字能力", section_keywords),
        ("基本策略", section_strategy)
    ]
}

impl MagicTheGatheringBasicRules {
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "万智牌（MTG）是集换式卡牌游戏的鼻祖",
            "设计者：Richard Garfield (1993年)",
            "发行商：威世智（Wizards of the Coast）",
            "玩家：2 人对战（多人模式可选）",
            "游戏时长：15-60 分钟",
            "核心机制：法力系统 + 堆叠系统",
            "游戏目标：将对手生命值降至 0",
        ]
    }

    pub fn section_victory(&self) -> Vec<&'static str> {
        vec![
            "【胜利方式】",
            "  1. 将对手生命值降至 0（最常见）",
            "  2. 对手无法从牌库抽牌（牌库耗尽）",
            "  3. 特殊胜利条件（如积攒 10 点中毒计数）",
            "  4. 对手认输",
            "",
            "【初始生命值】",
            "  - 标准：20 点",
            "  - 先锋赛：根据先锋牌设定",
            "  - 指挥官赛：40 点",
            "",
            "【失败条件】",
            "  - 生命值 ≤ 0",
            "  - 抽牌时牌库为空",
            "  - 获得 10+ 中毒计数",
        ]
    }

    pub fn section_card_types(&self) -> Vec<&'static str> {
        vec![
            "【主要类型】",
            "",
            "【地牌 Land】",
            "  - 提供法力（游戏资源）",
            "  - 每回合可使用一张地牌",
            "  - 不消耗法力即可使用",
            "",
            "【生物 Creature】",
            "  - 攻击和防守的主力",
            "  - 有力量和防御属性",
            "  - 召唤失调（首次出场不能攻击）",
            "",
            "【瞬间 Instant】",
            "  - 任意时机可施放",
            "  - 效果结算后进入坟墓场",
            "  - 常用于应对对手行动",
            "",
            "【法术 Sorcery】",
            "  - 只能在主阶段施放",
            "  - 堆叠清空时才能使用",
            "",
            "【神器 Artifact】",
            "  - 永久物，不限于颜色",
            "  - 包括神器生物、装备等",
            "",
            "【结界 Enchantment】",
            "  - 永久物，持续效果",
            "  - 包括灵气（附著其他永久物）",
            "",
            "【鹏洛客 Planeswalker】",
            "  - 代表盟友角色",
            "  - 有忠诚度点数",
            "  - 每回合可使用一次能力",
            "",
            "【部族 Tribal】",
            "  - 拥有生物类别的非生物牌",
        ]
    }

    pub fn section_colors(&self) -> Vec<&'static str> {
        vec![
            "【五色系统】",
            "",
            "【白色 White】",
            "  - 理念：秩序、正义、社区",
            "  - 特点：治疗、防御、小生物群",
            "  - 弱点：缺乏单卡优势",
            "",
            "【蓝色 Blue】",
            "  - 理念：知识、控制、完美",
            "  - 特点：抽牌、反击、操控",
            "  - 弱点：依赖 combo、节奏慢",
            "",
            "【黑色 Black】",
            "  - 理念：力量、死亡、牺牲",
            "  - 特点：破坏、复活、弃牌",
            "  - 弱点：需支付生命或其他代价",
            "",
            "【红色 Red】",
            "  - 理念：自由、情感、混沌",
            "  - 特点：直接伤害、快速攻击",
            "  - 弱点：缺乏后期优势",
            "",
            "【绿色 Green】",
            "  - 理念：自然、成长、本能",
            "  - 特点：法力加速、大生物",
            "  - 弱点：缺乏直接去除手段",
            "",
            "【无色 Colorless】",
            "  - 神器和某些特殊牌",
            "  - 不受颜色限制",
            "  - 通常需要通用法力",
        ]
    }

    pub fn section_zones(&self) -> Vec<&'static str> {
        vec![
            "【游戏区域】",
            "",
            "【牌库 Library】",
            "  - 牌面朝下，顺序不可改变",
            "  - 抽牌从顶端进行",
            "  - 被检索时可洗牌",
            "",
            "【手牌 Hand】",
            "  - 玩家私有的牌",
            "  - 其他玩家不可查看",
            "  - 结束阶段上限 7 张",
            "",
            "【战场 Battlefield】",
            "  - 放置永久物的区域",
            "  - 所有玩家可见",
            "  - 包括生物、地、神器、结界等",
            "",
            "【坟墓场 Graveyard】",
            "  - 已使用或被摧毁的牌",
            "  - 牌面朝上，顺序可查",
            "  - 可被某些效果检索",
            "",
            "【放逐区 Exile】",
            "  - 永久移出游戏",
            "  - 除非特殊效果否则无法返回",
            "",
            "【堆叠 Stack】",
            "  - 放置待结算的法术/能力",
            "  - 后进先出（LIFO）",
            "",
            "【统帅区 Command Zone】",
            "  - 放置指挥官或徽记",
            "  - 特殊游戏模式使用",
        ]
    }

    pub fn section_gameplay(&self) -> Vec<&'static str> {
        vec![
            "【游戏准备】",
            "  1. 双方洗牌并交换切牌",
            "  2. 各抽 7 张起始手牌",
            "  3. 起手调整：可放回重抽（少抽 1 张）",
            "  4. 随机决定先手玩家",
            "",
            "【游戏流程】",
            "  - 玩家交替进行回合",
            "  - 每回合包含多个阶段",
            "  - 双方可在适当时机响应",
            "",
            "【先手劣势】",
            "  - 先手玩家第一回合不能抽牌",
            "  - 平衡先手优势",
            "",
            "【平局规则】",
            "  - 双方同时生命 ≤ 0 为平局",
            "  - 特定卡牌可强制平局",
        ]
    }

    pub fn section_turn_structure(&self) -> Vec<&'static str> {
        vec![
            "【回合阶段】",
            "",
            "【1. 开始阶段 Beginning Phase】",
            "  - 重置步骤：将所有横置的牌重置",
            "  - 维持步骤：触发维持能力",
            "  - 抽牌步骤：从牌库抽 1 张",
            "",
            "【2. 战斗前主阶段 Precombat Main Phase】",
            "  - 可使用地牌",
            "  - 可施放法术、生物等",
            "  - 瞬间可在适当时机施放",
            "",
            "【3. 战斗阶段 Combat Phase】",
            "  - 开始步骤",
            "  - 宣告攻击者步骤",
            "  - 宣告阻挡者步骤",
            "  - 战斗伤害步骤",
            "  - 战斗结束步骤",
            "",
            "【4. 战斗后主阶段 Postcombat Main Phase】",
            "  - 与战斗前主阶段相同",
            "",
            "【5. 结束阶段 End Phase】",
            "  - 结束步骤：触发结束能力",
            "  - 清理步骤：手牌上限检查（7 张）",
            "  - 伤害清除，持续效果结束",
        ]
    }

    pub fn section_stack(&self) -> Vec<&'static str> {
        vec![
            "【堆叠机制】",
            "",
            "【核心概念】",
            "  - 牌/能力不立即结算",
            "  - 进入堆叠等待响应",
            "  - 双方都有机会响应",
            "",
            "【堆叠规则】",
            "  - 后进先出（LIFO）",
            "  - 双方都不响应时开始结算",
            "  - 结算从堆叠顶端开始",
            "",
            "【响应时机】",
            "  - 对手施放法术时",
            "  - 对手激活能力时",
            "  - 触发能力进入堆叠时",
            "  - 战斗阶段各步骤",
            "",
            "【优先权】",
            "  - 有优先权的玩家才能行动",
            "  - 施放法术后优先权转移",
            "  - 堆叠清空时主动玩家获得优先权",
            "",
            "【常见应用】",
            "  - 反击对手法术",
            "  - 在对手行动前响应",
            "  - 连锁触发能力",
        ]
    }

    pub fn section_costs(&self) -> Vec<&'static str> {
        vec![
            "【法力系统】",
            "",
            "【法力类型】",
            "  - 白色法力 {W}：平原产出",
            "  - 蓝色法力 {U}：海岛产出",
            "  - 黑色法力 {B}：沼泽产出",
            "  - 红色法力 {R}：山脉产出",
            "  - 绿色法力 {G}：森林产出",
            "  - 无色法力 {C}：废脉产出",
            "  - 通用法马 {数字}：任意颜色",
            "",
            "【施放费用】",
            "  - 卡牌右上角的费用",
            "  - 包含颜色要求和总费用",
            "  - 例如：{2}{U}{U} = 4 点（至少 2 点蓝色）",
            "",
            "【支付费用】",
            "  - 横置地牌产出法力",
            "  - 法力进入法力池",
            "  - 用法力支付施放费用",
            "  - 不使用的法力回合结束消失",
            "",
            "【替代费用】",
            "  - 某些卡牌可替代支付",
            "  - 如：应急法力（支付生命）",
            "  - 如：万世负担（牺牲生物）",
        ]
    }

    pub fn section_combat(&self) -> Vec<&'static str> {
        vec![
            "【战斗流程】",
            "",
            "【1. 宣告攻击者】",
            "  - 攻击方选择攻击生物",
            "  - 攻击生物横置",
            "  - 召唤失调的生物不能攻击",
            "  - 可选择多个生物同时攻击",
            "",
            "【2. 宣告阻挡者】",
            "  - 防守方选择阻挡生物",
            "  - 阻挡生物不需横置",
            "  - 一个阻挡者只能挡一个攻击者",
            "  - 多个阻挡者可围攻一个攻击者",
            "",
            "【3. 战斗伤害】",
            "  - 阻挡分配：攻击者分配伤害",
            "  - 同时造成伤害",
            "  - 伤害 = 力量值",
            "",
            "【4. 伤害结果】",
            "  - 防御 ≤ 伤害：生物死亡",
            "  - 未阻挡：对玩家造成伤害",
            "  - 有先攻：先造成伤害",
            "",
            "【特殊战斗】",
            "  - 飞行：只能被飞行阻挡",
            "  - 先攻：先造成伤害",
            "  -践踏：溢出伤害给玩家",
            "  - 威慑：只能被 2+ 生物阻挡",
        ]
    }

    pub fn section_keywords(&self) -> Vec<&'static str> {
        vec![
            "【常见关键字】",
            "",
            "【飞行 Flying】",
            "  - 只能被飞行生物阻挡",
            "",
            "【先攻 First Strike】",
            "  - 先造成战斗伤害",
            "  - 可在对方伤害前消灭阻挡者",
            "",
            "【连击 Double Strike】",
            "  - 造成两次战斗伤害",
            "  - 先攻伤害 + 普通伤害",
            "",
            "【践踏 Trample】",
            "  - 溢出伤害传递给玩家",
            "  - 超过阻挡者防御的伤害给对手",
            "",
            "【威慑 Menace】",
            "  - 需要至少 2 个生物才能阻挡",
            "",
            "【辟邪 Hexproof】",
            "  - 不能被对手法术/能力指定",
            "",
            "【守护 Defender】",
            "  - 不能攻击",
            "  - 只能防守",
            "",
            "【敏捷 Haste】",
            "  - 无召唤失调",
            "  - 出场即可攻击",
            "",
            "【闪现 Flash】",
            "  - 任意时机可施放",
            "  - 类似瞬间时机",
            "",
            "【死触 Deathtouch】",
            "  - 任何伤害都足以消灭生物",
            "",
            "【系命 Lifelink】",
            "  - 造成伤害时回复等量生命",
        ]
    }

    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "【套牌类型】",
            "",
            "【快攻 Aggro】",
            "  - 大量低费生物",
            "  - 快速造成伤害",
            "  - 红色/白色常见",
            "",
            "【控制 Control】",
            "  - 反击对手威胁",
            "  - 延续游戏节奏",
            "  - 蓝色常见",
            "",
            "【中速 Midrange】",
            "  - 平衡攻防",
            "  - 优质生物为主",
            "  - 绿色常见",
            "",
            "【组合技 Combo】",
            "  - 依赖特定卡牌组合",
            "  - 一回合内获胜",
            "  - 多色混合",
            "",
            "【基本建议】",
            "  - 起手保留 2-4 地牌",
            "  - 计算法力曲线",
            "  - 了解主流套牌",
            "  - 练习堆叠响应",
            "  - 记忆对手可能的手牌",
        ]
    }
}

#[cfg(test)]
mod mtg_tests {
    use super::*;
    use crate::rules::core::Rule;

    #[test]
    fn test_mtg_rules_creation() {
        let rules = MagicTheGatheringBasicRules::new();
        assert!(rules.explain().contains("万智牌"));
        assert!(rules.explain().contains("Richard Garfield"));
    }

    #[test]
    fn test_card_types() {
        let rules = MagicTheGatheringBasicRules::new();
        let types = rules.section_card_types();
        assert!(types.iter().any(|s| s.contains("地牌")));
        assert!(types.iter().any(|s| s.contains("生物")));
    }

    #[test]
    fn test_colors() {
        let rules = MagicTheGatheringBasicRules::new();
        let colors = rules.section_colors();
        assert!(colors.iter().any(|s| s.contains("白色")));
        assert!(colors.iter().any(|s| s.contains("蓝色")));
    }

    #[test]
    fn test_combat_system() {
        let rules = MagicTheGatheringBasicRules::new();
        let combat = rules.section_combat();
        assert!(combat.iter().any(|s| s.contains("攻击")));
        assert!(combat.iter().any(|s| s.contains("阻挡")));
    }

    #[test]
    fn test_keywords() {
        let rules = MagicTheGatheringBasicRules::new();
        let keywords = rules.section_keywords();
        assert!(keywords.iter().any(|s| s.contains("飞行")));
        assert!(keywords.iter().any(|s| s.contains("先攻")));
    }
}
