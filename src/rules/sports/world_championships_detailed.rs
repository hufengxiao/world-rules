//! 世界锦标赛详细规则
//!
//! 各运动项目世界锦标赛的完整规则体系，包括田径世锦赛、游泳世锦赛、
//! 体操世锦赛、举重世锦赛等主要项目的世锦赛规则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 世界田径锦标赛规则
#[derive(Debug, Clone)]
pub struct WorldAthleticsChampionshipsRules {
    metadata: RuleMetadata,
}

impl WorldAthleticsChampionshipsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界田径锦标赛规则", "IAAF世界田径锦标赛组织规则")
                .with_origin("世界田联章程")
                .with_tags(vec!["体育".into(), "田径".into(), "世锦赛".into()]),
        }
    }

    /// 赛事概况
    pub fn tournament_overview(&self) -> Vec<&'static str> {
        vec![
            "每2年举办一届（奇数年）",
            "首届于1983年在赫尔辛基举办",
            "由世界田联(World Athletics)主办",
            "超过200个国家和地区参赛",
            "约2000名运动员参赛",
            "48个比赛项目（男24+女24）",
        ]
    }

    /// 参赛资格
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "参赛标准：达到A标或B标成绩",
            "A标自动获得参赛资格",
            "B标需由各国协会选拔",
            "每个项目每国最多3人（A标）或1人（B标）",
            "接力项目：16支队伍参赛",
            "外卡名额：未达标国家可派1人",
            "卫冕冠军和钻石联赛冠军自动获得资格",
        ]
    }

    /// 项目设置
    pub fn event_categories(&self) -> Vec<&'static str> {
        vec![
            "短跑：100m、200m、400m",
            "中长跑：800m、1500m、5000m、10000m",
            "跨栏：110m栏、400m栏",
            "障碍跑：3000m障碍",
            "跳跃：跳高、撑杆跳、跳远、三级跳",
            "投掷：铅球、铁饼、链球、标枪",
            "全能：十项全能(男)、七项全能(女)",
            "接力：4x100m接力、4x400m接力",
            "竞走：20公里、35公里",
            "马拉松：马拉松项目",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "短跑项目采用多轮次赛制（预赛、复赛、半决赛、决赛）",
            "中长跑项目根据报名人数决定赛制",
            "跳跃和投掷项目有资格赛和决赛",
            "资格赛达标线或前12名进入决赛",
            "决赛每人6次试跳/试投",
            "成绩相同比较次优成绩",
            "风速超过2.0m/s成绩不记为纪录",
        ]
    }

    /// 奖牌设置
    pub fn medal_system(&self) -> Vec<&'static str> {
        vec![
            "每个项目颁发金、银、铜牌",
            "奖牌榜按金牌数排名",
            "团体项目颁发团体奖牌",
            "奖金分配：冠军60000美元",
            "亚军30000美元，季军20000美元",
            "破世界纪录额外奖励100000美元",
        ]
    }
}

impl Default for WorldAthleticsChampionshipsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldAthleticsChampionshipsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_athletics_championships")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界田径锦标赛规则",
            &[
                ("赛事概况", &self.tournament_overview()),
                ("参赛资格", &self.qualification_system()),
                ("项目设置", &self.event_categories()),
                ("比赛规则", &self.competition_rules()),
                ("奖牌设置", &self.medal_system()),
            ],
        )
    }
}

/// 世界游泳锦标赛规则
#[derive(Debug, Clone)]
pub struct WorldAquaticsChampionshipsRules {
    metadata: RuleMetadata,
}

impl WorldAquaticsChampionshipsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界游泳锦标赛规则", "FINA世界游泳锦标赛组织规则")
                .with_origin("世界泳联章程")
                .with_tags(vec!["体育".into(), "游泳".into(), "世锦赛".into()]),
        }
    }

    /// 赛事概况
    pub fn tournament_overview(&self) -> Vec<&'static str> {
        vec![
            "每2年举办一届（奇数年）",
            "首届于1973年在贝尔格莱德举办",
            "由世界泳联(World Aquatics)主办",
            "超过190个国家和地区参赛",
            "约2500名运动员参赛",
            "涵盖游泳、跳水、花样游泳、水球、公开水域",
        ]
    }

    /// 参赛标准
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "参赛标准：达到A标或B标时间",
            "A标：直接参赛资格",
            "B标：需各国泳协选拔",
            "每项最多2人（A标）或1人（B标）",
            "接力项目：16-20支队伍参赛",
            "东道主自动获得部分名额",
            "外卡名额：未达标国家可派男女各1人",
        ]
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "自由泳：50m、100m、200m、400m、800m、1500m",
            "仰泳：50m、100m、200m",
            "蛙泳：50m、100m、200m",
            "蝶泳：50m、100m、200m",
            "混合泳：200m、400m个人混合泳",
            "接力：4x100m自由泳、4x200m自由泳、4x100m混合泳",
            "公开水域：5km、10km、25km",
            "跳水：1m跳板、3m跳板、10m跳台、双人项目",
            "花样游泳：单人、双人、团体技术自选和自由自选",
            "水球：男女各16支队伍",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "泳池比赛采用预赛、半决赛、决赛制",
            "前16名进入半决赛",
            "前8名进入决赛",
            "跳水分预赛、半决赛、决赛",
            "花样游泳分技术自选和自由自选",
            "水球采用小组赛+淘汰赛制",
            "公开水域采用集体出发",
        ]
    }

    /// 纪录认证
    pub fn record_recognition(&self) -> Vec<&'static str> {
        vec![
            "世界纪录需在正式比赛中创造",
            "风速和温度需符合规定",
            "兴奋剂检测合格",
            "裁判认证签字",
            "世界泳联审核确认",
            "破纪录奖金：50000美元",
        ]
    }
}

impl Default for WorldAquaticsChampionshipsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldAquaticsChampionshipsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_aquatics_championships")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界游泳锦标赛规则",
            &[
                ("赛事概况", &self.tournament_overview()),
                ("参赛标准", &self.qualification_system()),
                ("比赛项目", &self.competition_events()),
                ("比赛规则", &self.competition_rules()),
                ("纪录认证", &self.record_recognition()),
            ],
        )
    }
}

/// 世界体操锦标赛规则
#[derive(Debug, Clone)]
pub struct WorldGymnasticsChampionshipsRules {
    metadata: RuleMetadata,
}

impl WorldGymnasticsChampionshipsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界体操锦标赛规则", "FIG世界体操锦标赛组织规则")
                .with_origin("国际体操联合会章程")
                .with_tags(vec!["体育".into(), "体操".into(), "世锦赛".into()]),
        }
    }

    /// 赛事概况
    pub fn tournament_overview(&self) -> Vec<&'static str> {
        vec![
            "竞技体操世锦赛每年举办一届",
            "非奥运年份举办个人世锦赛",
            "奥运年份举办团体世锦赛",
            "首届于1903年在安特卫普举办",
            "由国际体操联合会(FIG)主办",
            "超过80个国家和地区参赛",
        ]
    }

    /// 项目设置
    pub fn apparatus_categories(&self) -> Vec<&'static str> {
        vec![
            "男子：自由操、鞍马、吊环、跳马、双杠、单杠",
            "女子：跳马、高低杠、平衡木、自由操",
            "团体赛：6-5-4赛制（6人报名、5人上场、4人计分）",
            "全能赛：男子6项、女子4项",
            "单项决赛：每项8人参赛",
        ]
    }

    /// 评分规则
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "难度分(D分)：最高10分",
            "完成分(E分)：从10分扣减",
            "最终分数：D分+E分",
            "6名裁判评分，去掉最高最低分取平均",
            "难度裁判2人，完成裁判5人",
            "落地扣分：根据落地稳定性",
            "出界扣分：0.1分",
        ]
    }

    /// 参赛资格
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "团体赛：24支队伍参赛",
            "个人全能：24人参赛",
            "单项决赛：每项8人参赛",
            "洲际锦标赛成绩决定部分名额",
            "世锦赛前一年成绩决定种子",
            "东道主自动获得参赛名额",
        ]
    }

    /// 比赛流程
    pub fn competition_format(&self) -> Vec<&'static str> {
        vec![
            "资格赛：决定决赛资格",
            "团体决赛：前8名队伍",
            "全能决赛：前24名个人",
            "单项决赛：每项前8名",
            "决赛采用单轮制",
            "热身时间有限制",
        ]
    }
}

impl Default for WorldGymnasticsChampionshipsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldGymnasticsChampionshipsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_gymnastics_championships")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界体操锦标赛规则",
            &[
                ("赛事概况", &self.tournament_overview()),
                ("项目设置", &self.apparatus_categories()),
                ("评分规则", &self.scoring_system()),
                ("参赛资格", &self.qualification_system()),
                ("比赛流程", &self.competition_format()),
            ],
        )
    }
}

/// 世界举重锦标赛规则
#[derive(Debug, Clone)]
pub struct WorldWeightliftingChampionshipsRules {
    metadata: RuleMetadata,
}

impl WorldWeightliftingChampionshipsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界举重锦标赛规则", "IWF世界举重锦标赛组织规则")
                .with_origin("国际举重联合会章程")
                .with_tags(vec!["体育".into(), "举重".into(), "世锦赛".into()]),
        }
    }

    /// 赛事概况
    pub fn tournament_overview(&self) -> Vec<&'static str> {
        vec![
            "每年举办一届",
            "首届于1891年在伦敦举办",
            "由国际举重联合会(IWF)主办",
            "男女各10个重量级别",
            "约100个国家和地区参赛",
            "奥运年份世锦赛为奥运资格赛",
        ]
    }

    /// 重量级别
    pub fn weight_categories(&self) -> Vec<&'static str> {
        vec![
            "男子：55kg、61kg、67kg、73kg、81kg、89kg、96kg、102kg、109kg、+109kg",
            "女子：45kg、49kg、55kg、59kg、64kg、71kg、76kg、81kg、87kg、+87kg",
            "级别设置可根据赛事调整",
            "体重称重在赛前2小时进行",
            "超重1kg取消参赛资格",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "抓举(Snatch)和挺举(Clean & Jerk)两项",
            "每项最多3次试举机会",
            "总成绩=抓举+挺举最好成绩",
            "试举重量必须递增（至少1kg）",
            "试举时间限制：1分钟",
            "连续试举时间限制：2分钟",
            "破纪录必须增加至少0.5kg",
        ]
    }

    /// 裁判规则
    pub fn officiating_rules(&self) -> Vec<&'static str> {
        vec![
            "3名裁判判定试举是否有效",
            "2名以上裁判亮白灯为有效",
            "2名以上裁判亮红灯为无效",
            "裁判长可改判",
            "视频回放系统辅助判决",
            " Jury（陪审团）监督裁判工作",
        ]
    }

    /// 奖牌与纪录
    pub fn medals_and_records(&self) -> Vec<&'static str> {
        vec![
            "每级别颁发金、银、铜牌",
            "单项奖牌：抓举、挺举、总成绩各3枚",
            "世界纪录奖金：20000美元",
            "青年世界纪录奖金：10000美元",
            "总成绩纪录需两项都有成绩",
        ]
    }
}

impl Default for WorldWeightliftingChampionshipsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldWeightliftingChampionshipsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_weightlifting_championships")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界举重锦标赛规则",
            &[
                ("赛事概况", &self.tournament_overview()),
                ("重量级别", &self.weight_categories()),
                ("比赛规则", &self.competition_rules()),
                ("裁判规则", &self.officiating_rules()),
                ("奖牌纪录", &self.medals_and_records()),
            ],
        )
    }
}

/// 世界羽毛球锦标赛规则
#[derive(Debug, Clone)]
pub struct WorldBadmintonChampionshipsRules {
    metadata: RuleMetadata,
}

impl WorldBadmintonChampionshipsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界羽毛球锦标赛规则", "BWF世界羽毛球锦标赛组织规则")
                .with_origin("世界羽联章程")
                .with_tags(vec!["体育".into(), "羽毛球".into(), "世锦赛".into()]),
        }
    }

    /// 赛事概况
    pub fn tournament_overview(&self) -> Vec<&'static str> {
        vec![
            "每年举办一届（非奥运年）",
            "首届于1977年在雅加达举办",
            "由世界羽联(BWF)主办",
            "5个单项：男单、女单、男双、女双、混双",
            "约60个国家和地区参赛",
            "总奖金：100万美元以上",
        ]
    }

    /// 参赛资格
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "根据世界排名确定参赛资格",
            "每个项目64名选手参赛",
            "同协会最多4人（单打）或3对（双打）",
            "东道主外卡名额",
            "资格赛产生部分名额",
            "种子选手根据排名确定",
        ]
    }

    /// 比赛规则
    pub fn match_rules(&self) -> Vec<&'static str> {
        vec![
            "采用三局两胜制",
            "每局21分（领先2分或30分封顶）",
            "每球得分制",
            "局间休息2分钟",
            "11分技术暂停60秒",
            "挑战鹰眼有次数限制",
            "教练指导有时间和次数限制",
        ]
    }

    /// 抽签规则
    pub fn draw_rules(&self) -> Vec<&'static str> {
        vec![
            "种子选手分布在不同区域",
            "同协会选手尽量分散",
            "前8号种子按排名分入不同1/8区",
            "抽签在赛前举行",
            "种子保护：避免早期相遇",
        ]
    }

    /// 奖金分配
    pub fn prize_money(&self) -> Vec<&'static str> {
        vec![
            "冠军：约20万美元",
            "亚军：约10万美元",
            "四强：约4万美元",
            "八强：约2万美元",
            "十六强：约1万美元",
            "奖金需扣除税费",
        ]
    }
}

impl Default for WorldBadmintonChampionshipsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldBadmintonChampionshipsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_badminton_championships")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界羽毛球锦标赛规则",
            &[
                ("赛事概况", &self.tournament_overview()),
                ("参赛资格", &self.qualification_system()),
                ("比赛规则", &self.match_rules()),
                ("抽签规则", &self.draw_rules()),
                ("奖金分配", &self.prize_money()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_athletics_championships() {
        let rules = WorldAthleticsChampionshipsRules::new();
        assert!(!rules.tournament_overview().is_empty());
        assert!(rules.event_categories().len() >= 10);
    }

    #[test]
    fn test_aquatics_championships() {
        let rules = WorldAquaticsChampionshipsRules::new();
        assert!(!rules.qualification_system().is_empty());
        assert!(rules.competition_events().len() >= 10);
    }

    #[test]
    fn test_gymnastics_championships() {
        let rules = WorldGymnasticsChampionshipsRules::new();
        assert!(!rules.scoring_system().is_empty());
        assert!(rules.apparatus_categories().len() >= 5);
    }

    #[test]
    fn test_weightlifting_championships() {
        let rules = WorldWeightliftingChampionshipsRules::new();
        assert!(!rules.competition_rules().is_empty());
        assert!(rules.weight_categories().len() >= 5);
    }

    #[test]
    fn test_badminton_championships() {
        let rules = WorldBadmintonChampionshipsRules::new();
        assert!(!rules.match_rules().is_empty());
        assert!(rules.qualification_system().len() >= 6);
    }

    #[test]
    fn test_metadata() {
        let rules = WorldAthleticsChampionshipsRules::new();
        assert_eq!(rules.metadata().name, "世界田径锦标赛规则");
        assert_eq!(
            rules.category(),
            RuleCategory::sports("world_athletics_championships")
        );
    }

    #[test]
    fn test_rule_impl() {
        use crate::rules::core::Rule;
        let rules = WorldGymnasticsChampionshipsRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("体操"));
        assert!(explanation.contains("评分"));
    }
}
