//! 卡卡颂规则（Carcassonne）
//!
//! 德国经典板块放置桌游，地图拼接与领地占领。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CarcassonneRules,
    name: "卡卡颂规则",
    desc: "卡卡颂（Carcassonne）板块放置桌游规则",
    origin: "德国",
    tags: ["游戏", "桌游", "卡卡颂", "板块放置", "策略"],
}

impl CarcassonneRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "经典德国桌游，2000年出版",
            "设计师 Klaus-Jürgen Wrede",
            "玩家拼接地图板块",
            "派遣追随者占领领地",
            "最终得分最高者获胜",
        ]
    }

    /// 游戏组件
    pub fn section_components(&self) -> Vec<&'static str> {
        vec![
            "84个地形板块（含1个起始板块）",
            "40个追随者（每色8个）",
            "1个计分板",
            "每个玩家选择一种颜色",
            "扩展包增加更多板块和功能",
        ]
    }

    /// 地形类型
    pub fn section_terrain(&self) -> Vec<&'static str> {
        vec![
            "道路：线状地形，连接道路端点",
            "城市：封闭区域，城墙壁围绕",
            "修道院：单独建筑，周围9格",
            "农田：绿色区域，贯穿板块",
            "河流：特殊地形（扩展包）",
        ]
    }

    /// 游戏流程
    pub fn section_flow(&self) -> Vec<&'static str> {
        vec![
            "轮次：抽取板块 → 放置板块 → 派遣追随者",
            "板块必须与已有板块相邻",
            "边缘必须匹配（道路对道路，城市对城市）",
            "可选择放置追随者或不放",
            "每回合只能派一个追随者",
        ]
    }

    /// 追随者放置
    pub fn section_followers(&self) -> Vec<&'static str> {
        vec![
            "骑士：放在城市板块",
            "盗贼：放在道路上",
            "农夫：放在农田（重要长期收益）",
            "僧侣：放在修道院",
            "每个追随者只能放在新连接的地形",
        ]
    }

    /// 道路得分
    pub fn section_road(&self) -> Vec<&'static str> {
        vec![
            "道路完成：两端闭合（城市、路口或修道院）",
            "得分：道路板块数 × 1分",
            "闭合即计分，返还追随者",
            "未完成道路终局计分：每板块1分",
            "可多人共享同一道路",
        ]
    }

    /// 城市得分
    pub fn section_city(&self) -> Vec<&'static str> {
        vec![
            "城市完成：城墙完全闭合",
            "得分：城市板块数 × 2分",
            "含盾牌符号额外+2分",
            "闭合即计分，返还追随者",
            "未完成城市终局计分：每板块1分（含盾牌）",
        ]
    }

    /// 修道院得分
    pub fn section_monastery(&self) -> Vec<&'static str> {
        vec![
            "修道院完成：周围8格全部填满",
            "得分：修道院+周围板块共9分",
            "未完成终局：修道院+已填板块数",
            "追随者放于修道院中心",
            "可被道路或城市包围",
        ]
    }

    /// 农田得分（终局）
    pub fn section_fields(&self) -> Vec<&'static str> {
        vec![
            "农夫终局计分，不中途返还",
            "每个农夫供应一个完整城市",
            "原规则：每城市3分",
            "新版规则：每城市1分（简化）",
            "多个农夫供应同一城市需平分",
        ]
    }

    /// 终局计分
    pub fn section_endgame(&self) -> Vec<&'static str> {
        vec![
            "所有板块放置完毕后",
            "先计未完成的城市/道路/修道院",
            "最后计算农田（农夫）",
            "农夫得分往往决定胜负",
            "总分最高者获胜",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "农夫是关键得分来源",
            "早期放农夫占领大片农田",
            "小心农夫供应城市数量",
            "完成城市避免被他人共享",
            "阻止对手完成大城市",
        ]
    }

    /// 变体与扩展
    pub fn section_variants(&self) -> Vec<&'static str> {
        vec![
            "河流扩展：初始河流板块替代起始板块",
            "商人与建筑师：增加新追随者类型",
            "公主与龙：动态移除追随者",
            "围城：增加围城计分",
            "超过10个官方扩展包",
        ]
    }
}

impl Rule for CarcassonneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("carcassonne")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "卡卡颂规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("游戏组件", &self.section_components()),
                ("地形类型", &self.section_terrain()),
                ("游戏流程", &self.section_flow()),
                ("追随者放置", &self.section_followers()),
                ("道路得分", &self.section_road()),
                ("城市得分", &self.section_city()),
                ("修道院得分", &self.section_monastery()),
                ("农田得分（终局）", &self.section_fields()),
                ("终局计分", &self.section_endgame()),
                ("策略要点", &self.section_strategy()),
                ("变体与扩展", &self.section_variants()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carcassonne_rules_basic() {
        let rules = CarcassonneRules::new();
        assert_eq!(rules.metadata().name, "卡卡颂规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn carcassonne_has_terrain() {
        let rules = CarcassonneRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("道路") && explanation.contains("城市"));
        assert!(explanation.contains("修道院") || explanation.contains("农田"));
    }

    #[test]
    fn carcassonne_scoring() {
        let rules = CarcassonneRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("得分"));
    }

    #[test]
    fn carcassonne_followers() {
        let rules = CarcassonneRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("追随者") || explanation.contains("骑士"));
    }

    #[test]
    fn carcassonne_has_origin() {
        let rules = CarcassonneRules::new();
        assert_eq!(rules.metadata().origin, Some("德国".to_string()));
    }
}