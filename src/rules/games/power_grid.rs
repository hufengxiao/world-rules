//! 电力公司规则（Power Grid）
//!
//! 德国经典经济桌游，电力市场与网络建设。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PowerGridRules,
    name: "电力公司规则",
    desc: "电力公司（Power Grid）经济策略桌游规则",
    origin: "德国",
    tags: ["游戏", "桌游", "电力公司", "经济", "策略"],
}

impl PowerGridRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "德国桌游，2004年出版",
            "设计师 Friedemann Friese",
            "玩家经营电力公司",
            "购买电厂、建设网络、供应城市",
            "供应最多城市者获胜",
        ]
    }

    /// 游戏组件
    pub fn section_components(&self) -> Vec<&'static str> {
        vec![
            "地图：德国/美国等版本",
            "电厂牌：多种效率电厂",
            "资源：煤、石油、垃圾、核能",
            "钱币（ Elektro）：货币",
            "城市连接标记",
        ]
    }

    /// 游戏流程
    pub fn section_flow(&self) -> Vec<&'static str> {
        vec![
            "回合顺序：供应城市最多者最后",
            "阶段1：确定回合顺序",
            "阶段2：拍卖电厂",
            "阶段3：购买资源",
            "阶段4：建设网络",
            "阶段5：发电供应城市",
        ]
    }

    /// 电厂拍卖
    pub fn section_auction(&self) -> Vec<&'static str> {
        vec![
            "每回合可拍卖一个电厂",
            "最低出价=电厂底价",
            "中标者支付出价",
            "未中标者可买市场最低电厂",
            "电厂市场动态变化",
        ]
    }

    /// 电厂类型
    pub fn section_plant_types(&self) -> Vec<&'static str> {
        vec![
            "煤炭电厂：消耗煤资源",
            "石油电厂：消耗石油",
            "混合电厂：可消耗多种资源",
            "垃圾电厂：使用垃圾资源",
            "核能电厂：核能资源",
            "绿色电厂：风能、太阳能（无需资源）",
        ]
    }

    /// 资源购买
    pub fn section_resources(&self) -> Vec<&'static str> {
        vec![
            "资源市场分为两部分",
            "当前市场：价格较低",
            "未来市场：价格较高",
            "资源价格随供给变化",
            "每回合可买任意数量资源",
        ]
    }

    /// 网络建设
    pub fn section_building(&self) -> Vec<&'static str> {
        vec![
            "选择新城市连接",
            "支付连接费用+城市费用",
            "只能连接已有网络相邻城市",
            "网络费用取决于距离",
            "不同阶段城市开放规则不同",
        ]
    }

    /// 城市供应
    pub fn section_bureaucracy(&self) -> Vec<&'static str> {
        vec![
            "燃烧资源发电",
            "必须拥有足够资源",
            "电厂容量决定供应城市数",
            "供应城市获得收入",
            "收入随供应城市数增加",
        ]
    }

    /// 阶段系统
    pub fn section_phases(&self) -> Vec<&'static str> {
        vec![
            "阶段1：每城市可1家公司",
            "阶段2：每城市可2家公司",
            "阶段3：每城市可3家公司",
            "阶段触发：某区域被填满",
            "阶段变化影响城市开放",
        ]
    }

    /// 收入规则
    pub fn section_income(&self) -> Vec<&'static str> {
        vec![
            "供应城市数对应收入表",
            "每供应1个城市获得收入",
            "收入用于购买电厂/资源",
            "收入是主要资金来源",
            "收入表随城市数递增",
        ]
    }

    /// 游戏结束
    pub fn section_endgame(&self) -> Vec<&'static str> {
        vec![
            "触发条件：某玩家供应≥17城市",
            "或阶段3开始时某玩家供应≥17城市",
            "最终供应最多城市者获胜",
            "如平局比较金钱",
            "如再平局比较电厂数量",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "平衡电厂效率与成本",
            "控制资源市场价格",
            "关注回合顺序（后买有优势）",
            "适时抢占关键城市",
            "绿色电厂后期重要",
        ]
    }

    /// 地图变体
    pub fn section_maps(&self) -> Vec<&'static str> {
        vec![
            "德国地图：初始版本",
            "美国地图：区域差异大",
            "中国地图：连接费用低",
            "日本/韩国地图",
            "各地图策略不同",
        ]
    }
}

impl Rule for PowerGridRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("power_grid")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电力公司规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("游戏组件", &self.section_components()),
                ("游戏流程", &self.section_flow()),
                ("电厂拍卖", &self.section_auction()),
                ("电厂类型", &self.section_plant_types()),
                ("资源购买", &self.section_resources()),
                ("网络建设", &self.section_building()),
                ("城市供应", &self.section_bureaucracy()),
                ("阶段系统", &self.section_phases()),
                ("收入规则", &self.section_income()),
                ("游戏结束", &self.section_endgame()),
                ("策略要点", &self.section_strategy()),
                ("地图变体", &self.section_maps()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_grid_rules_basic() {
        let rules = PowerGridRules::new();
        assert_eq!(rules.metadata().name, "电力公司规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn power_grid_has_plants() {
        let rules = PowerGridRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("电厂"));
    }

    #[test]
    fn power_grid_resources() {
        let rules = PowerGridRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("资源") || explanation.contains("煤"));
    }

    #[test]
    fn power_grid_scoring() {
        let rules = PowerGridRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("供应") || explanation.contains("城市"));
    }

    #[test]
    fn power_grid_has_origin() {
        let rules = PowerGridRules::new();
        assert_eq!(rules.metadata().origin, Some("德国".to_string()));
    }
}