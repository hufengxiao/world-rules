//! 运动损伤处理规则
//!
//! 基于运动医学标准，包含损伤分类、现场处理、康复流程、返回运动决策等。
//!
//! # 规则体系
//!
//! - 急性损伤处理（PRICE原则）
//! - 慢性损伤管理
//! - 脑震荡评估流程
//! - 返回运动决策标准
//!
//! # Examples
//!
//! ```
//! use world_rules::rules::sports::sports_injury::SportsInjuryRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = SportsInjuryRules::new();
//! assert!(!rules.common_injury_types().is_empty());
//! assert!(!rules.price_protocol_steps().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 损伤严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InjurySeverity {
    /// 轻度（I度）
    Mild,
    /// 中度（II度）
    Moderate,
    /// 重度（III度）
    Severe,
    /// 危急生命
    Critical,
}

impl InjurySeverity {
    /// 获取程度名称
    pub fn name(&self) -> &'static str {
        match self {
            InjurySeverity::Mild => "轻度",
            InjurySeverity::Moderate => "中度",
            InjurySeverity::Severe => "重度",
            InjurySeverity::Critical => "危急",
        }
    }

    /// 是否需要立即就医
    pub fn requires_immediate_medical_care(&self) -> bool {
        matches!(self, InjurySeverity::Severe | InjurySeverity::Critical)
    }

    /// 预估恢复时间（天）
    pub fn estimated_recovery_days(&self) -> u32 {
        match self {
            InjurySeverity::Mild => 3..7,
            InjurySeverity::Moderate => 14..42,
            InjurySeverity::Severe => 42..180,
            InjurySeverity::Critical => 90..365,
        }
        .next()
        .unwrap_or(7)
    }
}

/// 损伤类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InjuryType {
    // 急性损伤
    /// 肌肉拉伤
    MuscleStrain,
    /// 韧带扭伤
    LigamentSprain,
    /// 骨折
    Fracture,
    /// 关节脱位
    Dislocation,
    /// 脑震荡
    Concussion,
    /// 挫伤/撞伤
    Contusion,
    /// 切割伤
    Laceration,
    /// 烧伤
    Burn,

    // 慢性损伤
    /// 肌腱炎
    Tendinitis,
    /// 滑囊炎
    Bursitis,
    /// 疲劳性骨折
    StressFracture,
    /// 关节炎
    Arthritis,
    /// 椎间盘突出
    DiscHerniation,
    /// 肩袖损伤
    RotatorCuffInjury,
    /// 网球肘
    TennisElbow,
    /// 跑步膝
    RunnersKnee,
}

impl InjuryType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            InjuryType::MuscleStrain => "肌肉拉伤",
            InjuryType::LigamentSprain => "韧带扭伤",
            InjuryType::Fracture => "骨折",
            InjuryType::Dislocation => "关节脱位",
            InjuryType::Concussion => "脑震荡",
            InjuryType::Contusion => "挫伤",
            InjuryType::Laceration => "切割伤",
            InjuryType::Burn => "烧伤",
            InjuryType::Tendinitis => "肌腱炎",
            InjuryType::Bursitis => "滑囊炎",
            InjuryType::StressFracture => "疲劳性骨折",
            InjuryType::Arthritis => "关节炎",
            InjuryType::DiscHerniation => "椎间盘突出",
            InjuryType::RotatorCuffInjury => "肩袖损伤",
            InjuryType::TennisElbow => "网球肘",
            InjuryType::RunnersKnee => "跑步膝",
        }
    }

    /// 是否为急性损伤
    pub fn is_acute(&self) -> bool {
        matches!(
            self,
            InjuryType::MuscleStrain
                | InjuryType::LigamentSprain
                | InjuryType::Fracture
                | InjuryType::Dislocation
                | InjuryType::Concussion
                | InjuryType::Contusion
                | InjuryType::Laceration
                | InjuryType::Burn
        )
    }

    /// 是否为慢性损伤
    pub fn is_chronic(&self) -> bool {
        !self.is_acute()
    }

    /// 常见运动项目
    pub fn common_sports(&self) -> Vec<&'static str> {
        match self {
            InjuryType::MuscleStrain => vec!["田径", "足球", "篮球", "游泳"],
            InjuryType::LigamentSprain => vec!["足球", "篮球", "滑雪", "橄榄球"],
            InjuryType::Fracture => vec!["足球", "自行车", "滑雪", "体操"],
            InjuryType::Dislocation => vec!["篮球", "足球", "橄榄球", "柔道"],
            InjuryType::Concussion => vec!["拳击", "足球", "橄榄球", "冰球"],
            InjuryType::Contusion => vec!["足球", "篮球", "冰球", "曲棍球"],
            InjuryType::Laceration => vec!["滑冰", "滑雪", "自行车", "攀岩"],
            InjuryType::Burn => vec!["赛车", "摩托车", "举重"],
            InjuryType::Tendinitis => vec!["网球", "游泳", "跑步", "排球"],
            InjuryType::Bursitis => vec!["跑步", "足球", "排球"],
            InjuryType::StressFracture => vec!["马拉松", "体操", "舞蹈"],
            InjuryType::Arthritis => vec!["足球", "篮球", "跑步"],
            InjuryType::DiscHerniation => vec!["举重", "体操", "摔跤"],
            InjuryType::RotatorCuffInjury => vec!["游泳", "网球", "棒球", "排球"],
            InjuryType::TennisElbow => vec!["网球", "羽毛球", "乒乓球"],
            InjuryType::RunnersKnee => vec!["马拉松", "越野跑", "篮球"],
        }
    }
}

/// PRICE 原则步骤
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceStep {
    /// P - Protection（保护）
    Protection,
    /// R - Rest（休息）
    Rest,
    /// I - Ice（冰敷）
    Ice,
    /// C - Compression（加压）
    Compression,
    /// E - Elevation（抬高）
    Elevation,
}

impl PriceStep {
    /// 获取步骤名称
    pub fn name(&self) -> &'static str {
        match self {
            PriceStep::Protection => "保护",
            PriceStep::Rest => "休息",
            PriceStep::Ice => "冰敷",
            PriceStep::Compression => "加压",
            PriceStep::Elevation => "抬高",
        }
    }

    /// 获取详细说明
    pub fn description(&self) -> &'static str {
        match self {
            PriceStep::Protection => "使用支具、绷带或夹板保护受伤部位，防止进一步损伤",
            PriceStep::Rest => "停止运动，避免负重和活动，让受伤组织得到休息",
            PriceStep::Ice => "冰敷15-20分钟，每2-3小时一次，减少出血、肿胀和疼痛",
            PriceStep::Compression => "使用弹性绷带适度加压包扎，减少肿胀和出血",
            PriceStep::Elevation => "将受伤部位抬高至心脏水平以上，促进血液回流，减少肿胀",
        }
    }

    /// 获取执行时间建议
    pub fn duration_recommendation(&self) -> &'static str {
        match self {
            PriceStep::Protection => "持续保护至医生评估",
            PriceStep::Rest => "急性期48-72小时",
            PriceStep::Ice => "15-20分钟/次，间隔至少1小时",
            PriceStep::Compression => "持续包扎，每4小时放松一次",
            PriceStep::Elevation => "尽可能长时间抬高",
        }
    }
}

/// 脑震荡评估等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcussionGrade {
    /// I级 - 轻度
    Grade1,
    /// II级 - 中度
    Grade2,
    /// III级 - 重度
    Grade3,
}

impl ConcussionGrade {
    /// 获取等级名称
    pub fn name(&self) -> &'static str {
        match self {
            ConcussionGrade::Grade1 => "I级（轻度）",
            ConcussionGrade::Grade2 => "II级（中度）",
            ConcussionGrade::Grade3 => "III级（重度）",
        }
    }

    /// 获取症状
    pub fn symptoms(&self) -> Vec<&'static str> {
        match self {
            ConcussionGrade::Grade1 => vec!["头痛", "头晕", "注意力不集中", "记忆轻微障碍"],
            ConcussionGrade::Grade2 => vec![
                "意识模糊",
                "定向障碍",
                "明显记忆障碍",
                "平衡障碍",
                "恶心呕吐",
            ],
            ConcussionGrade::Grade3 => vec!["意识丧失", "长时间记忆丧失", "严重头痛", "癫痫发作"],
        }
    }

    /// 最小休息时间（天）
    pub fn minimum_rest_days(&self) -> u32 {
        match self {
            ConcussionGrade::Grade1 => 7,
            ConcussionGrade::Grade2 => 14,
            ConcussionGrade::Grade3 => 30,
        }
    }

    /// 是否需要影像学检查
    pub fn requires_imaging(&self) -> bool {
        matches!(self, ConcussionGrade::Grade2 | ConcussionGrade::Grade3)
    }
}

/// 返回运动阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnToPlayStage {
    /// 第1阶段：症状限制活动
    Stage1,
    /// 第2阶段：轻度有氧运动
    Stage2,
    /// 第3阶段：运动专项训练
    Stage3,
    /// 第4阶段：非对抗训练
    Stage4,
    /// 第5阶段：完全对抗训练
    Stage5,
    /// 第6阶段：完全恢复比赛
    Stage6,
}

impl ReturnToPlayStage {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            ReturnToPlayStage::Stage1 => "症状限制活动",
            ReturnToPlayStage::Stage2 => "轻度有氧运动",
            ReturnToPlayStage::Stage3 => "运动专项训练",
            ReturnToPlayStage::Stage4 => "非对抗训练",
            ReturnToPlayStage::Stage5 => "完全对抗训练",
            ReturnToPlayStage::Stage6 => "完全恢复比赛",
        }
    }

    /// 获取活动内容
    pub fn activities(&self) -> Vec<&'static str> {
        match self {
            ReturnToPlayStage::Stage1 => vec!["休息", "日常生活活动"],
            ReturnToPlayStage::Stage2 => vec!["步行", "游泳", "固定自行车"],
            ReturnToPlayStage::Stage3 => vec!["跑步", "滑冰", "专项技术训练（无对抗）"],
            ReturnToPlayStage::Stage4 => vec!["复杂技术训练", "部分对抗训练"],
            ReturnToPlayStage::Stage5 => vec!["完全训练", "对抗练习", "团队战术"],
            ReturnToPlayStage::Stage6 => vec!["正式比赛"],
        }
    }

    /// 每阶段最短持续时间（天）
    pub fn minimum_duration_days(&self) -> u32 {
        match self {
            ReturnToPlayStage::Stage1 => 1,
            ReturnToPlayStage::Stage2 => 1,
            ReturnToPlayStage::Stage3 => 1,
            ReturnToPlayStage::Stage4 => 2,
            ReturnToPlayStage::Stage5 => 2,
            ReturnToPlayStage::Stage6 => 1,
        }
    }

    /// 进阶条件
    pub fn advancement_criteria(&self) -> Vec<&'static str> {
        match self {
            ReturnToPlayStage::Stage1 => vec!["无症状", "认知功能正常"],
            ReturnToPlayStage::Stage2 => vec!["运动无症状加重", "心率不超过70%最大心率"],
            ReturnToPlayStage::Stage3 => vec!["专项技术无障碍", "平衡测试通过"],
            ReturnToPlayStage::Stage4 => vec!["非对抗训练无症状", "教练评估通过"],
            ReturnToPlayStage::Stage5 => vec!["对抗训练无症状", "医疗评估通过"],
            ReturnToPlayStage::Stage6 => vec!["医生最终许可", "运动员同意"],
        }
    }
}

/// 运动损伤处理规则
#[derive(Debug, Clone)]
pub struct SportsInjuryRules {
    metadata: RuleMetadata,
}

impl Default for SportsInjuryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl SportsInjuryRules {
    /// 创建新的运动损伤规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "运动损伤处理规则",
                "基于运动医学标准的损伤分类、处理流程和康复指南",
            )
            .with_origin("国际运动医学联合会(FIMS)")
            .with_tags(vec![
                "运动医学".into(),
                "损伤处理".into(),
                "康复".into(),
                "PRICE".into(),
                "脑震荡".into(),
            ]),
        }
    }

    /// 获取常见损伤类型列表
    pub fn common_injury_types(&self) -> Vec<InjuryType> {
        vec![
            InjuryType::MuscleStrain,
            InjuryType::LigamentSprain,
            InjuryType::Fracture,
            InjuryType::Dislocation,
            InjuryType::Concussion,
            InjuryType::Contusion,
            InjuryType::Tendinitis,
            InjuryType::RotatorCuffInjury,
            InjuryType::RunnersKnee,
            InjuryType::TennisElbow,
        ]
    }

    /// 获取 PRICE 协议步骤
    pub fn price_protocol_steps(&self) -> Vec<PriceStep> {
        vec![
            PriceStep::Protection,
            PriceStep::Rest,
            PriceStep::Ice,
            PriceStep::Compression,
            PriceStep::Elevation,
        ]
    }

    /// 获取 PRICE 协议完整说明
    pub fn price_protocol_description(&self) -> Vec<String> {
        self.price_protocol_steps()
            .iter()
            .map(|step| {
                format!(
                    "{}: {}\n  执行时间: {}",
                    step.name(),
                    step.description(),
                    step.duration_recommendation()
                )
            })
            .collect()
    }

    /// 获取现场急救流程
    pub fn emergency_response_protocol(&self) -> Vec<String> {
        vec![
            "1. 确保现场安全，防止二次伤害".to_string(),
            "2. 评估伤者意识和呼吸".to_string(),
            "3. 呼叫急救服务（如需要）".to_string(),
            "4. 检查生命体征（脉搏、呼吸、血压）".to_string(),
            "5. 执行 PRICE 原则".to_string(),
            "6. 观察休克症状".to_string(),
            "7. 防止低体温".to_string(),
            "8. 记录损伤时间和处理措施".to_string(),
            "9. 准备转运（如需要）".to_string(),
            "10. 向医疗人员交接".to_string(),
        ]
    }

    /// 获取脑震荡评估流程
    pub fn concussion_assessment_protocol(&self) -> Vec<String> {
        vec![
            "立即移除运动员，不允许当日返回比赛".to_string(),
            "进行 SCAT5（运动脑震荡评估工具）评估".to_string(),
            "检查意识状态和定向力".to_string(),
            "评估记忆和认知功能".to_string(),
            "进行平衡测试".to_string(),
            "神经系统检查".to_string(),
            "确定脑震荡等级（I/II/III级）".to_string(),
            "制定休息和康复计划".to_string(),
            "设定返回运动时间表".to_string(),
            "获得医疗许可后逐步返回".to_string(),
        ]
    }

    /// 获取返回运动决策标准
    pub fn return_to_play_criteria(&self) -> Vec<String> {
        vec![
            "症状完全消失，包括休息和运动时".to_string(),
            "认知功能测试正常".to_string(),
            "平衡测试正常".to_string(),
            "神经系统检查无异常".to_string(),
            "完成6阶段返回运动流程".to_string(),
            "医疗专业人员许可".to_string(),
            "运动员自觉状态良好".to_string(),
            "运动专项能力测试通过".to_string(),
            "心理准备就绪".to_string(),
            "签署知情同意书".to_string(),
        ]
    }

    /// 获取返回运动阶段
    pub fn return_to_play_stages(&self) -> Vec<ReturnToPlayStage> {
        vec![
            ReturnToPlayStage::Stage1,
            ReturnToPlayStage::Stage2,
            ReturnToPlayStage::Stage3,
            ReturnToPlayStage::Stage4,
            ReturnToPlayStage::Stage5,
            ReturnToPlayStage::Stage6,
        ]
    }

    /// 获取预防措施
    pub fn prevention_measures(&self) -> Vec<String> {
        vec![
            "充分热身和拉伸".to_string(),
            "循序渐进增加运动强度".to_string(),
            "使用适当的保护装备".to_string(),
            "保持良好的身体素质".to_string(),
            "及时处理微小不适".to_string(),
            "充足的休息和恢复".to_string(),
            "正确的技术和姿势".to_string(),
            "合适的运动场地和器材".to_string(),
            "定期体能评估".to_string(),
            "营养均衡和充分补水".to_string(),
        ]
    }

    /// 获取常见运动损伤预防指南
    pub fn sport_specific_prevention_guides(&self) -> Vec<String> {
        vec![
            "足球：踝关节保护、股四头肌加强、热身充分".to_string(),
            "篮球：踝关节支持、落地技术训练、护膝使用".to_string(),
            "跑步：循序渐进、鞋子选择、路面变化、休息日安排".to_string(),
            "游泳：肩袖肌群训练、热身充分、技术正确".to_string(),
            "网球：肘关节保护、技术规范、场地选择".to_string(),
            "体操：核心力量、柔韧性训练、落地训练".to_string(),
            "滑雪：护具佩戴、技术训练、天气评估".to_string(),
            "举重：正确姿势、渐进负荷、充分热身".to_string(),
        ]
    }

    /// 获取康复原则
    pub fn rehabilitation_principles(&self) -> Vec<String> {
        vec![
            "个体化康复方案".to_string(),
            "循序渐进原则".to_string(),
            "主动参与原则".to_string(),
            "全面康复原则（生理、心理、社会）".to_string(),
            "预防再损伤原则".to_string(),
            "定期评估调整原则".to_string(),
            "多学科协作原则".to_string(),
            "运动员教育原则".to_string(),
        ]
    }

    /// 获取损伤评估工具
    pub fn assessment_tools(&self) -> Vec<String> {
        vec![
            "SCAT5（运动脑震荡评估工具）".to_string(),
            "BESS（平衡错误评分系统）".to_string(),
            "ImPACT（即刻脑震荡评估测试）".to_string(),
            "损伤严重程度评分（ISS）".to_string(),
            "牛津踝关节评分".to_string(),
            "Lysholm膝关节评分".to_string(),
            "Constant肩关节评分".to_string(),
            "VAS疼痛评分量表".to_string(),
        ]
    }

    /// 获取必须立即就医的情况
    pub fn immediate_medical_care_situations(&self) -> Vec<String> {
        vec![
            "意识丧失或意识模糊".to_string(),
            "严重出血".to_string(),
            "呼吸困难".to_string(),
            "疑似脊柱损伤".to_string(),
            "疑似骨折或脱位".to_string(),
            "严重头部损伤".to_string(),
            "胸腹部严重创伤".to_string(),
            "严重过敏反应".to_string(),
            "心脏骤停症状".to_string(),
            "中暑或热衰竭".to_string(),
        ]
    }

    /// 获取急救设备清单
    pub fn first_aid_equipment(&self) -> Vec<String> {
        vec![
            "急救包（绷带、消毒纱布、胶带）".to_string(),
            "冰袋或冰块".to_string(),
            "夹板（可塑型）".to_string(),
            "颈托".to_string(),
            "氧气袋".to_string(),
            "自动体外除颤器（AED）".to_string(),
            "担架或脊柱板".to_string(),
            "止血带".to_string(),
            "眼科冲洗液".to_string(),
            "手套和口罩".to_string(),
        ]
    }
}

impl Rule for SportsInjuryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Sports("运动医学".to_string())
    }

    fn validate(&self, _context: &ValidateContext) -> RuleResult<bool> {
        // 基础验证：规则是否正确配置
        Ok(true)
    }

    fn explain(&self) -> String {
        let mut explanation = String::new();
        explanation.push_str("# 运动损伤处理规则\n\n");
        explanation.push_str("基于国际运动医学标准，涵盖：\n\n");

        explanation.push_str("## 1. 损伤分类\n");
        explanation.push_str("- 急性损伤：拉伤、扭伤、骨折、脱位、脑震荡等\n");
        explanation.push_str("- 慢性损伤：肌腱炎、疲劳性骨折、关节炎等\n\n");

        explanation.push_str("## 2. 现场处理（PRICE原则）\n");
        for step in self.price_protocol_steps() {
            explanation.push_str(&format!("- {}: {}\n", step.name(), step.description()));
        }
        explanation.push('\n');

        explanation.push_str("## 3. 脑震荡管理\n");
        explanation.push_str("- SCAT5评估\n");
        explanation.push_str("- 阶梯式返回运动流程\n");
        explanation.push_str("- 最少休息时间要求\n\n");

        explanation.push_str("## 4. 返回运动决策\n");
        explanation.push_str("- 6阶段渐进式返回\n");
        explanation.push_str("- 医疗许可要求\n");
        explanation.push_str("- 症状监测\n");
        explanation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injury_severity() {
        assert_eq!(InjurySeverity::Mild.name(), "轻度");
        assert_eq!(InjurySeverity::Moderate.name(), "中度");
        assert_eq!(InjurySeverity::Severe.name(), "重度");
        assert_eq!(InjurySeverity::Critical.name(), "危急");

        // 危急和重度需要立即就医
        assert!(InjurySeverity::Critical.requires_immediate_medical_care());
        assert!(InjurySeverity::Severe.requires_immediate_medical_care());
        assert!(!InjurySeverity::Mild.requires_immediate_medical_care());
    }

    #[test]
    fn test_injury_type_classification() {
        // 急性损伤测试
        assert!(InjuryType::MuscleStrain.is_acute());
        assert!(InjuryType::Concussion.is_acute());
        assert!(InjuryType::Fracture.is_acute());

        // 慢性损伤测试
        assert!(InjuryType::Tendinitis.is_chronic());
        assert!(InjuryType::StressFracture.is_chronic());

        // 运动项目关联
        let sports = InjuryType::Concussion.common_sports();
        assert!(sports.contains(&"拳击"));
        assert!(sports.contains(&"足球"));
    }

    #[test]
    fn test_price_protocol() {
        let rules = SportsInjuryRules::new();
        let steps = rules.price_protocol_steps();

        assert_eq!(steps.len(), 5);
        assert_eq!(steps[0], PriceStep::Protection);
        assert_eq!(steps[4], PriceStep::Elevation);

        // 测试步骤说明
        let protection = PriceStep::Protection;
        assert!(protection.description().contains("保护"));
        assert!(!protection.duration_recommendation().is_empty());
    }

    #[test]
    fn test_concussion_grades() {
        // 等级命名
        assert!(ConcussionGrade::Grade1.name().contains("I级"));
        assert!(ConcussionGrade::Grade3.name().contains("III级"));

        // 症状测试
        let grade3_symptoms = ConcussionGrade::Grade3.symptoms();
        assert!(grade3_symptoms.contains(&"意识丧失"));

        // 休息时间
        assert!(
            ConcussionGrade::Grade1.minimum_rest_days()
                < ConcussionGrade::Grade3.minimum_rest_days()
        );

        // 影像学检查
        assert!(!ConcussionGrade::Grade1.requires_imaging());
        assert!(ConcussionGrade::Grade3.requires_imaging());
    }

    #[test]
    fn test_return_to_play_stages() {
        let rules = SportsInjuryRules::new();
        let stages = rules.return_to_play_stages();

        assert_eq!(stages.len(), 6);
        assert_eq!(stages[0], ReturnToPlayStage::Stage1);

        // 测试阶段内容
        let stage6 = ReturnToPlayStage::Stage6;
        assert!(stage6.name().contains("完全"));
        assert!(stage6.activities().contains(&"正式比赛"));
        assert!(stage6.minimum_duration_days() >= 1);
    }

    #[test]
    fn test_emergency_response_protocol() {
        let rules = SportsInjuryRules::new();
        let protocol = rules.emergency_response_protocol();

        assert!(protocol.len() >= 10);

        // 检查关键步骤
        let has_safety = protocol.iter().any(|p| p.contains("安全"));
        assert!(has_safety);

        let has_price = protocol.iter().any(|p| p.contains("PRICE"));
        assert!(has_price);
    }

    #[test]
    fn test_concussion_assessment() {
        let rules = SportsInjuryRules::new();
        let assessment = rules.concussion_assessment_protocol();

        // 检查SCAT5
        let has_scat5 = assessment.iter().any(|a| a.contains("SCAT5"));
        assert!(has_scat5);

        // 检查返回运动
        let has_return = assessment.iter().any(|a| a.contains("返回"));
        assert!(has_return);
    }

    #[test]
    fn test_prevention_measures() {
        let rules = SportsInjuryRules::new();
        let measures = rules.prevention_measures();

        assert!(measures.len() >= 10);

        // 检查关键措施
        let has_warmup = measures.iter().any(|m| m.contains("热身"));
        assert!(has_warmup);

        let has_rest = measures.iter().any(|m| m.contains("休息"));
        assert!(has_rest);
    }

    #[test]
    fn test_sport_specific_prevention() {
        let rules = SportsInjuryRules::new();
        let guides = rules.sport_specific_prevention_guides();

        // 检查运动特定预防
        let has_football = guides.iter().any(|g| g.contains("足球"));
        assert!(has_football);

        let has_basketball = guides.iter().any(|g| g.contains("篮球"));
        assert!(has_basketball);
    }

    #[test]
    fn test_rehabilitation_principles() {
        let rules = SportsInjuryRules::new();
        let principles = rules.rehabilitation_principles();

        assert!(!principles.is_empty());

        // 检查关键原则
        let has_individual = principles.iter().any(|p| p.contains("个体化"));
        assert!(has_individual);

        let has_progressive = principles.iter().any(|p| p.contains("循序渐进"));
        assert!(has_progressive);
    }

    #[test]
    fn test_assessment_tools() {
        let rules = SportsInjuryRules::new();
        let tools = rules.assessment_tools();

        assert!(tools.len() >= 5);

        // 检查SCAT5
        let has_scat5 = tools.iter().any(|t| t.contains("SCAT5"));
        assert!(has_scat5);

        // 检查疼痛评分
        let has_vas = tools.iter().any(|t| t.contains("VAS"));
        assert!(has_vas);
    }

    #[test]
    fn test_immediate_medical_care_situations() {
        let rules = SportsInjuryRules::new();
        let situations = rules.immediate_medical_care_situations();

        assert!(situations.len() >= 10);

        // 检查关键情况
        let has_unconscious = situations.iter().any(|s| s.contains("意识"));
        assert!(has_unconscious);

        let has_spine = situations.iter().any(|s| s.contains("脊柱"));
        assert!(has_spine);
    }

    #[test]
    fn test_first_aid_equipment() {
        let rules = SportsInjuryRules::new();
        let equipment = rules.first_aid_equipment();

        assert!(equipment.len() >= 10);

        // 检查关键设备
        let has_aed = equipment.iter().any(|e| e.contains("AED"));
        assert!(has_aed);

        let has_splint = equipment.iter().any(|e| e.contains("夹板"));
        assert!(has_splint);
    }

    #[test]
    fn test_rule_trait_implementation() {
        use crate::rules::core::Rule;

        let rules = SportsInjuryRules::new();

        // 测试 metadata
        let metadata = rules.metadata();
        assert!(metadata.name.contains("损伤"));

        // 测试 category
        let category = rules.category();
        assert!(matches!(category, RuleCategory::Sports(_)));

        // 测试 explain
        let explanation = rules.explain();
        assert!(explanation.contains("PRICE"));
        assert!(explanation.contains("脑震荡"));
    }

    #[test]
    fn test_price_step_descriptions() {
        assert!(PriceStep::Protection.description().contains("保护"));
        assert!(PriceStep::Ice.description().contains("冰敷"));
        assert!(PriceStep::Compression.description().contains("加压"));
        assert!(PriceStep::Elevation.description().contains("抬高"));
    }

    #[test]
    fn test_return_to_play_advancement_criteria() {
        let stage1 = ReturnToPlayStage::Stage1;
        let criteria = stage1.advancement_criteria();
        assert!(!criteria.is_empty());

        let stage5 = ReturnToPlayStage::Stage5;
        let criteria5 = stage5.advancement_criteria();
        assert!(criteria5.iter().any(|c| c.contains("医疗")));
    }
}
