//! 铁路大亨规则（Railway Tycoon / Age of Steam）
//!
//! 经典铁路建设经济策略桌游，玩家经营铁路公司。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: RailwayTycoonRules,
    name: "铁路大亨规则",
    desc: "铁路大亨（Railway Tycoon/Age of Steam）经济策略桌游规则",
    origin: "美国",
    tags: ["游戏", "桌游", "铁路", "经济", "策略"],
}

impl RailwayTycoonRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "经典铁路建设策略桌游",
            "玩家经营19世纪铁路公司",
            "建设铁路网络运输货物",
            "竞争最佳路线和城市",
            "财富最多者获胜",
        ]
    }

    /// 游戏组件
    pub fn section_components(&self) -> Vec<&'static str> {
        vec![
            "地图板：显示城市和地形",
            "铁路轨道标记：各颜色代表各玩家",
            "货物标记：各种类型货物",
            "钱币：游戏货币",
            "城市标记：发展等级指示",
            "玩家板：记录收入和支出",
        ]
    }

    /// 游戏准备
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "每人选择一个铁路公司",
            "获得起始资金（通常$50）",
            "放置城市货物标记",
            "设置初始货物供应",
            "确定起始玩家",
        ]
    }

    /// 回合流程
    pub fn section_turn_order(&self) -> Vec<&'static str> {
        vec![
            "阶段1：拍卖行动顺序",
            "阶段2：选择行动",
            "阶段3：建设铁路",
            "阶段4：运输货物",
            "阶段5：收入和支出",
        ]
    }

    /// 行动拍卖
    pub fn section_auction(&self) -> Vec<&'static str> {
        vec![
            "拍卖决定行动顺序",
            "出价最高者先选择行动",
            "出价从玩家资金支付",
            "未出价者最后选择",
            "策略：平衡资金和行动优势",
        ]
    }

    /// 可选行动
    pub fn section_actions(&self) -> Vec<&'static str> {
        vec![
            "建设铁路：铺设新轨道",
            "升级城市：增加城市等级",
            "运输货物：移动货物得分",
            "发展城市：添加新货物",
            "特殊行动：各种能力",
        ]
    }

    /// 铁路建设
    pub fn section_building(&self) -> Vec<&'static str> {
        vec![
            "支付建设费用",
            "平原：$1 每格",
            "山地：$2 每格",
            "河流：额外费用",
            "只能连接已有网络",
        ]
    }

    /// 城市系统
    pub fn section_cities(&self) -> Vec<&'static str> {
        vec![
            "城市有等级（1-6级）",
            "等级决定货物容量",
            "升级城市增加容量",
            "城市生产特定类型货物",
            "城市间可运输货物",
        ]
    }

    /// 货物运输
    pub fn section_shipping(&self) -> Vec<&'static str> {
        vec![
            "移动货物到目的地城市",
            "距离越长得分越高",
            "必须通过自己铁路",
            "每回合限运一次",
            "货物消耗后移除",
        ]
    }

    /// 货物类型
    pub fn section_goods(&self) -> Vec<&'static str> {
        vec![
            "煤炭（黑色）",
            "钢铁（灰色）",
            "货物（棕色）",
            "货物类型影响得分",
            "特定城市生产特定货物",
        ]
    }

    /// 收入系统
    pub fn section_income(&self) -> Vec<&'static str> {
        vec![
            "基础收入 = 城市连接数",
            "运输收入 = 运输距离",
            "总收入进入玩家资金",
            "必须支付维护费用",
            "净收入影响最终得分",
        ]
    }

    /// 债务规则
    pub fn section_debt(&self) -> Vec<&'static str> {
        vec![
            "资金不足可借款",
            "借款产生利息",
            "游戏结束扣除债务",
            "策略性借款是关键",
            "过度借款导致失败",
        ]
    }

    /// 游戏结束
    pub fn section_endgame(&self) -> Vec<&'static str> {
        vec![
            "触发条件：货物耗尽",
            "或某玩家资金归零",
            "计算最终得分",
            "资产价值 + 现金 - 债务",
            "最高分者获胜",
        ]
    }

    /// 计分规则
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "铁路网络长度得分",
            "城市连接数量得分",
            "运输货物数量得分",
            "城市升级等级得分",
            "扣除债务和利息",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "优先连接高产城市",
            "控制关键路线",
            "平衡建设和运输",
            "注意资金流动",
            "适时借贷扩大网络",
        ]
    }

    /// 地图变体
    pub fn section_maps(&self) -> Vec<&'static str> {
        vec![
            "美国东部：高密度城市",
            "欧洲：多样化地形",
            "日本：岛屿连接挑战",
            "中国：大规模网络",
            "各地图策略不同",
        ]
    }

    /// 变体规则
    pub fn section_variants(&self) -> Vec<&'static str> {
        vec![
            "Age of Steam：经典版本",
            "Railways of the World：简化版",
            "Railway Tycoon：地图扩展",
            "Steam：机制改良版",
            "各版本细节略有不同",
        ]
    }
}

impl Rule for RailwayTycoonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("railway_tycoon")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "铁路大亨规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("游戏组件", &self.section_components()),
                ("游戏准备", &self.section_setup()),
                ("回合流程", &self.section_turn_order()),
                ("行动拍卖", &self.section_auction()),
                ("可选行动", &self.section_actions()),
                ("铁路建设", &self.section_building()),
                ("城市系统", &self.section_cities()),
                ("货物运输", &self.section_shipping()),
                ("货物类型", &self.section_goods()),
                ("收入系统", &self.section_income()),
                ("债务规则", &self.section_debt()),
                ("游戏结束", &self.section_endgame()),
                ("计分规则", &self.section_scoring()),
                ("策略要点", &self.section_strategy()),
                ("地图变体", &self.section_maps()),
                ("变体规则", &self.section_variants()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn railway_tycoon_rules_basic() {
        let rules = RailwayTycoonRules::new();
        assert_eq!(rules.metadata().name, "铁路大亨规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn railway_tycoon_has_railway() {
        let rules = RailwayTycoonRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("铁路"));
    }

    #[test]
    fn railway_tycoon_has_goods() {
        let rules = RailwayTycoonRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("货物") || explanation.contains("运输"));
    }

    #[test]
    fn railway_tycoon_scoring() {
        let rules = RailwayTycoonRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("得分") || explanation.contains("收入"));
    }

    #[test]
    fn railway_tycoon_has_origin() {
        let rules = RailwayTycoonRules::new();
        assert_eq!(rules.metadata().origin, Some("美国".to_string()));
    }
}
