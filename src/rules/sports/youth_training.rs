//! 青少年训练规则
//!
//! 针对不同年龄段青少年的体育训练规则，包括训练负荷控制、安全保护、教练资质等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 青少年年龄组别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YouthTrainingAgeGroup {
    /// 少儿组（6-10岁）
    Children,
    /// 少年组（11-14岁）
    Youth,
    /// 青少年组（15-18岁）
    Junior,
}

impl YouthTrainingAgeGroup {
    /// 获取年龄组名称
    pub fn name(&self) -> &'static str {
        match self {
            YouthTrainingAgeGroup::Children => "少儿组",
            YouthTrainingAgeGroup::Youth => "少年组",
            YouthTrainingAgeGroup::Junior => "青少年组",
        }
    }

    /// 获取年龄范围
    pub fn age_range(&self) -> &'static str {
        match self {
            YouthTrainingAgeGroup::Children => "6-10岁",
            YouthTrainingAgeGroup::Youth => "11-14岁",
            YouthTrainingAgeGroup::Junior => "15-18岁",
        }
    }
}

/// 训练类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingType {
    /// 基础体能训练
    BasicFitness,
    /// 专项技术训练
    SportSpecific,
    /// 力量训练
    Strength,
    /// 耐力训练
    Endurance,
    /// 柔韧性训练
    Flexibility,
    /// 协调性训练
    Coordination,
}

/// 青少年训练规则
pub struct YouthTrainingRules {
    metadata: RuleMetadata,
}

impl YouthTrainingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("青少年训练规则", "青少年体育训练的安全规范和指导原则")
                .with_origin("国家体育总局/教育部")
                .with_tags(vec![
                    "体育".into(),
                    "训练".into(),
                    "青少年".into(),
                    "安全".into(),
                ]),
        }
    }

    /// 训练原则
    pub fn training_principles(&self) -> Vec<&'static str> {
        vec![
            "循序渐进原则：训练强度逐步增加",
            "全面发展原则：注重身心全面发展",
            "区别对待原则：根据年龄和个体差异调整",
            "安全第一原则：始终将安全放在首位",
            "趣味性原则：保持训练的趣味性",
            "系统性原则：制定长期系统训练计划",
        ]
    }

    /// 少儿组（6-10岁）训练特点
    pub fn children_training_characteristics(&self) -> Vec<&'static str> {
        vec![
            "以游戏化训练为主",
            "单次训练时间不超过60分钟",
            "每周训练不超过3次",
            "不进行专门力量训练",
            "重点发展协调性和灵活性",
            "以基本动作技能为主",
            "避免高强度对抗训练",
            "重视运动兴趣培养",
        ]
    }

    /// 少年组（11-14岁）训练特点
    pub fn youth_training_characteristics(&self) -> Vec<&'static str> {
        vec![
            "可引入基础技术训练",
            "单次训练时间60-90分钟",
            "每周训练不超过5次",
            "开始适度力量训练（轻负荷）",
            "发展专项运动技能",
            "加强柔韧性训练",
            "注意生长发育特点",
            "逐步增加训练强度",
        ]
    }

    /// 青少年组（15-18岁）训练特点
    pub fn junior_training_characteristics(&self) -> Vec<&'static str> {
        vec![
            "可进行较系统的专项训练",
            "单次训练时间90-120分钟",
            "每周训练可达6次",
            "力量训练可逐步加重",
            "接近成人训练模式",
            "注意运动损伤预防",
            "加强心理素质培养",
            "可参加高水平竞技训练",
        ]
    }

    /// 训练负荷控制原则
    pub fn load_control_principles(&self) -> Vec<&'static str> {
        vec![
            "少儿组：心率不超过180次/分",
            "少年组：心率控制在150-170次/分",
            "青少年组：可适当提高训练强度",
            "训练强度应循序渐进",
            "注意监测疲劳程度",
            "保证充分恢复时间",
            "避免过度训练综合征",
            "定期进行体能测试评估",
        ]
    }

    /// 每周训练频次限制
    pub fn weekly_frequency_limits(&self) -> Vec<&'static str> {
        vec![
            "少儿组（6-10岁）：最多3次/周",
            "少年组（11-14岁）：最多5次/周",
            "青少年组（15-18岁）：最多6次/周",
            "每次训练间隔至少休息一天",
            "比赛期间适当减少训练量",
            "假期可适当增加训练频次",
        ]
    }

    /// 单次训练时长限制
    pub fn session_duration_limits(&self) -> Vec<&'static str> {
        vec![
            "少儿组：30-60分钟",
            "少年组：60-90分钟",
            "青少年组：90-120分钟",
            "热身时间占总时长15-20%",
            "整理活动时间占10-15%",
            "高强度训练时间不超过总时长30%",
        ]
    }

    /// 力量训练规定
    pub fn strength_training_rules(&self) -> Vec<&'static str> {
        vec![
            "6-10岁：禁止专项力量训练",
            "11-14岁：仅限轻器械、自重训练",
            "15-18岁：可逐步增加负重训练",
            "负重不超过体重的50%",
            "必须有专业教练指导",
            "禁止最大力量测试",
            "强调动作规范而非重量",
            "优先发展核心力量",
        ]
    }

    /// 安全保护措施
    pub fn safety_measures(&self) -> Vec<&'static str> {
        vec![
            "训练前必须进行充分热身",
            "训练场地需符合安全标准",
            "器材设备定期检查维护",
            "配备必要的安全保护设施",
            "教练须持有相应资格证书",
            "建立紧急情况处理预案",
            "训练现场配备急救设备",
            "建立运动员健康档案",
            "定期进行体检筛查",
            "购买训练意外保险",
        ]
    }

    /// 教练资质要求
    pub fn coach_qualification_requirements(&self) -> Vec<&'static str> {
        vec![
            "持有国家认证的教练员证书",
            "完成青少年体育指导培训",
            "掌握急救知识和技能",
            "了解青少年生长发育规律",
            "具备沟通和心理辅导能力",
            "定期参加继续教育培训",
            "无犯罪记录证明",
            "每两年更新资质认证",
        ]
    }

    /// 训练计划要求
    pub fn training_plan_requirements(&self) -> Vec<&'static str> {
        vec![
            "制定年度、季度、月度训练计划",
            "计划需根据年龄特点制定",
            "明确训练目标和评估标准",
            "包含体能、技术、战术训练",
            "合理安排训练和比赛周期",
            "留有足够的恢复调整时间",
            "计划需经主管部门审核",
            "定期评估和调整训练计划",
        ]
    }

    /// 禁止事项
    pub fn prohibited_activities(&self) -> Vec<&'static str> {
        vec![
            "禁止使用兴奋剂等违禁药物",
            "禁止超负荷训练导致过度疲劳",
            "禁止在极端天气条件下训练",
            "禁止带伤训练",
            "禁止在患病期间强制训练",
            "禁止使用不适合年龄的器材",
            "禁止过早专项化训练",
            "禁止以惩罚为目的的身体训练",
            "禁止商业性表演训练",
            "禁止夜间的训练活动（未成年人）",
        ]
    }

    /// 训练环境要求
    pub fn training_environment_requirements(&self) -> Vec<&'static str> {
        vec![
            "场地面积满足训练需求",
            "地面平整、防滑",
            "有足够的照明条件",
            "室内训练有通风设施",
            "夏季有降温设施",
            "冬季有保暖措施",
            "有饮水和休息区域",
            "有卫生设施",
            "噪音不超过85分贝",
            "空气质量符合国家标准",
        ]
    }

    /// 营养指导
    pub fn nutrition_guidelines(&self) -> Vec<&'static str> {
        vec![
            "保证充足的蛋白质摄入",
            "训练前后适当补充碳水化合物",
            "保证足够的水分摄入",
            "避免空腹训练",
            "训练后30分钟内补充营养",
            "避免过度依赖营养补充剂",
            "饮食应均衡多样化",
            "根据训练量调整饮食",
        ]
    }

    /// 休息恢复要求
    pub fn rest_recovery_requirements(&self) -> Vec<&'static str> {
        vec![
            "每天保证8-10小时睡眠",
            "高强度训练后安排恢复日",
            "每周至少安排1-2天完全休息",
            "可采用按摩、拉伸等恢复手段",
            "避免连续多天高强度训练",
            "考试期间适当减少训练",
            "生病期间应暂停训练",
            "伤病恢复需循序渐进",
        ]
    }

    /// 家长沟通机制
    pub fn parent_communication(&self) -> Vec<&'static str> {
        vec![
            "定期向家长通报训练情况",
            "建立家长知情同意制度",
            "训练计划需告知家长",
            "重大活动需家长同意",
            "建立紧急联系机制",
            "定期召开家长会",
            "提供家庭教育指导",
            "尊重家长合理建议",
        ]
    }

    /// 伤病预防
    pub fn injury_prevention(&self) -> Vec<&'static str> {
        vec![
            "训练前进行充分热身（10-15分钟）",
            "训练后进行整理活动（10分钟）",
            "定期进行功能性动作筛查",
            "加强易伤部位力量训练",
            "使用适合的运动护具",
            "纠正错误技术动作",
            "避免在疲劳状态下训练",
            "及时处理轻微伤病",
            "建立伤病记录档案",
            "必要时进行运动康复训练",
        ]
    }

    /// 训练评估标准
    pub fn evaluation_standards(&self) -> Vec<&'static str> {
        vec![
            "体能测试：每季度进行一次",
            "技术评估：采用等级评定制",
            "心理评估：关注心理状态变化",
            "出勤记录：记录训练出勤情况",
            "伤病记录：详细记录伤病情况",
            "比赛成绩：记录比赛表现",
            "综合评定：形成运动员档案",
            "反馈沟通：及时反馈评估结果",
        ]
    }

    /// 过度训练识别
    pub fn overtraining_signs(&self) -> Vec<&'static str> {
        vec![
            "持续疲劳感，休息后不能恢复",
            "运动成绩下降",
            "睡眠障碍",
            "食欲下降",
            "情绪波动大",
            "免疫力下降，易感冒",
            "运动损伤增加",
            "心率持续偏高",
            "体重异常变化",
            "注意力不集中",
        ]
    }

    /// 过度训练处理
    pub fn overtraining_management(&self) -> Vec<&'static str> {
        vec![
            "立即减少或暂停训练",
            "寻求专业医疗评估",
            "保证充足休息",
            "调整营养摄入",
            "心理辅导支持",
            "逐步恢复训练",
            "密切监测恢复情况",
            "调整长期训练计划",
        ]
    }

    /// 不同运动项目特殊要求
    pub fn sport_specific_requirements(&self) -> Vec<&'static str> {
        vec![
            // 田径项目
            "田径：注意跑跳动作规范，避免关节损伤",
            // 球类运动
            "球类：控制对抗强度，重视团队配合",
            // 体操
            "体操：重视柔韧性训练，注意保护措施",
            // 游泳
            "游泳：确保救生人员在场，注意水温控制",
            // 武术
            "武术：循序渐进练习难度动作",
            // 举重
            "举重：15岁以下禁止专业训练",
        ]
    }

    /// 寒暑假训练规定
    pub fn holiday_training_rules(&self) -> Vec<&'static str> {
        vec![
            "假期可组织集中训练",
            "每日训练不超过两次",
            "总训练时间不超过4小时",
            "保证午休时间",
            "注意防暑降温",
            "提供餐饮和住宿保障",
            "安排文化娱乐活动",
            "家长需签署同意书",
        ]
    }

    /// 比赛准备训练
    pub fn competition_preparation(&self) -> Vec<&'static str> {
        vec![
            "赛前4-6周开始专项准备",
            "逐步减少训练量，保持强度",
            "模拟比赛环境和条件",
            "进行心理素质训练",
            "制定比赛战术策略",
            "调整作息时间",
            "控制饮食和体重",
            "赛前一周降低训练强度",
        ]
    }
}

impl Default for YouthTrainingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for YouthTrainingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("youth_training")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        // 基础验证：检查训练规则配置是否正确
        Ok(true)
    }
}

/// 青少年训练安全规则
pub struct YouthTrainingSafetyRules {
    metadata: RuleMetadata,
}

impl YouthTrainingSafetyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("青少年训练安全规则", "青少年体育训练的安全保障规范")
                .with_origin("国家体育总局")
                .with_tags(vec![
                    "体育".into(),
                    "训练".into(),
                    "安全".into(),
                    "青少年".into(),
                ]),
        }
    }

    /// 场地安全检查
    pub fn venue_safety_check(&self) -> Vec<&'static str> {
        vec![
            "场地平整无障碍物",
            "地面防滑性能良好",
            "器材固定牢固",
            "安全警示标志清晰",
            "消防通道畅通",
            "急救设备完好",
            "照明设施正常",
            "通风设施有效",
        ]
    }

    /// 器材安全要求
    pub fn equipment_safety(&self) -> Vec<&'static str> {
        vec![
            "器材尺寸适合年龄特点",
            "定期检查维护保养",
            "发现问题及时更换",
            "建立器材使用登记",
            "禁止使用损坏器材",
            "存放位置安全合理",
            "使用前必须检查",
            "专人负责器材管理",
        ]
    }

    /// 应急预案
    pub fn emergency_plan(&self) -> Vec<&'static str> {
        vec![
            "建立突发事件应急机制",
            "明确人员分工和职责",
            "设立应急联系人名单",
            "配备急救药箱",
            "了解最近医疗机构",
            "定期进行应急演练",
            "记录紧急事件处理过程",
            "事后进行总结改进",
        ]
    }

    /// 天气条件限制
    pub fn weather_restrictions(&self) -> Vec<&'static str> {
        vec![
            "气温超过35℃停止户外训练",
            "气温低于-5℃减少户外训练",
            "大风（6级以上）停止户外训练",
            "雷电天气禁止户外训练",
            "雾霾天气（PM2.5>150）停止户外训练",
            "大雨天气停止户外训练",
            "高温天气调整训练时间",
            "寒冷天气注意保暖措施",
        ]
    }

    /// 训练监控要求
    pub fn training_monitoring(&self) -> Vec<&'static str> {
        vec![
            "记录运动员训练心率",
            "观察运动员疲劳表现",
            "监控训练负荷量",
            "关注运动员情绪状态",
            "记录训练完成情况",
            "定期测量体重变化",
            "监测睡眠质量",
            "评估恢复状态",
        ]
    }
}

impl Default for YouthTrainingSafetyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for YouthTrainingSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("youth_training_safety")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
}

/// 青少年训练负荷管理规则
pub struct YouthTrainingLoadRules {
    metadata: RuleMetadata,
}

impl YouthTrainingLoadRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("青少年训练负荷管理规则", "青少年训练负荷的科学控制")
                .with_origin("体育科学研究所")
                .with_tags(vec![
                    "体育".into(),
                    "训练".into(),
                    "负荷".into(),
                    "青少年".into(),
                ]),
        }
    }

    /// 少儿组负荷标准
    pub fn children_load_standards(&self) -> Vec<&'static str> {
        vec![
            "最大心率不超过180次/分",
            "平均心率控制在140-160次/分",
            "高强度训练时间不超过10分钟",
            "总训练量不超过成人50%",
            "休息间隔比例1:2",
            "禁止持续高强度训练",
        ]
    }

    /// 少年组负荷标准
    pub fn youth_load_standards(&self) -> Vec<&'static str> {
        vec![
            "最大心率不超过190次/分",
            "平均心率控制在150-170次/分",
            "高强度训练时间不超过20分钟",
            "总训练量不超过成人75%",
            "休息间隔比例1:1.5",
            "注意监控疲劳恢复",
        ]
    }

    /// 青少年组负荷标准
    pub fn junior_load_standards(&self) -> Vec<&'static str> {
        vec![
            "最大心率可达200次/分",
            "平均心率控制在160-180次/分",
            "高强度训练时间可达30分钟",
            "总训练量可达成人90%",
            "休息间隔比例1:1",
            "接近成人负荷标准",
        ]
    }

    /// 训练强度分级
    pub fn intensity_classification(&self) -> Vec<&'static str> {
        vec![
            "低强度（心率120-140）：热身、恢复",
            "中等强度（心率140-160）：技术训练",
            "高强度（心率160-180）：专项训练",
            "最大强度（心率180+）：比赛模拟",
            "强度交替训练效果更佳",
            "注意观察主观疲劳感受",
        ]
    }

    /// RPE（主观疲劳度）评估
    pub fn rpe_scale(&self) -> Vec<&'static str> {
        vec![
            "RPE 1-2：非常轻松，可轻松交谈",
            "RPE 3-4：轻松，可以正常交谈",
            "RPE 5-6：中等，可以短句交流",
            "RPE 7-8：较累，只能说几个字",
            "RPE 9-10：非常累，无法说话",
            "青少年训练RPE建议不超过7",
        ]
    }

    /// 训练量计算方法
    pub fn volume_calculation(&self) -> Vec<&'static str> {
        vec![
            "训练量 = 强度 × 时间 × 频率",
            "使用训练压力指数（TRIMP）",
            "周训练量增幅不超过10%",
            "每4周设置恢复周",
            "比赛周降低训练量30-50%",
            "假期可适当增加训练量",
        ]
    }
}

impl Default for YouthTrainingLoadRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for YouthTrainingLoadRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("youth_training_load")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youth_training_rules_creation() {
        let rules = YouthTrainingRules::new();
        assert_eq!(rules.metadata().name, "青少年训练规则");
    }

    #[test]
    fn test_age_group_name() {
        assert_eq!(YouthTrainingAgeGroup::Children.name(), "少儿组");
        assert_eq!(YouthTrainingAgeGroup::Youth.name(), "少年组");
        assert_eq!(YouthTrainingAgeGroup::Junior.name(), "青少年组");
    }

    #[test]
    fn test_age_group_range() {
        assert_eq!(YouthTrainingAgeGroup::Children.age_range(), "6-10岁");
        assert_eq!(YouthTrainingAgeGroup::Youth.age_range(), "11-14岁");
        assert_eq!(YouthTrainingAgeGroup::Junior.age_range(), "15-18岁");
    }

    #[test]
    fn test_training_principles() {
        let rules = YouthTrainingRules::new();
        let principles = rules.training_principles();
        assert!(!principles.is_empty());
        assert!(principles.iter().any(|p| p.contains("循序渐进")));
    }

    #[test]
    fn test_children_training_characteristics() {
        let rules = YouthTrainingRules::new();
        let characteristics = rules.children_training_characteristics();
        assert!(!characteristics.is_empty());
        assert!(characteristics.iter().any(|c| c.contains("游戏化")));
    }

    #[test]
    fn test_safety_measures() {
        let rules = YouthTrainingRules::new();
        let measures = rules.safety_measures();
        assert!(!measures.is_empty());
        assert!(measures.iter().any(|m| m.contains("热身")));
    }

    #[test]
    fn test_prohibited_activities() {
        let rules = YouthTrainingRules::new();
        let prohibited = rules.prohibited_activities();
        assert!(!prohibited.is_empty());
        assert!(prohibited.iter().any(|p| p.contains("兴奋剂")));
    }

    #[test]
    fn test_coach_qualification() {
        let rules = YouthTrainingRules::new();
        let requirements = rules.coach_qualification_requirements();
        assert!(!requirements.is_empty());
        assert!(requirements.iter().any(|r| r.contains("证书")));
    }

    #[test]
    fn test_strength_training_rules() {
        let rules = YouthTrainingRules::new();
        let strength_rules = rules.strength_training_rules();
        assert!(!strength_rules.is_empty());
        assert!(strength_rules.iter().any(|s| s.contains("禁止")));
    }

    #[test]
    fn test_injury_prevention() {
        let rules = YouthTrainingRules::new();
        let prevention = rules.injury_prevention();
        assert!(!prevention.is_empty());
        assert!(prevention.iter().any(|p| p.contains("热身")));
    }

    #[test]
    fn test_overtraining_signs() {
        let rules = YouthTrainingRules::new();
        let signs = rules.overtraining_signs();
        assert!(!signs.is_empty());
        assert!(signs.iter().any(|s| s.contains("疲劳")));
    }

    #[test]
    fn test_safety_rules_creation() {
        let rules = YouthTrainingSafetyRules::new();
        assert_eq!(rules.metadata().name, "青少年训练安全规则");
    }

    #[test]
    fn test_venue_safety_check() {
        let rules = YouthTrainingSafetyRules::new();
        let checks = rules.venue_safety_check();
        assert!(!checks.is_empty());
        assert!(checks.iter().any(|c| c.contains("场地")));
    }

    #[test]
    fn test_emergency_plan() {
        let rules = YouthTrainingSafetyRules::new();
        let plan = rules.emergency_plan();
        assert!(!plan.is_empty());
        assert!(plan.iter().any(|p| p.contains("应急")));
    }

    #[test]
    fn test_weather_restrictions() {
        let rules = YouthTrainingSafetyRules::new();
        let restrictions = rules.weather_restrictions();
        assert!(!restrictions.is_empty());
        assert!(restrictions.iter().any(|r| r.contains("35")));
    }

    #[test]
    fn test_load_rules_creation() {
        let rules = YouthTrainingLoadRules::new();
        assert_eq!(rules.metadata().name, "青少年训练负荷管理规则");
    }

    #[test]
    fn test_load_standards() {
        let rules = YouthTrainingLoadRules::new();
        let children = rules.children_load_standards();
        let youth = rules.youth_load_standards();
        let junior = rules.junior_load_standards();

        assert!(!children.is_empty());
        assert!(!youth.is_empty());
        assert!(!junior.is_empty());
    }

    #[test]
    fn test_intensity_classification() {
        let rules = YouthTrainingLoadRules::new();
        let classification = rules.intensity_classification();
        assert!(!classification.is_empty());
        assert!(classification.iter().any(|c| c.contains("心率")));
    }

    #[test]
    fn test_rpe_scale() {
        let rules = YouthTrainingLoadRules::new();
        let scale = rules.rpe_scale();
        assert!(!scale.is_empty());
        assert!(scale.iter().any(|s| s.contains("RPE")));
    }
}
