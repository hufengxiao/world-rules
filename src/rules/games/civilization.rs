//! 文明系列规则（Civilization Series）
//!
//! Firaxis 开发的经典回合制策略游戏系列，模拟人类文明发展历程。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilizationRules,
    name: "文明系列规则",
    desc: "文明（Civilization）回合制策略游戏规则",
    origin: "美国",
    tags: ["游戏", "策略", "文明", "回合制"],
}

impl CivilizationRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "Sid Meier's Civilization 系列",
            "首作 1991 年发行",
            "玩家领导一个文明从远古到未来",
            "通过科技、文化、军事、外交等途径发展",
            "追求胜利条件之一即可获胜",
        ]
    }

    /// 游戏胜利条件
    pub fn section_victory(&self) -> Vec<&'static str> {
        vec![
            "统治胜利：征服所有其他文明",
            "科技胜利：建造飞船到达 Alpha Centauri",
            "文化胜利：吸引足够数量的国际游客",
            "宗教胜利：使所有文明信仰你的宗教",
            "外交胜利：获得足够的外交支持票",
            "分数胜利：时限结束后分数最高者",
        ]
    }

    /// 城市系统
    pub fn section_cities(&self) -> Vec<&'static str> {
        vec![
            "城市是文明的核心单位",
            "城市生产单位和建筑",
            "城市人口提供劳动力",
            "城市居民需要住房和维护",
            "城市可建造区域（城区）",
            "城市产出生产力、金钱、科技、文化",
        ]
    }

    /// 科技树
    pub fn section_technology(&self) -> Vec<&'static str> {
        vec![
            "科技解锁新单位和建筑",
            "科技研究需要科技点数",
            "尤里卡时刻加速特定科技",
            "科技树分支选择策略重要",
            "科技领先是胜利关键之一",
            "每个时代有独特科技",
        ]
    }

    /// 市政树
    pub fn section_civics(&self) -> Vec<&'static str> {
        vec![
            "市政解锁政策和政府形态",
            "市政研究需要文化点数",
            "启发时刻加速特定市政",
            "政策卡提供各种加成",
            "政府形态影响政策槽位",
            "市政决定社会制度演进",
        ]
    }

    /// 军事系统
    pub fn section_military(&self) -> Vec<&'static str> {
        vec![
            "军事单位分为远古/古典/中世纪等时代",
            "单位可获得升级和经验",
            "单位有攻击力、防御力、移动力",
            "单位需要维护费用",
            "攻城需要特定单位",
            "核武器可在游戏中后期使用",
        ]
    }

    /// 外交系统
    pub fn section_diplomacy(&self) -> Vec<&'static str> {
        vec![
            "文明间可建立关系",
            "关系等级：宣战、谴责、中立、友好、同盟",
            "贸易路线提供金钱和关系加成",
            "世界议会在游戏中后期召开",
            "可投票决定世界政策",
            "间谍可执行秘密行动",
        ]
    }

    /// 宗教系统
    pub fn section_religion(&self) -> Vec<&'static str> {
        vec![
            "创建宗教提供独特加成",
            "宗教可传播到其他城市",
            "宗教单位：使徒、上师、审判官",
            "宗教战斗决定信仰影响力",
            "宗教建筑和圣地提供信仰",
            "宗教是文化胜利途径之一",
        ]
    }

    /// 文化系统
    pub fn section_culture(&self) -> Vec<&'static str> {
        vec![
            "文化产出巨作和地标",
            "考古学家可挖掘文物",
            "艺术家/音乐家/作家可创作巨作",
            "博物馆提供文物和巨作槽位",
            "主题加成需配套收藏",
            "国际游客数量决定文化胜利",
        ]
    }

    /// 时代系统
    pub fn section_eras(&self) -> Vec<&'static str> {
        vec![
            "远古时代 → 古典时代 → 中世纪",
            "→ 文艺复兴 → 工业 → 现代 → 原子时代",
            "→ 信息时代 → 未来时代",
            "每个时代解锁新单位和建筑",
            "时代分数影响黄金时代/黑暗时代",
            "黄金时代提供额外加成",
        ]
    }

    /// 资源系统
    pub fn section_resources(&self) -> Vec<&'static str> {
        vec![
            "战略资源：铁、马、火药、石油、铝等",
            "奢侈资源：葡萄酒、丝绸、香料等",
            "加成资源：小麦、牛、羊等",
            "资源需要相应科技才能开发",
            "资源提供各种加成和幸福度",
            "战略资源用于建造特定单位",
        ]
    }

    /// 地形与移动
    pub fn section_terrain(&self) -> Vec<&'static str> {
        vec![
            "六边形地图格子",
            "地形：平原、草原、沙漠、冻土等",
            "地貌：森林、雨林、沼泽、洪水平原",
            "河流提供水源和防御加成",
            "山脉和丘陵影响移动",
            "海洋和大陆架有独特资源",
        ]
    }

    /// 文明特色
    pub fn section_civilizations(&self) -> Vec<&'static str> {
        vec![
            "每个文明有独特能力",
            "每个文明有独特单位",
            "每个文明有独特建筑/区域",
            "领袖有独特议程和加成",
            "AI 领袖有独特性格",
            "文明组合影响游戏策略",
        ]
    }

    /// 游戏设置
    pub fn section_game_setup(&self) -> Vec<&'static str> {
        vec![
            "地图大小：小/中/大/巨大",
            "难度级别：定居者→神级",
            "游戏速度：马拉松/史诗/标准/快速",
            "胜利条件可自定义开关",
            "可设置起始时代",
            "可禁用特定游戏元素",
        ]
    }

    /// 多人游戏
    pub fn section_multiplayer(&self) -> Vec<&'static str> {
        vec![
            "支持多人对战",
            "异步回合可选",
            "可保存多人游戏进度",
            "联盟和团队模式",
            "多人游戏选项丰富",
            "AI 可填充玩家席位",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "平衡发展多条路线",
            "选址影响城市产出",
            "科技/市政路线选择关键",
            "注意文明议程和外交",
            "黄金时代时机重要",
            "早期扩张需平衡发展",
        ]
    }

    /// 系列作品
    pub fn section_series(&self) -> Vec<&'static str> {
        vec![
            "Civilization I (1991)",
            "Civilization II (1996)",
            "Civilization III (2001)",
            "Civilization IV (2005)",
            "Civilization V (2010)",
            "Civilization VI (2016)",
        ]
    }
}

impl Rule for CivilizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("civilization")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "文明系列规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("胜利条件", &self.section_victory()),
                ("城市系统", &self.section_cities()),
                ("科技树", &self.section_technology()),
                ("市政树", &self.section_civics()),
                ("军事系统", &self.section_military()),
                ("外交系统", &self.section_diplomacy()),
                ("宗教系统", &self.section_religion()),
                ("文化系统", &self.section_culture()),
                ("时代系统", &self.section_eras()),
                ("资源系统", &self.section_resources()),
                ("地形与移动", &self.section_terrain()),
                ("文明特色", &self.section_civilizations()),
                ("游戏设置", &self.section_game_setup()),
                ("多人游戏", &self.section_multiplayer()),
                ("策略要点", &self.section_strategy()),
                ("系列作品", &self.section_series()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civilization_rules_basic() {
        let rules = CivilizationRules::new();
        assert_eq!(rules.metadata().name, "文明系列规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn civilization_has_victory() {
        let rules = CivilizationRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("胜利"));
    }

    #[test]
    fn civilization_has_technology() {
        let rules = CivilizationRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("科技"));
    }

    #[test]
    fn civilization_has_cities() {
        let rules = CivilizationRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("城市"));
    }

    #[test]
    fn civilization_has_origin() {
        let rules = CivilizationRules::new();
        assert_eq!(rules.metadata().origin, Some("美国".to_string()));
    }

    #[test]
    fn civilization_has_series() {
        let rules = CivilizationRules::new();
        let series = rules.section_series();
        assert!(series.len() >= 6);
    }
}