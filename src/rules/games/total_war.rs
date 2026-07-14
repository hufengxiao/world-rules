//! 全面战争系列规则（Total War Series）
//!
//! Creative Assembly 开发的战略与战术结合的游戏系列，融合回合制战略地图和即时战术战斗。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: TotalWarRules,
    name: "全面战争系列规则",
    desc: "全面战争（Total War）战略战术游戏规则",
    origin: "英国",
    tags: ["游戏", "策略", "全面战争", "战术"],
}

impl TotalWarRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "Creative Assembly 开发，2000 年首作",
            "结合回合制战略与即时战术",
            "战略地图管理帝国和军队",
            "实时战斗指挥具体战役",
            "追求军事征服或政治胜利",
        ]
    }

    /// 游戏模式
    pub fn section_modes(&self) -> Vec<&'static str> {
        vec![
            "战役模式：战略+战术结合",
            "战略回合：移动军队、管理城市",
            "战术战斗：实时指挥作战",
            "自动战斗：AI 计算战斗结果",
            "多人对战模式",
        ]
    }

    /// 战略地图
    pub fn section_campaign_map(&self) -> Vec<&'static str> {
        vec![
            "基于历史地理的战役地图",
            "省份和定居点系统",
            "军队在地图上移动",
            "地形影响移动和战斗",
            "可见性范围系统",
            "季节变化影响战略",
        ]
    }

    /// 省份管理
    pub fn section_provinces(&self) -> Vec<&'static str> {
        vec![
            "省份是经济和军事基础",
            "定居点提供税收和征兵",
            "建筑系统提升省份能力",
            "公共秩序影响稳定",
            "粮食供应军队和城市",
            "特殊资源提供加成",
        ]
    }

    /// 军事系统
    pub fn section_military(&self) -> Vec<&'static str> {
        vec![
            "将军领导军队",
            "军队由多种单位组成",
            "单位类型：步兵、骑兵、弓箭手等",
            "单位有攻击、防御、士气属性",
            "单位可升级和获得经验",
            "维护费用需要财政支持",
        ]
    }

    /// 战术战斗
    pub fn section_battles(&self) -> Vec<&'static str> {
        vec![
            "实时战术战斗是系列核心",
            "地形影响战斗优势",
            "单位编队和阵型重要",
            "士气系统：单位会溃逃",
            "疲劳影响单位战斗能力",
            "侧翼和背刺造成额外伤害",
        ]
    }

    /// 攻城战
    pub fn section_sieges(&self) -> Vec<&'static str> {
        vec![
            "定居点战斗有城墙和建筑",
            "攻城武器可破坏城墙",
            "防守方有城墙优势",
            "攻城塔和云梯可登墙",
            "攻城战需要策略和耐心",
            "定居点中心是最终目标",
        ]
    }

    /// 外交系统
    pub fn section_diplomacy(&self) -> Vec<&'static str> {
        vec![
            "可与其他派系建立关系",
            "关系等级影响互动",
            "条约：联盟、互不侵犯、贸易",
            "可宣战、要求臣服",
            "外交可靠度影响信任",
            "联姻和傀儡系统（部分作品）",
        ]
    }

    /// 经济系统
    pub fn section_economy(&self) -> Vec<&'static str> {
        vec![
            "税收是主要收入来源",
            "贸易路线提供额外收入",
            "建筑维护消耗财政",
            "军队维护费用昂贵",
            "经济平衡影响军事规模",
            "海上贸易和港口重要",
        ]
    }

    /// 科技系统
    pub fn section_technology(&self) -> Vec<&'static str> {
        vec![
            "科技树解锁新单位和建筑",
            "科技研究需要时间",
            "科技分支选择策略重要",
            "军事科技 vs 经济科技",
            "文化/宗教科技（部分作品）",
            "科技领先提供战略优势",
        ]
    }

    /// 将军和人物
    pub fn section_characters(&self) -> Vec<&'static str> {
        vec![
            "将军领导军队",
            "将军有技能树和特性",
            "将军可装备物品",
            "随从提供加成",
            "将军忠诚度重要（部分作品）",
            "将领死亡影响战局",
        ]
    }

    /// 派系特色
    pub fn section_factions(&self) -> Vec<&'static str> {
        vec![
            "每个派系有独特单位",
            "派系有独特能力和加成",
            "派系有独特起始位置",
            "派系目标影响策略",
            "派系文化影响外交关系",
            "派系领袖有独特性格",
        ]
    }

    /// 历史背景
    pub fn section_historical_periods(&self) -> Vec<&'static str> {
        vec![
            "罗马：全面战争 - 罗马时代",
            "中世纪2：全面战争 - 中世纪",
            "帝国：全面战争 - 火药时代",
            "拿破仑：全面战争 - 拿破仑战争",
            "幕府：全面战争 - 日本战国",
            "战锤：全面战争 - 奇幻世界",
        ]
    }

    /// 系列作品
    pub fn section_series(&self) -> Vec<&'static str> {
        vec![
            "Shogun: Total War (2000)",
            "Medieval: Total War (2002)",
            "Rome: Total War (2004)",
            "Medieval II: Total War (2006)",
            "Empire: Total War (2009)",
            "Napoleon: Total War (2010)",
            "Shogun 2: Total War (2011)",
            "Rome II: Total War (2013)",
            "Attila: Total War (2015)",
            "Warhammer: Total War (2016-2022)",
            "Three Kingdoms: Total War (2019)",
            "Pharaoh: Total War (2023)",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "经济基础决定军事规模",
            "外交可避免多线作战",
            "地形是战斗关键",
            "士气管理决定战斗胜负",
            "将军技能搭配单位类型",
            "科技路线选择影响中后期",
        ]
    }

    /// 战锤特色（奇幻系列）
    pub fn section_warhammer_features(&self) -> Vec<&'static str> {
        vec![
            "战锤系列加入奇幻元素",
            "魔法系统和施法单位",
            "巨兽和怪物单位",
            "英雄和传说单位",
            "混沌势力和独特机制",
            "矮人、精灵、兽人等种族",
        ]
    }

    /// 三国特色（三国系列）
    pub fn section_three_kingdoms_features(&self) -> Vec<&'static str> {
        vec![
            "三国：全面战争聚焦中国历史",
            "人物关系系统重要",
            "单挑系统（决斗）",
            "派系间谍系统",
            "声望影响外交和胜利",
            "历史模式和演义模式可选",
        ]
    }

    /// 游戏难度
    pub fn section_difficulty(&self) -> Vec<&'static str> {
        vec![
            "难度影响 AI 行为",
            "战斗难度：AI 战术能力",
            "战役难度：经济和外交加成",
            "传奇难度：永久死亡",
            "难度影响成就解锁",
            "难度可分别调整",
        ]
    }
}

impl Rule for TotalWarRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("total_war")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "全面战争系列规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("游戏模式", &self.section_modes()),
                ("战略地图", &self.section_campaign_map()),
                ("省份管理", &self.section_provinces()),
                ("军事系统", &self.section_military()),
                ("战术战斗", &self.section_battles()),
                ("攻城战", &self.section_sieges()),
                ("外交系统", &self.section_diplomacy()),
                ("经济系统", &self.section_economy()),
                ("科技系统", &self.section_technology()),
                ("将军和人物", &self.section_characters()),
                ("派系特色", &self.section_factions()),
                ("历史背景", &self.section_historical_periods()),
                ("系列作品", &self.section_series()),
                ("策略要点", &self.section_strategy()),
                ("战锤特色", &self.section_warhammer_features()),
                ("三国特色", &self.section_three_kingdoms_features()),
                ("游戏难度", &self.section_difficulty()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_war_rules_basic() {
        let rules = TotalWarRules::new();
        assert_eq!(rules.metadata().name, "全面战争系列规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn total_war_has_battles() {
        let rules = TotalWarRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("战斗"));
    }

    #[test]
    fn total_war_has_military() {
        let rules = TotalWarRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("军事") || explanation.contains("单位"));
    }

    #[test]
    fn total_war_has_series() {
        let rules = TotalWarRules::new();
        let series = rules.section_series();
        assert!(series.len() >= 10);
    }

    #[test]
    fn total_war_has_origin() {
        let rules = TotalWarRules::new();
        assert_eq!(rules.metadata().origin, Some("英国".to_string()));
    }

    #[test]
    fn total_war_historical_periods() {
        let rules = TotalWarRules::new();
        let periods = rules.section_historical_periods();
        assert!(periods.iter().any(|p| p.contains("罗马")));
    }
}
