//! Overwatch 电子竞技规则
//!
//! 暴雪娱乐开发的团队射击游戏 Overwatch 的完整比赛规则，
//! 包括英雄角色、游戏模式、地图规则、比赛规则等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// Overwatch 游戏机制规则
#[derive(Debug, Clone)]
pub struct OverwatchGameMechanicsRules {
    metadata: RuleMetadata,
}

impl OverwatchGameMechanicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "Overwatch游戏机制规则",
                "Overwatch回合、时间、重生等基础游戏机制",
            )
            .with_origin("Blizzard Entertainment官方规则")
            .with_tags(vec!["电竞".into(), "FPS".into(), "Overwatch".into()]),
        }
    }

    /// 比赛回合规则
    pub fn match_rules(&self) -> Vec<&'static str> {
        vec![
            "标准比赛为先赢3局（BO5）",
            "总决赛可能为BO7（先赢4局）",
            "每局比赛有时间限制（因地图而异）",
            "攻防模式：进攻方需夺取目标点",
            "推车模式：进攻方需将运载目标推至终点",
            "控制模式：双方争夺中立目标点",
            "混合模式：先夺取目标点再推车",
            "平局决胜：必要时进行决胜局",
        ]
    }

    /// 时间规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "攻防/推车地图：进攻时间4分钟",
            "控制地图：每回合无时间限制",
            "加时机制：目标点附近延长比赛",
            "重生时间：10-12秒（随比赛进程变化）",
            "英雄切换：可在重生室更换英雄",
            "准备时间：每局开始前有准备阶段",
            "技术暂停：由裁判判定",
            "战术暂停：每队每场最多2次",
        ]
    }

    /// 重生规则
    pub fn respawn_rules(&self) -> Vec<&'static str> {
        vec![
            "死亡后在重生室重生",
            "重生时间：基础10秒",
            "随比赛进程重生时间逐渐增加",
            "重生后可以选择更换英雄",
            "重生室出口有保护区域",
            "重生室停留时间计入总时间",
            "团队重生机制（特定模式）",
            "比赛结束时重生室关闭",
        ]
    }

    /// 生命值和伤害规则
    pub fn health_damage_rules(&self) -> Vec<&'static str> {
        vec![
            "英雄基础生命值：100-700（因英雄而异）",
            "护甲：减少伤害（每点护甲减少5点伤害）",
            "护盾：可恢复的生命值",
            "爆头伤害：1.5-2倍伤害（因武器而异）",
            "暴击伤害：某些技能有暴击加成",
            "治疗：支援英雄可治疗队友",
            "临时护盾：某些技能提供临时护盾",
            "无敌状态：某些终极技能提供无敌",
        ]
    }
}

impl Default for OverwatchGameMechanicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OverwatchGameMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("overwatch_game_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Overwatch游戏机制规则",
            &[
                ("比赛规则", &self.match_rules()),
                ("时间规则", &self.timing_rules()),
                ("重生规则", &self.respawn_rules()),
                ("生命值和伤害", &self.health_damage_rules()),
            ],
        )
    }
}

/// Overwatch 英雄规则
#[derive(Debug, Clone)]
pub struct OverwatchHeroRules {
    metadata: RuleMetadata,
}

impl OverwatchHeroRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Overwatch英雄规则", "Overwatch英雄角色分类、技能和规则")
                .with_origin("Blizzard Entertainment官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Overwatch".into()]),
        }
    }

    /// 英雄角色分类
    pub fn hero_roles(&self) -> Vec<&'static str> {
        vec![
            "重装（Tank）：保护团队、吸收伤害",
            "输出（Damage）：造成伤害、击杀敌人",
            "支援（Support）：治疗队友、提供增益",
            "团队组成：1-2重装、2-4输出、2支援",
            "角色队列：强制2-2-2阵容",
            "英雄限制：每队最多1名相同英雄",
            "英雄切换：可在重生室更换",
            "角色技能：每个英雄有独特技能组合",
        ]
    }

    /// 重装英雄
    pub fn tank_heroes(&self) -> Vec<&'static str> {
        vec![
            "D.Va：机甲、防御矩阵、推进器、自毁",
            "莱因哈特：火箭重锤、屏障力场、冲锋、裂地猛击",
            "温斯顿：特斯拉炮、喷射背包、屏障发射器、原始暴怒",
            "查莉娅：粒子炮、粒子屏障、投射屏障、重力喷涌",
            "奥丽莎：强化聚能炮、屏障旋盾、站住！、轰天猛击",
            "西格玛：超能之球、实验屏障、动能俘获、巨石轰击",
            "破坏球：霰弹枪、动力铁球、工程抓钩、地雷禁区",
            "路霸：碎裂枪、呼吸器、钩子、猛兽撕咬",
            "玛伊拉：等离子枪、液态镭射、挥发性地刺、石化猛击",
            "拉玛塔：冥火发射器、虚空屏障、虚空涌动、歼灭",
            "Wrecking Ball：滚球形态、抓钩、地雷",
        ]
    }

    /// 输出英雄
    pub fn damage_heroes(&self) -> Vec<&'static str> {
        vec![
            "卡西迪：和平捍卫者、战术翻滚、神射手、磁性手雷",
            "死神：地狱火霰弹枪、幽灵形态、暗影步、死亡绽放",
            "源氏：手里剑、影、闪、龙刃",
            "半藏：风、岚、音、竜",
            "狂鼠：榴弹发射器、震荡地雷、钢铁陷阱、炸弹轮胎",
            "麦克雷（卡西迪原名）：和平捍卫者",
            "美：冰霜冲击枪、冰墙、急冻、暴风雪",
            "法老之鹰：火箭发射器、喷射推进背包、震荡冲击、弹幕",
            "死神：地狱火霰弹枪、幽灵形态",
            "士兵：76：重型脉冲步枪、螺旋火箭、生物力场、战术目镜",
            "黑影：冲锋枪、隐匿黑客、传送器、电磁脉冲",
            "秩序之光：光子发射器、哨戒炮、光子屏障、能量黑洞",
            "托比昂：铆钉枪、锻造锤、部署炮塔、熔火核心",
            "猎空：脉冲双枪、闪回、闪现、炸弹",
            "回声：三连火炮、粘性炸弹、飞行、仿制",
            "索杰恩：能量步枪、干扰射击、威力滑铲、超频",
            " Venture：原型重锤、护盾挖掘、钻地冲锋、掘地三尺",
            "Cassidy：和平捍卫者（原麦克雷）",
        ]
    }

    /// 支援英雄
    pub fn support_heroes(&self) -> Vec<&'static str> {
        vec![
            "安娜：生物步枪、生物手雷、睡眠镖、纳米激素",
            "布丽吉塔：火箭连枷、盾击、激励盔甲、集结号令",
            "卢西奥：音波枪、滑墙、音乐交叉淡出、音障",
            "天使：天使杖、守护天使、重生、女武神",
            "莫伊拉：生化之触、生化之握、消散、聚合射线",
            "禅雅塔：毁灭珠、谐/乱珠、瞬、超凡入圣",
            "巴蒂斯特：生化发射器、维生力场、幻象力场、增幅矩阵",
            "伊拉：生命之握、苦无、强音、音域",
            "生命之织者：生命之握、花瓣平台、生命之树",
        ]
    }

    /// 终极技能规则
    pub fn ultimate_rules(&self) -> Vec<&'static str> {
        vec![
            "终极技能需要充能才能使用",
            "充能方式：造成伤害、治疗、承受伤害",
            "充能速度因英雄而异",
            "终极技能可被队友看到充能进度",
            "死亡后终极技能充能损失15%",
            "终极技能可在使用前取消（部分英雄）",
            "终极技能有施放时间（部分英雄）",
            "终极技能效果因英雄而异",
        ]
    }
}

impl Default for OverwatchHeroRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OverwatchHeroRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("overwatch_heroes")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Overwatch英雄规则",
            &[
                ("英雄分类", &self.hero_roles()),
                ("重装英雄", &self.tank_heroes()),
                ("输出英雄", &self.damage_heroes()),
                ("支援英雄", &self.support_heroes()),
                ("终极技能", &self.ultimate_rules()),
            ],
        )
    }
}

/// Overwatch 地图规则
#[derive(Debug, Clone)]
pub struct OverwatchMapRules {
    metadata: RuleMetadata,
}

impl OverwatchMapRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Overwatch地图规则", "Overwatch竞技地图布局和规则")
                .with_origin("Blizzard Entertainment官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Overwatch".into()]),
        }
    }

    /// 地图列表
    pub fn map_list(&self) -> Vec<&'static str> {
        vec![
            // 攻防地图
            "阿努比斯神殿（Anubis）",
            "花村（Hanamura）",
            "月球基地（Horizon Lunar Colony）",
            "巴黎（Paris）",
            "地平线月球基地（Horizon）",
            // 推车地图
            "多拉多（Dorado）",
            "好莱坞（Hollywood）",
            "国王大道（King's Row）",
            "66号公路（Route 66）",
            "监测站：直布罗陀（Watchpoint: Gibraltar）",
            "哈瓦那（Havana）",
            "渣客镇（Junkertown）",
            "里阿尔托（Rialto）",
            // 混合地图
            "艾兴瓦尔德（Eichenwalde）",
            "好莱坞（Hollywood）",
            "国王大道（King's Row）",
            "努巴尼（Numbani）",
            "暴雪世界（Blizzard World）",
            // 控制地图
            "伊利奥斯（Ilios）",
            "漓江塔（Lijiang Tower）",
            "尼泊尔（Nepal）",
            "绿洲城（Oasis）",
            // 推进地图
            "卡纳尔街（Colosseo）",
            "新皇后街（New Queen Street）",
            "雪佛兰街（Esperanca）",
        ]
    }

    /// 攻防地图规则
    pub fn assault_map_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻方需夺取2个目标点",
            "目标点1：夺取时间较短",
            "目标点2：夺取时间较长",
            "夺取进度：可部分夺取并保留",
            "进攻时间：4分钟（可加时）",
            "加时机制：在目标点附近触发",
            "防守方需要阻止进攻方夺取目标点",
            "每队各攻防一次",
        ]
    }

    /// 推车地图规则
    pub fn escort_map_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻方需将运载目标推至终点",
            "运载目标移动：进攻方附近时前进",
            "防守方附近：运载目标停止",
            "检查点：通过后不可逆转",
            "进攻时间：可因检查点延长时间",
            "距离分数：记录推车最远距离",
            "平局决胜：比较推车距离",
            "加时机制：在运载目标附近触发",
        ]
    }

    /// 混合地图规则
    pub fn hybrid_map_rules(&self) -> Vec<&'static str> {
        vec![
            "先夺取目标点，再推车",
            "夺取目标点后开始推车",
            "结合攻防和推车规则",
            "通常有1个目标点和推车路线",
            "进攻时间：夺取目标点后获得额外时间",
            "检查点：推车路线上的检查点",
            "综合评分：目标点+推车距离",
            "平局决胜：比较推车距离",
        ]
    }

    /// 控制地图规则
    pub fn control_map_rules(&self) -> Vec<&'static str> {
        vec![
            "双方争夺1个中立目标点",
            "先达到100%的队伍获胜",
            "争夺进度：双方人数对比",
            "目标点位置：地图中心",
            "回合制：先赢得2回合获胜",
            "无时间限制（传统规则）",
            "回合间有短暂准备时间",
            "回合胜利：达到100%或敌方全部死亡",
        ]
    }

    /// 推进地图规则
    pub fn push_map_rules(&self) -> Vec<&'static str> {
        vec![
            "双方各推一个机器人到终点",
            "机器人：中央出发，向敌方推进",
            "玩家附近：机器人前进",
            "敌方附近：机器人停止",
            "推进距离：记录最远距离",
            "胜利条件：推到终点或距离更远",
            "平局决胜：比较推进距离",
            "时间限制：与推车地图类似",
        ]
    }
}

impl Default for OverwatchMapRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OverwatchMapRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("overwatch_maps")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Overwatch地图规则",
            &[
                ("地图列表", &self.map_list()),
                ("攻防地图", &self.assault_map_rules()),
                ("推车地图", &self.escort_map_rules()),
                ("混合地图", &self.hybrid_map_rules()),
                ("控制地图", &self.control_map_rules()),
                ("推进地图", &self.push_map_rules()),
            ],
        )
    }
}

/// Overwatch 比赛规则
#[derive(Debug, Clone)]
pub struct OverwatchCompetitionRules {
    metadata: RuleMetadata,
}

impl OverwatchCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Overwatch比赛规则", "Overwatch正式比赛赛制和规则")
                .with_origin("Blizzard Entertainment官方规则")
                .with_tags(vec!["电竞".into(), "FPS".into(), "Overwatch".into()]),
        }
    }

    /// 赛制规则
    pub fn format_rules(&self) -> Vec<&'static str> {
        vec![
            "标准赛制：BO5（先赢3局）",
            "总决赛：BO7（先赢4局）",
            "地图池：从预设地图池中选择",
            "选图规则：双方轮流选择地图",
            "地图胜利：完成目标或时间优势",
            "平局处理：进行决胜局",
            "决胜局：控制地图（通常是伊利奥斯）",
            "积分制：胜利积1分，失败0分",
        ]
    }

    /// 队伍规则
    pub fn team_rules(&self) -> Vec<&'static str> {
        vec![
            "场上队员：6人",
            "替补队员：最多2人",
            "教练：可1人，负责战术暂停",
            "选手年龄：最低18岁（OWL）",
            "选手认证：需注册并验证身份",
            "队伍阵容：角色队列2-2-2",
            "英雄限制：每队1名相同英雄",
            "队伍更换：需提前申报",
        ]
    }

    /// 暂停规则
    pub fn pause_rules(&self) -> Vec<&'static str> {
        vec![
            "战术暂停：每队每场2次，每次60秒",
            "技术暂停：由裁判判定，无限次",
            "医疗暂停：选手健康问题，最长5分钟",
            "暂停请求：仅教练或队长可申请",
            "暂停时机：回合结束后或死球状态",
            "暂停后：比赛从暂停点继续",
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
}

impl Default for OverwatchCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OverwatchCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("overwatch_competition")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Overwatch比赛规则",
            &[
                ("赛制规则", &self.format_rules()),
                ("队伍规则", &self.team_rules()),
                ("暂停规则", &self.pause_rules()),
                ("装备规则", &self.equipment_rules()),
                ("违规规则", &self.violation_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_mechanics_rules() {
        let rules = OverwatchGameMechanicsRules::new();
        assert!(!rules.match_rules().is_empty());
        assert!(!rules.timing_rules().is_empty());
        assert!(!rules.respawn_rules().is_empty());
        assert!(!rules.health_damage_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_hero_rules() {
        let rules = OverwatchHeroRules::new();
        assert!(!rules.hero_roles().is_empty());
        assert!(!rules.tank_heroes().is_empty());
        assert!(!rules.damage_heroes().is_empty());
        assert!(!rules.support_heroes().is_empty());
        assert!(!rules.ultimate_rules().is_empty());
    }

    #[test]
    fn test_map_rules() {
        let rules = OverwatchMapRules::new();
        assert!(!rules.map_list().is_empty());
        assert!(!rules.assault_map_rules().is_empty());
        assert!(!rules.escort_map_rules().is_empty());
        assert!(!rules.hybrid_map_rules().is_empty());
        assert!(!rules.control_map_rules().is_empty());
        assert!(!rules.push_map_rules().is_empty());
    }

    #[test]
    fn test_competition_rules() {
        let rules = OverwatchCompetitionRules::new();
        assert!(!rules.format_rules().is_empty());
        assert!(!rules.team_rules().is_empty());
        assert!(!rules.pause_rules().is_empty());
        assert!(!rules.equipment_rules().is_empty());
        assert!(!rules.violation_rules().is_empty());
    }

    #[test]
    fn test_metadata() {
        let rules = OverwatchGameMechanicsRules::new();
        assert_eq!(rules.metadata().name, "Overwatch游戏机制规则");
        assert!(rules.metadata().tags.contains(&"电竞".to_string()));
    }

    #[test]
    fn test_category() {
        let rules = OverwatchGameMechanicsRules::new();
        let category = rules.category();
        assert!(category.to_string().contains("overwatch"));
    }

    #[test]
    fn test_validate() {
        let rules = OverwatchGameMechanicsRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        let result = rules.validate(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_explain_format() {
        let rules = OverwatchGameMechanicsRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("Overwatch游戏机制规则"));
        assert!(explanation.contains("比赛规则"));
        assert!(explanation.contains("时间规则"));
    }

    #[test]
    fn test_hero_roles_count() {
        let rules = OverwatchHeroRules::new();
        assert_eq!(rules.hero_roles().len(), 8);
    }

    #[test]
    fn test_map_count() {
        let rules = OverwatchMapRules::new();
        assert!(rules.map_list().len() >= 20);
    }

    #[test]
    fn test_team_size() {
        let rules = OverwatchCompetitionRules::new();
        let team_rules = rules.team_rules();
        assert!(team_rules.iter().any(|r| r.contains("6人")));
    }

    #[test]
    fn test_bo5_format() {
        let rules = OverwatchCompetitionRules::new();
        let format_rules = rules.format_rules();
        assert!(format_rules.iter().any(|r| r.contains("BO5")));
    }

    #[test]
    fn test_respawn_time() {
        let rules = OverwatchGameMechanicsRules::new();
        let respawn = rules.respawn_rules();
        assert!(respawn.iter().any(|r| r.contains("10秒")));
    }

    #[test]
    fn test_tank_heroes_count() {
        let rules = OverwatchHeroRules::new();
        assert!(rules.tank_heroes().len() >= 8);
    }

    #[test]
    fn test_support_heroes_count() {
        let rules = OverwatchHeroRules::new();
        assert!(rules.support_heroes().len() >= 7);
    }
}
