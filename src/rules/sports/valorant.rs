//! Valorant 电子竞技规则
//!
//! Riot Games 开发的战术射击游戏 Valorant 的完整比赛规则，
//! 包括游戏机制、角色技能、地图规则、比赛规则等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// Valorant 游戏机制规则
#[derive(Debug, Clone)]
pub struct ValorantGameMechanicsRules {
    metadata: RuleMetadata,
}

impl ValorantGameMechanicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Valorant游戏机制规则", "Valorant回合、时间、经济等基础游戏机制")
                .with_origin("Riot Games官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Valorant".into()]),
        }
    }

    /// 回合规则
    pub fn round_rules(&self) -> Vec<&'static str> {
        vec![
            "标准比赛为12回合制（先到13分获胜）",
            "加时赛采用突然死亡制（先得2分者胜）",
            "每回合时间限制为1分40秒（竞技模式）",
            "进攻方需要安放爆能器或消灭防守方获胜",
            "防守方需要阻止爆能器安放或拆除爆能器",
            "爆能器安放后有40秒爆炸倒计时",
            "回合结束后有15秒准备时间",
            "半场交换阵营（进攻/防守互换）",
        ]
    }

    /// 经济系统规则
    pub fn economy_rules(&self) -> Vec<&'static str> {
        vec![
            "初始回合每人800 creds",
            "回合胜利奖励2700 creds",
            "回合失败奖励1900 creds（连续失败递增）",
            "连续失败加成：+500/1000/1000 creds",
            "击杀奖励：200 creds/人",
            "爆能器安放奖励：300 creds（全队）",
            "爆能器拆除奖励：300 creds（全队）",
            "技能购买：免费（每回合自动充能）",
            "护甲价格：轻型护甲400 creds，重型护甲1000 creds",
            "武器价格范围：免费（经典手枪）至4700 creds（暴徒）",
            "每回合可借贷最多至900 creds（团队经济系统）",
        ]
    }

    /// 时间规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "购买阶段：30秒",
            "回合时间：1分40秒",
            "爆能器安放后：40秒爆炸倒计时",
            "技能充能时间：随角色不同",
            "重生时间：无（角色死亡不复活）",
            "半场休息：15秒",
            "技术暂停：由裁判判定",
            "战术暂停：每队2次/场，每次60秒",
        ]
    }

    /// 武器规则
    pub fn weapon_rules(&self) -> Vec<&'static str> {
        vec![
            "手枪：经典、短弩、狂怒、鬼魅、警长",
            "冲锋枪：恶棍、刺针",
            "步枪：幻象、破坏者",
            "狙击枪：元帅、间谍",
            "机枪：奥丁",
            "霰弹枪：重型猎枪、法官",
            "每种武器有独特的伤害、射速、弹匣容量",
            "武器有后坐力模式和弹道散布",
            "爆头伤害加成：多数武器爆头伤害x2-x3",
        ]
    }
}

impl Default for ValorantGameMechanicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ValorantGameMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("valorant_game_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Valorant游戏机制规则",
            &[
                ("回合规则", &self.round_rules()),
                ("经济系统", &self.economy_rules()),
                ("时间规则", &self.timing_rules()),
                ("武器规则", &self.weapon_rules()),
            ],
        )
    }
}

/// Valorant 角色规则
#[derive(Debug, Clone)]
pub struct ValorantAgentRules {
    metadata: RuleMetadata,
}

impl ValorantAgentRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Valorant角色规则", "Valorant角色（特工）分类、技能和规则")
                .with_origin("Riot Games官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Valorant".into()]),
        }
    }

    /// 角色分类
    pub fn agent_classes(&self) -> Vec<&'static str> {
        vec![
            "决斗者（Duelist）：擅长进攻和击杀",
            "先锋（Initiator）：擅长获取信息和开启战斗",
            "控场者（Controller）：擅长烟雾和地图控制",
            "哨兵（Sentinel）：擅长防守和信息获取",
            "每队可选任意角色组合",
            "同队角色可以重复选择（不限制角色唯一性）",
        ]
    }

    /// 决斗者角色
    pub fn duelists(&self) -> Vec<&'static str> {
        vec![
            "Jett：极速、风刃、上升气流、刀阵风暴",
            "Reyna：视线、吞噬、公然致盲、君临天下",
            "Phoenix：火花闪光、火热手感、炽焰手雷、再燃时刻",
            "Raze：爆破包、破片手雷、绘彩特务、以此为傲",
            "Yoru：欺诈、闪光、空间穿梭、超维入侵",
            "Neon：高速通道、继电器、高速滑行、超负荷",
            "Iso：双重接触、暗夜削弱、隐藏优势、毁灭降临",
            "决斗者特点：高机动性、强进攻能力",
        ]
    }

    /// 先锋角色
    pub fn initiators(&self) -> Vec<&'static str> {
        vec![
            "Sova：震慑箭、侦查箭、电击箭、狩猎怒吼",
            "Breach：闪光弹、闪光、聚变震荡、滚地雷",
            "Skye：引导之光、升空雷、丛林尖啸、搜索者",
            "KAY/O：闪光/驱动、点射/空包弹、碎片/残骸、NULL/CMD",
            "Fade：劫持、吞噬、潜伏、夜幕降临",
            "Gekko：冲撞、雕花、正壁、金丝翼",
            "先驱者特点：获取敌人位置、开启战斗",
        ]
    }

    /// 控场者角色
    pub fn controllers(&self) -> Vec<&'static str> {
        vec![
            "Brimstone：激励信标、振奋之星、燃烧弹、天降正义",
            "Omen：阴影烟幕、暗影信使、黑暗侵袭、噩梦升腾",
            "Viper：蛇吻、毒幕、剧毒护手、毒蛇巢穴",
            "Astra：星辰烟幕、新星震荡、万有引力、宇宙分裂",
            "Harbor：破浪、飞流直下、瞭望塔、翻江倒海",
            "Clove：命运恩赐、短暂欢愉、猛力射击、哥特生活",
            "控场者特点：烟雾遮蔽、区域控制",
        ]
    }

    /// 哨兵角色
    pub fn sentinels(&self) -> Vec<&'static str> {
        vec![
            "Sage：屏障球体、缓慢球体、治愈球体、复活",
            "Cypher：陷阱线、网络铁笼、间谍摄像机、神经窃取",
            "Killjoy：警报器、纳米蜂群、炮台、锁定",
            "Chamber：商标、头hunter、枪手、观光客",
            "Deadlock：声纳探测、重力地雷、屏障盾、歼灭",
            "Vyse：隐匿、裂岩、弧盾、荆棘领域",
            "哨兵特点：防守据点、获取信息",
        ]
    }

    /// 技能使用规则
    pub fn skill_rules(&self) -> Vec<&'static str> {
        vec![
            "小技能：每回合自动充能（可购买额外充能）",
            "终极技能：需要通过击杀、吸取光球充能",
            "终极技能充能：击杀+1点，吸取光球+3点，安放爆能器+1点",
            "终极技能所需点数：各角色不同（6-8点）",
            "技能可穿透特定表面（墙体厚度限制）",
            "技能可被队友看到标记",
            "技能伤害可对队友造成无伤害视觉效果",
        ]
    }
}

impl Default for ValorantAgentRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ValorantAgentRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("valorant_agents")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Valorant角色规则",
            &[
                ("角色分类", &self.agent_classes()),
                ("决斗者", &self.duelists()),
                ("先锋", &self.initiators()),
                ("控场者", &self.controllers()),
                ("哨兵", &self.sentinels()),
                ("技能使用", &self.skill_rules()),
            ],
        )
    }
}

/// Valorant 地图规则
#[derive(Debug, Clone)]
pub struct ValorantMapRules {
    metadata: RuleMetadata,
}

impl ValorantMapRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Valorant地图规则", "Valorant竞技地图布局和规则")
                .with_origin("Riot Games官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Valorant".into()]),
        }
    }

    /// 地图列表
    pub fn map_list(&self) -> Vec<&'static str> {
        vec![
            "Ascent（上升）- 意大利威尼斯",
            "Bind（绑定）- 摩洛哥城市",
            "Haven（避风港）- 不明地点",
            "Split（分裂）- 摩天大楼",
            "Icebox（冰箱）- 北极研究站",
            "Breeze（微风）- 百慕大三角",
            "Fracture（断裂）- 葡萄牙研究设施",
            "Pearl（珍珠）- 葡萄牙欧米茄世界",
            "Lotus（莲花）- 印度古迹",
            "Sunset（日落）- 洛杉矶",
            "Abyss（深渊）- 瑞士研究设施",
        ]
    }

    /// 地图通用规则
    pub fn common_map_rules(&self) -> Vec<&'static str> {
        vec![
            "每个地图有2-3个爆能点（A/B或A/B/C）",
            "爆能点有爆炸范围（安放后影响区域）",
            "地图有多个通道连接爆能点和出生点",
            "地图有高低差和可穿透表面",
            "特定地图有传送装置（Bind）",
            "特定地图有自动门（Fracture）",
            "特定地图有多层结构（Icebox）",
            "地图元素：箱子、窗户、通风口、高台",
        ]
    }

    /// Ascent 地图规则
    pub fn ascent_rules(&self) -> Vec<&'static str> {
        vec![
            "地点：意大利威尼斯",
            "爆能点：A、B两点",
            "特点：中庭开阔、多通道",
            "A点：位于市场区域，有多个入口",
            "B点：位于后院区域，通道较少",
            "关键位置：中庭、市场、后院、前门",
            "建议策略：控制中庭、分路进攻",
        ]
    }

    /// Haven 地图规则
    pub fn haven_rules(&self) -> Vec<&'static str> {
        vec![
            "地点：不明设施",
            "爆能点：A、B、C三点（独特）",
            "特点：三点布局、广阔地图",
            "A点：位于车库区域",
            "B点：位于中央大厅",
            "C点：位于仓库区域",
            "关键位置：车库、大厅、仓库、窗口",
            "建议策略：分散防守、快速转点",
        ]
    }

    /// Bind 地图规则
    pub fn bind_rules(&self) -> Vec<&'static str> {
        vec![
            "地点：摩洛哥城市",
            "爆能点：A、B两点",
            "特点：有传送装置（独特）",
            "A点：传送从A到B",
            "B点：传送从B到A",
            "传送规则：单向、瞬间传送",
            "关键位置：市场、钩索、浴室",
            "建议策略：利用传送进行快速转移",
        ]
    }

    /// Split 地图规则
    pub fn split_rules(&self) -> Vec<&'static str> {
        vec![
            "地点：摩天大楼",
            "爆能点：A、B两点",
            "特点：垂直布局、绳索升降",
            "A点：位于高台区域",
            "B点：位于底层区域",
            "绳索：可快速升降楼层",
            "关键位置：天台、地下室、中庭",
            "建议策略：控制垂直空间、利用绳索",
        ]
    }
}

impl Default for ValorantMapRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ValorantMapRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("valorant_maps")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Valorant地图规则",
            &[
                ("地图列表", &self.map_list()),
                ("通用规则", &self.common_map_rules()),
                ("Ascent", &self.ascent_rules()),
                ("Haven", &self.haven_rules()),
                ("Bind", &self.bind_rules()),
                ("Split", &self.split_rules()),
            ],
        )
    }
}

/// Valorant 比赛规则
#[derive(Debug, Clone)]
pub struct ValorantCompetitionRules {
    metadata: RuleMetadata,
}

impl ValorantCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Valorant比赛规则", "Valorant正式比赛赛制和规则")
                .with_origin("Riot Games官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Valorant".into()]),
        }
    }

    /// 赛制规则
    pub fn format_rules(&self) -> Vec<&'static str> {
        vec![
            "小组赛：BO1（一局定胜负）",
            "淘汰赛：BO3（三局两胜）",
            "总决赛：BO5（五局三胜）",
            "每局为标准竞技模式（先到13分）",
            "加时赛：先得2分者胜（无限加时）",
            "换边：半场交换阵营",
            "地图选择：由双方队伍选图",
            "剩余地图作为决胜图",
        ]
    }

    /// 队伍规则
    pub fn team_rules(&self) -> Vec<&'static str> {
        vec![
            "场上队员：5人",
            "替补队员：最多2人",
            "教练：可1人，负责战术暂停",
            "选手年龄：最低16岁",
            "选手认证：需注册并验证身份",
            "队伍名称：需符合命名规范",
            "赞助商logo：需经赛事方审批",
            "队伍更换：需提前申报并获得批准",
        ]
    }

    /// 暂停规则
    pub fn pause_rules(&self) -> Vec<&'static str> {
        vec![
            "战术暂停：每队每局2次，每次60秒",
            "技术暂停：由裁判判定，无限次",
            "医疗暂停：选手健康问题，最长5分钟",
            "暂停请求：仅教练或队长可申请",
            "暂停时机：回合结束后或死球状态",
            "暂停后：购买阶段重置",
            "未使用的暂停不可累积",
            "恶意使用暂停可能受到处罚",
        ]
    }

    /// 装备规则
    pub fn equipment_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛电脑：由赛事方提供统一配置",
            "显示器：240Hz+刷新率，24-27英寸",
            "鼠标：选手可自带，需通过检测",
            "键盘：选手可自带，需通过检测",
            "耳机：选手可自带，需通过检测",
            "鼠标垫：选手可自带，需通过检测",
            "禁止第三方软件：禁止任何外挂",
            "网络连接：赛事方提供稳定网络",
            "帧率要求：最低144 FPS",
        ]
    }

    /// 违规规则
    pub fn violation_rules(&self) -> Vec<&'static str> {
        vec![
            "作弊行为：永久禁赛",
            "恶意利用Bug：回合判负或比赛判负",
            "不当言论：警告、罚款或禁赛",
            "延迟比赛：警告或比赛判负",
            "信息泄露：比赛判负",
            "虚假报备：取消比赛资格",
            "队伍串通：取消双方资格",
            "轻微违规：口头警告",
            "严重违规：取消比赛资格",
        ]
    }

    /// 奖金分配
    pub fn prize_distribution(&self) -> Vec<&'static str> {
        vec![
            "冠军：40-50%总奖金池",
            "亚军：20-25%总奖金池",
            "季军：10-15%总奖金池（2队）",
            "八强：5-8%总奖金池（4队）",
            "十六强：其余奖金按比例分配",
            "MVP选手：额外奖金奖励",
            "战队积分：决定赛季排名",
            "区域资格：根据积分分配世界赛名额",
        ]
    }
}

impl Default for ValorantCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ValorantCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("valorant_competition")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Valorant比赛规则",
            &[
                ("赛制规则", &self.format_rules()),
                ("队伍规则", &self.team_rules()),
                ("暂停规则", &self.pause_rules()),
                ("装备规则", &self.equipment_rules()),
                ("违规规则", &self.violation_rules()),
                ("奖金分配", &self.prize_distribution()),
            ],
        )
    }
}

/// Valorant 裁判规则
#[derive(Debug, Clone)]
pub struct ValorantRefereeRules {
    metadata: RuleMetadata,
}

impl ValorantRefereeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Valorant裁判规则", "Valorant比赛裁判职责和判罚规则")
                .with_origin("Riot Games官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Valorant".into()]),
        }
    }

    /// 裁判职责
    pub fn referee_duties(&self) -> Vec<&'static str> {
        vec![
            "主裁判：负责比赛整体流程和重大判罚",
            "助理裁判：协助主裁判、记录比赛数据",
            "技术裁判：处理技术问题和装备检查",
            "赛前检查：验证选手身份和装备",
            "赛中监督：观察比赛、处理争议",
            "赛后报告：撰写比赛报告和判罚记录",
            "紧急决策：在争议时做出最终判决",
            "赛事协调：与赛事方和队伍沟通",
        ]
    }

    /// 判罚权限
    pub fn referee_authority(&self) -> Vec<&'static str> {
        vec![
            "口头警告：针对轻微违规",
            "回合重赛：技术问题或争议回合",
            "回合判负：利用Bug或恶意行为",
            "比赛判负：严重违规或多次违规",
            "取消资格：作弊或极端行为",
            "暂停授权：批准暂停申请",
            "装备检查：检查选手装备合规性",
            "赛场控制：管理观众和现场秩序",
        ]
    }

    /// 技术问题处理
    pub fn technical_issues(&self) -> Vec<&'static str> {
        vec![
            "游戏崩溃：回合重赛（如果影响结果）",
            "网络断线：技术暂停，等待重连",
            "装备故障：技术暂停，更换装备",
            "服务器问题：暂停比赛，等待修复",
            "选手掉线：等待最多5分钟",
            "无法重连：回合判负或比赛延期",
            "Bug利用：回合判负",
            "恶意断线：比赛判负",
        ]
    }

    /// 争议处理
    pub fn dispute_handling(&self) -> Vec<&'static str> {
        vec![
            "队伍申诉：比赛结束后24小时内提交",
            "证据提交：需提供视频或日志证据",
            "裁判审查：裁判组审查申诉材料",
            "最终裁决：裁判长做出最终决定",
            "申诉结果：维持原判或改判",
            "申诉期限：逾期不予受理",
            "申诉费用：可能需要缴纳申诉保证金",
            "恶意申诉：警告或处罚",
        ]
    }
}

impl Default for ValorantRefereeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ValorantRefereeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("valorant_referee")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Valorant裁判规则",
            &[
                ("裁判职责", &self.referee_duties()),
                ("判罚权限", &self.referee_authority()),
                ("技术问题", &self.technical_issues()),
                ("争议处理", &self.dispute_handling()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_mechanics_rules() {
        let rules = ValorantGameMechanicsRules::new();
        assert!(!rules.round_rules().is_empty());
        assert!(!rules.economy_rules().is_empty());
        assert!(!rules.timing_rules().is_empty());
        assert!(!rules.weapon_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_agent_rules() {
        let rules = ValorantAgentRules::new();
        assert!(!rules.agent_classes().is_empty());
        assert!(!rules.duelists().is_empty());
        assert!(!rules.initiators().is_empty());
        assert!(!rules.controllers().is_empty());
        assert!(!rules.sentinels().is_empty());
        assert!(!rules.skill_rules().is_empty());
    }

    #[test]
    fn test_map_rules() {
        let rules = ValorantMapRules::new();
        assert!(!rules.map_list().is_empty());
        assert!(!rules.common_map_rules().is_empty());
        assert!(!rules.ascent_rules().is_empty());
        assert!(!rules.haven_rules().is_empty());
        assert!(!rules.bind_rules().is_empty());
        assert!(!rules.split_rules().is_empty());
    }

    #[test]
    fn test_competition_rules() {
        let rules = ValorantCompetitionRules::new();
        assert!(!rules.format_rules().is_empty());
        assert!(!rules.team_rules().is_empty());
        assert!(!rules.pause_rules().is_empty());
        assert!(!rules.equipment_rules().is_empty());
        assert!(!rules.violation_rules().is_empty());
        assert!(!rules.prize_distribution().is_empty());
    }

    #[test]
    fn test_referee_rules() {
        let rules = ValorantRefereeRules::new();
        assert!(!rules.referee_duties().is_empty());
        assert!(!rules.referee_authority().is_empty());
        assert!(!rules.technical_issues().is_empty());
        assert!(!rules.dispute_handling().is_empty());
    }

    #[test]
    fn test_metadata() {
        let rules = ValorantGameMechanicsRules::new();
        assert_eq!(rules.metadata().name, "Valorant游戏机制规则");
        assert!(rules.metadata().tags.contains(&"电竞".to_string()));
    }

    #[test]
    fn test_category() {
        let rules = ValorantGameMechanicsRules::new();
        let category = rules.category();
        assert!(category.to_string().contains("valorant"));
    }

    #[test]
    fn test_validate() {
        let rules = ValorantGameMechanicsRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        let result = rules.validate(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_explain_format() {
        let rules = ValorantGameMechanicsRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("Valorant游戏机制规则"));
        assert!(explanation.contains("回合规则"));
        assert!(explanation.contains("经济系统"));
    }

    #[test]
    fn test_round_count() {
        let rules = ValorantGameMechanicsRules::new();
        assert_eq!(rules.round_rules().len(), 8);
    }

    #[test]
    fn test_economy_starting_creds() {
        let rules = ValorantGameMechanicsRules::new();
        let economy = rules.economy_rules();
        assert!(economy.iter().any(|r| r.contains("800 creds")));
    }

    #[test]
    fn test_agent_classes_count() {
        let rules = ValorantAgentRules::new();
        assert_eq!(rules.agent_classes().len(), 6);
    }

    #[test]
    fn test_map_count() {
        let rules = ValorantMapRules::new();
        assert!(rules.map_list().len() >= 10);
    }

    #[test]
    fn test_team_size() {
        let rules = ValorantCompetitionRules::new();
        let team_rules = rules.team_rules();
        assert!(team_rules.iter().any(|r| r.contains("5人")));
    }

    #[test]
    fn test_violation_cheating() {
        let rules = ValorantCompetitionRules::new();
        let violations = rules.violation_rules();
        assert!(violations.iter().any(|r| r.contains("作弊")));
    }
}