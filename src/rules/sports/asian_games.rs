//! 亚运会规则
//!
//! 亚洲运动会完整规则体系，包括组织架构、参赛资格、比赛项目、
//! 奖牌体系等完整规则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 亚运会组织架构规则
#[derive(Debug, Clone)]
pub struct AsianGamesOrganizationRules {
    metadata: RuleMetadata,
}

impl AsianGamesOrganizationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "亚运会组织架构规则",
                "亚洲奥林匹克理事会组织架构和运营规则",
            )
            .with_origin("OCA章程")
            .with_tags(vec!["体育".into(), "亚运会".into(), "组织".into()]),
        }
    }

    /// OCA组织架构
    pub fn oca_structure(&self) -> Vec<&'static str> {
        vec![
            "OCA（亚洲奥林匹克理事会）是亚运会最高管理机构",
            "OCA大会是最高决策机构，每年召开一次",
            "OCA执行委员会负责日常事务管理",
            "OCA主席由大会选举产生，任期8年",
            "45个成员国/地区奥委会",
            "5个区域组织：东亚、东南亚、南亚、中亚、西亚",
        ]
    }

    /// 亚运会历史
    pub fn games_history(&self) -> Vec<&'static str> {
        vec![
            "首届亚运会于1951年在新德里举办",
            "每4年举办一届（奥运年后一年）",
            "1990年起增设冬季亚运会",
            "2022年杭州亚运会为第19届",
            "参赛国家和地区从11个增至45个",
            "比赛项目从6个大项增至40+大项",
        ]
    }

    /// 主办城市遴选
    pub fn host_selection(&self) -> Vec<&'static str> {
        vec![
            "主办城市由OCA大会投票决定",
            "申办城市需提前6年提交申请",
            "OCA评估团实地考察",
            "评估标准：场馆、交通、住宿、安保",
            "轮换原则：五大区域轮流举办",
            "联合举办：可由多城市联合举办",
        ]
    }

    /// 组委会规则
    pub fn organizing_committee(&self) -> Vec<&'static str> {
        vec![
            "每届亚运会成立组委会(HAGOC)",
            "组委会由主办城市政府和国家奥委会组建",
            "组委会负责赛事筹备和运营",
            "必须在赛前5年成立",
            "负责场馆建设和设施准备",
            "负责志愿者招募和培训",
            "负责亚运村运营",
        ]
    }
}

impl Default for AsianGamesOrganizationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AsianGamesOrganizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("asian_games_organization")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "亚运会组织架构规则",
            &[
                ("OCA架构", &self.oca_structure()),
                ("历史沿革", &self.games_history()),
                ("主办遴选", &self.host_selection()),
                ("组委会规则", &self.organizing_committee()),
            ],
        )
    }
}

/// 亚运会参赛资格规则
#[derive(Debug, Clone)]
pub struct AsianGamesQualificationRules {
    metadata: RuleMetadata,
}

impl AsianGamesQualificationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("亚运会参赛资格规则", "运动员和代表团的参赛资格规则")
                .with_origin("OCA章程")
                .with_tags(vec!["体育".into(), "亚运会".into(), "资格".into()]),
        }
    }

    /// 参赛资格原则
    pub fn eligibility_principles(&self) -> Vec<&'static str> {
        vec![
            "运动员必须代表OCA成员国/地区参赛",
            "运动员必须持有参赛国国籍或永久居留权",
            "运动员必须遵守OCA反兴奋剂条例",
            "运动员年龄限制由各项目规定",
            "非职业运动员优先原则",
            "归化运动员需满足居住年限要求",
        ]
    }

    /// 参赛名额分配
    pub fn quota_allocation(&self) -> Vec<&'static str> {
        vec![
            "运动员总数限制15000人",
            "各项目名额由OCA和国际单项组织协商",
            "东道主自动获得部分名额",
            "各国代表团人数由各NOC决定",
            "技术官员由各国际组织指派",
            "集体球类项目每队名额固定",
        ]
    }

    /// 资格赛体系
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "亚洲锦标赛作为主要资格来源",
            "亚洲杯、亚洲预选赛分配名额",
            "世界排名决定部分资格",
            "东道主保送名额",
            "外卡名额由三方委员会分配",
            "资格赛时间线：赛前1年开始",
        ]
    }

    /// 代表团组成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "代表团由运动员和官员组成",
            "团长负责代表团整体管理",
            "教练员人数由各项目规定",
            "队医、理疗师按运动员比例配备",
            "代表团官员总数不得超过运动员30%",
            "技术官员不属于代表团",
        ]
    }
}

impl Default for AsianGamesQualificationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AsianGamesQualificationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("asian_games_qualification")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "亚运会参赛资格规则",
            &[
                ("参赛原则", &self.eligibility_principles()),
                ("名额分配", &self.quota_allocation()),
                ("资格赛体系", &self.qualification_system()),
                ("代表团组成", &self.team_composition()),
            ],
        )
    }
}

/// 亚运会比赛项目规则
#[derive(Debug, Clone)]
pub struct AsianGamesSportsProgramRules {
    metadata: RuleMetadata,
}

impl AsianGamesSportsProgramRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("亚运会比赛项目规则", "比赛项目设置和竞赛规则")
                .with_origin("OCA竞赛规程")
                .with_tags(vec!["体育".into(), "亚运会".into(), "项目".into()]),
        }
    }

    /// 必设项目（奥运项目）
    pub fn olympic_sports(&self) -> Vec<&'static str> {
        vec![
            "田径、游泳、花样游泳、跳水、水球",
            "羽毛球、篮球、拳击、皮划艇、自行车",
            "马术、击剑、足球、高尔夫、体操",
            "手球、曲棍球、柔道、现代五项、赛艇",
            "七人制橄榄球、帆船、射击、乒乓球",
            "跆拳道、网球、铁人三项、排球、举重",
            "摔跤、射箭、竞技体操、艺术体操",
            "蹦床、攀岩、滑板、冲浪、空手道",
        ]
    }

    /// 亚洲特色项目
    pub fn asian_sports(&self) -> Vec<&'static str> {
        vec![
            "武术（套路、散打）",
            "卡巴迪",
            "藤球",
            "壁球",
            "板球",
            "沙滩排球",
            "柔术",
            "克柔术",
            "桑搏",
            "电子竞技（表演项目）",
        ]
    }

    /// 项目设置规则
    pub fn sports_selection(&self) -> Vec<&'static str> {
        vec![
            "必设项目：所有奥运项目必须设项",
            "选设项目：东道主可提议增设",
            "表演项目：东道主可设置表演项目",
            "小项设置：由各国际单项组织决定",
            "性别平等：男女项目数量平衡",
            "新增项目需OCA批准",
        ]
    }

    /// 竞赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "采用国际单项组织最新竞赛规则",
            "裁判员由国际组织派遣",
            "技术代表监督比赛进行",
            "抗议和申诉按规定程序",
            "兴奋剂检测严格执行",
            "比赛服装符合规定",
        ]
    }
}

impl Default for AsianGamesSportsProgramRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AsianGamesSportsProgramRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("asian_games_sports_program")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "亚运会比赛项目规则",
            &[
                ("奥运项目", &self.olympic_sports()),
                ("亚洲特色项目", &self.asian_sports()),
                ("项目设置规则", &self.sports_selection()),
                ("竞赛规则", &self.competition_rules()),
            ],
        )
    }
}

/// 亚运会奖牌规则
#[derive(Debug, Clone)]
pub struct AsianGamesMedalRules {
    metadata: RuleMetadata,
}

impl AsianGamesMedalRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("亚运会奖牌规则", "奖牌颁发和奖励规则")
                .with_origin("OCA章程")
                .with_tags(vec!["体育".into(), "亚运会".into(), "奖牌".into()]),
        }
    }

    /// 奖牌设置
    pub fn medal_system(&self) -> Vec<&'static str> {
        vec![
            "每个项目颁发金、银、铜牌",
            "团体项目每队颁发奖牌",
            "奖牌榜按金牌数排名",
            "金牌相同比银牌，银牌相同比铜牌",
            "不设总奖牌榜官方排名",
            "并列成绩并列奖牌",
        ]
    }

    /// 颁奖仪式
    pub fn award_ceremony(&self) -> Vec<&'static str> {
        vec![
            "颁奖在决赛后立即进行",
            "OCA代表颁发奖牌",
            "东道国代表陪同颁奖",
            "升国旗奏国歌",
            "获奖运动员须穿着正式领奖服",
            "颁奖嘉宾按规定站位",
        ]
    }

    /// 破纪录奖励
    pub fn record_rewards(&self) -> Vec<&'static str> {
        vec![
            "破亚洲纪录额外奖励",
            "破世界纪录额外奖励",
            "纪录需经技术代表认证",
            "兴奋剂检测合格后确认",
            "奖金金额由组委会确定",
        ]
    }

    /// 团体总成绩
    pub fn team_standings(&self) -> Vec<&'static str> {
        vec![
            "按金牌数排名",
            "金、银、铜牌均计入总成绩",
            "东道主通常有主场优势",
            "中国、日本、韩国为传统三强",
            "奖牌分布反映各国体育实力",
        ]
    }
}

impl Default for AsianGamesMedalRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AsianGamesMedalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("asian_games_medal")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "亚运会奖牌规则",
            &[
                ("奖牌设置", &self.medal_system()),
                ("颁奖仪式", &self.award_ceremony()),
                ("纪录奖励", &self.record_rewards()),
                ("团体总成绩", &self.team_standings()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_rules() {
        let rules = AsianGamesOrganizationRules::new();
        assert!(!rules.oca_structure().is_empty());
        assert!(rules.games_history().len() >= 6);
    }

    #[test]
    fn test_qualification_rules() {
        let rules = AsianGamesQualificationRules::new();
        assert!(!rules.eligibility_principles().is_empty());
        assert!(rules.qualification_system().len() >= 6);
    }

    #[test]
    fn test_sports_program_rules() {
        let rules = AsianGamesSportsProgramRules::new();
        assert!(rules.olympic_sports().len() >= 8);
        assert!(rules.asian_sports().len() >= 10);
    }

    #[test]
    fn test_medal_rules() {
        let rules = AsianGamesMedalRules::new();
        assert!(!rules.medal_system().is_empty());
        assert!(rules.award_ceremony().len() >= 6);
    }

    #[test]
    fn test_metadata() {
        let rules = AsianGamesOrganizationRules::new();
        assert_eq!(rules.metadata().name, "亚运会组织架构规则");
        assert_eq!(
            rules.category(),
            RuleCategory::sports("asian_games_organization")
        );
    }

    #[test]
    fn test_rule_impl() {
        use crate::rules::core::Rule;
        let rules = AsianGamesQualificationRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("参赛"));
        assert!(explanation.contains("资格"));
    }
}
