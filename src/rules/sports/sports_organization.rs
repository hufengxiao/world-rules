//! 体育组织规则
//!
//! 包含国际体育组织、国家体育组织、体育协会、体育俱乐部等规则体系。
//!
//! # 规则体系
//!
//! - 国际体育组织（IOC、国际单项联合会）
//! - 国家体育组织（国家奥委会、国家单项协会）
//! - 体育协会（职业联盟、业余协会）
//! - 体育俱乐部（职业俱乐部、业余俱乐部）
//!
//! # Examples
//!
//! ```
//! use world_rules::rules::sports::sports_organization::SportsOrganizationRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = SportsOrganizationRules::new();
//! assert!(!rules.international_federations().is_empty());
//! assert!(rules.ioc_members_count() > 100);
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 组织类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganizationType {
    /// 国际奥委会
    InternationalOlympicCommittee,
    /// 国际单项体育联合会
    InternationalFederation,
    /// 国家奥委会
    NationalOlympicCommittee,
    /// 国家单项协会
    NationalFederation,
    /// 职业联盟
    ProfessionalLeague,
    /// 业余协会
    AmateurAssociation,
    /// 职业俱乐部
    ProfessionalClub,
    /// 业余俱乐部
    AmateurClub,
}

impl OrganizationType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            OrganizationType::InternationalOlympicCommittee => "国际奥委会",
            OrganizationType::InternationalFederation => "国际单项体育联合会",
            OrganizationType::NationalOlympicCommittee => "国家奥委会",
            OrganizationType::NationalFederation => "国家单项协会",
            OrganizationType::ProfessionalLeague => "职业联盟",
            OrganizationType::AmateurAssociation => "业余协会",
            OrganizationType::ProfessionalClub => "职业俱乐部",
            OrganizationType::AmateurClub => "业余俱乐部",
        }
    }

    /// 是否为国际组织
    pub fn is_international(&self) -> bool {
        matches!(
            self,
            OrganizationType::InternationalOlympicCommittee
                | OrganizationType::InternationalFederation
        )
    }

    /// 是否为国家组织
    pub fn is_national(&self) -> bool {
        matches!(
            self,
            OrganizationType::NationalOlympicCommittee | OrganizationType::NationalFederation
        )
    }

    /// 是否为职业组织
    pub fn is_professional(&self) -> bool {
        matches!(
            self,
            OrganizationType::ProfessionalLeague | OrganizationType::ProfessionalClub
        )
    }
}

/// 国际单项体育联合会
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InternationalFederation {
    /// 国际足联
    FIFA,
    /// 国际篮联
    FIBA,
    /// 国际泳联
    FINA,
    /// 国际田联
    WorldAthletics,
    /// 国际自盟
    UCI,
    /// 国际网联
    ITF,
    /// 国际乒联
    ITTF,
    /// 国际羽联
    BWF,
    /// 国际排联
    FIVB,
    /// 国际冰联
    IIHF,
    /// 国际体操联合会
    FIG,
    /// 国际举重联合会
    IWF,
    /// 国际拳击联合会
    IBA,
    /// 国际柔道联合会
    IJF,
    /// 世界跆拳道
    WT,
}

impl InternationalFederation {
    /// 获取名称
    pub fn name(&self) -> &'static str {
        match self {
            InternationalFederation::FIFA => "国际足联",
            InternationalFederation::FIBA => "国际篮联",
            InternationalFederation::FINA => "国际泳联",
            InternationalFederation::WorldAthletics => "国际田联",
            InternationalFederation::UCI => "国际自盟",
            InternationalFederation::ITF => "国际网联",
            InternationalFederation::ITTF => "国际乒联",
            InternationalFederation::BWF => "国际羽联",
            InternationalFederation::FIVB => "国际排联",
            InternationalFederation::IIHF => "国际冰联",
            InternationalFederation::FIG => "国际体操联合会",
            InternationalFederation::IWF => "国际举重联合会",
            InternationalFederation::IBA => "国际拳击联合会",
            InternationalFederation::IJF => "国际柔道联合会",
            InternationalFederation::WT => "世界跆拳道",
        }
    }

    /// 获取英文全称
    pub fn full_name_en(&self) -> &'static str {
        match self {
            InternationalFederation::FIFA => "Fédération Internationale de Football Association",
            InternationalFederation::FIBA => "Fédération Internationale de Basketball",
            InternationalFederation::FINA => "Fédération Internationale de Natation",
            InternationalFederation::WorldAthletics => "World Athletics",
            InternationalFederation::UCI => "Union Cycliste Internationale",
            InternationalFederation::ITF => "International Tennis Federation",
            InternationalFederation::ITTF => "International Table Tennis Federation",
            InternationalFederation::BWF => "Badminton World Federation",
            InternationalFederation::FIVB => "Fédération Internationale de Volleyball",
            InternationalFederation::IIHF => "International Ice Hockey Federation",
            InternationalFederation::FIG => "Fédération Internationale de Gymnastique",
            InternationalFederation::IWF => "International Weightlifting Federation",
            InternationalFederation::IBA => "International Boxing Association",
            InternationalFederation::IJF => "International Judo Federation",
            InternationalFederation::WT => "World Taekwondo",
        }
    }

    /// 获取总部所在地
    pub fn headquarters(&self) -> (&'static str, &'static str) {
        match self {
            InternationalFederation::FIFA => ("瑞士", "苏黎世"),
            InternationalFederation::FIBA => ("瑞士", "米村"),
            InternationalFederation::FINA => ("瑞士", "洛桑"),
            InternationalFederation::WorldAthletics => ("摩纳哥", "蒙特卡洛"),
            InternationalFederation::UCI => ("瑞士", "艾格勒"),
            InternationalFederation::ITF => ("英国", "伦敦"),
            InternationalFederation::ITTF => ("瑞士", "洛桑"),
            InternationalFederation::BWF => ("马来西亚", "吉隆坡"),
            InternationalFederation::FIVB => ("瑞士", "洛桑"),
            InternationalFederation::IIHF => ("瑞士", "苏黎世"),
            InternationalFederation::FIG => ("瑞士", "洛桑"),
            InternationalFederation::IWF => ("瑞士", "洛桑"),
            InternationalFederation::IBA => ("瑞士", "洛桑"),
            InternationalFederation::IJF => ("瑞士", "洛桑"),
            InternationalFederation::WT => ("韩国", "首尔"),
        }
    }

    /// 获取成立年份
    pub fn founded_year(&self) -> u32 {
        match self {
            InternationalFederation::FIFA => 1904,
            InternationalFederation::FIBA => 1932,
            InternationalFederation::FINA => 1908,
            InternationalFederation::WorldAthletics => 1912,
            InternationalFederation::UCI => 1900,
            InternationalFederation::ITF => 1913,
            InternationalFederation::ITTF => 1926,
            InternationalFederation::BWF => 1934,
            InternationalFederation::FIVB => 1947,
            InternationalFederation::IIHF => 1908,
            InternationalFederation::FIG => 1881,
            InternationalFederation::IWF => 1905,
            InternationalFederation::IBA => 1946,
            InternationalFederation::IJF => 1951,
            InternationalFederation::WT => 1973,
        }
    }

    /// 获取会员协会数量（估算）
    pub fn member_count(&self) -> u32 {
        match self {
            InternationalFederation::FIFA => 211,
            InternationalFederation::FIBA => 213,
            InternationalFederation::FINA => 209,
            InternationalFederation::WorldAthletics => 231,
            InternationalFederation::UCI => 198,
            InternationalFederation::ITF => 213,
            InternationalFederation::ITTF => 227,
            InternationalFederation::BWF => 176,
            InternationalFederation::FIVB => 222,
            InternationalFederation::IIHF => 83,
            InternationalFederation::FIG => 156,
            InternationalFederation::IWF => 193,
            InternationalFederation::IBA => 198,
            InternationalFederation::IJF => 205,
            InternationalFederation::WT => 211,
        }
    }
}

/// 国家奥委会职责
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NOCResponsibility {
    /// 选拔奥运代表队
    TeamSelection,
    /// 组织奥运备战
    OlympicPreparation,
    /// 推广奥林匹克运动
    OlympicPromotion,
    /// 发展青少年体育
    YouthDevelopment,
    /// 反兴奋剂教育
    AntiDopingEducation,
    /// 运动员权益保护
    AthleteWelfare,
    /// 国际体育交流
    InternationalExchange,
}

impl NOCResponsibility {
    /// 获取职责名称
    pub fn name(&self) -> &'static str {
        match self {
            NOCResponsibility::TeamSelection => "选拔奥运代表队",
            NOCResponsibility::OlympicPreparation => "组织奥运备战",
            NOCResponsibility::OlympicPromotion => "推广奥林匹克运动",
            NOCResponsibility::YouthDevelopment => "发展青少年体育",
            NOCResponsibility::AntiDopingEducation => "反兴奋剂教育",
            NOCResponsibility::AthleteWelfare => "运动员权益保护",
            NOCResponsibility::InternationalExchange => "国际体育交流",
        }
    }

    /// 获取优先级
    pub fn priority(&self) -> u32 {
        match self {
            NOCResponsibility::TeamSelection => 1,
            NOCResponsibility::OlympicPreparation => 2,
            NOCResponsibility::AntiDopingEducation => 3,
            NOCResponsibility::AthleteWelfare => 4,
            NOCResponsibility::YouthDevelopment => 5,
            NOCResponsibility::OlympicPromotion => 6,
            NOCResponsibility::InternationalExchange => 7,
        }
    }
}

/// 职业联盟类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfessionalLeagueType {
    /// 足球联赛
    FootballLeague,
    /// 篮球联赛
    BasketballLeague,
    /// 棒球联赛
    BaseballLeague,
    /// 冰球联赛
    HockeyLeague,
    /// 网球巡回赛
    TennisTour,
    /// 高尔夫巡回赛
    GolfTour,
    /// 综合格斗联盟
    MMALeague,
    /// 电子竞技联赛
    EsportsLeague,
}

impl ProfessionalLeagueType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            ProfessionalLeagueType::FootballLeague => "足球联赛",
            ProfessionalLeagueType::BasketballLeague => "篮球联赛",
            ProfessionalLeagueType::BaseballLeague => "棒球联赛",
            ProfessionalLeagueType::HockeyLeague => "冰球联赛",
            ProfessionalLeagueType::TennisTour => "网球巡回赛",
            ProfessionalLeagueType::GolfTour => "高尔夫巡回赛",
            ProfessionalLeagueType::MMALeague => "综合格斗联盟",
            ProfessionalLeagueType::EsportsLeague => "电子竞技联赛",
        }
    }

    /// 是否有工资帽
    pub fn has_salary_cap(&self) -> bool {
        matches!(
            self,
            ProfessionalLeagueType::BasketballLeague
                | ProfessionalLeagueType::FootballLeague
                | ProfessionalLeagueType::HockeyLeague
        )
    }

    /// 是否有选秀制度
    pub fn has_draft_system(&self) -> bool {
        matches!(
            self,
            ProfessionalLeagueType::BasketballLeague
                | ProfessionalLeagueType::FootballLeague
                | ProfessionalLeagueType::BaseballLeague
                | ProfessionalLeagueType::HockeyLeague
        )
    }
}

/// 俱乐部运营领域
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClubOperationArea {
    /// 青训体系
    YouthDevelopment,
    /// 球员转会
    PlayerTransfer,
    /// 商业开发
    CommercialDevelopment,
    /// 球迷服务
    FanEngagement,
    /// 场馆运营
    VenueOperation,
    /// 社区关系
    CommunityRelations,
    /// 媒体传播
    MediaBroadcasting,
}

impl ClubOperationArea {
    /// 获取领域名称
    pub fn name(&self) -> &'static str {
        match self {
            ClubOperationArea::YouthDevelopment => "青训体系",
            ClubOperationArea::PlayerTransfer => "球员转会",
            ClubOperationArea::CommercialDevelopment => "商业开发",
            ClubOperationArea::FanEngagement => "球迷服务",
            ClubOperationArea::VenueOperation => "场馆运营",
            ClubOperationArea::CommunityRelations => "社区关系",
            ClubOperationArea::MediaBroadcasting => "媒体传播",
        }
    }

    /// 是否为核心运营领域
    pub fn is_core_area(&self) -> bool {
        matches!(
            self,
            ClubOperationArea::YouthDevelopment
                | ClubOperationArea::CommercialDevelopment
                | ClubOperationArea::FanEngagement
        )
    }
}

/// 会员资格要求
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MembershipRequirement {
    /// 国家认可
    NationalRecognition,
    /// 章程合规
    ConstitutionCompliance,
    /// 反兴奋剂承诺
    AntiDopingCommitment,
    /// 财务透明
    FinancialTransparency,
    /// 治理结构
    GovernanceStructure,
    /// 定期审查
    PeriodicReview,
}

impl MembershipRequirement {
    /// 获取要求名称
    pub fn name(&self) -> &'static str {
        match self {
            MembershipRequirement::NationalRecognition => "国家认可",
            MembershipRequirement::ConstitutionCompliance => "章程合规",
            MembershipRequirement::AntiDopingCommitment => "反兴奋剂承诺",
            MembershipRequirement::FinancialTransparency => "财务透明",
            MembershipRequirement::GovernanceStructure => "治理结构",
            MembershipRequirement::PeriodicReview => "定期审查",
        }
    }

    /// 是否为强制要求
    pub fn is_mandatory(&self) -> bool {
        matches!(
            self,
            MembershipRequirement::NationalRecognition
                | MembershipRequirement::AntiDopingCommitment
        )
    }
}

/// 组织治理原则
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernancePrinciple {
    /// 透明度
    Transparency,
    /// 问责制
    Accountability,
    /// 民主参与
    DemocraticParticipation,
    /// 权力制衡
    ChecksAndBalances,
    /// 利益冲突管理
    ConflictOfInterestManagement,
    /// 运动员代表参与
    AthleteRepresentation,
}

impl GovernancePrinciple {
    /// 获取原则名称
    pub fn name(&self) -> &'static str {
        match self {
            GovernancePrinciple::Transparency => "透明度",
            GovernancePrinciple::Accountability => "问责制",
            GovernancePrinciple::DemocraticParticipation => "民主参与",
            GovernancePrinciple::ChecksAndBalances => "权力制衡",
            GovernancePrinciple::ConflictOfInterestManagement => "利益冲突管理",
            GovernancePrinciple::AthleteRepresentation => "运动员代表参与",
        }
    }

    /// 获取重要程度（1-5，5最重要）
    pub fn importance_level(&self) -> u32 {
        match self {
            GovernancePrinciple::Transparency => 5,
            GovernancePrinciple::Accountability => 5,
            GovernancePrinciple::DemocraticParticipation => 4,
            GovernancePrinciple::ChecksAndBalances => 4,
            GovernancePrinciple::ConflictOfInterestManagement => 5,
            GovernancePrinciple::AthleteRepresentation => 3,
        }
    }
}

/// 体育组织规则
#[derive(Debug, Clone)]
pub struct SportsOrganizationRules {
    /// 规则元数据
    metadata: RuleMetadata,
}

impl SportsOrganizationRules {
    /// 创建新的体育组织规则
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("体育组织规则", "国际/国家体育组织规则体系"),
        }
    }

    /// 获取所有组织类型
    pub fn organization_types(&self) -> Vec<OrganizationType> {
        vec![
            OrganizationType::InternationalOlympicCommittee,
            OrganizationType::InternationalFederation,
            OrganizationType::NationalOlympicCommittee,
            OrganizationType::NationalFederation,
            OrganizationType::ProfessionalLeague,
            OrganizationType::AmateurAssociation,
            OrganizationType::ProfessionalClub,
            OrganizationType::AmateurClub,
        ]
    }

    /// 获取所有国际单项体育联合会
    pub fn international_federations(&self) -> Vec<InternationalFederation> {
        vec![
            InternationalFederation::FIFA,
            InternationalFederation::FIBA,
            InternationalFederation::FINA,
            InternationalFederation::WorldAthletics,
            InternationalFederation::UCI,
            InternationalFederation::ITF,
            InternationalFederation::ITTF,
            InternationalFederation::BWF,
            InternationalFederation::FIVB,
            InternationalFederation::IIHF,
            InternationalFederation::FIG,
            InternationalFederation::IWF,
            InternationalFederation::IBA,
            InternationalFederation::IJF,
            InternationalFederation::WT,
        ]
    }

    /// 获取所有国家奥委会职责
    pub fn noc_responsibilities(&self) -> Vec<NOCResponsibility> {
        vec![
            NOCResponsibility::TeamSelection,
            NOCResponsibility::OlympicPreparation,
            NOCResponsibility::OlympicPromotion,
            NOCResponsibility::YouthDevelopment,
            NOCResponsibility::AntiDopingEducation,
            NOCResponsibility::AthleteWelfare,
            NOCResponsibility::InternationalExchange,
        ]
    }

    /// 获取所有职业联盟类型
    pub fn professional_league_types(&self) -> Vec<ProfessionalLeagueType> {
        vec![
            ProfessionalLeagueType::FootballLeague,
            ProfessionalLeagueType::BasketballLeague,
            ProfessionalLeagueType::BaseballLeague,
            ProfessionalLeagueType::HockeyLeague,
            ProfessionalLeagueType::TennisTour,
            ProfessionalLeagueType::GolfTour,
            ProfessionalLeagueType::MMALeague,
            ProfessionalLeagueType::EsportsLeague,
        ]
    }

    /// 获取所有俱乐部运营领域
    pub fn club_operation_areas(&self) -> Vec<ClubOperationArea> {
        vec![
            ClubOperationArea::YouthDevelopment,
            ClubOperationArea::PlayerTransfer,
            ClubOperationArea::CommercialDevelopment,
            ClubOperationArea::FanEngagement,
            ClubOperationArea::VenueOperation,
            ClubOperationArea::CommunityRelations,
            ClubOperationArea::MediaBroadcasting,
        ]
    }

    /// 获取所有会员资格要求
    pub fn membership_requirements(&self) -> Vec<MembershipRequirement> {
        vec![
            MembershipRequirement::NationalRecognition,
            MembershipRequirement::ConstitutionCompliance,
            MembershipRequirement::AntiDopingCommitment,
            MembershipRequirement::FinancialTransparency,
            MembershipRequirement::GovernanceStructure,
            MembershipRequirement::PeriodicReview,
        ]
    }

    /// 获取所有治理原则
    pub fn governance_principles(&self) -> Vec<GovernancePrinciple> {
        vec![
            GovernancePrinciple::Transparency,
            GovernancePrinciple::Accountability,
            GovernancePrinciple::DemocraticParticipation,
            GovernancePrinciple::ChecksAndBalances,
            GovernancePrinciple::ConflictOfInterestManagement,
            GovernancePrinciple::AthleteRepresentation,
        ]
    }

    /// 获取 IOC 会员数量
    pub fn ioc_members_count(&self) -> u32 {
        105 // IOC 委员人数
    }

    /// 获取国家奥委会数量
    pub fn nocs_count(&self) -> u32 {
        206 // 被认可的国家奥委会数量
    }

    /// 获取夏季奥运项目数
    pub fn summer_olympic_sports_count(&self) -> u32 {
        41 // 2024 巴黎奥运会项目数
    }

    /// 获取冬季奥运项目数
    pub fn winter_olympic_sports_count(&self) -> u32 {
        15 // 2026 米兰-科尔蒂纳冬奥会项目数
    }

    /// 获取奥运周期（年）
    pub fn olympic_cycle_years(&self) -> u32 {
        4
    }

    /// 获取 IOC 总部信息
    pub fn ioc_headquarters(&self) -> (&'static str, &'static str) {
        ("瑞士", "洛桑")
    }

    /// 判断是否为奥运项目
    pub fn is_olympic_sport(&self, federation: InternationalFederation) -> bool {
        matches!(
            federation,
            InternationalFederation::FIFA
                | InternationalFederation::FIBA
                | InternationalFederation::FINA
                | InternationalFederation::WorldAthletics
                | InternationalFederation::UCI
                | InternationalFederation::ITF
                | InternationalFederation::ITTF
                | InternationalFederation::BWF
                | InternationalFederation::FIVB
                | InternationalFederation::IIHF
                | InternationalFederation::FIG
                | InternationalFederation::IWF
                | InternationalFederation::IJF
                | InternationalFederation::WT
        )
    }

    /// 获取会员会费（瑞士法郎/年）
    pub fn membership_fee_chf(&self, org_type: OrganizationType) -> u32 {
        match org_type {
            OrganizationType::InternationalFederation => 50_000,
            OrganizationType::NationalOlympicCommittee => 10_000,
            OrganizationType::NationalFederation => 5_000,
            OrganizationType::ProfessionalLeague => 100_000,
            OrganizationType::AmateurAssociation => 500,
            OrganizationType::ProfessionalClub => 20_000,
            OrganizationType::AmateurClub => 100,
            OrganizationType::InternationalOlympicCommittee => 0, // IOC 是最高组织
        }
    }

    /// 获取任期限制（年）
    pub fn term_limit_years(&self, org_type: OrganizationType) -> u32 {
        match org_type {
            OrganizationType::InternationalOlympicCommittee => 8, // IOC 主席任期
            OrganizationType::InternationalFederation => 12,      // 一般 IF 主席任期
            OrganizationType::NationalOlympicCommittee => 8,
            OrganizationType::NationalFederation => 8,
            OrganizationType::ProfessionalLeague => 10,
            _ => 0, // 其他组织无任期限制
        }
    }

    /// 检查是否符合治理标准
    pub fn meets_governance_standards(&self, principles: &[GovernancePrinciple]) -> bool {
        // 至少满足透明度、问责制、利益冲突管理
        principles.iter().any(|p| {
            matches!(
                p,
                GovernancePrinciple::Transparency
                    | GovernancePrinciple::Accountability
                    | GovernancePrinciple::ConflictOfInterestManagement
            )
        })
    }

    /// 获取运动员代表最低比例（%）
    pub fn min_athlete_representation_percentage(&self) -> u32 {
        20 // 运动员代表至少占20%
    }

    /// 获取财务审查周期（年）
    pub fn financial_audit_cycle_years(&self) -> u32 {
        1 // 每年审计
    }

    /// 获取兴奋剂检测频率（次/年）
    pub fn doping_test_frequency(&self, org_type: OrganizationType) -> u32 {
        match org_type {
            OrganizationType::ProfessionalLeague => 12, // 每月检测
            OrganizationType::InternationalFederation => 24,
            OrganizationType::NationalFederation => 12,
            _ => 6, // 其他组织每两月检测
        }
    }
}

impl Default for SportsOrganizationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SportsOrganizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("organization")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_types() {
        let rules = SportsOrganizationRules::new();
        let types = rules.organization_types();
        assert_eq!(types.len(), 8);
        assert!(OrganizationType::InternationalOlympicCommittee.is_international());
        assert!(OrganizationType::NationalOlympicCommittee.is_national());
        assert!(OrganizationType::ProfessionalClub.is_professional());
    }

    #[test]
    fn test_international_federations() {
        let rules = SportsOrganizationRules::new();
        let federations = rules.international_federations();
        assert_eq!(federations.len(), 15);
        assert_eq!(InternationalFederation::FIFA.name(), "国际足联");
        assert!(InternationalFederation::FIFA.member_count() > 200);
    }

    #[test]
    fn test_if_headquarters() {
        let fifa = InternationalFederation::FIFA;
        let (country, city) = fifa.headquarters();
        assert_eq!(country, "瑞士");
        assert_eq!(city, "苏黎世");
    }

    #[test]
    fn test_if_founded_years() {
        assert!(InternationalFederation::FIFA.founded_year() > 1900);
        assert!(InternationalFederation::FIBA.founded_year() > 1930);
    }

    #[test]
    fn test_noc_responsibilities() {
        let rules = SportsOrganizationRules::new();
        let responsibilities = rules.noc_responsibilities();
        assert_eq!(responsibilities.len(), 7);
        assert_eq!(NOCResponsibility::TeamSelection.priority(), 1);
        assert_eq!(NOCResponsibility::AntiDopingEducation.priority(), 3);
    }

    #[test]
    fn test_professional_league_types() {
        let rules = SportsOrganizationRules::new();
        let leagues = rules.professional_league_types();
        assert_eq!(leagues.len(), 8);
        assert!(ProfessionalLeagueType::BasketballLeague.has_salary_cap());
        assert!(ProfessionalLeagueType::FootballLeague.has_draft_system());
        assert!(!ProfessionalLeagueType::TennisTour.has_salary_cap());
    }

    #[test]
    fn test_club_operation_areas() {
        let rules = SportsOrganizationRules::new();
        let areas = rules.club_operation_areas();
        assert_eq!(areas.len(), 7);
        assert!(ClubOperationArea::YouthDevelopment.is_core_area());
        assert!(ClubOperationArea::CommercialDevelopment.is_core_area());
        assert!(!ClubOperationArea::MediaBroadcasting.is_core_area());
    }

    #[test]
    fn test_membership_requirements() {
        let rules = SportsOrganizationRules::new();
        let requirements = rules.membership_requirements();
        assert_eq!(requirements.len(), 6);
        assert!(MembershipRequirement::NationalRecognition.is_mandatory());
        assert!(MembershipRequirement::AntiDopingCommitment.is_mandatory());
        assert!(!MembershipRequirement::FinancialTransparency.is_mandatory());
    }

    #[test]
    fn test_governance_principles() {
        let rules = SportsOrganizationRules::new();
        let principles = rules.governance_principles();
        assert_eq!(principles.len(), 6);
        assert_eq!(GovernancePrinciple::Transparency.importance_level(), 5);
        assert_eq!(GovernancePrinciple::Accountability.importance_level(), 5);
    }

    #[test]
    fn test_ioc_info() {
        let rules = SportsOrganizationRules::new();
        assert!(rules.ioc_members_count() > 100);
        assert!(rules.nocs_count() > 200);
        assert_eq!(rules.olympic_cycle_years(), 4);
    }

    #[test]
    fn test_olympic_sports_count() {
        let rules = SportsOrganizationRules::new();
        assert!(rules.summer_olympic_sports_count() > 30);
        assert!(rules.winter_olympic_sports_count() > 10);
    }

    #[test]
    fn test_ioc_headquarters() {
        let rules = SportsOrganizationRules::new();
        let (country, city) = rules.ioc_headquarters();
        assert_eq!(country, "瑞士");
        assert_eq!(city, "洛桑");
    }

    #[test]
    fn test_is_olympic_sport() {
        let rules = SportsOrganizationRules::new();
        assert!(rules.is_olympic_sport(InternationalFederation::FIFA));
        assert!(rules.is_olympic_sport(InternationalFederation::FIBA));
    }

    #[test]
    fn test_membership_fee() {
        let rules = SportsOrganizationRules::new();
        assert!(rules.membership_fee_chf(OrganizationType::InternationalFederation) > 0);
        assert!(rules.membership_fee_chf(OrganizationType::ProfessionalLeague) > 0);
        assert_eq!(
            rules.membership_fee_chf(OrganizationType::InternationalOlympicCommittee),
            0
        );
    }

    #[test]
    fn test_term_limits() {
        let rules = SportsOrganizationRules::new();
        assert!(rules.term_limit_years(OrganizationType::InternationalOlympicCommittee) > 0);
        assert!(rules.term_limit_years(OrganizationType::InternationalFederation) > 0);
    }

    #[test]
    fn test_governance_standards() {
        let rules = SportsOrganizationRules::new();
        let good_principles = vec![
            GovernancePrinciple::Transparency,
            GovernancePrinciple::Accountability,
        ];
        assert!(rules.meets_governance_standards(&good_principles));

        let bad_principles = vec![GovernancePrinciple::DemocraticParticipation];
        assert!(!rules.meets_governance_standards(&bad_principles));
    }

    #[test]
    fn test_athlete_representation() {
        let rules = SportsOrganizationRules::new();
        assert!(rules.min_athlete_representation_percentage() >= 20);
    }

    #[test]
    fn test_financial_audit() {
        let rules = SportsOrganizationRules::new();
        assert_eq!(rules.financial_audit_cycle_years(), 1);
    }

    #[test]
    fn test_doping_test_frequency() {
        let rules = SportsOrganizationRules::new();
        assert!(rules.doping_test_frequency(OrganizationType::ProfessionalLeague) > 0);
        assert!(rules.doping_test_frequency(OrganizationType::InternationalFederation) > 0);
    }
}
