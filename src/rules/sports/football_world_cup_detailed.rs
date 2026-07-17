//! 世界杯详细规则
//!
//! FIFA世界杯足球赛完整规则体系，包括组织架构、参赛资格、小组赛、
//! 淘汰赛、决赛、奖项等完整规则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 世界杯组织架构规则
#[derive(Debug, Clone)]
pub struct WorldCupOrganizationRules {
    metadata: RuleMetadata,
}

impl WorldCupOrganizationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯组织架构规则", "FIFA世界杯组织架构和运营规则")
                .with_origin("FIFA章程")
                .with_tags(vec!["体育".into(), "足球".into(), "世界杯".into()]),
        }
    }

    /// FIFA组织架构
    pub fn fifa_structure(&self) -> Vec<&'static str> {
        vec![
            "FIFA（国际足球联合会）是世界杯最高管理机构",
            "FIFA大会是最高决策机构，每年召开一次",
            "FIFA理事会负责日常事务管理",
            "FIFA主席由大会选举产生，任期4年",
            "6个大洲足球联合会：UEFA、CONMEBOL、CONCACAF、CAF、AFC、OFC",
            "211个成员国/地区足球协会",
        ]
    }

    /// 世界杯历史与演变
    pub fn tournament_history(&self) -> Vec<&'static str> {
        vec![
            "首届世界杯于1930年在乌拉圭举办",
            "每4年举办一届，except 1942-1946因二战停办",
            "参赛队伍从13支(1930)扩展到32支(1998-2022)",
            "2026年起扩展至48支球队参赛",
            "共举办22届(截至2022年卡塔尔世界杯)",
            "巴西5次夺冠历史最多",
            "德国、意大利各4次夺冠",
            "阿根廷、法国、乌拉圭各2次夺冠",
        ]
    }

    /// 主办国遴选
    pub fn host_selection(&self) -> Vec<&'static str> {
        vec![
            "主办国由FIFA大会投票决定",
            "申办国家需提交详细举办方案",
            "FIFA考察团实地评估基础设施",
            "评估标准：场馆、交通、住宿、安保",
            "轮换原则：各大洲轮流举办",
            "2026年起由多国联合举办（美国、加拿大、墨西哥）",
        ]
    }

    /// 组委会规则
    pub fn organizing_committee(&self) -> Vec<&'static str> {
        vec![
            "每届世界杯成立组委会(OC)",
            "组委会由主办国足协和政府共同组建",
            "组委会负责赛事筹备和运营",
            "必须在赛前7年成立",
            "负责场馆建设和设施准备",
            "负责志愿者招募和培训",
            "负责安保和医疗保障",
            "负责媒体服务和广播运营",
        ]
    }
}

impl Default for WorldCupOrganizationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldCupOrganizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_cup_organization")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界杯组织架构规则",
            &[
                ("FIFA架构", &self.fifa_structure()),
                ("历史演变", &self.tournament_history()),
                ("主办国遴选", &self.host_selection()),
                ("组委会规则", &self.organizing_committee()),
            ],
        )
    }
}

/// 世界杯参赛资格规则
#[derive(Debug, Clone)]
pub struct WorldCupQualificationRules {
    metadata: RuleMetadata,
}

impl WorldCupQualificationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯参赛资格规则", "各大洲预选赛和参赛资格规则")
                .with_origin("FIFA章程")
                .with_tags(vec!["体育".into(), "足球".into(), "世界杯".into()]),
        }
    }

    /// 参赛名额分配（32队版本）
    pub fn quota_allocation_32(&self) -> Vec<&'static str> {
        vec![
            "欧洲(UEFA)：13个名额",
            "非洲(CAF)：5个名额",
            "亚洲(AFC)：4.5个名额（4个直接+1个附加赛）",
            "南美洲(CONMEBOL)：4.5个名额（4个直接+1个附加赛）",
            "中北美及加勒比(CONCACAF)：3.5个名额（3个直接+1个附加赛）",
            "大洋洲(OFC)：0.5个名额（需参加附加赛）",
            "东道主：自动获得1个名额",
        ]
    }

    /// 参赛名额分配（48队版本，2026年起）
    pub fn quota_allocation_48(&self) -> Vec<&'static str> {
        vec![
            "欧洲(UEFA)：16个名额",
            "非洲(CAF)：9个名额",
            "亚洲(AFC)：8个名额",
            "南美洲(CONMEBOL)：6个名额",
            "中北美及加勒比(CONCACAF)：6个名额",
            "大洋洲(OFC)：1个名额",
            "东道主：自动获得名额（最多3个东道主）",
            "附加赛：2个名额通过跨洲附加赛决定",
        ]
    }

    /// 预选赛赛制
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "预选赛在世界杯前2-3年开始",
            "各大洲采用不同的预选赛赛制",
            "欧洲：小组赛+附加赛",
            "南美洲：联赛制(所有球队相互比赛)",
            "亚洲：多阶段小组赛+附加赛",
            "非洲：多阶段淘汰赛+小组赛",
            "中北美：多阶段小组赛",
            "大洋洲：淘汰赛制",
        ]
    }

    /// 参赛资格条件
    pub fn eligibility_requirements(&self) -> Vec<&'static str> {
        vec![
            "球员必须持有参赛国国籍",
            "球员必须符合FIFA身份认定规则",
            "21岁后更改国籍需在该国居住满5年",
            "球员不得在预选赛期间代表其他国家参赛",
            "球队必须按时提交报名名单",
            "每队最终名单23-26人（根据赛事规定）",
        ]
    }
}

impl Default for WorldCupQualificationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldCupQualificationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_cup_qualification")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界杯参赛资格规则",
            &[
                ("名额分配(32队)", &self.quota_allocation_32()),
                ("名额分配(48队)", &self.quota_allocation_48()),
                ("预选赛赛制", &self.qualification_system()),
                ("参赛资格条件", &self.eligibility_requirements()),
            ],
        )
    }
}

/// 世界杯小组赛规则
#[derive(Debug, Clone)]
pub struct WorldCupGroupStageRules {
    metadata: RuleMetadata,
}

impl WorldCupGroupStageRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯小组赛规则", "小组赛分组、比赛和排名规则")
                .with_origin("FIFA竞赛规程")
                .with_tags(vec!["体育".into(), "足球".into(), "世界杯".into()]),
        }
    }

    /// 分组规则（32队版本）
    pub fn group_draw_32(&self) -> Vec<&'static str> {
        vec![
            "32支球队分为8个小组(A-H组)",
            "每组4支球队",
            "抽签按FIFA排名分档",
            "第1档：东道主+前7名球队",
            "第2-4档：按排名依次分档",
            "同洲回避原则：欧洲最多2队同组，其他洲不能同组",
            "抽签顺序：第1档→第4档",
        ]
    }

    /// 分组规则（48队版本，2026年起）
    pub fn group_draw_48(&self) -> Vec<&'static str> {
        vec![
            "48支球队分为12个小组",
            "每组4支球队",
            "前2名+8个最佳第3名晋级32强",
            "抽签按FIFA排名分4档",
            "东道主自动进入第1档",
            "同洲回避原则保留",
        ]
    }

    /// 小组赛比赛规则
    pub fn match_rules(&self) -> Vec<&'static str> {
        vec![
            "小组内单循环比赛",
            "每队进行3场比赛",
            "比赛时间：90分钟(上下半场各45分钟)",
            "胜一场得3分，平一场得1分，负一场得0分",
            "小组赛不进行加时赛",
            "积分决定小组排名",
        ]
    }

    /// 排名规则
    pub fn ranking_rules(&self) -> Vec<&'static str> {
        vec![
            "积分高者排名靠前",
            "同分比较顺序：",
            "1. 净胜球",
            "2. 总进球数",
            "3. 相互比赛积分",
            "4. 相互比赛净胜球",
            "5. 相互比赛进球数",
            "6. 公平竞赛积分（黄牌-1，红牌-3）",
            "7. FIFA抽签决定",
        ]
    }

    /// 晋级规则
    pub fn advancement_rules(&self) -> Vec<&'static str> {
        vec![
            "32队制：每组前2名晋级16强",
            "48队制：每组第1名+4个最佳第2名+8个最佳第3名晋级32强",
            "晋级球队进入淘汰赛阶段",
            "小组第3名（32队制）直接淘汰",
        ]
    }
}

impl Default for WorldCupGroupStageRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldCupGroupStageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_cup_group_stage")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界杯小组赛规则",
            &[
                ("分组规则(32队)", &self.group_draw_32()),
                ("分组规则(48队)", &self.group_draw_48()),
                ("比赛规则", &self.match_rules()),
                ("排名规则", &self.ranking_rules()),
                ("晋级规则", &self.advancement_rules()),
            ],
        )
    }
}

/// 世界杯淘汰赛规则
#[derive(Debug, Clone)]
pub struct WorldCupKnockoutRules {
    metadata: RuleMetadata,
}

impl WorldCupKnockoutRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯淘汰赛规则", "16强赛至半决赛的淘汰赛规则")
                .with_origin("FIFA竞赛规程")
                .with_tags(vec!["体育".into(), "足球".into(), "世界杯".into()]),
        }
    }

    /// 淘汰赛对阵规则
    pub fn bracket_rules(&self) -> Vec<&'static str> {
        vec![
            "16强赛：小组第1vs其他组第2",
            "同组球队不能在16强相遇",
            "同一协会球队尽量避开",
            "1/4决赛：16强胜者交叉对阵",
            "半决赛：1/4决赛胜者对阵",
            "淘汰赛采用单场淘汰制",
        ]
    }

    /// 加时赛规则
    pub fn extra_time_rules(&self) -> Vec<&'static str> {
        vec![
            "淘汰赛90分钟平局进入加时赛",
            "加时赛分上下半场各15分钟",
            "加时赛共30分钟",
            "加时赛前有5分钟休息",
            "加时赛半场交换场地",
            "加时赛前可进行第6次换人",
        ]
    }

    /// 点球决胜规则
    pub fn penalty_shootout_rules(&self) -> Vec<&'static str> {
        vec![
            "加时赛平局进行点球决胜",
            "双方各派5名球员轮流罚球",
            "5轮后领先一方获胜",
            "5轮平局则进入单轮决胜",
            "单轮决胜：双方各派1人罚球，领先者胜",
            "罚球球员必须为比赛结束时场上球员",
            "门将必须留在门线上直到球被踢出",
            "门将可以更换（若球队还有换人名额）",
        ]
    }

    /// 换人规则
    pub fn substitution_rules(&self) -> Vec<&'static str> {
        vec![
            "常规时间可换5人",
            "加时赛可额外换第6人",
            "换人必须在3个换人窗口完成",
            "中场休息换人不计入换人窗口",
            "脑震荡换人不计入换人名额",
            "加时赛前可额外换人",
        ]
    }

    /// VAR技术规则
    pub fn var_rules(&self) -> Vec<&'static str> {
        vec![
            "VAR（视频助理裁判）技术全面应用",
            "VAR可介入：进球、点球、红牌、身份错误",
            "VAR裁判由主裁判主动查看",
            "场边监视器供主裁判查看回放",
            "VAR决定由主裁判最终确认",
            "进球后VAR自动检查有效性",
            "点球判罚VAR可介入",
        ]
    }
}

impl Default for WorldCupKnockoutRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldCupKnockoutRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_cup_knockout")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界杯淘汰赛规则",
            &[
                ("对阵规则", &self.bracket_rules()),
                ("加时赛规则", &self.extra_time_rules()),
                ("点球决胜", &self.penalty_shootout_rules()),
                ("换人规则", &self.substitution_rules()),
                ("VAR技术", &self.var_rules()),
            ],
        )
    }
}

/// 世界杯决赛规则
#[derive(Debug, Clone)]
pub struct WorldCupFinalRules {
    metadata: RuleMetadata,
}

impl WorldCupFinalRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯决赛规则", "决赛和三四名决赛规则")
                .with_origin("FIFA竞赛规程")
                .with_tags(vec!["体育".into(), "足球".into(), "世界杯".into()]),
        }
    }

    /// 决赛规则
    pub fn final_match_rules(&self) -> Vec<&'static str> {
        vec![
            "决赛在半决赛胜者之间进行",
            "决赛场地为赛前确定的决赛场馆",
            "决赛90分钟平局进入加时赛",
            "加时赛30分钟(上下半场各15分钟)",
            "加时赛平局进行点球决胜",
            "决赛无重赛，必须分出胜负",
        ]
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> Vec<&'static str> {
        vec![
            "半决赛负者进行三四名决赛",
            "三四名决赛在决赛前一天举行",
            "90分钟平局直接进入点球决胜",
            "不进行加时赛（部分赛事规定）",
            "第三名获得铜牌",
            "第四名无奖牌",
        ]
    }

    /// 开闭幕式规则
    pub fn ceremony_rules(&self) -> Vec<&'static str> {
        vec![
            "开幕式在揭幕战前举行",
            "开幕式包括文艺表演和仪式环节",
            "国际足联主席致辞",
            "东道国元首宣布开幕",
            "闭幕式在决赛后举行",
            "颁奖典礼包括奖杯、奖牌和奖金颁发",
            "冠军队获得大力神杯",
            "冠军队永久保留奖杯复制品",
        ]
    }

    /// 颁奖规则
    pub fn award_ceremony(&self) -> Vec<&'static str> {
        vec![
            "颁奖典礼在决赛结束后立即进行",
            "国际足联主席颁发奖杯",
            "东道国元首出席颁奖",
            "冠军球队获得大力神杯",
            "亚军获得银牌",
            "第三名获得铜牌",
            "金球奖颁发给最佳球员",
            "金靴奖颁发给最佳射手",
            "金手套奖颁发给最佳门将",
            "最佳年轻球员奖",
        ]
    }
}

impl Default for WorldCupFinalRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldCupFinalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_cup_final")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界杯决赛规则",
            &[
                ("决赛规则", &self.final_match_rules()),
                ("三四名决赛", &self.third_place_match()),
                ("开闭幕式", &self.ceremony_rules()),
                ("颁奖规则", &self.award_ceremony()),
            ],
        )
    }
}

/// 世界杯奖项规则
#[derive(Debug, Clone)]
pub struct WorldCupAwardsRules {
    metadata: RuleMetadata,
}

impl WorldCupAwardsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯奖项规则", "个人奖项和团队奖项规则")
                .with_origin("FIFA竞赛规程")
                .with_tags(vec!["体育".into(), "足球".into(), "世界杯".into()]),
        }
    }

    /// 团队奖项
    pub fn team_awards(&self) -> Vec<&'static str> {
        vec![
            "冠军：大力神杯、金牌、奖金4200万美元",
            "亚军：银牌、奖金3000万美元",
            "第三名：铜牌、奖金2700万美元",
            "第四名：奖金2500万美元",
            "16强：奖金1300万美元",
            "小组赛：奖金900万美元",
            "公平竞赛奖：红黄牌最少的球队",
        ]
    }

    /// 个人奖项
    pub fn individual_awards(&self) -> Vec<&'static str> {
        vec![
            "金球奖：赛事最佳球员(记者投票)",
            "银球奖：第二佳球员",
            "铜球奖：第三佳球员",
            "金靴奖：最佳射手(进球+助攻优先)",
            "银靴奖：第二射手",
            "铜靴奖：第三射手",
            "金手套奖：最佳门将",
            "最佳年轻球员奖：21岁以下最佳",
        ]
    }

    /// 金靴奖计算规则
    pub fn golden_boot_rules(&self) -> Vec<&'static str> {
        vec![
            "进球数决定排名",
            "进球相同比较助攻数",
            "助攻相同比较上场时间（少者优先）",
            "小组赛进球权重与其他比赛相同",
            "点球进球计入总进球",
            "需至少参加淘汰赛阶段",
        ]
    }

    /// 金球奖评选规则
    pub fn golden_ball_rules(&self) -> Vec<&'static str> {
        vec![
            "由媒体代表投票选出",
            "评选范围：所有参赛球员",
            "投票在决赛后进行",
            "投票者来自各大洲媒体",
            "考虑球员整体表现和影响力",
            "冠军球队球员通常有优势",
        ]
    }

    /// 纪录与统计
    pub fn records_statistics(&self) -> Vec<&'static str> {
        vec![
            "最多进球：克洛泽16球(德国)",
            "最多出场：马特乌斯25场(德国)",
            "最多夺冠：巴西5次",
            "最年轻进球：贝利17岁239天",
            "最年长进球：罗杰·米拉42岁39天",
            "单届最多进球：方丹13球(1958)",
            "最多帽子戏法：4人并列",
            "最快进球：哈坎·苏克10.8秒",
        ]
    }
}

impl Default for WorldCupAwardsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WorldCupAwardsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("world_cup_awards")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "世界杯奖项规则",
            &[
                ("团队奖项", &self.team_awards()),
                ("个人奖项", &self.individual_awards()),
                ("金靴奖规则", &self.golden_boot_rules()),
                ("金球奖规则", &self.golden_ball_rules()),
                ("纪录统计", &self.records_statistics()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_rules() {
        let rules = WorldCupOrganizationRules::new();
        assert!(!rules.fifa_structure().is_empty());
        assert!(rules.explain().contains("FIFA"));
    }

    #[test]
    fn test_qualification_rules() {
        let rules = WorldCupQualificationRules::new();
        let quota_32 = rules.quota_allocation_32();
        assert_eq!(quota_32.len(), 7);
        let quota_48 = rules.quota_allocation_48();
        assert_eq!(quota_48.len(), 8);
    }

    #[test]
    fn test_group_stage_rules() {
        let rules = WorldCupGroupStageRules::new();
        let group_draw = rules.group_draw_32();
        assert!(group_draw.contains(&"32支球队分为8个小组(A-H组)"));
        assert_eq!(rules.ranking_rules().len(), 9);
    }

    #[test]
    fn test_knockout_rules() {
        let rules = WorldCupKnockoutRules::new();
        assert!(!rules.extra_time_rules().is_empty());
        assert!(rules.penalty_shootout_rules().len() >= 8);
    }

    #[test]
    fn test_final_rules() {
        let rules = WorldCupFinalRules::new();
        assert!(!rules.final_match_rules().is_empty());
        assert!(rules.award_ceremony().len() >= 10);
    }

    #[test]
    fn test_awards_rules() {
        let rules = WorldCupAwardsRules::new();
        assert!(!rules.team_awards().is_empty());
        assert!(rules.individual_awards().len() >= 8);
    }

    #[test]
    fn test_metadata() {
        let rules = WorldCupOrganizationRules::new();
        assert_eq!(rules.metadata().name, "世界杯组织架构规则");
        assert_eq!(
            rules.category(),
            RuleCategory::sports("world_cup_organization")
        );
    }

    #[test]
    fn test_rule_impl() {
        use crate::rules::core::Rule;
        let rules = WorldCupGroupStageRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("小组赛"));
        assert!(explanation.contains("排名"));
    }
}
