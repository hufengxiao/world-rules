//! PUBG (绝地求生) 电子竞技规则
//!
//! Krafton 开发的大逃杀游戏 PUBG: Battlegrounds 的完整比赛规则，
//! 包括游戏机制、地图规则、比赛规则、积分系统等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// PUBG 游戏机制规则
#[derive(Debug, Clone)]
pub struct PubgGameMechanicsRules {
    metadata: RuleMetadata,
}

impl PubgGameMechanicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("PUBG游戏机制规则", "PUBG回合、缩圈、生存等基础游戏机制")
                .with_origin("Krafton官方规则")
                .with_tags(vec!["电竞".into(), "大逃杀".into(), "PUBG".into()]),
        }
    }

    /// 比赛基本规则
    pub fn match_rules(&self) -> Vec<&'static str> {
        vec![
            "标准比赛：100名玩家（或队伍）参与",
            "队伍模式：单人、双人、四人小队",
            "胜利条件：成为最后存活的一队/一人",
            "比赛开始：所有玩家从飞机跳伞",
            "跳伞阶段：自由选择降落地点",
            "安全区域：随时间逐渐缩小",
            "蓝圈：致命电圈，接触即受伤害",
            "地图元素：建筑物、车辆、武器、装备",
        ]
    }

    /// 缩圈规则
    pub fn zone_rules(&self) -> Vec<&'static str> {
        vec![
            "白圈：下一个安全区域",
            "蓝圈：当前安全区域边界",
            "红圈：即将受到轰炸的区域",
            "缩圈速度：随阶段变化",
            "圈外伤害：每秒递增伤害",
            "缩圈阶段：通常7-8个阶段",
            "决赛圈：最后阶段极小区域",
            "缩圈时间：每阶段约2-5分钟",
        ]
    }

    /// 装备规则
    pub fn equipment_rules(&self) -> Vec<&'static str> {
        vec![
            "武器分类：手枪、冲锋枪、步枪、狙击枪、霰弹枪、机枪",
            "护甲等级：1级、2级、3级（防弹衣和头盔）",
            "背包等级：1级、2级、3级（容量递增）",
            "配件系统：瞄准镜、消音器、握把、弹匣等",
            "药品道具：急救包、止痛药、能量饮料、绷带",
            "投掷道具：手雷、烟雾弹、闪光弹、燃烧瓶",
            "近战武器：平底锅、镰刀、砍刀（可抵挡子弹）",
            "载具：轿车、吉普、摩托、船、直升机",
        ]
    }

    /// 毒圈伤害规则
    pub fn damage_rules(&self) -> Vec<&'static str> {
        vec![
            "圈外伤害：初始0.4 DPS，递增至11 DPS",
            "阶段1伤害：0.4 DPS",
            "阶段2伤害：0.6 DPS",
            "阶段3伤害：0.8-1.0 DPS",
            "阶段4伤害：1.2-1.5 DPS",
            "阶段5伤害：2.0-3.0 DPS",
            "阶段6伤害：4.0-5.0 DPS",
            "决赛圈伤害：11.0 DPS",
        ]
    }

    /// 时间规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛总时长：通常30-35分钟",
            "跳伞阶段：约2分钟",
            "第一阶段：约4分钟",
            "决赛圈：比赛最后5分钟",
            "击杀结算：实时显示",
            "观战模式：死亡后可观看队友",
            "比赛结束：只剩一队时结束",
            "重连规则：断线后可重连（有限制）",
        ]
    }
}

impl Default for PubgGameMechanicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PubgGameMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pubg_game_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "PUBG游戏机制规则",
            &[
                ("比赛规则", &self.match_rules()),
                ("缩圈规则", &self.zone_rules()),
                ("装备规则", &self.equipment_rules()),
                ("伤害规则", &self.damage_rules()),
                ("时间规则", &self.timing_rules()),
            ],
        )
    }
}

/// PUBG 地图规则
#[derive(Debug, Clone)]
pub struct PubgMapRules {
    metadata: RuleMetadata,
}

impl PubgMapRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("PUBG地图规则", "PUBG竞技地图布局和规则")
                .with_origin("Krafton官方规则")
                .with_tags(vec!["电竞".into(), "大逃杀".into(), "PUBG".into()]),
        }
    }

    /// 地图列表
    pub fn map_list(&self) -> Vec<&'static str> {
        vec![
            "艾伦格（Erangel）- 8x8 km 经典地图",
            "米拉玛（Miramar）- 8x8 km 沙漠地图",
            "萨诺（Sanhok）- 4x4 km 丛林地图",
            "雪地（Vikendi）- 6x6 km 雪地地图",
            "卡拉金（Karakin）- 2x2 km 小地图",
            "泰戈（Taego）- 8x8 km 韩国风格地图",
            "帝斯顿（Deston）- 8x8 km 城市地图",
        ]
    }

    /// 艾伦格地图规则
    pub fn erangel_rules(&self) -> Vec<&'static str> {
        vec![
            "地图大小：8x8 km",
            "地形特点：岛屿、城市、山地、农田",
            "主要城市：P城、R城、军事基地、核电站",
            "资源分布：中等密度",
            "载具数量：较多",
            "缩圈时间：标准",
            "适合战术：防守和进攻均衡",
            "电竞使用频率：高",
        ]
    }

    /// 米拉玛地图规则
    pub fn miramar_rules(&self) -> Vec<&'static str> {
        vec![
            "地图大小：8x8 km",
            "地形特点：沙漠、城市、山地",
            "主要城市：Pecado、San Martin、Hacienda",
            "资源分布：分散，需驾车转移",
            "载具数量：最多",
            "缩圈时间：较长",
            "适合战术：载具战、远距离狙击",
            "电竞使用频率：中等",
        ]
    }

    /// 萨诺地图规则
    pub fn sanhok_rules(&self) -> Vec<&'static str> {
        vec![
            "地图大小：4x4 km",
            "地形特点：丛林、河流、山地",
            "主要区域：训练营、天堂度假村、废墟",
            "资源分布：密集",
            "载具数量：较少",
            "缩圈时间：快速",
            "适合战术：近战、激进打法",
            "电竞使用频率：中等",
        ]
    }

    /// 地图通用规则
    pub fn common_map_rules(&self) -> Vec<&'static str> {
        vec![
            "跳伞高度：最高800米",
            "滑翔速度：最快234 km/h",
            "载具刷新：随机位置",
            "船只：水边固定位置",
            "空投：比赛中期随机位置",
            "信号枪：召唤超级空投",
            "轰炸区：随机红圈轰炸",
            "安全区缩小：逐渐收缩",
        ]
    }
}

impl Default for PubgMapRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PubgMapRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pubg_maps")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "PUBG地图规则",
            &[
                ("地图列表", &self.map_list()),
                ("艾伦格", &self.erangel_rules()),
                ("米拉玛", &self.miramar_rules()),
                ("萨诺", &self.sanhok_rules()),
                ("通用规则", &self.common_map_rules()),
            ],
        )
    }
}

/// PUBG 比赛规则
#[derive(Debug, Clone)]
pub struct PubgCompetitionRules {
    metadata: RuleMetadata,
}

impl PubgCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("PUBG比赛规则", "PUBG正式比赛赛制和规则")
                .with_origin("Krafton官方规则")
                .with_tags(vec!["电竞".into(), "大逃杀".into(), "PUBG".into()]),
        }
    }

    /// 赛制规则
    pub fn format_rules(&self) -> Vec<&'static str> {
        vec![
            "小组赛：多轮积分制",
            "淘汰赛：多轮积分制",
            "总决赛：多轮积分制",
            "每场比赛：16支队伍（64名选手）",
            "比赛轮数：通常6-8轮/天",
            "地图选择：轮流或指定地图池",
            "积分制：生存排名+击杀分",
            "平局处理：比较击杀分",
        ]
    }

    /// 积分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "第1名：15分",
            "第2名：12分",
            "第3名：10分",
            "第4名：8分",
            "第5名：6分",
            "第6名：4分",
            "第7-8名：3分",
            "第9-12名：2分",
            "第13-16名：1分",
            "击杀分：每击杀1分（上限7分）",
        ]
    }

    /// 队伍规则
    pub fn team_rules(&self) -> Vec<&'static str> {
        vec![
            "队伍人数：4人",
            "替补选手：最多2人",
            "教练：可1人",
            "选手年龄：最低16岁",
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
            "断线重连：限时3分钟",
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
            "禁止第三方软件",
            "帧率要求：最低144 FPS",
            "网络连接：赛事方提供",
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

impl Default for PubgCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PubgCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pubg_competition")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "PUBG比赛规则",
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

/// PUBG 战术规则
#[derive(Debug, Clone)]
pub struct PubgTacticsRules {
    metadata: RuleMetadata,
}

impl PubgTacticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("PUBG战术规则", "PUBG常见战术和策略")
                .with_origin("职业战队经验")
                .with_tags(vec!["电竞".into(), "大逃杀".into(), "PUBG".into()]),
        }
    }

    /// 跳伞战术
    pub fn parachute_tactics(&self) -> Vec<&'static str> {
        vec![
            "落点选择：资源丰富、载具充足",
            "垂直跳伞：最快到达目标",
            "滑翔跳伞：最远距离",
            "团队跳伞：分散但可控范围",
            "避战跳伞：远离航线",
            "抢点跳伞：热门区域",
            "观察跳伞：观察敌人落点",
            "安全跳伞：边缘区域",
        ]
    }

    /// 进攻战术
    pub fn offensive_tactics(&self) -> Vec<&'static str> {
        vec![
            "突袭战术：快速接近敌人",
            "包夹战术：多角度进攻",
            "烟雾掩护：使用烟雾弹",
            "载具突袭：开车冲点",
            "手雷轰炸：密集投掷",
            "狙击压制：远距离压制",
            "破门战术：突破建筑",
            "决赛圈进攻：激进打法",
        ]
    }

    /// 防守战术
    pub fn defensive_tactics(&self) -> Vec<&'static str> {
        vec![
            "阵地防守：占领制高点",
            "建筑防守：占领房屋",
            "载具防守：使用载具作掩体",
            "烟雾防守：制造烟雾墙",
            "决赛圈防守：卡点防守",
            "分散防守：多点防守",
            "移动防守：缓慢推进",
            "伏地战术：隐藏位置",
        ]
    }

    /// 决赛圈战术
    pub fn endgame_tactics(&self) -> Vec<&'static str> {
        vec![
            "位置选择：圈中心有利",
            "掩体利用：寻找掩体",
            "投掷道具：决赛圈关键",
            "载具利用：载具作掩体",
            "分散站位：避免团灭",
            "信息收集：观察敌人位置",
            "时机选择：等待他人交战",
            "最后一击：决赛圈击杀",
        ]
    }
}

impl Default for PubgTacticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PubgTacticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pubg_tactics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "PUBG战术规则",
            &[
                ("跳伞战术", &self.parachute_tactics()),
                ("进攻战术", &self.offensive_tactics()),
                ("防守战术", &self.defensive_tactics()),
                ("决赛圈战术", &self.endgame_tactics()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_mechanics_rules() {
        let rules = PubgGameMechanicsRules::new();
        assert!(!rules.match_rules().is_empty());
        assert!(!rules.zone_rules().is_empty());
        assert!(!rules.equipment_rules().is_empty());
        assert!(!rules.damage_rules().is_empty());
        assert!(!rules.timing_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_map_rules() {
        let rules = PubgMapRules::new();
        assert!(!rules.map_list().is_empty());
        assert!(!rules.erangel_rules().is_empty());
        assert!(!rules.miramar_rules().is_empty());
        assert!(!rules.sanhok_rules().is_empty());
        assert!(!rules.common_map_rules().is_empty());
    }

    #[test]
    fn test_competition_rules() {
        let rules = PubgCompetitionRules::new();
        assert!(!rules.format_rules().is_empty());
        assert!(!rules.scoring_rules().is_empty());
        assert!(!rules.team_rules().is_empty());
        assert!(!rules.pause_rules().is_empty());
        assert!(!rules.equipment_rules().is_empty());
        assert!(!rules.violation_rules().is_empty());
    }

    #[test]
    fn test_tactics_rules() {
        let rules = PubgTacticsRules::new();
        assert!(!rules.parachute_tactics().is_empty());
        assert!(!rules.offensive_tactics().is_empty());
        assert!(!rules.defensive_tactics().is_empty());
        assert!(!rules.endgame_tactics().is_empty());
    }

    #[test]
    fn test_metadata() {
        let rules = PubgGameMechanicsRules::new();
        assert_eq!(rules.metadata().name, "PUBG游戏机制规则");
        assert!(rules.metadata().tags.contains(&"电竞".to_string()));
    }

    #[test]
    fn test_category() {
        let rules = PubgGameMechanicsRules::new();
        let category = rules.category();
        assert!(category.to_string().contains("pubg"));
    }

    #[test]
    fn test_validate() {
        let rules = PubgGameMechanicsRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        let result = rules.validate(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_explain_format() {
        let rules = PubgGameMechanicsRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("PUBG游戏机制规则"));
        assert!(explanation.contains("比赛规则"));
        assert!(explanation.contains("缩圈规则"));
    }

    #[test]
    fn test_zone_damage() {
        let rules = PubgGameMechanicsRules::new();
        let damage = rules.damage_rules();
        assert!(damage.iter().any(|r| r.contains("DPS")));
    }

    #[test]
    fn test_map_count() {
        let rules = PubgMapRules::new();
        assert!(rules.map_list().len() >= 6);
    }

    #[test]
    fn test_scoring_first_place() {
        let rules = PubgCompetitionRules::new();
        let scoring = rules.scoring_rules();
        assert!(scoring
            .iter()
            .any(|r| r.contains("第1名") && r.contains("15分")));
    }

    #[test]
    fn test_team_size() {
        let rules = PubgCompetitionRules::new();
        let team_rules = rules.team_rules();
        assert!(team_rules.iter().any(|r| r.contains("4人")));
    }

    #[test]
    fn test_player_count() {
        let rules = PubgGameMechanicsRules::new();
        let match_rules = rules.match_rules();
        assert!(match_rules.iter().any(|r| r.contains("100")));
    }
}
