//! UNO 详细规则
//!
//! UNO 是一款流行的家庭卡牌游戏，由 Merle Robbins 于 1971 年发明。
//! 玩家需要尽快打出手中的牌，并在剩下一张牌时喊出"UNO"。

use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: UnoDetailedRules,
    name: "UNO详细规则",
    desc: "UNO卡牌游戏完整规则",
    origin: "美国",
    tags: ["游戏", "卡牌", "家庭游戏"],
    category: RuleCategory::games("uno"),
    sections: [
        ("游戏概述", section_overview),
        ("牌组构成", section_deck),
        ("游戏准备", section_setup),
        ("基本玩法", section_gameplay),
        ("特殊牌详解", section_special_cards),
        ("出牌规则", section_playing_rules),
        ("喊UNO规则", section_uno_call),
        ("计分规则", section_scoring),
        ("游戏变体", section_variants),
        ("策略技巧", section_strategy)
    ]
}

impl UnoDetailedRules {
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "UNO 是 2-10 人的卡牌游戏",
            "目标：尽快打出手中的所有牌",
            "获胜条件：第一个出完所有牌的玩家获胜",
            "游戏时长：约 30 分钟",
            "适合年龄：7岁以上",
            "设计者：Merle Robbins (1971年)",
            "发行商：Mattel（美泰）",
        ]
    }

    pub fn section_deck(&self) -> Vec<&'static str> {
        vec![
            "标准 UNO 牌组：108 张牌",
            "数字牌（76张）：",
            "  - 4 种颜色（红、黄、绿、蓝）",
            "  - 每种颜色 1-9 各 2 张，0 各 1 张",
            "动作牌（24张）：",
            "  - 跳过牌：每种颜色 2 张，共 8 张",
            "  - 反转牌：每种颜色 2 张，共 8 张",
            "  - +2 牌：每种颜色 2 张，共 8 张",
            "万能牌（8张）：",
            "  - 变色牌：4 张",
            "  - +4 变色牌：4 张",
            "空白牌（4张）：用于自定义规则",
        ]
    }

    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "选择发牌者：首次游戏随机选择",
            "发牌：每人 7 张牌，牌面朝下",
            "翻开牌堆：从抽牌堆翻开一张牌作为起始牌",
            "起始牌处理：",
            "  - 如果是数字牌：正常开始",
            "  - 如果是动作牌：执行相应动作",
            "  - 如果是万能牌：放回牌堆重新翻开",
            "确定出牌方向：顺时针或逆时针",
            "抽牌堆：剩余牌作为抽牌堆",
        ]
    }

    pub fn section_gameplay(&self) -> Vec<&'static str> {
        vec![
            "轮次顺序：按当前方向依次进行",
            "玩家回合流程：",
            "  1. 检查是否可以出牌",
            "  2. 选择出牌或抽牌",
            "  3. 执行牌的效果",
            "  4. 检查 UNO 状态",
            "出牌条件：",
            "  - 颜色相同",
            "  - 数字/符号相同",
            "  - 使用万能牌",
            "抽牌规则：无法出牌时抽 1 张",
            "抽牌后可选择出牌或保留",
        ]
    }

    pub fn section_special_cards(&self) -> Vec<&'static str> {
        vec![
            "【跳过牌 Skip】",
            "  - 效果：跳过下一位玩家",
            "  - 2 人游戏：跳过对手，自己继续",
            "  - 策略：阻止关键玩家出牌",
            "",
            "【反转牌 Reverse】",
            "  - 效果：改变出牌方向",
            "  - 顺时针变逆时针，或相反",
            "  - 2 人游戏：跳过对手，自己继续",
            "  - 策略：控制出牌顺序",
            "",
            "【+2 牌 Draw Two】",
            "  - 效果：下一位玩家抽 2 张牌并跳过",
            "  - 受影响玩家本轮无法出牌",
            "  - 可叠加：+2 +2 = +4",
            "",
            "【变色牌 Wild】",
            "  - 效果：任意时间可出",
            "  - 选择任意颜色继续",
            "  - 无法出牌时的救命牌",
            "",
            "【+4 变色牌 Wild Draw Four】",
            "  - 效果：下家抽 4 张并跳过",
            "  - 选择任意颜色继续",
            "  - 出牌限制：手上无其他可出牌时",
            "  - 争议：被质疑时需展示手牌",
        ]
    }

    pub fn section_playing_rules(&self) -> Vec<&'static str> {
        vec![
            "出牌时机：轮到自己时",
            "出牌限制：",
            "  - 必须匹配颜色或数字/符号",
            "  - 万能牌可随时出（+4 有条件限制）",
            "匹配规则：",
            "  - 颜色匹配：红配红、黄配黄等",
            "  - 数字匹配：数字相同即可",
            "  - 符号匹配：跳过配跳过、反转配反转等",
            "特殊出牌：",
            "  - 可连续出多张相同的数字/符号牌",
            "  - 变色牌可选择任意颜色",
            "抽牌选项：",
            "  - 无法出牌时必须抽 1 张",
            "  - 抽牌后可立即出牌或保留",
        ]
    }

    pub fn section_uno_call(&self) -> Vec<&'static str> {
        vec![
            "【核心规则】",
            "  - 手牌剩余 1 张时必须喊'UNO'",
            "  - 未喊被其他玩家发现：罚抽 2 张",
            "",
            "【喊 UNO 时机】",
            "  - 出牌后立即喊（在下一个玩家出牌前）",
            "  - 建议：出牌同时喊'UNO'",
            "",
            "【惩罚规则】",
            "  - 未喊被抓住：抽 2 张牌",
            "  - 喊错（手牌多于 2 张）：无惩罚",
            "  - 多次违规可增加惩罚",
            "",
            "【策略建议】",
            "  - 出牌前准备好喊 UNO",
            "  - 快速出牌防止被抓住",
            "  - 观察其他玩家是否喊 UNO",
        ]
    }

    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "【计分方式】",
            "  - 获胜者得分",
            "  - 计算其他玩家手牌分数总和",
            "",
            "【牌面分值】",
            "  - 数字牌（0-9）：牌面数字分",
            "  - 跳过/反转/+2：各 20 分",
            "  - 变色牌：50 分",
            "  - +4 变色牌：50 分",
            "",
            "【游戏结束】",
            "  - 方式一：先达到 500 分获胜",
            "  - 方式二：单局胜负制",
            "  - 方式三：限时积分制",
            "",
            "【团队模式计分】",
            "  - 队友分数相加",
            "  - 先达到目标的队伍获胜",
        ]
    }

    pub fn section_variants(&self) -> Vec<&'static str> {
        vec![
            "【标准变体】",
            "  - Jump-In：持有相同牌可立即出牌",
            "  - Seven-0：出 7 可与任意玩家换牌",
            "  - 七零规则：出 0 时所有人交换手牌",
            "",
            "【特殊规则】",
            "  - 堆叠规则：+2 可被 +2 抵消并累加",
            "  - 强制出牌：抽牌后必须出牌",
            "  - 禁言模式：喊 UNO 算犯规",
            "",
            "【UNO Spin】",
            "  - 转盘决定额外规则",
            "  - 更复杂的游戏机制",
            "",
            "【UNO Flip】",
            "  - 双面牌设计",
            "  - 翻转改变游戏规则",
            "",
            "【UNO Minecraft】",
            "  - Minecraft 主题牌面",
            "  - 特殊方块牌",
        ]
    }

    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "【基本策略】",
            "  - 保留万能牌到关键时刻",
            "  - 优先出高分牌",
            "  - 控制颜色流向",
            "",
            "【进阶策略】",
            "  - 记忆已出的牌",
            "  - 预测对手手牌",
            "  - 合理使用 +2/+4",
            "  - 反转牌改变攻击目标",
            "",
            "【防守策略】",
            "  - 保留变色牌应对紧急情况",
            "  - 避免被连续 +2/+4",
            "  - 观察对手手牌数量",
            "",
            "【进攻策略】",
            "  - 对手接近胜利时使用 +4",
            "  - 连续使用跳过牌",
            "  - 利用反转牌控制节奏",
            "",
            "【团队策略】",
            "  - 保护队友不被惩罚",
            "  - 配合队友出牌",
            "  - 联合攻击对手",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::Rule;

    #[test]
    fn test_uno_rules_creation() {
        let rules = UnoDetailedRules::new();
        assert!(rules.explain().contains("UNO"));
        assert!(rules.explain().contains("108"));
    }

    #[test]
    fn test_deck_composition() {
        let rules = UnoDetailedRules::new();
        let deck = rules.section_deck();
        assert!(deck.iter().any(|s| s.contains("108")));
        assert!(deck.iter().any(|s| s.contains("数字牌")));
    }

    #[test]
    fn test_special_cards() {
        let rules = UnoDetailedRules::new();
        let special = rules.section_special_cards();
        assert!(special.iter().any(|s| s.contains("跳过")));
        assert!(special.iter().any(|s| s.contains("反转")));
        assert!(special.iter().any(|s| s.contains("+2")));
    }

    #[test]
    fn test_uno_calling_rules() {
        let rules = UnoDetailedRules::new();
        let uno_call = rules.section_uno_call();
        assert!(uno_call.iter().any(|s| s.contains("UNO")));
        assert!(uno_call.iter().any(|s| s.contains("惩罚")));
    }

    #[test]
    fn test_scoring_system() {
        let rules = UnoDetailedRules::new();
        let scoring = rules.section_scoring();
        assert!(scoring.iter().any(|s| s.contains("500")));
        assert!(scoring.iter().any(|s| s.contains("50 分")));
    }
}
