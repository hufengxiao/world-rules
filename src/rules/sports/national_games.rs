//! 全运会规则
//!
//! 中国全国运动会完整规则体系，包括组织架构、参赛资格、比赛项目、
//! 奖牌体系等完整规则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 全运会组织架构规则
#[derive(Debug, Clone)]
pub struct NationalGamesOrganizationRules {
    metadata: RuleMetadata,
}

impl NationalGamesOrganizationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("全运会组织架构规则", "全国运动会组织架构和运营规则")
                .with_origin("体育总局章程")
                .with_tags(vec!["体育".into(), "全运会".into(), "组织".into()]),
        }
    }

    /// 组织架构
    pub fn organization_structure(&self) -> Vec<&'static str> {
        vec![
            "国家体育总局是全运会最高管理机构",
            "中华全国体育总会负责协调组织",
            "中国奥委会负责国际联络",
            "主办省市政府成立组委会",
            "各省市体育部门负责代表团组建",
            "各单项协会负责项目竞赛组织",
        ]
    }

    /// 全运会历史
    pub fn games_history(&self) -> Vec<&'static str> {
        vec![
            "首届全运会于1959年在北京举办",
            "每4年举办一届（奥运年后一年）",
            "已举办15届（截至2021年陕西全运会）",
            "参赛单位从29个增至46个",
            "比赛项目从36个大项增至54个大项",
            "2001年起实行申办制",
            "2005年起允许部分项目外省举办",
        ]
    }

    /// 主办城市遴选
    pub fn host_selection(&self) -> Vec<&'static str> {
        vec![
            "主办城市由国务院批准",
            "申办省市需提前5年提交申请",
            "体育总局组织评估考察",
            "评估标准：场馆、交通、住宿、安保",
            "轮换原则：东、中、西部轮换",
            "联合举办：可由多城市联合举办",
        ]
    }

    /// 组委会规则
    pub fn organizing_committee(&self) -> Vec<&'static str> {
        vec![
            "每届全运会成立组委会",
            "组委会由国家体育总局和主办省市共同组建",
            "组委会负责赛事筹备和运营",
            "必须在赛前4年成立",
            "负责场馆建设和设施准备",
            "负责志愿者招募和培训",
            "负责全运村运营",
        ]
    }
}

impl Default for NationalGamesOrganizationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NationalGamesOrganizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("national_games_organization")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "全运会组织架构规则",
            &[
                ("组织架构", &self.organization_structure()),
                ("历史沿革", &self.games_history()),
                ("主办遴选", &self.host_selection()),
                ("组委会规则", &self.organizing_committee()),
            ],
        )
    }
}

/// 全运会参赛资格规则
#[derive(Debug, Clone)]
pub struct NationalGamesQualificationRules {
    metadata: RuleMetadata,
}

impl NationalGamesQualificationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("全运会参赛资格规则", "运动员和代表团的参赛资格规则")
                .with_origin("体育总局竞赛规程")
                .with_tags(vec!["体育".into(), "全运会".into(), "资格".into()]),
        }
    }

    /// 参赛单位
    pub fn participating_units(&self) -> Vec<&'static str> {
        vec![
            "46个参赛单位：31个省区市、14个行业体协、1个解放军",
            "每个单位成立代表团参赛",
            "代表团团长由省市体育局长担任",
            "代表团规模由各省市自行决定",
            "集体项目可联合组队参赛",
            "交流运动员代表注册单位参赛",
        ]
    }

    /// 参赛资格原则
    pub fn eligibility_principles(&self) -> Vec<&'static str> {
        vec![
            "运动员必须在中国注册",
            "运动员必须代表注册单位参赛",
            "运动员必须通过资格赛选拔",
            "运动员必须通过反兴奋剂检测",
            "运动员年龄限制由各项目规定",
            "职业运动员可参赛（符合条件）",
            "解放军运动员可代表原籍参赛",
        ]
    }

    /// 资格赛体系
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "资格赛在全运会前一年开始",
            "全国锦标赛成绩作为主要资格来源",
            "全国冠军赛分配部分名额",
            "积分排名决定部分资格",
            "东道主保送部分名额",
            "外卡名额由各协会分配",
            "资格赛时间线：赛前1年开始",
        ]
    }

    /// 参赛名额
    pub fn quota_rules(&self) -> Vec<&'static str> {
        vec![
            "运动员总数限制12000人",
            "各项目名额由国家体育总局确定",
            "每项每单位最多3人（个人项目）",
            "集体球类项目每队名额固定",
            "技术官员由各协会指派",
            "裁判员由国家体育总局选派",
        ]
    }
}

impl Default for NationalGamesQualificationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NationalGamesQualificationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("national_games_qualification")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "全运会参赛资格规则",
            &[
                ("参赛单位", &self.participating_units()),
                ("参赛原则", &self.eligibility_principles()),
                ("资格赛体系", &self.qualification_system()),
                ("名额规则", &self.quota_rules()),
            ],
        )
    }
}

/// 全运会比赛项目规则
#[derive(Debug, Clone)]
pub struct NationalGamesSportsProgramRules {
    metadata: RuleMetadata,
}

impl NationalGamesSportsProgramRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("全运会比赛项目规则", "比赛项目设置和竞赛规则")
                .with_origin("体育总局竞赛规程")
                .with_tags(vec!["体育".into(), "全运会".into(), "项目".into()]),
        }
    }

    /// 夏季项目
    pub fn summer_sports(&self) -> Vec<&'static str> {
        vec![
            "田径、游泳、跳水、花样游泳、水球",
            "羽毛球、篮球、拳击、皮划艇、自行车",
            "马术、击剑、足球、高尔夫、体操",
            "手球、曲棍球、柔道、现代五项、赛艇",
            "七人制橄榄球、帆船、射击、乒乓球",
            "跆拳道、网球、铁人三项、排球、举重",
            "摔跤、射箭、空手道、攀岩、滑板",
            "武术套路、武术散打",
        ]
    }

    /// 冬季项目
    pub fn winter_sports(&self) -> Vec<&'static str> {
        vec![
            "速度滑冰、短道速滑、花样滑冰",
            "冰球、冰壶、雪车",
            "高山滑雪、越野滑雪、跳台滑雪",
            "自由式滑雪、单板滑雪",
            "冬季两项、北欧两项",
        ]
    }

    /// 项目设置规则
    pub fn sports_selection(&self) -> Vec<&'static str> {
        vec![
            "必设项目：所有奥运项目必须设项",
            "选设项目：武术等中国特色项目",
            "表演项目：可设置表演项目",
            "小项设置：参照奥运会和国际比赛",
            "性别平等：男女项目数量平衡",
            "新增项目需国家体育总局批准",
        ]
    }

    /// 竞赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "采用国际单项组织最新竞赛规则",
            "裁判员由国家体育总局选派",
            "技术代表监督比赛进行",
            "抗议和申诉按规定程序",
            "兴奋剂检测严格执行",
            "比赛服装符合规定",
        ]
    }
}

impl Default for NationalGamesSportsProgramRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NationalGamesSportsProgramRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("national_games_sports_program")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "全运会比赛项目规则",
            &[
                ("夏季项目", &self.summer_sports()),
                ("冬季项目", &self.winter_sports()),
                ("项目设置规则", &self.sports_selection()),
                ("竞赛规则", &self.competition_rules()),
            ],
        )
    }
}

/// 全运会奖牌规则
#[derive(Debug, Clone)]
pub struct NationalGamesMedalRules {
    metadata: RuleMetadata,
}

impl NationalGamesMedalRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("全运会奖牌规则", "奖牌颁发和奖励规则")
                .with_origin("体育总局章程")
                .with_tags(vec!["体育".into(), "全运会".into(), "奖牌".into()]),
        }
    }

    /// 奖牌设置
    pub fn medal_system(&self) -> Vec<&'static str> {
        vec![
            "每个项目颁发金、银、铜牌",
            "团体项目每队颁发奖牌",
            "奖牌榜按金牌数排名",
            "金牌相同比银牌，银牌相同比铜牌",
            "并列成绩并列奖牌",
            "奥运冠军计入双计分（2017年起）",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "金牌计1枚金牌",
            "银牌计1枚银牌",
            "铜牌计1枚铜牌",
            "团体项目金牌按人数计入总数",
            "奥运会成绩按1:2计入（金牌=2金）",
            "集体球类项目金牌按2金计入",
        ]
    }

    /// 颁奖仪式
    pub fn award_ceremony(&self) -> Vec<&'static str> {
        vec![
            "颁奖在决赛后立即进行",
            "体育总局领导颁发奖牌",
            "升国旗奏国歌",
            "获奖运动员须穿着正式领奖服",
            "颁奖嘉宾按规定站位",
            "团体项目全体队员上台领奖",
        ]
    }

    /// 团体总成绩
    pub fn team_standings(&self) -> Vec<&'static str> {
        vec![
            "按金牌数排名",
            "公布奖牌榜和总分榜",
            "体育道德风尚奖",
            "代表团贡献奖",
            "东道主通常有主场优势",
            "山东、广东、辽宁为传统强省",
        ]
    }
}

impl Default for NationalGamesMedalRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NationalGamesMedalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("national_games_medal")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "全运会奖牌规则",
            &[
                ("奖牌设置", &self.medal_system()),
                ("计分规则", &self.scoring_rules()),
                ("颁奖仪式", &self.award_ceremony()),
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
        let rules = NationalGamesOrganizationRules::new();
        assert!(!rules.organization_structure().is_empty());
        assert!(rules.games_history().len() >= 7);
    }

    #[test]
    fn test_qualification_rules() {
        let rules = NationalGamesQualificationRules::new();
        assert!(!rules.participating_units().is_empty());
        assert!(rules.eligibility_principles().len() >= 7);
    }

    #[test]
    fn test_sports_program_rules() {
        let rules = NationalGamesSportsProgramRules::new();
        assert!(rules.summer_sports().len() >= 8);
        assert!(rules.winter_sports().len() >= 5);
    }

    #[test]
    fn test_medal_rules() {
        let rules = NationalGamesMedalRules::new();
        assert!(!rules.medal_system().is_empty());
        assert!(rules.scoring_rules().len() >= 6);
    }

    #[test]
    fn test_metadata() {
        let rules = NationalGamesOrganizationRules::new();
        assert_eq!(rules.metadata().name, "全运会组织架构规则");
        assert_eq!(
            rules.category(),
            RuleCategory::sports("national_games_organization")
        );
    }

    #[test]
    fn test_rule_impl() {
        use crate::rules::core::Rule;
        let rules = NationalGamesQualificationRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("参赛"));
        assert!(explanation.contains("资格"));
    }
}