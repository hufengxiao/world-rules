//! 游戏王（Yu-Gi-Oh!）基础规则
//!
//! 游戏王是由高桥和希创作的日本集换式卡牌游戏（OCG/TCG），
//! 基于同名漫画《游戏王》中的魔法与巫师游戏。

use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: YuGiOhBasicRules,
    name: "游戏王基础规则",
    desc: "Yu-Gi-Oh! Trading Card Game 基础规则",
    origin: "日本",
    tags: ["游戏", "卡牌", "TCG", "OCG"],
    category: RuleCategory::games("yugioh"),
    sections: [
        ("游戏概述", section_overview),
        ("胜利条件", section_victory),
        ("卡牌类型", section_card_types),
        ("怪兽卡", section_monsters),
        ("魔法卡", section_spells),
        ("陷阱卡", section_traps),
        ("游戏区域", section_zones),
        ("回合流程", section_turn),
        ("战斗系统", section_combat),
        ("连锁系统", section_chains),
        ("召唤规则", section_summoning),
        ("额外卡组", section_extra_deck),
        ("禁忌限制", section_banlist)
    ]
}

impl YuGiOhBasicRules {
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "游戏王是由高桥和希创作的集换式卡牌游戏",
            "首次发行：1999年（OCG日本）/ 2002年（TCG国际）",
            "玩家：2人对战",
            "游戏时长：15-45 分钟",
            "核心机制：召唤怪兽 + 战斗 + 魔法陷阱",
            "灵感来源：漫画《游戏王》中的魔法与巫师",
            "发行商：科乐美（Konami）",
        ]
    }

    pub fn section_victory(&self) -> Vec<&'static str> {
        vec![
            "【胜利方式】",
            "  1. 将对手生命值降至 0（主要方式）",
            "  2. 对手牌组耗尽无法抽牌（牌组破坏）",
            "  3. 特殊胜利卡牌效果（如 Exodia）",
            "  4. 对手认输",
            "",
            "【初始生命值】",
            "  - 标准：8000 点（OCG）/ 8000 点（TCG）",
            "  - 传统赛：8000 点",
            "  - 动画规则：4000 点",
            "",
            "【失败条件】",
            "  - 生命值 ≤ 0",
            "  - 抽牌时牌组为空",
            "  - 特殊胜利卡生效",
        ]
    }

    pub fn section_card_types(&self) -> Vec<&'static str> {
        vec![
            "【主要类型】",
            "",
            "【怪兽卡 Monster】",
            "  - 主要战斗单位",
            "  - 有攻击力、守备力、等级",
            "  - 包括通常怪兽和效果怪兽",
            "",
            "【魔法卡 Spell】",
            "  - 产生各种效果",
            "  - 速攻魔法可在对手回合使用",
            "  - 通常魔法只能在自己回合",
            "",
            "【陷阱卡 Trap】",
            "  - 盖放后下回合激活",
            "  - 可在对手回合响应",
            "  - 三种类型：通常、反击、永续",
            "",
            "【额外卡组类型】",
            "  - 融合怪兽",
            "  - 同调怪兽",
            "  - 超量怪兽",
            "  - 连接怪兽",
            "  - 灵摆怪兽",
        ]
    }

    pub fn section_monsters(&self) -> Vec<&'static str> {
        vec![
            "【怪兽卡分类】",
            "",
            "【通常怪兽 Normal】",
            "  - 无特殊效果",
            "  - 只有基础属性",
            "  - 橙色卡框",
            "",
            "【效果怪兽 Effect】",
            "  - 拥有特殊能力",
            "  - 多种效果类型",
            "  - 橙色卡框（有效果文本）",
            "",
            "【仪式怪兽 Ritual】",
            "  - 需要仪式魔法召唤",
            "  - 蓝色卡框",
            "",
            "【融合怪兽 Fusion】",
            "  - 需要融合召唤",
            "  - 紫色卡框",
            "",
            "【同调怪兽 Synchro】",
            "  - 需要调整 + 非调整怪兽",
            "  - 白色卡框",
            "",
            "【超量怪兽 Xyz】",
            "  - 需要同等级怪兽叠加",
            "  - 黑色卡框",
            "",
            "【连接怪兽 Link】",
            "  - 需要连接召唤",
            "  - 深蓝色卡框",
            "  - 有连接标记",
            "",
            "【灵摆怪兽 Pendulum】",
            "  - 可作怪兽或魔法",
            "  - 绿色卡框",
        ]
    }

    pub fn section_spells(&self) -> Vec<&'static str> {
        vec![
            "【魔法卡类型】",
            "",
            "【通常魔法 Normal Spell】",
            "  - 自己回合主阶段激活",
            "  - 激活后进入墓地",
            "  - 绿色卡框",
            "",
            "【速攻魔法 Quick-Play】",
            "  - 可在对手回合激活",
            "  - 盖放后本回合不可激活",
            "  - 图标：闪电符号",
            "",
            "【永续魔法 Continuous】",
            "  - 激活后留在场上",
            "  - 持续产生效果",
            "  - 图标：无限符号",
            "",
            "【装备魔法 Equip】",
            "  - 附着在怪兽上",
            "  - 增强或削弱怪兽",
            "  - 图标：十字符号",
            "",
            "【场地魔法 Field】",
            "  - 放置在场地区",
            "  - 影响全场或特定玩家",
            "  - 图标：十字/太阳",
            "",
            "【仪式魔法 Ritual】",
            "  - 用于仪式召唤",
            "  - 特定仪式怪兽专用",
            "  - 蓝色卡框",
        ]
    }

    pub fn section_traps(&self) -> Vec<&'static str> {
        vec![
            "【陷阱卡类型】",
            "",
            "【通常陷阱 Normal Trap】",
            "  - 盖放后下回合激活",
            "  - 激活后进入墓地",
            "  - 粉红色卡框",
            "",
            "【反击陷阱 Counter Trap】",
            "  - 可以连锁响应其他卡",
            "  - 通常用于无效对手行动",
            "  - 图标：箭头符号",
            "",
            "【永续陷阱 Continuous】",
            "  - 激活后留在场上",
            "  - 持续产生效果",
            "  - 图标：无限符号",
            "",
            "【激活规则】",
            "  - 必须盖放一回合",
            "  - 可以在对手回合激活",
            "  - 陷阱卡不能立即激活",
            "  - 激活需要满足条件",
        ]
    }

    pub fn section_zones(&self) -> Vec<&'static str> {
        vec![
            "【场上区域】",
            "",
            "【怪兽区 Monster Zone】",
            "  - 5 个怪兽格",
            "  - 放置怪兽卡",
            "  - 主要怪兽区域",
            "",
            "【魔法陷阱区 Spell/Trap Zone】",
            "  - 5 个魔法陷阱格",
            "  - 放置魔法和陷阱卡",
            "",
            "【场地魔法区 Field Zone】",
            "  - 放置场地魔法卡",
            "  - 只能有一张场地魔法",
            "",
            "【墓地 Graveyard】",
            "  - 已使用的卡牌",
            "  - 可被效果复活",
            "",
            "【牌组 Deck】",
            "  - 抽牌来源",
            "  - 牌面朝下",
            "",
            "【额外卡组 Extra Deck】",
            "  - 放置融合/同调/超量/连接怪兽",
            "  - 最多 15 张",
            "",
            "【除外区 Banished Zone】",
            "  - 被除外的卡牌",
            "  - 通常无法返回",
            "",
            "【灵摆区 Pendulum Zone】",
            "  - 左右各一个灵摆格",
            "  - 放置灵摆怪兽作魔法",
        ]
    }

    pub fn section_turn(&self) -> Vec<&'static str> {
        vec![
            "【回合流程】",
            "",
            "【1. 抽牌阶段 Draw Phase】",
            "  - 从牌组抽 1 张牌",
            "  - 先攻玩家第一回合不抽牌",
            "",
            "【2. 准备阶段 Standby Phase】",
            "  - 触发准备阶段效果",
            "  - 维持费用支付",
            "",
            "【3. 主阶段1 Main Phase 1】",
            "  - 召唤怪兽（通常召唤一次）",
            "  - 激活魔法卡",
            "  - 盖放魔法陷阱",
            "  - 改变怪兽表示形式",
            "",
            "【4. 战斗阶段 Battle Phase】",
            "  - 宣告攻击",
            "  - 处理战斗伤害",
            "  - 先攻玩家第一回合不能战斗",
            "",
            "【5. 主阶段2 Main Phase 2】",
            "  - 与主阶段 1 相同",
            "  - 通常召唤次数已用完",
            "",
            "【6. 结束阶段 End Phase】",
            "  - 触发结束阶段效果",
            "  - 手牌上限检查（6 张）",
            "  - 回合结束",
        ]
    }

    pub fn section_combat(&self) -> Vec<&'static str> {
        vec![
            "【战斗流程】",
            "",
            "【攻击宣告】",
            "  - 选择攻击怪兽",
            "  - 选择攻击对象（怪兽或玩家）",
            "  - 对手可响应（激活陷阱等）",
            "",
            "【攻击步骤】",
            "  1. 攻击宣言",
            "  2. 攻击响应（陷阱/速攻魔法）",
            "  3. 伤害计算",
            "  4. 伤害步骤",
            "  5. 战斗结束",
            "",
            "【怪兽对怪兽】",
            "  - 攻击怪兽 vs 守备怪兽",
            "  - 攻击力 vs 攻击力（攻击表示）",
            "  - 攻击力 vs 守备力（守备表示）",
            "",
            "【伤害计算】",
            "  - 攻击表示：攻击力比较",
            "  - 守备表示：攻击力 vs 守备力",
            "  - 攻击方高：防御方怪兽破坏",
            "  - 攻击方低：攻击方怪兽破坏",
            "  - 相等：双方怪兽都破坏",
            "",
            "【直接攻击】",
            "  - 对手无怪兽时可攻击玩家",
            "  - 造成等量生命值伤害",
        ]
    }

    pub fn section_chains(&self) -> Vec<&'static str> {
        vec![
            "【连锁机制】",
            "",
            "【连锁概念】",
            "  - 卡牌效果不立即结算",
            "  - 可响应对手的行动",
            "  - 后发动的卡先结算",
            "",
            "【连锁规则】",
            "  - 后进先出（LIFO）",
            "  - 连锁中不可插入其他行动",
            "  - 连锁结算后才可继续行动",
            "",
            "【连锁示例】",
            "  - 连锁 1：玩家A激活魔法",
            "  - 连锁 2：玩家B激活陷阱",
            "  - 连锁 3：玩家A激活速攻魔法",
            "  - 结算：连锁 3 → 连锁 2 → 连锁 1",
            "",
            "【响应时机】",
            "  - 对手激活卡牌时",
            "  - 对手召唤怪兽时",
            "  - 伤害步骤时",
            "  - 特定条件触发",
            "",
            "【优先权】",
            "  - 回合玩家有优先权",
            "  - 连锁后优先权转移",
            "  - 双方放弃后结算连锁",
        ]
    }

    pub fn section_summoning(&self) -> Vec<&'static str> {
        vec![
            "【召唤方式】",
            "",
            "【通常召唤 Normal Summon】",
            "  - 每回合一次",
            "  - 从手牌召唤到场上",
            "  - 等级 5+ 需要祭品",
            "",
            "【覆盖召唤 Set】",
            "  - 怪兽背面守备表示",
            "  - 算作通常召唤",
            "",
            "【特殊召唤 Special Summon】",
            "  - 不受通常召唤限制",
            "  - 可多次使用",
            "  - 各种特殊召唤方式",
            "",
            "【祭品召唤 Tribute Summon】",
            "  - 等级 5-6：释放 1 只怪兽",
            "  - 等级 7+：释放 2 只怪兽",
            "",
            "【融合召唤 Fusion Summon】",
            "  - 融合怪兽 + 融合素材",
            "  - 通过融合卡或效果",
            "",
            "【同调召唤 Synchro Summon】",
            "  - 调整怪兽 + 非调整怪兽",
            "  - 等级之和等于同调怪兽等级",
            "",
            "【超量召唤 Xyz Summon】",
            "  - 2+ 同等级怪兽叠加",
            "  - 作为超量怪兽的素材",
            "",
            "【连接召唤 Link Summon】",
            "  - 依据连接值和连接标记",
            "  - 使用场上怪兽作为素材",
        ]
    }

    pub fn section_extra_deck(&self) -> Vec<&'static str> {
        vec![
            "【额外卡组规则】",
            "",
            "【卡组限制】",
            "  - 最多 15 张卡",
            "  - 只能放融合/同调/超量/连接怪兽",
            "  - 灵摆怪兽也可放入",
            "",
            "【融合怪兽】",
            "  - 紫色卡框",
            "  - 需要融合魔法",
            "  - 特定素材融合",
            "",
            "【同调怪兽】",
            "  - 白色卡框",
            "  - 调整 + 非调整",
            "  - 等级相加",
            "",
            "【超量怪兽】",
            "  - 黑色卡框",
            "  - 同等级怪兽叠加",
            "  - 有阶级（Rank）而非等级",
            "",
            "【连接怪兽】",
            "  - 深蓝色卡框",
            "  - 有连接值（Link Rating）",
            "  - 有连接标记",
            "",
            "【召唤方式】",
            "  - 从额外卡组特殊召唤",
            "  - 不占用通常召唤",
            "  - 各有特定召唤条件",
        ]
    }

    pub fn section_banlist(&self) -> Vec<&'static str> {
        vec![
            "【禁忌限制表】",
            "",
            "【禁止 Forbidden】",
            "  - 不能在竞技卡组中使用",
            "  - 过强或破坏平衡的卡",
            "  - 如：Exodia 五部件（部分）",
            "",
            "【限制 Limited】",
            "  - 卡组中只能有 1 张",
            "  - 强力但可接受的卡",
            "  - 如：雷击、死者苏生（部分时期）",
            "",
            "【准限制 Semi-Limited】",
            "  - 卡组中只能有 2 张",
            "  - 中等强度的卡",
            "",
            "【无限制 Unlimited】",
            "  - 卡组中最多 3 张",
            "  - 普通强度的卡",
            "",
            "【适用范围】",
            "  - OCG 和 TCG 有不同禁限表",
            "  - 定期更新（每年数次）",
            "  - 影响竞技环境",
        ]
    }
}

#[cfg(test)]
mod yugioh_tests {
    use super::*;
    use crate::rules::core::Rule;

    #[test]
    fn test_yugioh_rules_creation() {
        let rules = YuGiOhBasicRules::new();
        assert!(rules.explain().contains("游戏王"));
        assert!(rules.explain().contains("高桥和希"));
    }

    #[test]
    fn test_monster_types() {
        let rules = YuGiOhBasicRules::new();
        let monsters = rules.section_monsters();
        assert!(monsters.iter().any(|s| s.contains("通常怪兽")));
        assert!(monsters.iter().any(|s| s.contains("效果怪兽")));
    }

    #[test]
    fn test_spell_types() {
        let rules = YuGiOhBasicRules::new();
        let spells = rules.section_spells();
        assert!(spells.iter().any(|s| s.contains("速攻魔法")));
        assert!(spells.iter().any(|s| s.contains("场地魔法")));
    }

    #[test]
    fn test_combat_system() {
        let rules = YuGiOhBasicRules::new();
        let combat = rules.section_combat();
        assert!(combat.iter().any(|s| s.contains("攻击")));
        assert!(combat.iter().any(|s| s.contains("伤害")));
    }
}
