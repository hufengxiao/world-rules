//! 皇权争夺规则（Dominion）
//!
//! 美国经典卡牌构建桌游，牌库构建与王国扩张。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: DominionRules,
    name: "皇权争夺规则",
    desc: "皇权争夺（Dominion）牌库构建桌游规则",
    origin: "美国",
    tags: ["游戏", "桌游", "皇权争夺", "牌库构建", "策略"],
}

impl DominionRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "美国桌游，2008年出版",
            "设计师 Donald X. Vaccarino",
            "开创牌库构建（Deck Building）类型",
            "玩家构建自己的牌库",
            "最终胜利点数最高者获胜",
        ]
    }

    /// 游戏组件
    pub fn section_components(&self) -> Vec<&'static str> {
        vec![
            "基本牌：财宝牌、胜利牌、诅咒牌",
            "王国牌：每局选10种，共252种",
            "每个玩家初始牌库：7铜币+3庄园",
            "行动牌堆、财宝牌堆、胜利牌堆",
            "扩展包大幅增加王国牌种类",
        ]
    }

    /// 牌型分类
    pub fn section_card_types(&self) -> Vec<&'static str> {
        vec![
            "财宝牌：铜币(1)、银币(2)、金币(3)",
            "胜利牌：庄园(1VP)、公国(3VP)、省份(6VP)",
            "诅咒牌：诅咒(-1VP)，用于攻击牌",
            "行动牌：提供各种效果",
            "反应牌：可抵消攻击牌效果",
        ]
    }

    /// 游戏流程
    pub fn section_flow(&self) -> Vec<&'static str> {
        vec![
            "每回合：行动阶段 → 财宝阶段 → 购买阶段 → 清理阶段",
            "行动阶段：打出行动牌，消耗行动点",
            "财宝阶段：打出财宝牌获得金币",
            "购买阶段：购买牌堆中的牌",
            "清理阶段：弃掉所有打出的牌和手牌",
        ]
    }

    /// 资源系统
    pub fn section_resources(&self) -> Vec<&'static str> {
        vec![
            "行动点（Actions）：每回合1个，行动牌消耗",
            "购买点（Buys）：每回合1个，每次购买消耗",
            "金币（Coins）：打出财宝牌累积",
            "行动牌可增加各种资源",
            "手牌数量默认每回合5张",
        ]
    }

    /// 购买规则
    pub fn section_buy(&self) -> Vec<&'static str> {
        vec![
            "购买牌放入弃牌堆",
            "回合结束时洗入牌库",
            "只能购买牌堆顶的牌",
            "牌堆耗尽后不可购买",
            "每回合最多购买等于购买点数",
        ]
    }

    /// 牌库管理
    pub fn section_deck(&self) -> Vec<&'static str> {
        vec![
            "牌库 shuffled 后放于面前",
            "手牌从牌库抽取",
            "弃牌堆放于旁边",
            "牌库耗尽时弃牌堆洗回牌库",
            "胜利牌通常不打出，仅占牌库空间",
        ]
    }

    /// 行动牌类型
    pub fn section_action_types(&self) -> Vec<&'static str> {
        vec![
            "抽牌：增加手牌",
            "加行动：增加行动点",
            "加购买：增加购买点",
            "加金币：增加当回合金币",
            "攻击：影响其他玩家",
        ]
    }

    /// 常见行动牌
    pub fn section_common_actions(&self) -> Vec<&'static str> {
        vec![
            "村庄：+1行动+2卡片，1成本",
            "工坊：获得成本≤4的牌，3成本",
            "市集：+1行动+1购买+1金币+1卡片，5成本",
            "民兵：其他玩家弃到3张，获得+2金币，4成本",
            " remodel：弃牌获得成本+2的牌，4成本",
        ]
    }

    /// 游戏结束
    pub fn section_endgame(&self) -> Vec<&'static str> {
        vec![
            "任一条件触发结束：",
            "省份牌堆耗尽",
            "或任三种牌堆耗尽",
            "玩家计算牌库中胜利点",
            "胜利点最高者获胜",
        ]
    }

    /// 计分
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "庄园：1胜利点",
            "公国：3胜利点",
            "省份：6胜利点",
            "诅咒：-1胜利点",
            "某些王国牌提供胜利点",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "平衡行动链与购买力",
            "早期建立财宝基础",
            "注意胜利牌稀释牌库",
            "使用清库牌控制牌库大小",
            "观察对手策略并应对",
        ]
    }

    /// 扩展包
    pub fn section_expansions(&self) -> Vec<&'static str> {
        vec![
            "繁荣：加入殖民地牌(10VP)",
            "异国：新机制如牌库上放牌",
            "海滨：持续效果牌",
            "炼金术：药剂成本机制",
            "超过15个官方扩展",
        ]
    }
}

impl Rule for DominionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("dominion")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "皇权争夺规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("游戏组件", &self.section_components()),
                ("牌型分类", &self.section_card_types()),
                ("游戏流程", &self.section_flow()),
                ("资源系统", &self.section_resources()),
                ("购买规则", &self.section_buy()),
                ("牌库管理", &self.section_deck()),
                ("行动牌类型", &self.section_action_types()),
                ("常见行动牌", &self.section_common_actions()),
                ("游戏结束", &self.section_endgame()),
                ("计分", &self.section_scoring()),
                ("策略要点", &self.section_strategy()),
                ("扩展包", &self.section_expansions()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dominion_rules_basic() {
        let rules = DominionRules::new();
        assert_eq!(rules.metadata().name, "皇权争夺规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn dominion_has_card_types() {
        let rules = DominionRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("财宝牌") && explanation.contains("胜利牌"));
        assert!(explanation.contains("行动牌"));
    }

    #[test]
    fn dominion_resources() {
        let rules = DominionRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("行动点") || explanation.contains("购买点"));
    }

    #[test]
    fn dominion_scoring() {
        let rules = DominionRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("胜利点") || explanation.contains("省份"));
    }

    #[test]
    fn dominion_has_origin() {
        let rules = DominionRules::new();
        assert_eq!(rules.metadata().origin, Some("美国".to_string()));
    }
}