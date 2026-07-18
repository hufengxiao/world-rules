//! 体育设施规则
//!
//! 涵盖体育场馆、运动场地、体育设备和安全防护的规则体系。
//!
//! # 规则体系
//!
//! - 国际体育联合会设施标准
//! - 奥运会场馆要求
//! - 国家体育场馆建设规范
//! - 专业赛事场地标准
//!
//! # Examples
//!
//! ```
//! use world_rules::rules::sports::sports_facility::SportsFacilityRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = SportsFacilityRules::new();
//! assert!(!rules.stadium_standards().is_empty());
//! assert!(!rules.safety_requirements().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 体育场馆类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StadiumType {
    /// 综合性体育场
    MultiPurposeStadium,
    /// 足球场
    FootballStadium,
    /// 篮球馆
    BasketballArena,
    /// 游泳馆
    AquaticsCenter,
    /// 田径场
    AthleticsStadium,
    /// 体育馆
    IndoorArena,
    /// 网球场
    TennisComplex,
    /// 滑冰馆
    IceArena,
    /// 滑雪场
    SkiResort,
    /// 高尔夫球场
    GolfCourse,
}

impl StadiumType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            StadiumType::MultiPurposeStadium => "综合性体育场",
            StadiumType::FootballStadium => "足球场",
            StadiumType::BasketballArena => "篮球馆",
            StadiumType::AquaticsCenter => "游泳馆",
            StadiumType::AthleticsStadium => "田径场",
            StadiumType::IndoorArena => "体育馆",
            StadiumType::TennisComplex => "网球场",
            StadiumType::IceArena => "滑冰馆",
            StadiumType::SkiResort => "滑雪场",
            StadiumType::GolfCourse => "高尔夫球场",
        }
    }
}

/// 场地质量等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldQualityLevel {
    /// 国际A级（奥运会、世锦赛）
    InternationalA,
    /// 国际B级（洲际比赛）
    InternationalB,
    /// 国家级（全国比赛）
    National,
    /// 省级（省级比赛）
    Provincial,
    /// 业余级（训练、娱乐）
    Amateur,
}

impl FieldQualityLevel {
    /// 获取等级名称
    pub fn name(&self) -> &'static str {
        match self {
            FieldQualityLevel::InternationalA => "国际A级",
            FieldQualityLevel::InternationalB => "国际B级",
            FieldQualityLevel::National => "国家级",
            FieldQualityLevel::Provincial => "省级",
            FieldQualityLevel::Amateur => "业余级",
        }
    }

    /// 获取最低容纳人数要求
    pub fn min_capacity(&self) -> u32 {
        match self {
            FieldQualityLevel::InternationalA => 40000,
            FieldQualityLevel::InternationalB => 20000,
            FieldQualityLevel::National => 10000,
            FieldQualityLevel::Provincial => 5000,
            FieldQualityLevel::Amateur => 500,
        }
    }
}

/// 体育设备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentType {
    /// 比赛器材
    CompetitionEquipment,
    /// 训练器材
    TrainingEquipment,
    /// 裁判器材
    OfficiatingEquipment,
    /// 安全器材
    SafetyEquipment,
    /// 医疗器材
    MedicalEquipment,
    /// 计时计分器材
    TimingScoringEquipment,
}

/// 体育设备认证标准
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCertification {
    /// 国际比赛认证
    InternationalCompetition,
    /// 国家级认证
    NationalCompetition,
    /// 训练级认证
    TrainingGrade,
    /// 业余级认证
    AmateurGrade,
}

impl EquipmentCertification {
    /// 获取认证名称
    pub fn name(&self) -> &'static str {
        match self {
            EquipmentCertification::InternationalCompetition => "国际比赛认证",
            EquipmentCertification::NationalCompetition => "国家级认证",
            EquipmentCertification::TrainingGrade => "训练级认证",
            EquipmentCertification::AmateurGrade => "业余级认证",
        }
    }
}

/// 设备检验周期
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectionInterval {
    /// 每日检验
    Daily,
    /// 每周检验
    Weekly,
    /// 每月检验
    Monthly,
    /// 每季度检验
    Quarterly,
    /// 每年检验
    Annually,
}

impl InspectionInterval {
    /// 获取检验周期名称
    pub fn name(&self) -> &'static str {
        match self {
            InspectionInterval::Daily => "每日",
            InspectionInterval::Weekly => "每周",
            InspectionInterval::Monthly => "每月",
            InspectionInterval::Quarterly => "每季度",
            InspectionInterval::Annually => "每年",
        }
    }

    /// 转换为天数
    pub fn days(&self) -> u32 {
        match self {
            InspectionInterval::Daily => 1,
            InspectionInterval::Weekly => 7,
            InspectionInterval::Monthly => 30,
            InspectionInterval::Quarterly => 90,
            InspectionInterval::Annually => 365,
        }
    }
}

/// 设备检验标准
#[derive(Debug, Clone, PartialEq)]
pub struct EquipmentInspectionStandard {
    /// 设备名称
    pub equipment_name: String,
    /// 设备类型
    pub equipment_type: EquipmentType,
    /// 检验周期
    pub interval: InspectionInterval,
    /// 检验标准描述
    pub standard: String,
}

impl EquipmentInspectionStandard {
    /// 创建新检验标准
    pub fn new(
        equipment_name: &str,
        equipment_type: EquipmentType,
        interval: InspectionInterval,
        standard: &str,
    ) -> Self {
        Self {
            equipment_name: equipment_name.to_string(),
            equipment_type,
            interval,
            standard: standard.to_string(),
        }
    }
}

impl EquipmentType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            EquipmentType::CompetitionEquipment => "比赛器材",
            EquipmentType::TrainingEquipment => "训练器材",
            EquipmentType::OfficiatingEquipment => "裁判器材",
            EquipmentType::SafetyEquipment => "安全器材",
            EquipmentType::MedicalEquipment => "医疗器材",
            EquipmentType::TimingScoringEquipment => "计时计分器材",
        }
    }
}

/// 安全设施类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SafetyFacilityType {
    /// 消防设施
    FireSafety,
    /// 紧急出口
    EmergencyExit,
    /// 医疗站
    MedicalStation,
    /// 监控系统
    SurveillanceSystem,
    /// 广播系统
    PublicAddress,
    /// 照明系统
    LightingSystem,
    /// 通风系统
    Ventilation,
    /// 座椅安全
    SeatingSafety,
    /// 围栏护栏
    Barriers,
    /// 无障碍设施
    Accessibility,
}

impl SafetyFacilityType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            SafetyFacilityType::FireSafety => "消防设施",
            SafetyFacilityType::EmergencyExit => "紧急出口",
            SafetyFacilityType::MedicalStation => "医疗站",
            SafetyFacilityType::SurveillanceSystem => "监控系统",
            SafetyFacilityType::PublicAddress => "广播系统",
            SafetyFacilityType::LightingSystem => "照明系统",
            SafetyFacilityType::Ventilation => "通风系统",
            SafetyFacilityType::SeatingSafety => "座椅安全",
            SafetyFacilityType::Barriers => "围栏护栏",
            SafetyFacilityType::Accessibility => "无障碍设施",
        }
    }

    /// 获取检查频率（天）
    pub fn inspection_frequency_days(&self) -> u32 {
        match self {
            SafetyFacilityType::FireSafety => 30,
            SafetyFacilityType::EmergencyExit => 7,
            SafetyFacilityType::MedicalStation => 1,
            SafetyFacilityType::SurveillanceSystem => 7,
            SafetyFacilityType::PublicAddress => 7,
            SafetyFacilityType::LightingSystem => 30,
            SafetyFacilityType::Ventilation => 30,
            SafetyFacilityType::SeatingSafety => 90,
            SafetyFacilityType::Barriers => 30,
            SafetyFacilityType::Accessibility => 30,
        }
    }
}

/// 场地规格标准
#[derive(Debug, Clone, PartialEq)]
pub struct FieldSpecification {
    /// 场地名称
    pub name: String,
    /// 长度（米）
    pub length_meters: f64,
    /// 宽度（米）
    pub width_meters: f64,
    /// 面积（平方米）
    pub area_sqm: f64,
    /// 允许误差（米）
    pub tolerance_meters: f64,
}

impl FieldSpecification {
    /// 创建新场地规格
    pub fn new(name: &str, length: f64, width: f64, tolerance: f64) -> Self {
        Self {
            name: name.to_string(),
            length_meters: length,
            width_meters: width,
            area_sqm: length * width,
            tolerance_meters: tolerance,
        }
    }

    /// 验证场地尺寸是否符合标准
    pub fn validate_dimensions(&self, actual_length: f64, actual_width: f64) -> bool {
        let length_ok = (actual_length - self.length_meters).abs() <= self.tolerance_meters;
        let width_ok = (actual_width - self.width_meters).abs() <= self.tolerance_meters;
        length_ok && width_ok
    }
}

/// 场地表面类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldSurfaceType {
    /// 天然草坪
    NaturalGrass,
    /// 人工草坪
    ArtificialTurf,
    /// 塑胶跑道
    SyntheticTrack,
    /// 木地板
    WoodenFloor,
    /// 混凝土
    Concrete,
    /// 红土
    Clay,
    /// 硬地（丙烯酸）
    HardCourt,
    /// 草地
    Grass,
    /// 沙地
    Sand,
    /// 冰面
    Ice,
    /// 雪面
    Snow,
    /// 水面
    Water,
}

impl FieldSurfaceType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            FieldSurfaceType::NaturalGrass => "天然草坪",
            FieldSurfaceType::ArtificialTurf => "人工草坪",
            FieldSurfaceType::SyntheticTrack => "塑胶跑道",
            FieldSurfaceType::WoodenFloor => "木地板",
            FieldSurfaceType::Concrete => "混凝土",
            FieldSurfaceType::Clay => "红土",
            FieldSurfaceType::HardCourt => "硬地",
            FieldSurfaceType::Grass => "草地",
            FieldSurfaceType::Sand => "沙地",
            FieldSurfaceType::Ice => "冰面",
            FieldSurfaceType::Snow => "雪面",
            FieldSurfaceType::Water => "水面",
        }
    }

    /// 获取维护频率（天）
    pub fn maintenance_frequency_days(&self) -> u32 {
        match self {
            FieldSurfaceType::NaturalGrass => 1,
            FieldSurfaceType::ArtificialTurf => 7,
            FieldSurfaceType::SyntheticTrack => 30,
            FieldSurfaceType::WoodenFloor => 7,
            FieldSurfaceType::Concrete => 90,
            FieldSurfaceType::Clay => 1,
            FieldSurfaceType::HardCourt => 30,
            FieldSurfaceType::Grass => 1,
            FieldSurfaceType::Sand => 7,
            FieldSurfaceType::Ice => 1,
            FieldSurfaceType::Snow => 1,
            FieldSurfaceType::Water => 1,
        }
    }
}

/// 场地认证等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldCertificationLevel {
    /// 国际足联认证
    FifaCertified,
    /// 国际篮联认证
    FibaCertified,
    /// 国际泳联认证
    FinaCertified,
    /// 国际网联认证
    ItfCertified,
    /// 国际田联认证
    IaafCertified,
    /// 奥运会认证
    OlympicCertified,
    /// 国家级认证
    NationalCertified,
}

impl FieldCertificationLevel {
    /// 获取认证名称
    pub fn name(&self) -> &'static str {
        match self {
            FieldCertificationLevel::FifaCertified => "国际足联认证",
            FieldCertificationLevel::FibaCertified => "国际篮联认证",
            FieldCertificationLevel::FinaCertified => "国际泳联认证",
            FieldCertificationLevel::ItfCertified => "国际网联认证",
            FieldCertificationLevel::IaafCertified => "国际田联认证",
            FieldCertificationLevel::OlympicCertified => "奥运会认证",
            FieldCertificationLevel::NationalCertified => "国家级认证",
        }
    }

    /// 获取认证有效期（年）
    pub fn validity_years(&self) -> u32 {
        match self {
            FieldCertificationLevel::FifaCertified => 3,
            FieldCertificationLevel::FibaCertified => 5,
            FieldCertificationLevel::FinaCertified => 5,
            FieldCertificationLevel::ItfCertified => 3,
            FieldCertificationLevel::IaafCertified => 5,
            FieldCertificationLevel::OlympicCertified => 4,
            FieldCertificationLevel::NationalCertified => 3,
        }
    }
}

/// 场地维护要求
#[derive(Debug, Clone, PartialEq)]
pub struct FieldMaintenanceRequirement {
    /// 要求名称
    pub name: String,
    /// 表面类型
    pub surface_type: FieldSurfaceType,
    /// 检查频率（天）
    pub inspection_frequency_days: u32,
    /// 检查标准
    pub standard: String,
}

impl FieldMaintenanceRequirement {
    /// 创建新维护要求
    pub fn new(
        name: &str,
        surface_type: FieldSurfaceType,
        frequency: u32,
        standard: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            surface_type,
            inspection_frequency_days: frequency,
            standard: standard.to_string(),
        }
    }
}

/// 安全检查项
#[derive(Debug, Clone, PartialEq)]
pub struct SafetyCheckItem {
    /// 检查项名称
    pub name: String,
    /// 检查项类型
    pub check_type: SafetyFacilityType,
    /// 是否必须
    pub mandatory: bool,
    /// 检查标准
    pub standard: String,
}

impl SafetyCheckItem {
    /// 创建新检查项
    pub fn new(name: &str, check_type: SafetyFacilityType, mandatory: bool, standard: &str) -> Self {
        Self {
            name: name.to_string(),
            check_type,
            mandatory,
            standard: standard.to_string(),
        }
    }
}

/// 体育设施规则
#[derive(Debug, Clone)]
pub struct SportsFacilityRules {
    metadata: RuleMetadata,
}

impl SportsFacilityRules {
    /// 创建新规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("体育设施规则", "体育场馆、场地、设备和安全防护规则体系")
                .with_origin("国际体育联合会")
                .with_tags(vec![
                    "体育".into(),
                    "设施".into(),
                    "场馆".into(),
                    "安全".into(),
                ]),
        }
    }

    /// 获取体育场馆标准
    pub fn stadium_standards(&self) -> Vec<StadiumType> {
        vec![
            StadiumType::MultiPurposeStadium,
            StadiumType::FootballStadium,
            StadiumType::BasketballArena,
            StadiumType::AquaticsCenter,
            StadiumType::AthleticsStadium,
            StadiumType::IndoorArena,
            StadiumType::TennisComplex,
            StadiumType::IceArena,
            StadiumType::SkiResort,
            StadiumType::GolfCourse,
        ]
    }

    /// 获取场地质量等级
    pub fn field_quality_levels(&self) -> Vec<FieldQualityLevel> {
        vec![
            FieldQualityLevel::InternationalA,
            FieldQualityLevel::InternationalB,
            FieldQualityLevel::National,
            FieldQualityLevel::Provincial,
            FieldQualityLevel::Amateur,
        ]
    }

    /// 获取设备类型列表
    pub fn equipment_types(&self) -> Vec<EquipmentType> {
        vec![
            EquipmentType::CompetitionEquipment,
            EquipmentType::TrainingEquipment,
            EquipmentType::OfficiatingEquipment,
            EquipmentType::SafetyEquipment,
            EquipmentType::MedicalEquipment,
            EquipmentType::TimingScoringEquipment,
        ]
    }

    /// 获取安全设施类型
    pub fn safety_facility_types(&self) -> Vec<SafetyFacilityType> {
        vec![
            SafetyFacilityType::FireSafety,
            SafetyFacilityType::EmergencyExit,
            SafetyFacilityType::MedicalStation,
            SafetyFacilityType::SurveillanceSystem,
            SafetyFacilityType::PublicAddress,
            SafetyFacilityType::LightingSystem,
            SafetyFacilityType::Ventilation,
            SafetyFacilityType::SeatingSafety,
            SafetyFacilityType::Barriers,
            SafetyFacilityType::Accessibility,
        ]
    }

    /// 获取安全要求列表
    pub fn safety_requirements(&self) -> Vec<SafetyCheckItem> {
        vec![
            SafetyCheckItem::new(
                "消防设备检查",
                SafetyFacilityType::FireSafety,
                true,
                "每30天检查一次，确保灭火器、消火栓正常工作",
            ),
            SafetyCheckItem::new(
                "紧急出口畅通",
                SafetyFacilityType::EmergencyExit,
                true,
                "紧急出口必须保持畅通，有明确标识",
            ),
            SafetyCheckItem::new(
                "医疗站配置",
                SafetyFacilityType::MedicalStation,
                true,
                "赛事期间必须配备医护人员和急救设备",
            ),
            SafetyCheckItem::new(
                "监控覆盖",
                SafetyFacilityType::SurveillanceSystem,
                true,
                "公共区域必须有监控覆盖",
            ),
            SafetyCheckItem::new(
                "广播系统",
                SafetyFacilityType::PublicAddress,
                true,
                "必须能清晰传达紧急广播",
            ),
            SafetyCheckItem::new(
                "照明标准",
                SafetyFacilityType::LightingSystem,
                true,
                "比赛区域照明需达到赛事要求",
            ),
            SafetyCheckItem::new(
                "无障碍通道",
                SafetyFacilityType::Accessibility,
                true,
                "必须提供残疾人无障碍通道和座位",
            ),
            SafetyCheckItem::new(
                "座椅安全",
                SafetyFacilityType::SeatingSafety,
                true,
                "座椅需固定牢固，无破损",
            ),
        ]
    }

    /// 获取标准场地规格
    pub fn standard_field_specifications(&self) -> Vec<FieldSpecification> {
        vec![
            FieldSpecification::new("足球场", 105.0, 68.0, 2.0),
            FieldSpecification::new("篮球场", 28.0, 15.0, 0.5),
            FieldSpecification::new("网球场", 23.77, 10.97, 0.1),
            FieldSpecification::new("排球场", 18.0, 9.0, 0.1),
            FieldSpecification::new("游泳池(50m)", 50.0, 25.0, 0.03),
            FieldSpecification::new("羽毛球场", 13.4, 6.1, 0.1),
            FieldSpecification::new("乒乓球台", 2.74, 1.525, 0.005),
            FieldSpecification::new("田径跑道(400m)", 400.0, 1.22, 0.01),
        ]
    }

    /// 验证场馆是否满足赛事等级要求
    pub fn validate_stadium_for_level(
        &self,
        _stadium_type: StadiumType,
        capacity: u32,
        level: FieldQualityLevel,
    ) -> bool {
        capacity >= level.min_capacity()
    }

    /// 获取设备检验标准
    pub fn equipment_inspection_standards(&self) -> Vec<(&'static str, u32)> {
        vec![
            ("比赛用球", 30),  // 每30天检验
            ("计时设备", 90),   // 每90天检验
            ("裁判设备", 180),  // 每180天检验
            ("安全设备", 30),   // 每30天检验
            ("医疗设备", 7),    // 每7天检验
        ]
    }

    /// 获取场地表面类型列表
    pub fn field_surface_types(&self) -> Vec<FieldSurfaceType> {
        vec![
            FieldSurfaceType::NaturalGrass,
            FieldSurfaceType::ArtificialTurf,
            FieldSurfaceType::SyntheticTrack,
            FieldSurfaceType::WoodenFloor,
            FieldSurfaceType::Concrete,
            FieldSurfaceType::Clay,
            FieldSurfaceType::HardCourt,
            FieldSurfaceType::Grass,
            FieldSurfaceType::Sand,
            FieldSurfaceType::Ice,
            FieldSurfaceType::Snow,
            FieldSurfaceType::Water,
        ]
    }

    /// 获取场地认证等级列表
    pub fn field_certification_levels(&self) -> Vec<FieldCertificationLevel> {
        vec![
            FieldCertificationLevel::FifaCertified,
            FieldCertificationLevel::FibaCertified,
            FieldCertificationLevel::FinaCertified,
            FieldCertificationLevel::ItfCertified,
            FieldCertificationLevel::IaafCertified,
            FieldCertificationLevel::OlympicCertified,
            FieldCertificationLevel::NationalCertified,
        ]
    }

    /// 获取场地维护要求
    pub fn field_maintenance_requirements(&self) -> Vec<FieldMaintenanceRequirement> {
        vec![
            FieldMaintenanceRequirement::new(
                "天然草坪维护",
                FieldSurfaceType::NaturalGrass,
                1,
                "每日修剪、浇水、施肥，保持草坪平整",
            ),
            FieldMaintenanceRequirement::new(
                "人工草坪清洁",
                FieldSurfaceType::ArtificialTurf,
                7,
                "每周清洁，定期填充颗粒，检查接缝",
            ),
            FieldMaintenanceRequirement::new(
                "塑胶跑道检查",
                FieldSurfaceType::SyntheticTrack,
                30,
                "每月检查跑道平整度、弹性、排水系统",
            ),
            FieldMaintenanceRequirement::new(
                "木地板维护",
                FieldSurfaceType::WoodenFloor,
                7,
                "每周打蜡、清洁，检查地板平整度",
            ),
            FieldMaintenanceRequirement::new(
                "红土场地维护",
                FieldSurfaceType::Clay,
                1,
                "每日洒水、平整，保持湿度适中",
            ),
            FieldMaintenanceRequirement::new(
                "硬地维护",
                FieldSurfaceType::HardCourt,
                30,
                "每月清洁、检查裂缝、修补涂层",
            ),
            FieldMaintenanceRequirement::new(
                "冰面维护",
                FieldSurfaceType::Ice,
                1,
                "每日清冰、浇水，保持冰面平整",
            ),
        ]
    }

    /// 验证场地表面是否适合特定运动
    pub fn validate_surface_for_sport(
        &self,
        surface_type: FieldSurfaceType,
        sport_name: &str,
    ) -> bool {
        match sport_name {
            "足球" => matches!(
                surface_type,
                FieldSurfaceType::NaturalGrass | FieldSurfaceType::ArtificialTurf
            ),
            "篮球" => matches!(surface_type, FieldSurfaceType::WoodenFloor | FieldSurfaceType::Concrete),
            "网球" => matches!(
                surface_type,
                FieldSurfaceType::Clay
                    | FieldSurfaceType::HardCourt
                    | FieldSurfaceType::Grass
            ),
            "游泳" => matches!(surface_type, FieldSurfaceType::Water),
            "田径" => matches!(surface_type, FieldSurfaceType::SyntheticTrack),
            "冰球" | "花样滑冰" | "速度滑冰" => matches!(surface_type, FieldSurfaceType::Ice),
            "滑雪" => matches!(surface_type, FieldSurfaceType::Snow),
            "沙滩排球" => matches!(surface_type, FieldSurfaceType::Sand),
            _ => true, // 其他运动默认接受
        }
    }
}

impl Default for SportsFacilityRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SportsFacilityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sports_facility")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        let explanation = [
            "=== 体育设施规则 ===".to_string(),
            "".to_string(),
            "本规则涵盖以下方面：".to_string(),
            "1. 体育场馆标准：综合性体育场、专项运动馆等".to_string(),
            "2. 场地质量等级：国际A级到业余级".to_string(),
            "3. 体育设备标准：比赛、训练、安全器材".to_string(),
            "4. 安全防护规则：消防、医疗、无障碍设施".to_string(),
        ];
        explanation.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stadium_standards() {
        let rules = SportsFacilityRules::new();
        let standards = rules.stadium_standards();
        assert!(!standards.is_empty());
        assert_eq!(standards.len(), 10);
    }

    #[test]
    fn test_field_quality_levels() {
        let rules = SportsFacilityRules::new();
        let levels = rules.field_quality_levels();
        assert_eq!(levels.len(), 5);

        // 国际A级要求最高容纳人数
        assert_eq!(FieldQualityLevel::InternationalA.min_capacity(), 40000);
        assert_eq!(FieldQualityLevel::Amateur.min_capacity(), 500);
    }

    #[test]
    fn test_equipment_types() {
        let rules = SportsFacilityRules::new();
        let types = rules.equipment_types();
        assert_eq!(types.len(), 6);
    }

    #[test]
    fn test_safety_facility_types() {
        let rules = SportsFacilityRules::new();
        let types = rules.safety_facility_types();
        assert_eq!(types.len(), 10);

        // 验证检查频率
        assert_eq!(SafetyFacilityType::FireSafety.inspection_frequency_days(), 30);
        assert_eq!(SafetyFacilityType::EmergencyExit.inspection_frequency_days(), 7);
        assert_eq!(SafetyFacilityType::MedicalStation.inspection_frequency_days(), 1);
    }

    #[test]
    fn test_safety_requirements() {
        let rules = SportsFacilityRules::new();
        let requirements = rules.safety_requirements();
        assert!(!requirements.is_empty());

        // 所有项目都应该是必须的
        for item in &requirements {
            assert!(item.mandatory);
        }
    }

    #[test]
    fn test_field_specification() {
        let spec = FieldSpecification::new("足球场", 105.0, 68.0, 2.0);
        assert_eq!(spec.name, "足球场");
        assert_eq!(spec.length_meters, 105.0);
        assert_eq!(spec.width_meters, 68.0);
        assert_eq!(spec.area_sqm, 7140.0);

        // 验证尺寸
        assert!(spec.validate_dimensions(105.0, 68.0));
        assert!(spec.validate_dimensions(106.5, 69.0)); // 在误差范围内
        assert!(!spec.validate_dimensions(110.0, 75.0)); // 超出误差范围
    }

    #[test]
    fn test_standard_field_specifications() {
        let rules = SportsFacilityRules::new();
        let specs = rules.standard_field_specifications();
        assert!(!specs.is_empty());

        // 验证常见场地规格存在
        let football_field = specs.iter().find(|s| s.name == "足球场");
        assert!(football_field.is_some());
    }

    #[test]
    fn test_validate_stadium_for_level() {
        let rules = SportsFacilityRules::new();

        // 50000人体育场满足国际A级
        assert!(rules.validate_stadium_for_level(
            StadiumType::FootballStadium,
            50000,
            FieldQualityLevel::InternationalA,
        ));

        // 10000人体育场不满足国际A级
        assert!(!rules.validate_stadium_for_level(
            StadiumType::FootballStadium,
            10000,
            FieldQualityLevel::InternationalA,
        ));

        // 10000人体育场满足国家级
        assert!(rules.validate_stadium_for_level(
            StadiumType::FootballStadium,
            10000,
            FieldQualityLevel::National,
        ));
    }

    #[test]
    fn test_equipment_inspection_standards() {
        let rules = SportsFacilityRules::new();
        let standards = rules.equipment_inspection_standards();
        assert!(!standards.is_empty());

        // 验证医疗设备检验频率最短
        let medical = standards.iter().find(|(name, _)| *name == "医疗设备");
        assert!(medical.is_some());
        assert_eq!(medical.unwrap().1, 7);
    }

    #[test]
    fn test_stadium_type_names() {
        assert_eq!(StadiumType::FootballStadium.name(), "足球场");
        assert_eq!(StadiumType::BasketballArena.name(), "篮球馆");
        assert_eq!(StadiumType::AquaticsCenter.name(), "游泳馆");
    }

    #[test]
    fn test_field_quality_level_names() {
        assert_eq!(FieldQualityLevel::InternationalA.name(), "国际A级");
        assert_eq!(FieldQualityLevel::National.name(), "国家级");
        assert_eq!(FieldQualityLevel::Amateur.name(), "业余级");
    }

    #[test]
    fn test_equipment_type_names() {
        assert_eq!(EquipmentType::CompetitionEquipment.name(), "比赛器材");
        assert_eq!(EquipmentType::SafetyEquipment.name(), "安全器材");
    }

    #[test]
    fn test_safety_facility_type_names() {
        assert_eq!(SafetyFacilityType::FireSafety.name(), "消防设施");
        assert_eq!(SafetyFacilityType::EmergencyExit.name(), "紧急出口");
        assert_eq!(SafetyFacilityType::Accessibility.name(), "无障碍设施");
    }

    #[test]
    fn test_rule_trait() {
        let rules = SportsFacilityRules::new();
        assert_eq!(rules.metadata().name, "体育设施规则");

        let explanation = rules.explain();
        assert!(!explanation.is_empty());
        assert!(explanation.contains("体育设施规则"));
    }

    #[test]
    fn test_default_implementation() {
        let rules = SportsFacilityRules::default();
        assert_eq!(rules.metadata().name, "体育设施规则");
    }

    #[test]
    fn test_field_surface_types() {
        let rules = SportsFacilityRules::new();
        let surfaces = rules.field_surface_types();
        assert_eq!(surfaces.len(), 12);
    }

    #[test]
    fn test_field_surface_type_names() {
        assert_eq!(FieldSurfaceType::NaturalGrass.name(), "天然草坪");
        assert_eq!(FieldSurfaceType::ArtificialTurf.name(), "人工草坪");
        assert_eq!(FieldSurfaceType::SyntheticTrack.name(), "塑胶跑道");
        assert_eq!(FieldSurfaceType::WoodenFloor.name(), "木地板");
    }

    #[test]
    fn test_field_surface_maintenance_frequency() {
        // 天然草坪需要每日维护
        assert_eq!(FieldSurfaceType::NaturalGrass.maintenance_frequency_days(), 1);
        // 塑胶跑道需要每月维护
        assert_eq!(FieldSurfaceType::SyntheticTrack.maintenance_frequency_days(), 30);
    }

    #[test]
    fn test_field_certification_levels() {
        let rules = SportsFacilityRules::new();
        let levels = rules.field_certification_levels();
        assert_eq!(levels.len(), 7);
    }

    #[test]
    fn test_field_certification_level_names() {
        assert_eq!(FieldCertificationLevel::FifaCertified.name(), "国际足联认证");
        assert_eq!(FieldCertificationLevel::FibaCertified.name(), "国际篮联认证");
        assert_eq!(FieldCertificationLevel::OlympicCertified.name(), "奥运会认证");
    }

    #[test]
    fn test_field_certification_validity() {
        // FIFA认证有效期3年
        assert_eq!(FieldCertificationLevel::FifaCertified.validity_years(), 3);
        // FIBA认证有效期5年
        assert_eq!(FieldCertificationLevel::FibaCertified.validity_years(), 5);
    }

    #[test]
    fn test_field_maintenance_requirements() {
        let rules = SportsFacilityRules::new();
        let requirements = rules.field_maintenance_requirements();
        assert!(!requirements.is_empty());

        // 验证天然草坪维护要求
        let grass_req = requirements.iter().find(|r| r.name == "天然草坪维护");
        assert!(grass_req.is_some());
        assert_eq!(grass_req.unwrap().inspection_frequency_days, 1);
    }

    #[test]
    fn test_validate_surface_for_sport() {
        let rules = SportsFacilityRules::new();

        // 足球适合天然草坪和人工草坪
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::NaturalGrass, "足球"));
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::ArtificialTurf, "足球"));
        assert!(!rules.validate_surface_for_sport(FieldSurfaceType::WoodenFloor, "足球"));

        // 篮球适合木地板和混凝土
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::WoodenFloor, "篮球"));
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::Concrete, "篮球"));
        assert!(!rules.validate_surface_for_sport(FieldSurfaceType::NaturalGrass, "篮球"));

        // 网球适合红土、硬地和草地
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::Clay, "网球"));
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::HardCourt, "网球"));
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::Grass, "网球"));

        // 游泳适合水面
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::Water, "游泳"));

        // 冰球适合冰面
        assert!(rules.validate_surface_for_sport(FieldSurfaceType::Ice, "冰球"));
    }
}