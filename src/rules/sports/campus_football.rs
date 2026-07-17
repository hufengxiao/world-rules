//! 校园足球规则
//!
//! 针对中小学的校园足球运动规则，包括年龄分组、场地规格、比赛时间等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 校园足球年龄组别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampusFootballAgeGroup {
    /// 小学低年级组（1-3年级）
    PrimaryJunior,
    /// 小学高年级组（4-6年级）
    PrimarySenior,
    /// 初中组
    JuniorHigh,
    /// 高中组
    SeniorHigh,
}

impl CampusFootballAgeGroup {
    /// 获取年龄组名称
    pub fn name(&self) -> &'static str {
        match self {
            CampusFootballAgeGroup::PrimaryJunior => "小学低年级组",
            CampusFootballAgeGroup::PrimarySenior => "小学高年级组",
            CampusFootballAgeGroup::JuniorHigh => "初中组",
            CampusFootballAgeGroup::SeniorHigh => "高中组",
        }
    }

    /// 获取年级范围
    pub fn grade_range(&self) -> &'static str {
        match self {
            CampusFootballAgeGroup::PrimaryJunior => "1-3年级",
            CampusFootballAgeGroup::PrimarySenior => "4-6年级",
            CampusFootballAgeGroup::JuniorHigh => "7-9年级",
            CampusFootballAgeGroup::SeniorHigh => "10-12年级",
        }
    }
}

/// 校园足球规则
pub struct CampusFootballRules {
    metadata: RuleMetadata,
}

impl CampusFootballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("校园足球规则", "中小学足球运动规则和年级分组")
                .with_origin("教育部/中国足协")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "校园".into(),
                    "青少年".into(),
                ]),
        }
    }

    /// 年龄分组标准
    pub fn age_classifications(&self) -> Vec<&'static str> {
        vec![
            "小学低年级组: 1-3年级",
            "小学高年级组: 4-6年级",
            "初中组: 7-9年级",
            "高中组: 10-12年级",
            "按学籍分组",
            "跨年级需特殊审批",
        ]
    }

    /// 球场规格（按年龄组）
    pub fn field_dimensions(&self, age_group: CampusFootballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusFootballAgeGroup::PrimaryJunior => {
                vec![
                    "长度: 30-40米",
                    "宽度: 20-30米",
                    "禁区: 8米×4米",
                    "点球点: 6米",
                    "球门: 1.8米×1.2米",
                    "场地标记: 清晰可见",
                ]
            }
            CampusFootballAgeGroup::PrimarySenior => {
                vec![
                    "长度: 50-60米",
                    "宽度: 30-40米",
                    "禁区: 12米×5米",
                    "点球点: 9米",
                    "球门: 2.0米×1.5米",
                    "中圈半径: 6米",
                ]
            }
            CampusFootballAgeGroup::JuniorHigh => {
                vec![
                    "长度: 60-70米",
                    "宽度: 40-50米",
                    "禁区: 14米×5米",
                    "点球点: 10米",
                    "球门: 2.2米×1.7米",
                    "中圈半径: 7米",
                ]
            }
            CampusFootballAgeGroup::SeniorHigh => {
                vec![
                    "长度: 70-80米",
                    "宽度: 50-60米",
                    "禁区: 16米×6米",
                    "点球点: 11米",
                    "球门: 2.4米×2.0米（接近标准）",
                    "中圈半径: 9.15米",
                ]
            }
        }
    }

    /// 比赛时间（按年龄组）
    pub fn match_duration(&self, age_group: CampusFootballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusFootballAgeGroup::PrimaryJunior => {
                vec![
                    "比赛时间: 2×20分钟",
                    "中场休息: 10分钟",
                    "可缩短至15分钟/半场",
                    "允许多次暂停",
                    "补水时间: 每半场1次",
                ]
            }
            CampusFootballAgeGroup::PrimarySenior => {
                vec![
                    "比赛时间: 2×25分钟",
                    "中场休息: 10分钟",
                    "补水时间: 每半场1次",
                    "允许医疗暂停",
                ]
            }
            CampusFootballAgeGroup::JuniorHigh => {
                vec![
                    "比赛时间: 2×30分钟",
                    "中场休息: 15分钟",
                    "补水时间: 每半场1次",
                    "接近标准比赛时间",
                ]
            }
            CampusFootballAgeGroup::SeniorHigh => {
                vec![
                    "比赛时间: 2×35-45分钟",
                    "中场休息: 15分钟",
                    "接近标准比赛时间",
                    "淘汰赛可加时",
                ]
            }
        }
    }

    /// 球员人数（按年龄组）
    pub fn player_count(&self, age_group: CampusFootballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusFootballAgeGroup::PrimaryJunior => {
                vec![
                    "场上队员: 5人制",
                    "替补人数: 不限",
                    "轮换规则: 自由换人",
                    "全员参与原则",
                    "守门员: 可轮换",
                ]
            }
            CampusFootballAgeGroup::PrimarySenior => {
                vec![
                    "场上队员: 7人制",
                    "替补人数: 不限",
                    "轮换规则: 自由换人",
                    "鼓励全员参与",
                ]
            }
            CampusFootballAgeGroup::JuniorHigh => {
                vec![
                    "场上队员: 8-9人制",
                    "替补人数: 最多7人",
                    "换人次数: 最多5次",
                    "被换下可再上场",
                ]
            }
            CampusFootballAgeGroup::SeniorHigh => {
                vec![
                    "场上队员: 11人制",
                    "替补人数: 最多7人",
                    "换人次数: 最多5次",
                    "被换下可再上场",
                    "接近标准规则",
                ]
            }
        }
    }

    /// 越位规则调整（按年龄组）
    pub fn offside_rules(&self, age_group: CampusFootballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusFootballAgeGroup::PrimaryJunior => {
                vec![
                    "不设越位规则",
                    "鼓励进攻参与",
                    "注重技术培养",
                ]
            }
            CampusFootballAgeGroup::PrimarySenior => {
                vec![
                    "简化越位规则",
                    "仅禁区线适用",
                    "教练可场边指导",
                    "逐步引入标准规则",
                ]
            }
            CampusFootballAgeGroup::JuniorHigh | CampusFootballAgeGroup::SeniorHigh => {
                vec![
                    "执行标准越位规则",
                    "边裁判定",
                    "教学比赛可放宽",
                ]
            }
        }
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "危险动作: 铲球、冲撞",
            "手球: 故意手球犯规",
            "拉拽、推搡",
            "语言不当: 警告/罚下",
            "守门员犯规: 间接任意球",
            "累计犯规: 记录并警告",
            "恶劣犯规: 直接红牌",
            "尊重裁判判罚",
        ]
    }

    /// 安全保护措施
    pub fn safety_measures(&self) -> Vec<&'static str> {
        vec![
            "赛前体检: 必须提供健康证明",
            "护具要求: 护腿板必须佩戴",
            "医疗站: 必须配备医护人员",
            "急救设备: AED、担架、急救包",
            "天气监测: 高温/雷电暂停",
            "场地检查: 无障碍物",
            "安全距离: 场边至少2米",
            "保险: 意外伤害保险",
        ]
    }

    /// 参赛资格要求
    pub fn eligibility_requirements(&self) -> Vec<&'static str> {
        vec![
            "学籍证明: 在校学生证明",
            "年龄限制: 符合年龄组要求",
            "健康证明: 近期体检报告",
            "家长同意书: 未满18岁必须",
            "保险: 意外伤害保险",
            "注册: 校园足球注册（如需要）",
            "学业要求: 成绩合格",
            "纪律要求: 无严重违纪记录",
        ]
    }

    /// 教练和裁判要求
    pub fn staff_requirements(&self) -> Vec<&'static str> {
        vec![
            "主教练: 具备足球教练资格",
            "助理教练: 协助训练和比赛",
            "裁判: 校园足球裁判证书",
            "边裁: 至少2名",
            "第四官员: 记录比赛",
            "医疗人员: 具备急救资格",
        ]
    }

    /// 训练要求
    pub fn training_guidelines(&self) -> Vec<&'static str> {
        vec![
            "每周训练: 不少于2次",
            "训练时长: 每次60-90分钟",
            "技术训练: 基本功优先",
            "战术训练: 简单战术",
            "体能训练: 适度原则",
            "趣味性: 保持兴趣",
            "安全第一: 避免伤病",
            "学业平衡: 学习优先",
        ]
    }

    /// 装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "球衣: 统一队服",
            "球鞋: 胶钉或碎钉鞋",
            "护腿板: 必须佩戴",
            "守门员手套: 允许",
            "禁止: 钢钉鞋、护具硬质部分",
            "禁止: 首饰、手表（非计时）",
            "队服编号: 清晰可见",
        ]
    }

    /// 比赛组织
    pub fn match_organization(&self) -> Vec<&'static str> {
        vec![
            "班级联赛: 校内比赛",
            "校际联赛: 区域比赛",
            "市级联赛: 城市比赛",
            "省级比赛: 省级决赛",
            "全国比赛: 校园足球总决赛",
            "主客场制: 优先安排",
            "赛程安排: 不影响学业",
        ]
    }

    /// 比赛纪律
    pub fn match_discipline(&self) -> Vec<&'static str> {
        vec![
            "尊重裁判: 服从判罚",
            "尊重对手: 友谊第一",
            "禁止假摔: 体育道德",
            "禁止暴力: 严厉处罚",
            "观众行为: 文明观赛",
            "教练行为: 禁止过激行为",
            "家长行为: 理性支持",
        ]
    }
}

impl Default for CampusFootballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CampusFootballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("campus_football")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【校园足球规则】\n\n\
            年龄分组标准:\n{}\n\n\
            比赛时间（示例-高中组）:\n{}\n\n\
            球员人数（示例-高中组）:\n{}\n\n\
            安全保护措施:\n{}\n\n\
            参赛资格要求:\n{}\n\n\
            训练要求:\n{}",
            self.age_classifications()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.match_duration(CampusFootballAgeGroup::SeniorHigh)
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.player_count(CampusFootballAgeGroup::SeniorHigh)
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_measures()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.eligibility_requirements()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.training_guidelines()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campus_football_basic() {
        let rules = CampusFootballRules::new();
        assert_eq!(rules.metadata().name, "校园足球规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_age_groups() {
        let rules = CampusFootballRules::new();
        let classifications = rules.age_classifications();
        assert!(classifications.len() >= 4);
        assert!(classifications.iter().any(|c| c.contains("小学")));
        assert!(classifications.iter().any(|c| c.contains("初中")));
        assert!(classifications.iter().any(|c| c.contains("高中")));
    }

    #[test]
    fn test_field_dimensions() {
        let rules = CampusFootballRules::new();
        
        // 小学低年级组场地
        let primary_junior = rules.field_dimensions(CampusFootballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|f| f.contains("30-40米")));
        assert!(primary_junior.iter().any(|f| f.contains("球门")));
        
        // 高中组场地
        let senior_high = rules.field_dimensions(CampusFootballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|f| f.contains("70-80米")));
    }

    #[test]
    fn test_match_duration() {
        let rules = CampusFootballRules::new();
        
        // 小学低年级组比赛时间
        let primary_junior = rules.match_duration(CampusFootballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|m| m.contains("20分钟")));
        
        // 高中组比赛时间
        let senior_high = rules.match_duration(CampusFootballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|m| m.contains("35-45分钟")));
    }

    #[test]
    fn test_player_count() {
        let rules = CampusFootballRules::new();
        
        // 小学低年级组5人制
        let primary_junior = rules.player_count(CampusFootballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|p| p.contains("5人制")));
        
        // 高中组11人制
        let senior_high = rules.player_count(CampusFootballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|p| p.contains("11人制")));
    }

    #[test]
    fn test_offside_rules() {
        let rules = CampusFootballRules::new();
        
        // 小学低年级组不设越位
        let primary_junior = rules.offside_rules(CampusFootballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|o| o.contains("不设越位")));
        
        // 高中组标准越位
        let senior_high = rules.offside_rules(CampusFootballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|o| o.contains("标准越位")));
    }

    #[test]
    fn test_safety_measures() {
        let rules = CampusFootballRules::new();
        let safety = rules.safety_measures();
        assert!(safety.iter().any(|s| s.contains("护腿板")));
        assert!(safety.iter().any(|s| s.contains("医疗")));
        assert!(safety.len() >= 6);
    }

    #[test]
    fn test_eligibility_requirements() {
        let rules = CampusFootballRules::new();
        let eligibility = rules.eligibility_requirements();
        assert!(eligibility.iter().any(|e| e.contains("学籍")));
        assert!(eligibility.iter().any(|e| e.contains("健康证明")));
        assert!(eligibility.len() >= 6);
    }

    #[test]
    fn test_category() {
        let rules = CampusFootballRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_age_group_enum() {
        assert_eq!(CampusFootballAgeGroup::PrimaryJunior.name(), "小学低年级组");
        assert_eq!(CampusFootballAgeGroup::PrimarySenior.grade_range(), "4-6年级");
        assert_eq!(CampusFootballAgeGroup::JuniorHigh.name(), "初中组");
    }

    #[test]
    fn test_equipment_requirements() {
        let rules = CampusFootballRules::new();
        let equipment = rules.equipment_requirements();
        assert!(equipment.iter().any(|e| e.contains("护腿板")));
        assert!(equipment.iter().any(|e| e.contains("球衣")));
    }

    #[test]
    fn test_match_organization() {
        let rules = CampusFootballRules::new();
        let organization = rules.match_organization();
        assert!(organization.iter().any(|o| o.contains("班级联赛")));
        assert!(organization.iter().any(|o| o.contains("校际联赛")));
    }
}