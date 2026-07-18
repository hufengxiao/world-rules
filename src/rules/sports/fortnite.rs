//! Fortnite 电子竞技规则
//!
//! Epic Games 开发的大逃杀游戏 Fortnite 的完整比赛规则，
//! 包括游戏机制、建造规则、比赛规则、积分系统等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// Fortnite 游戏机制规则
#[derive(Debug, Clone)]
pub struct FortniteGameMechanicsRules {
    metadata: RuleMetadata,
}

impl FortniteGameMechanicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "Fortnite游戏机制规则",
                "Fortnite回合、建造、生存等基础游戏机制",
            )
            .with_origin("Epic Games官方规则")
            .with_tags(vec!["电竞".into(), "大逃杀".into(), "Fortnite".into()]),
        }
    }

    /// 比赛基本规则
    pub fn match_rules(&self) -> Vec<&'static str> {
        vec![
            "标准比赛：100名玩家参与",
            "游戏模式：单人、双人、三人、四人小队",
            "胜利条件：成为最后存活的一队/一人",
            "比赛开始：所有玩家从战斗巴士跳伞",
            "跳伞阶段：自由选择降落地点",
            "安全区域：随暴风雨圈缩小",
            "暴风雨圈：致命区域，接触即受伤害",
            "地图元素：建筑物、宝箱、武器、材料",
        ]
    }

    /// 建造规则
    pub fn building_rules(&self) -> Vec<&'static str> {
        vec![
            "建筑材料：木材、石材、金属",
            "建筑类型：墙、楼梯、地板、屋顶",
            "材料获取：砍伐树木、挖掘石头、破坏建筑",
            "建筑速度：极快（可秒建）",
            "建筑血量：因材料而异",
            "建筑修复：可修复受损建筑",
            "陷阱建筑：可在建筑上放置陷阱",
            "编辑建筑：可编辑已建造结构",
        ]
    }

    /// 武器规则
    pub fn weapon_rules(&self) -> Vec<&'static str> {
        vec![
            "武器稀有度：普通、罕见、稀有、史诗、传奇",
            "武器类型：突击步枪、霰弹枪、狙击枪、手枪、冲锋枪",
            "弹药类型：轻型弹药、中型弹药、重型弹药、霰弹",
            "武器伤害：因武器和稀有度而异",
            "爆头伤害：基础伤害的1.5-2倍",
            "武器耐久：无耐久度限制",
            "武器配件：可改装（部分模式）",
            "武器刷新：地图随机位置",
        ]
    }

    /// 物品规则
    pub fn item_rules(&self) -> Vec<&'static str> {
        vec![
            "治疗物品：绷带、急救包、护盾药水、大护盾药水",
            "投掷物品：手雷、黏性手雷、远程炸弹、臭气弹",
            "移动物品：裂隙、跃跃板、抓钩",
            "陷阱物品：伤害陷阱、冰冻陷阱",
            "特殊物品：护盾药水、史莱姆",
            "载具：汽车、卡车、直升机、船",
            "宝箱：固定和移动宝箱",
            "空投：随机空投补给",
        ]
    }

    /// 暴风雨圈规则
    pub fn storm_rules(&self) -> Vec<&'static str> {
        vec![
            "暴风雨圈：比赛开始后逐渐缩小",
            "圈外伤害：每秒1-10点伤害（随阶段递增）",
            "安全区：白色圆圈标记",
            "暴风雨移动：逐渐移动到新位置",
            "缩圈阶段：通常7-8个阶段",
            "决赛圈：最后阶段极小区域",
            "圈外警告：屏幕边缘变紫色",
            "移动速度：圈外移速减慢",
        ]
    }

    /// 时间规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛总时长：通常20-25分钟",
            "跳伞阶段：约1分钟",
            "第一阶段：约3分钟",
            "决赛圈：比赛最后5分钟",
            "击杀结算：实时显示",
            "观战模式：死亡后可观看队友",
            "比赛结束：只剩一队时结束",
            "重连规则：断线后可重连（有限制）",
        ]
    }
}

impl Default for FortniteGameMechanicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FortniteGameMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fortnite_game_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Fortnite游戏机制规则",
            &[
                ("比赛规则", &self.match_rules()),
                ("建造规则", &self.building_rules()),
                ("武器规则", &self.weapon_rules()),
                ("物品规则", &self.item_rules()),
                ("暴风雨圈", &self.storm_rules()),
                ("时间规则", &self.timing_rules()),
            ],
        )
    }
}

/// Fortnite 地图规则
#[derive(Debug, Clone)]
pub struct FortniteMapRules {
    metadata: RuleMetadata,
}

impl FortniteMapRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Fortnite地图规则", "Fortnite竞技地图布局和规则")
                .with_origin("Epic Games官方规则")
                .with_tags(vec!["电竞".into(), "大逃杀".into(), "Fortnite".into()]),
        }
    }

    /// 地图特点
    pub fn map_features(&self) -> Vec<&'static str> {
        vec![
            "地图大小：约3x3 km",
            "地形多样：城市、森林、山地、沙漠、雪地",
            "动态地图：随赛季更新变化",
            "移动宝箱：可移动位置",
            "裂隙位置：随机刷新",
            "NPC角色：可交互NPC",
            "命名区域：各区域有独特名称",
            "特殊地点：隐藏宝箱地点",
        ]
    }

    /// 主要区域
    pub fn main_areas(&self) -> Vec<&'static str> {
        vec![
            "据点区域：高资源、高竞争",
            "城市区域：密集建筑、丰富宝箱",
            "郊区区域：中等资源、适中竞争",
            "野外区域：分散资源、低竞争",
            "特殊区域：独特机制（如裂隙）",
            "隐藏区域：秘密宝箱位置",
            "水域区域：可游泳、可使用船只",
            "空中区域：可使用直升机",
        ]
    }

    /// 宝箱刷新规则
    pub fn chest_rules(&self) -> Vec<&'static str> {
        vec![
            "固定宝箱：建筑物内的固定位置",
            "移动宝箱：随机位置刷新",
            "宝箱类型：普通、稀有、史诗、传奇",
            "宝箱内容：武器、弹药、护盾",
            "宝箱刷新：比赛开始时随机",
            "宝箱争夺：热门区域宝箱竞争激烈",
            "宝箱声音：可听到宝箱开启声",
            "宝箱可见：部分宝箱可见性",
        ]
    }

    /// 裂隙规则
    pub fn rift_rules(&self) -> Vec<&'static str> {
        vec![
            "裂隙类型：固定裂隙、道具裂隙",
            "裂隙效果：传送至空中",
            "裂隙位置：地图固定位置",
            "裂隙使用：每队限次使用",
            "裂隙战术：快速转移",
            "裂隙消失：使用后消失",
            "裂隙刷新：比赛中期刷新",
            "裂隙声音：可听到裂隙声",
        ]
    }
}

impl Default for FortniteMapRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FortniteMapRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fortnite_maps")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Fortnite地图规则",
            &[
                ("地图特点", &self.map_features()),
                ("主要区域", &self.main_areas()),
                ("宝箱规则", &self.chest_rules()),
                ("裂隙规则", &self.rift_rules()),
            ],
        )
    }
}

/// Fortnite 比赛规则
#[derive(Debug, Clone)]
pub struct FortniteCompetitionRules {
    metadata: RuleMetadata,
}

impl FortniteCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Fortnite比赛规则", "Fortnite正式比赛赛制和规则")
                .with_origin("Epic Games官方规则")
                .with_tags(vec!["电竞".into(), "大逃杀".into(), "Fortnite".into()]),
        }
    }

    /// 赛制规则
    pub fn format_rules(&self) -> Vec<&'static str> {
        vec![
            "小组赛：多轮积分制",
            "淘汰赛：多轮积分制",
            "总决赛：多轮积分制",
            "每场比赛：100名玩家（单人）或25队（四人）",
            "比赛轮数：通常6轮",
            "地图选择：当前赛季地图",
            "积分制：生存排名+击杀分",
            "平局处理：比较击杀分",
        ]
    }

    /// 积分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "胜利皇冠：10分（第一名）",
            "第2名：7分",
            "第3名：5分",
            "第4名：5分",
            "第5-10名：3分",
            "第11-25名：2分",
            "第26-50名：1分",
            "淘汰分：每击杀1分",
            "淘汰分上限：无限制",
        ]
    }

    /// 队伍规则
    pub fn team_rules(&self) -> Vec<&'static str> {
        vec![
            "队伍人数：4人（四人模式）",
            "替补选手：最多2人",
            "教练：可1人",
            "选手年龄：最低13岁",
            "选手认证：需注册并验证身份",
            "队伍名称：需符合命名规范",
            "赞助商logo：需经审批",
            "队伍变更：需提前申报",
        ]
    }

    /// 暂停规则
    pub fn pause_rules(&self) -> Vec<&'static str> {
        vec![
            "技术暂停：由裁判判定",
            "医疗暂停：选手健康问题",
            "暂停请求：仅队长可申请",
            "暂停时长：最长5分钟",
            "断线重连：限时5分钟",
            "无法重连：队伍继续比赛",
            "恶意断线：可能受处罚",
            "服务器问题：统一暂停",
        ]
    }

    /// 装备规则
    pub fn equipment_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛电脑：由赛事方提供",
            "显示器：240Hz+刷新率",
            "鼠标：选手可自带",
            "键盘：选手可自带",
            "耳机：选手可自带",
            "控制器：可使用控制器",
            "禁止第三方软件",
            "帧率要求：最低144 FPS",
        ]
    }

    /// 违规规则
    pub fn violation_rules(&self) -> Vec<&'static str> {
        vec![
            "作弊行为：永久禁赛",
            "恶意利用Bug：取消成绩",
            "不当言论：警告或禁赛",
            "延迟比赛：警告",
            "信息泄露：取消比赛资格",
            "虚假报备：取消资格",
            "队伍串通：取消双方资格",
            "轻微违规：口头警告",
        ]
    }
}

impl Default for FortniteCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FortniteCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fortnite_competition")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Fortnite比赛规则",
            &[
                ("赛制规则", &self.format_rules()),
                ("积分规则", &self.scoring_rules()),
                ("队伍规则", &self.team_rules()),
                ("暂停规则", &self.pause_rules()),
                ("装备规则", &self.equipment_rules()),
                ("违规规则", &self.violation_rules()),
            ],
        )
    }
}

/// Fortnite 建造战术规则
#[derive(Debug, Clone)]
pub struct FortniteBuildingTacticsRules {
    metadata: RuleMetadata,
}

impl FortniteBuildingTacticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Fortnite建造战术规则", "Fortnite建造战术和策略")
                .with_origin("职业战队经验")
                .with_tags(vec!["电竞".into(), "大逃杀".into(), "Fortnite".into()]),
        }
    }

    /// 防守建造
    pub fn defensive_building(&self) -> Vec<&'static str> {
        vec![
            "掩体建造：快速建造墙壁",
            "塔楼建造：高地优势",
            "盒子建造：四面围护",
            "斜板防御：防御高处敌人",
            "地板建造：防止地下攻击",
            "修复建造：修复受损建筑",
            "陷阱放置：建造内放陷阱",
            "多层建造：多层结构",
        ]
    }

    /// 进攻建造
    pub fn offensive_building(&self) -> Vec<&'static str> {
        vec![
            "推进建造：向前推进",
            "高地建造：快速抢占高地",
            "包围建造：包围敌人",
            "冲板建造：向上冲刺",
            "破墙建造：破坏敌人建筑",
            "连建：连续建造",
            "双板：双斜板进攻",
            "顶板：向头顶建造",
        ]
    }

    /// 高级技巧
    pub fn advanced_techniques(&self) -> Vec<&'static str> {
        vec![
            "90度建造：快速向上",
            "隧道建造：快速通道",
            "侧跳建造：侧向建造",
            "空中建造：跳伞时建造",
            "下落建造：下落时建造",
            "编辑技巧：快速编辑建筑",
            "陷阱战术：诱敌入陷阱",
            "躲藏建造：隐藏位置",
        ]
    }

    /// 材料管理
    pub fn material_management(&self) -> Vec<&'static str> {
        vec![
            "材料收集：开局快速收集",
            "材料分配：团队共享",
            "材料保存：决赛圈关键",
            "木材优先：建造最快",
            "金属备选：最坚固",
            "材料上限：500个/类型",
            "材料获取速度：因工具而异",
            "材料争夺：资源点竞争",
        ]
    }
}

impl Default for FortniteBuildingTacticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FortniteBuildingTacticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fortnite_building_tactics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Fortnite建造战术规则",
            &[
                ("防守建造", &self.defensive_building()),
                ("进攻建造", &self.offensive_building()),
                ("高级技巧", &self.advanced_techniques()),
                ("材料管理", &self.material_management()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_mechanics_rules() {
        let rules = FortniteGameMechanicsRules::new();
        assert!(!rules.match_rules().is_empty());
        assert!(!rules.building_rules().is_empty());
        assert!(!rules.weapon_rules().is_empty());
        assert!(!rules.item_rules().is_empty());
        assert!(!rules.storm_rules().is_empty());
        assert!(!rules.timing_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_map_rules() {
        let rules = FortniteMapRules::new();
        assert!(!rules.map_features().is_empty());
        assert!(!rules.main_areas().is_empty());
        assert!(!rules.chest_rules().is_empty());
        assert!(!rules.rift_rules().is_empty());
    }

    #[test]
    fn test_competition_rules() {
        let rules = FortniteCompetitionRules::new();
        assert!(!rules.format_rules().is_empty());
        assert!(!rules.scoring_rules().is_empty());
        assert!(!rules.team_rules().is_empty());
        assert!(!rules.pause_rules().is_empty());
        assert!(!rules.equipment_rules().is_empty());
        assert!(!rules.violation_rules().is_empty());
    }

    #[test]
    fn test_building_tactics_rules() {
        let rules = FortniteBuildingTacticsRules::new();
        assert!(!rules.defensive_building().is_empty());
        assert!(!rules.offensive_building().is_empty());
        assert!(!rules.advanced_techniques().is_empty());
        assert!(!rules.material_management().is_empty());
    }

    #[test]
    fn test_metadata() {
        let rules = FortniteGameMechanicsRules::new();
        assert_eq!(rules.metadata().name, "Fortnite游戏机制规则");
        assert!(rules.metadata().tags.contains(&"电竞".to_string()));
    }

    #[test]
    fn test_category() {
        let rules = FortniteGameMechanicsRules::new();
        let category = rules.category();
        assert!(category.to_string().contains("fortnite"));
    }

    #[test]
    fn test_validate() {
        let rules = FortniteGameMechanicsRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        let result = rules.validate(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_explain_format() {
        let rules = FortniteGameMechanicsRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("Fortnite游戏机制规则"));
        assert!(explanation.contains("比赛规则"));
        assert!(explanation.contains("建造规则"));
    }

    #[test]
    fn test_building_materials() {
        let rules = FortniteGameMechanicsRules::new();
        let building = rules.building_rules();
        assert!(building.iter().any(|r| r.contains("木材")));
    }

    #[test]
    fn test_weapon_rarity() {
        let rules = FortniteGameMechanicsRules::new();
        let weapons = rules.weapon_rules();
        assert!(weapons.iter().any(|r| r.contains("传奇")));
    }

    #[test]
    fn test_scoring_victory_crown() {
        let rules = FortniteCompetitionRules::new();
        let scoring = rules.scoring_rules();
        assert!(scoring.iter().any(|r| r.contains("胜利皇冠")));
    }

    #[test]
    fn test_team_size() {
        let rules = FortniteCompetitionRules::new();
        let team_rules = rules.team_rules();
        assert!(team_rules.iter().any(|r| r.contains("4人")));
    }

    #[test]
    fn test_player_count() {
        let rules = FortniteGameMechanicsRules::new();
        let match_rules = rules.match_rules();
        assert!(match_rules.iter().any(|r| r.contains("100")));
    }
}