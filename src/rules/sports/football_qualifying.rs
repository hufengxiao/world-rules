//!世界杯预选赛规则 - FIFA World Cup Qualification

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 大洲预选赛区域
#[derive(Debug, Clone, PartialEq)]
pub enum QualificationZone {
    /// 欧洲（UEFA）
    Europe,
    /// 南美（CONMEBOL）
    SouthAmerica,
    /// 亚洲（AFC）
    Asia,
    /// 非洲（CAF）
    Africa,
    /// 北美及加勒比（CONCACAF）
    NorthAmerica,
    /// 大洋洲（OFC）
    Oceania,
}

/// 世界杯预选赛规则
pub struct FootballQualifyingRules {
    metadata: RuleMetadata,
}

impl FootballQualifyingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯预选赛规则", "FIFA世界杯预选赛规则")
                .with_origin("FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "预选赛".into(),
                ]),
        }
    }

    ///世界杯参赛席位
    pub fn world_cup_slots(&self) -> u8 {
        48 // 2026起扩军至48队
    }

    /// 各大洲席位分配
    pub fn zone_slots(&self, zone: QualificationZone) -> u8 {
        match zone {
            QualificationZone::Europe => 16,
            QualificationZone::SouthAmerica => 6,
            QualificationZone::Asia => 8,
            QualificationZone::Africa => 9,
            QualificationZone::NorthAmerica => 6,
            QualificationZone::Oceania => 1,
        }
    }

    /// 欧洲预选赛规则
    pub fn europe_qualification_rule(&self) -> String {
        "小组赛12组，各组第1直接晋级，各组第2进入附加赛".to_string()
    }

    /// 南美预选赛规则
    pub fn south_america_qualification_rule(&self) -> String {
        "10队主客场循环赛，前6名直接晋级".to_string()
    }

    /// 亚洲预选赛规则
    pub fn asia_qualification_rule(&self) -> String {
        "多阶段淘汰制，最终阶段分组决出晋级名额".to_string()
    }

    /// 非洲预选赛规则
    pub fn africa_qualification_rule(&self) -> String {
        "多阶段淘汰制，最终阶段分组决出晋级名额".to_string()
    }

    /// 北美预选赛规则
    pub fn north_america_qualification_rule(&self) -> String {
        "最终阶段8角赛，前3名直接晋级，第4名进入附加赛".to_string()
    }

    /// 大洋洲预选赛规则
    pub fn oceania_qualification_rule(&self) -> String {
        "小组赛加淘汰赛，胜者进入跨洲附加赛".to_string()
    }

    /// 获取大洲预选赛规则
    pub fn get_zone_rule(&self, zone: QualificationZone) -> String {
        match zone {
            QualificationZone::Europe => self.europe_qualification_rule(),
            QualificationZone::SouthAmerica => self.south_america_qualification_rule(),
            QualificationZone::Asia => self.asia_qualification_rule(),
            QualificationZone::Africa => self.africa_qualification_rule(),
            QualificationZone::NorthAmerica => self.north_america_qualification_rule(),
            QualificationZone::Oceania => self.oceania_qualification_rule(),
        }
    }

    /// 跨洲附加赛规则
    pub fn intercontinental_playoff_rule(&self) -> String {
        "部分大洲队伍进行跨洲附加赛争夺剩余席位".to_string()
    }

    /// 东道主自动晋级
    pub fn host_auto_qualification(&self) -> String {
        "东道主自动获得世界杯参赛资格".to_string()
    }

    /// 预选赛周期
    pub fn qualification_duration(&self) -> String {
        "预选赛通常持续2-3年".to_string()
    }
}

impl Default for FootballQualifyingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballQualifyingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_qualifying")
    }

    fn explain(&self) -> String {
        format!(
            "【世界杯预选赛规则】\n\n\
            世界杯席位: {} 支球队\n\n\
            各大洲席位:\n\
            - 欧洲: {} 席位\n\
            - 南美: {} 席位\n\
            - 亚洲: {} 席位\n\
            - 非洲: {} 席位\n\
            - 北美: {} 席位\n\
            - 大洋洲: {} 席位\n\n\
            各大洲规则:\n\
            - 欧洲: {}\n\
            - 南美: {}\n\
            - 亚洲: {}\n\
            - 非洲: {}\n\
            - 北美: {}\n\
            - 大洋洲: {}\n\n\
            附加赛: {}\n\
            东道主: {}\n\
            预选赛周期: {}",
            self.world_cup_slots(),
            self.zone_slots(QualificationZone::Europe),
            self.zone_slots(QualificationZone::SouthAmerica),
            self.zone_slots(QualificationZone::Asia),
            self.zone_slots(QualificationZone::Africa),
            self.zone_slots(QualificationZone::NorthAmerica),
            self.zone_slots(QualificationZone::Oceania),
            self.europe_qualification_rule(),
            self.south_america_qualification_rule(),
            self.asia_qualification_rule(),
            self.africa_qualification_rule(),
            self.north_america_qualification_rule(),
            self.oceania_qualification_rule(),
            self.intercontinental_playoff_rule(),
            self.host_auto_qualification(),
            self.qualification_duration()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_qualifying_basic() {
        let rules = FootballQualifyingRules::new();
        assert_eq!(rules.world_cup_slots(), 48);
    }

    #[test]
    fn test_zone_slots() {
        let rules = FootballQualifyingRules::new();
        assert_eq!(rules.zone_slots(QualificationZone::Europe), 16);
        assert_eq!(rules.zone_slots(QualificationZone::SouthAmerica), 6);
        assert_eq!(rules.zone_slots(QualificationZone::Asia), 8);
    }

    #[test]
    fn test_zone_rules() {
        let rules = FootballQualifyingRules::new();
        assert!(rules.europe_qualification_rule().contains("小组"));
        assert!(rules.south_america_qualification_rule().contains("循环"));
    }

    #[test]
    fn test_get_zone_rule() {
        let rules = FootballQualifyingRules::new();
        let rule = rules.get_zone_rule(QualificationZone::Europe);
        assert!(rule.contains("小组"));
    }

    #[test]
    fn test_host_auto_qualification() {
        let rules = FootballQualifyingRules::new();
        assert!(rules.host_auto_qualification().contains("自动"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballQualifyingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_qualifying")
        );
    }
}
