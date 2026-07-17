//! 奥运会详细规则
//!
//! 奥林匹克运动会完整规则体系，包括组织架构、参赛资格、赛事管理、
//! 竞技规则、奖牌体系、开闭幕式等完整规则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 奥运会组织规则
#[derive(Debug, Clone)]
pub struct OlympicOrganizationRules {
    metadata: RuleMetadata,
}

impl OlympicOrganizationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会组织规则", "国际奥委会组织架构和运营规则")
                .with_origin("国际奥委会章程")
                .with_tags(vec!["体育".into(), "奥运".into(), "组织".into()]),
        }
    }

    /// 国际奥委会组织架构
    pub fn ioc_structure(&self) -> Vec<&'static str> {
        vec![
            "国际奥委会(IOC)是奥林匹克运动的最高管理机构",
            "IOC全会是最高决策机构，每年召开一次",
            "IOC执行委员会负责日常事务管理",
            "IOC主席由全会选举产生，任期8年",
            "IOC委员由全会选举，最多115名",
            "IOC委员必须代表奥林匹克运动利益",
        ]
    }

    /// 国际单项体育组织
    pub fn international_federations(&self) -> Vec<&'static str> {
        vec![
            "夏季奥运会包含28个大项、33个小项",
            "冬季奥运会包含7个大项",
            "每项运动由国际单项体育联合会(IFs)管理",
            "IFs必须获得IOC承认才能进入奥运会",
            "IFs负责制定各自运动的技术规则",
            "IFs组织奥运资格赛",
            "IFs派遣技术官员和裁判",
        ]
    }

    /// 国家奥委会
    pub fn national_committees(&self) -> Vec<&'static str> {
        vec![
            "206个国家/地区奥委会(NOCs)",
            "NOCs负责本国/地区运动员参赛",
            "NOCs选拔和管理代表团",
            "NOCs组织本国奥运选拔赛",
            "NOCs负责运动员注册和报名",
            "NOCs获得IOC资助和分配名额",
        ]
    }

    /// 组委会规则
    pub fn organizing_committee(&self) -> Vec<&'static str> {
        vec![
            "每届奥运会成立组委会(OCOG)",
            "组委会由主办城市和国家奥委会共同组建",
            "组委会负责赛事筹备和运营",
            "组委会必须在赛前7年成立",
            "组委会受IOC执行委员会监督",
            "组委会负责场馆建设和设施准备",
            "组委会负责志愿者招募和培训",
            "组委会负责奥运村运营",
        ]
    }
}

impl Default for OlympicOrganizationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OlympicOrganizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("olympic_organization")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会组织规则",
            &[
                ("国际奥委会架构", &self.ioc_structure()),
                ("国际单项体育组织", &self.international_federations()),
                ("国家奥委会", &self.national_committees()),
                ("组委会规则", &self.organizing_committee()),
            ],
        )
    }
}

/// 奥运会参赛资格规则
#[derive(Debug, Clone)]
pub struct OlympicQualificationRules {
    metadata: RuleMetadata,
}

impl OlympicQualificationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会参赛资格规则", "运动员和代表团的参赛资格规则")
                .with_origin("奥林匹克宪章")
                .with_tags(vec!["体育".into(), "奥运".into(), "资格".into()]),
        }
    }

    /// 参赛资格基本原则
    pub fn qualification_principles(&self) -> Vec<&'static str> {
        vec![
            "运动员必须代表其国籍所属国家/地区参赛",
            "运动员不得在奥运会期间接受商业赞助",
            "运动员必须遵守《奥林匹克反兴奋剂条例》",
            "运动员必须签署《参赛资格条件》",
            "运动员年龄限制由各IFs规定",
            "运动员必须参加资格赛或获得外卡",
        ]
    }

    /// 参赛名额分配
    pub fn quota_allocation(&self) -> Vec<&'static str> {
        vec![
            "夏季奥运会运动员总数限制10500人",
            "冬季奥运会运动员总数限制2900人",
            "各项目名额由IOC和IFs协商确定",
            "东道主自动获得部分名额",
            "各国代表团人数由NOC决定",
            "性别平等：男女运动员名额对等",
            "新增项目必须保证性别平等",
        ]
    }

    /// 资格赛体系
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "世界锦标赛成绩作为主要资格来源",
            "洲际锦标赛分配部分名额",
            "奥运积分排名系统",
            "世界排名决定部分资格",
            "东道主保送名额",
            "外卡/邀请名额(三方委员会分配)",
            "资格赛时间线：赛前1年开始",
        ]
    }

    /// 代表团组成规则
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "代表团由运动员和官员组成",
            "团长负责代表团整体管理",
            "教练员人数由各项目规定",
            "队医、理疗师按运动员比例配备",
            "代表团官员总数不得超过运动员50%",
            "技术官员由IFs指派，不属代表团",
        ]
    }
}

impl Default for OlympicQualificationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OlympicQualificationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("olympic_qualification")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会参赛资格规则",
            &[
                ("参赛基本原则", &self.qualification_principles()),
                ("名额分配", &self.quota_allocation()),
                ("资格赛体系", &self.qualification_system()),
                ("代表团组成", &self.team_composition()),
            ],
        )
    }
}

/// 奥运会竞赛规则
#[derive(Debug, Clone)]
pub struct OlympicCompetitionRules {
    metadata: RuleMetadata,
}

impl OlympicCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会竞赛规则", "奥运赛事竞赛组织规则")
                .with_origin("奥林匹克宪章")
                .with_tags(vec!["体育".into(), "奥运".into(), "竞赛".into()]),
        }
    }

    /// 赛程安排规则
    pub fn schedule_rules(&self) -> Vec<&'static str> {
        vec![
            "夏季奥运会比赛期16天",
            "冬季奥运会比赛期17天",
            "开幕式为第一天晚上",
            "闭幕式为最后一天晚上",
            "每天比赛时间通常08:00-23:00",
            "热门项目安排在黄金时段",
            "跨时区转播需考虑全球观众",
        ]
    }

    /// 抽签和分组规则
    pub fn draw_rules(&self) -> Vec<&'static str> {
        vec![
            "抽签由IFs主持，IOC监督",
            "种子排名根据世界排名确定",
            "东道主享有一定优势",
            "同协会运动员分组避开",
            "抽签公开进行，媒体可旁听",
            "特殊项目采用蛇形分组",
        ]
    }

    /// 比赛场地规则
    pub fn venue_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛场地必须符合IFs标准",
            "场馆必须通过IOC验收",
            "训练场地数量和质量要求",
            "热身场地距离比赛场地不超过30分钟",
            "场馆必须配备无障碍设施",
            "场馆必须配备反兴奋剂检测室",
            "媒体中心和混合采访区要求",
        ]
    }

    /// 裁判和技术官员规则
    pub fn officiating_rules(&self) -> Vec<&'static str> {
        vec![
            "技术官员由IFs选派",
            "裁判必须持有国际裁判证",
            "裁判不得执法本国运动员比赛(特定项目)",
            "裁判人数由各项目规则确定",
            "裁判需参加赛前培训",
            "争议判罚可申请仲裁",
        ]
    }

    /// 比赛纪律规则
    pub fn discipline_rules(&self) -> Vec<&'static str> {
        vec![
            "参赛运动员必须遵守运动员誓言",
            "不得有任何形式的歧视行为",
            "不得进行政治宣传",
            "不得故意消极比赛",
            "不得操纵比赛结果",
            "违反纪律可被取消成绩",
            "严重违规可被终身禁赛",
        ]
    }
}

impl Default for OlympicCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OlympicCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("olympic_competition")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会竞赛规则",
            &[
                ("赛程安排", &self.schedule_rules()),
                ("抽签分组", &self.draw_rules()),
                ("比赛场地", &self.venue_rules()),
                ("裁判技术官员", &self.officiating_rules()),
                ("比赛纪律", &self.discipline_rules()),
            ],
        )
    }
}

/// 奥运会奖牌规则
#[derive(Debug, Clone)]
pub struct OlympicMedalRules {
    metadata: RuleMetadata,
}

impl OlympicMedalRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会奖牌规则", "奥运奖牌颁发和排名规则")
                .with_origin("奥林匹克宪章")
                .with_tags(vec!["体育".into(), "奥运".into(), "奖牌".into()]),
        }
    }

    /// 奖牌设计规则
    pub fn medal_design(&self) -> Vec<&'static str> {
        vec![
            "金牌：至少含金6克，直径至少60mm",
            "银牌：纯银制作，直径至少60mm",
            "铜牌：铜合金制作，直径至少60mm",
            "奖牌正面必须包含奥运五环",
            "奖牌正面必须包含运动项目名称",
            "组委会可自定义奖牌背面设计",
        ]
    }

    /// 颁奖仪式规则
    pub fn ceremony_rules(&self) -> Vec<&'static str> {
        vec![
            "颁奖仪式在比赛结束后立即进行",
            "前三名运动员参加颁奖仪式",
            "升获奖运动员国家/地区旗帜",
            "奏冠军所属国家/地区国歌",
            "由IOC委员或IFs代表颁奖",
            "运动员必须穿着比赛服或领奖服",
            "颁奖顺序：铜牌、银牌、金牌",
        ]
    }

    /// 奖牌榜规则
    pub fn medal_table_rules(&self) -> Vec<&'static str> {
        vec![
            "官方不设国家奖牌榜排名",
            "媒体可自行统计奖牌榜",
            "金牌优先排名为常用方式",
            "奖牌总数排名为另一种方式",
            "IOC尊重各国媒体统计方式",
            "团体项目计为1枚奖牌",
        ]
    }

    /// 并列奖牌规则
    pub fn tie_rules(&self) -> Vec<&'static str> {
        vec![
            "田径跳跃投掷项目可并列冠军",
            "游泳决赛可并列奖牌",
            "体操单项可并列奖牌",
            "并列奖牌时下一排名空缺",
            "团体项目不得并列",
            "半决赛并列时采用抽签",
        ]
    }
}

impl Default for OlympicMedalRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OlympicMedalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("olympic_medal")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会奖牌规则",
            &[
                ("奖牌设计", &self.medal_design()),
                ("颁奖仪式", &self.ceremony_rules()),
                ("奖牌榜", &self.medal_table_rules()),
                ("并列奖牌", &self.tie_rules()),
            ],
        )
    }
}

/// 奥运会开闭幕式规则
#[derive(Debug, Clone)]
pub struct OlympicCeremonyRules {
    metadata: RuleMetadata,
}

impl OlympicCeremonyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会开闭幕式规则", "奥运开闭幕式仪式规则")
                .with_origin("奥林匹克宪章")
                .with_tags(vec!["体育".into(), "奥运".into(), "仪式".into()]),
        }
    }

    /// 开幕式必需环节
    pub fn opening_essential(&self) -> Vec<&'static str> {
        vec![
            "IOC主席和东道主国家元首入场",
            "各代表团入场(希腊首先，东道主最后)",
            "IOC主席致辞",
            "东道主国家元首宣布开幕",
            "奥运五环旗入场并升起",
            "奥运圣火点燃",
            "运动员、裁判、教练代表宣誓",
            "和平鸽象征性放飞(鸽子或影像)",
        ]
    }

    /// 闭幕式必需环节
    pub fn closing_essential(&self) -> Vec<&'static str> {
        vec![
            "各代表团混合入场",
            "颁奖仪式(男子马拉松为传统项目)",
            "新主办城市接旗仪式",
            "IOC主席宣布闭幕",
            "奥运圣火熄灭",
            "五环旗降下",
            "下届主办城市文艺表演",
            "奥运会旗移交下届主办城市",
        ]
    }

    /// 圣火传递规则
    pub fn torch_relay(&self) -> Vec<&'static str> {
        vec![
            "圣火在希腊奥林匹亚采集",
            "采集仪式由女祭司主持",
            "利用凹面镜聚焦阳光引燃",
            "圣火通过飞机运送到主办国",
            "传递路线覆盖主办国主要城市",
            "传递时间通常100天左右",
            "传递方式包括跑步、自行车、船只等",
            "传递期间圣火不得熄灭",
            "备用火种灯随队携带",
        ]
    }

    /// 文艺表演规则
    pub fn artistic_performance(&self) -> Vec<&'static str> {
        vec![
            "开闭幕式文艺表演由组委会设计",
            "表演需展现东道主文化特色",
            "表演内容需IOC批准",
            "表演时间开幕式约4小时",
            "表演时间闭幕式约2.5小时",
            "表演必须包含奥林匹克精神元素",
            "禁止任何政治宣传内容",
        ]
    }
}

impl Default for OlympicCeremonyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OlympicCeremonyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("olympic_ceremony")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会开闭幕式规则",
            &[
                ("开幕式必需环节", &self.opening_essential()),
                ("闭幕式必需环节", &self.closing_essential()),
                ("圣火传递", &self.torch_relay()),
                ("文艺表演", &self.artistic_performance()),
            ],
        )
    }
}

/// 奥运会反兴奋剂规则
#[derive(Debug, Clone)]
pub struct OlympicAntiDopingRules {
    metadata: RuleMetadata,
}

impl OlympicAntiDopingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会反兴奋剂规则", "奥运反兴奋剂检测和处罚规则")
                .with_origin("WADA反兴奋剂条例")
                .with_tags(vec!["体育".into(), "奥运".into(), "反兴奋剂".into()]),
        }
    }

    /// 检测规则
    pub fn testing_rules(&self) -> Vec<&'static str> {
        vec![
            "赛内检测：颁奖运动员必须检测",
            "赛内检测：随机抽取其他运动员",
            "赛外检测：随时可能进行",
            "尿检为基本检测方式",
            "血检用于特定物质检测",
            "样本分为A瓶和B瓶",
            "B瓶检测需运动员或代表在场",
            "检测实验室必须获得WADA认证",
        ]
    }

    /// 违禁物质
    pub fn prohibited_substances(&self) -> Vec<&'static str> {
        vec![
            "蛋白同化制剂(类固醇类)",
            "肽类激素和相关物质",
            "β2激动剂",
            "激素拮抗剂和调节剂",
            "利尿剂和其他掩蔽剂",
            "兴奋剂",
            "麻醉剂",
            "大麻素类",
            "糖皮质激素(赛内禁用)",
            "β受体阻断剂(特定项目)",
        ]
    }

    /// 违规处罚
    pub fn sanctions(&self) -> Vec<&'static str> {
        vec![
            "首次严重违规：禁赛4年",
            "首次一般违规：禁赛2年",
            "奖牌获得者违规：取消成绩和奖牌",
            "团体项目：可能取消全队成绩",
            "未成年运动员：处罚可减轻",
            "主动配合调查：处罚可减轻",
            "累犯：禁赛期加倍",
            "终身禁赛适用于极严重违规",
        ]
    }

    /// 治疗用药豁免
    pub fn therapeutic_exemption(&self) -> Vec<&'static str> {
        vec![
            "运动员因病需使用违禁物质可申请TUE",
            "TUE必须赛前申请(紧急情况除外)",
            "TUE由IFs或NADO审批",
            "TUE必须证明药物必需性",
            "TUE不得用于提升运动表现",
            "WADA可复议TUE决定",
        ]
    }
}

impl Default for OlympicAntiDopingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OlympicAntiDopingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("olympic_anti_doping")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会反兴奋剂规则",
            &[
                ("检测规则", &self.testing_rules()),
                ("违禁物质", &self.prohibited_substances()),
                ("违规处罚", &self.sanctions()),
                ("治疗用药豁免", &self.therapeutic_exemption()),
            ],
        )
    }
}

/// 奥运会仲裁规则
#[derive(Debug, Clone)]
pub struct OlympicArbitrationRules {
    metadata: RuleMetadata,
}

impl OlympicArbitrationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会仲裁规则", "国际体育仲裁法院奥运仲裁规则")
                .with_origin("CAS奥运仲裁规则")
                .with_tags(vec!["体育".into(), "奥运".into(), "仲裁".into()]),
        }
    }

    /// CAS管辖范围
    pub fn cas_jurisdiction(&self) -> Vec<&'static str> {
        vec![
            "CAS奥运仲裁分院(Ad Hoc Division)管辖奥运争议",
            "管辖范围包括资格争议、纪律处罚、成绩认定",
            "必须在奥运期间提交申请",
            "必须在颁奖前作出裁决",
            "仲裁裁决为终局裁决",
            "可在赛后向CAS普通分院上诉",
        ]
    }

    /// 仲裁程序
    pub fn arbitration_procedure(&self) -> Vec<&'static str> {
        vec![
            "申请必须书面提交",
            "申请书需说明事实、法律依据、请求",
            "仲裁庭由1名或3名仲裁员组成",
            "仲裁员由CAS主席指定",
            "被申请人有权答辩",
            "仲裁庭可举行听证",
            "裁决必须在24小时内作出(紧急)",
            "裁决必须说明理由",
        ]
    }

    /// 争议类型
    pub fn dispute_types(&self) -> Vec<&'static str> {
        vec![
            "参赛资格争议",
            "比赛结果争议",
            "裁判判罚争议",
            "兴奋剂处罚争议",
            "纪律处罚争议",
            "技术规则解释争议",
        ]
    }

    /// 裁决执行
    pub fn enforcement(&self) -> Vec<&'static str> {
        vec![
            "裁决自送达之日起生效",
            "IOC、IFs、NOCs必须执行裁决",
            "不服裁决可向瑞士联邦法院上诉",
            "上诉理由仅限于程序违法",
            "上诉不影响裁决执行",
        ]
    }
}

impl Default for OlympicArbitrationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OlympicArbitrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("olympic_arbitration")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会仲裁规则",
            &[
                ("CAS管辖范围", &self.cas_jurisdiction()),
                ("仲裁程序", &self.arbitration_procedure()),
                ("争议类型", &self.dispute_types()),
                ("裁决执行", &self.enforcement()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_olympic_organization_rules() {
        let rules = OlympicOrganizationRules::new();
        assert!(!rules.ioc_structure().is_empty());
        assert!(!rules.international_federations().is_empty());
        assert!(!rules.national_committees().is_empty());
        assert!(!rules.organizing_committee().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_olympic_qualification_rules() {
        let rules = OlympicQualificationRules::new();
        assert!(!rules.qualification_principles().is_empty());
        assert!(!rules.quota_allocation().is_empty());
        assert!(!rules.qualification_system().is_empty());
        assert!(!rules.team_composition().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_olympic_competition_rules() {
        let rules = OlympicCompetitionRules::new();
        assert!(!rules.schedule_rules().is_empty());
        assert!(!rules.draw_rules().is_empty());
        assert!(!rules.venue_rules().is_empty());
        assert!(!rules.officiating_rules().is_empty());
        assert!(!rules.discipline_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_olympic_medal_rules() {
        let rules = OlympicMedalRules::new();
        assert!(!rules.medal_design().is_empty());
        assert!(!rules.ceremony_rules().is_empty());
        assert!(!rules.medal_table_rules().is_empty());
        assert!(!rules.tie_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_olympic_ceremony_rules() {
        let rules = OlympicCeremonyRules::new();
        assert!(!rules.opening_essential().is_empty());
        assert!(!rules.closing_essential().is_empty());
        assert!(!rules.torch_relay().is_empty());
        assert!(!rules.artistic_performance().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_olympic_anti_doping_rules() {
        let rules = OlympicAntiDopingRules::new();
        assert!(!rules.testing_rules().is_empty());
        assert!(!rules.prohibited_substances().is_empty());
        assert!(!rules.sanctions().is_empty());
        assert!(!rules.therapeutic_exemption().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_olympic_arbitration_rules() {
        let rules = OlympicArbitrationRules::new();
        assert!(!rules.cas_jurisdiction().is_empty());
        assert!(!rules.arbitration_procedure().is_empty());
        assert!(!rules.dispute_types().is_empty());
        assert!(!rules.enforcement().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_all_rules_validate() {
        assert!(OlympicOrganizationRules::new()
            .validate(&ValidateContext::default())
            .is_ok());
        assert!(OlympicQualificationRules::new()
            .validate(&ValidateContext::default())
            .is_ok());
        assert!(OlympicCompetitionRules::new()
            .validate(&ValidateContext::default())
            .is_ok());
        assert!(OlympicMedalRules::new()
            .validate(&ValidateContext::default())
            .is_ok());
        assert!(OlympicCeremonyRules::new()
            .validate(&ValidateContext::default())
            .is_ok());
        assert!(OlympicAntiDopingRules::new()
            .validate(&ValidateContext::default())
            .is_ok());
        assert!(OlympicArbitrationRules::new()
            .validate(&ValidateContext::default())
            .is_ok());
    }
}
