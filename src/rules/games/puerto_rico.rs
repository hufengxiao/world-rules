//! 波多黎各规则（Puerto Rico）
//!
//! 德国经典策略桌游，殖民建设与资源管理。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PuertoRicoRules,
    name: "波多黎各规则",
    desc: "波多黎各（Puerto Rico）殖民策略桌游规则",
    origin: "德国",
    tags: ["游戏", "桌游", "波多黎各", "策略", "资源管理"],
}

impl PuertoRicoRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "德国桌游，2002年出版",
            "设计师 Andreas Seyfarth",
            "玩家扮演波多黎各殖民者",
            "建设城市、种植作物、运输货物",
            "胜利点最高者获胜",
        ]
    }

    /// 游戏组件
    pub fn section_components(&self) -> Vec<&'static str> {
        vec![
            "总督牌：1张，标记起始玩家",
            "角色牌：8种，每回合选择",
            "建筑牌：各种建筑",
            "货物：玉米、靛蓝、糖、烟草、咖啡",
            "钱币（ doubloons）：金币",
        ]
    }

    /// 角牌系统
    pub fn section_roles(&self) -> Vec<&'static str> {
        vec![
            "每回合每人选一个角色牌",
            "角色牌提供特权",
            "选角色者获得额外好处",
            "其他玩家执行基础版本",
            "角色牌循环使用",
        ]
    }

    /// 角牌种类
    pub fn section_role_types(&self) -> Vec<&'static str> {
        vec![
            "建筑师：建造建筑",
            "监工：生产货物",
            "商人：出售货物",
            "船长：运送货物",
            "拓荒者：获得种植地块",
            "工匠：生产货物（监工同义）",
            "贸易商：同商人",
            "总督：下一轮起始",
        ]
    }

    /// 建筑类型
    pub fn section_buildings(&self) -> Vec<&'static str> {
        vec![
            "生产建筑：对应作物种类",
            "紫色建筑：提供特殊能力",
            "大型建筑：终局加分",
            "建筑需花费 doubloons",
            "建筑空间有限（12格）",
        ]
    }

    /// 作物生产
    pub fn section_production(&self) -> Vec<&'static str> {
        vec![
            "玉米：无需工厂即可生产",
            "靛蓝、糖、烟草、咖啡：需对应工厂",
            "监工角色触发生产",
            "种植地块+工厂才能生产",
            "货物放入玩家仓库",
        ]
    }

    /// 货物运输
    pub fn section_shipping(&self) -> Vec<&'static str> {
        vec![
            "船长角色触发运输",
            "货物运往欧洲或新世界",
            "运往欧洲获得金币",
            "运往新世界获得胜利点",
            "货物必须轮换运输",
        ]
    }

    /// 贸易规则
    pub fn section_trade(&self) -> Vec<&'static str> {
        vec![
            "商人角色触发贸易",
            "出售货物获得金币",
            "市场价格随机决定",
            "每人只能出售一个货物",
            "市场价格出售后上升",
        ]
    }

    /// 金币使用
    pub fn section_doubloons(&self) -> Vec<&'static str> {
        vec![
            "建造建筑花费金币",
            "某些角色提供金币",
            "出售货物获得金币",
            "金币不能直接换胜利点",
            "建筑提供游戏内好处",
        ]
    }

    /// 游戏结束
    pub fn section_endgame(&self) -> Vec<&'static str> {
        vec![
            "任一条件触发结束：",
            "总督牌用完",
            "或任一种货物用完",
            "或建筑空间全部填满",
            "计算胜利点决定胜负",
        ]
    }

    /// 计分规则
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "运送货物获得胜利点",
            "大型建筑终局加分",
            "加成建筑提供额外VP",
            "每4金币等于1VP（终局）",
            "总VP最高者获胜",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "早期生产基础设施建设",
            "中期运输获取胜利点",
            "角色选择影响全局节奏",
            "控制货物市场",
            "平衡生产与运输",
        ]
    }

    /// 策略流派
    pub fn section_strategies(&self) -> Vec<&'static str> {
        vec![
            "生产策略：大量生产低成本作物",
            "运输策略：早期运输获取VP",
            "建筑策略：追求大型建筑加分",
            "混合策略：灵活应变",
            "角色选择是关键决策",
        ]
    }
}

impl Rule for PuertoRicoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("puerto_rico")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "波多黎各规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("游戏组件", &self.section_components()),
                ("角色牌系统", &self.section_roles()),
                ("角色牌种类", &self.section_role_types()),
                ("建筑类型", &self.section_buildings()),
                ("作物生产", &self.section_production()),
                ("货物运输", &self.section_shipping()),
                ("贸易规则", &self.section_trade()),
                ("金币使用", &self.section_doubloons()),
                ("游戏结束", &self.section_endgame()),
                ("计分规则", &self.section_scoring()),
                ("策略要点", &self.section_strategy()),
                ("策略流派", &self.section_strategies()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puerto_rico_rules_basic() {
        let rules = PuertoRicoRules::new();
        assert_eq!(rules.metadata().name, "波多黎各规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn puerto_rico_has_roles() {
        let rules = PuertoRicoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("角色牌") || explanation.contains("建筑师"));
    }

    #[test]
    fn puerto_rico_production() {
        let rules = PuertoRicoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("货物") || explanation.contains("生产"));
    }

    #[test]
    fn puerto_rico_scoring() {
        let rules = PuertoRicoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("胜利点") || explanation.contains("VP"));
    }

    #[test]
    fn puerto_rico_has_origin() {
        let rules = PuertoRicoRules::new();
        assert_eq!(rules.metadata().origin, Some("德国".to_string()));
    }
}