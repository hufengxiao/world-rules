//! 青少年田径规则
//!
//! 针对不同年龄段青少年的田径运动规则，包括年龄分组、器材规格调整等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 青少年年龄组别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YouthAgeGroup {
    /// 少儿组（7-10岁）
    Children,
    /// 少年组（11-14岁）
    Youth,
    /// 青少年组（15-18岁）
    Junior,
    /// 青年组（19-22岁）
    YoungAdult,
}

impl YouthAgeGroup {
    /// 获取年龄组名称
    pub fn name(&self) -> &'static str {
        match self {
            YouthAgeGroup::Children => "少儿组",
            YouthAgeGroup::Youth => "少年组",
            YouthAgeGroup::Junior => "青少年组",
            YouthAgeGroup::YoungAdult => "青年组",
        }
    }

    /// 获取年龄范围
    pub fn age_range(&self) -> &'static str {
        match self {
            YouthAgeGroup::Children => "7-10岁",
            YouthAgeGroup::Youth => "11-14岁",
            YouthAgeGroup::Junior => "15-18岁",
            YouthAgeGroup::YoungAdult => "19-22岁",
        }
    }
}

/// 青少年田径规则
pub struct YouthAthleticsRules {
    metadata: RuleMetadata,
}

impl YouthAthleticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("青少年田径规则", "青少年田径运动规则和年龄分组")
                .with_origin("WA/中国田协")
                .with_tags(vec![
                    "体育".into(),
                    "田径".into(),
                    "青少年".into(),
                    "校园".into(),
                ]),
        }
    }

    /// 年龄分组标准
    pub fn age_classifications(&self) -> Vec<&'static str> {
        vec![
            "少儿组（Children）: 7-10岁",
            "少年组（Youth）: 11-14岁",
            "青少年组（Junior）: 15-18岁",
            "青年组（U23）: 19-22岁",
            "年龄验证: 需提供出生证明",
            "年龄分组以比赛当年为准",
        ]
    }

    /// 各年龄组跑道规格调整
    pub fn track_adjustments(&self) -> Vec<&'static str> {
        vec![
            "少儿组: 60米、100米",
            "少年组: 100米、200米、400米",
            "青少年组: 标准成人项目",
            "跨栏高度随年龄调整",
            "栏间距随年龄调整",
            "接力棒规格可缩小",
        ]
    }

    /// 少儿组（7-10岁）跑项设置
    pub fn children_events(&self) -> Vec<&'static str> {
        vec![
            "60米跑",
            "100米跑",
            "立定跳远",
            "投掷轻器械（垒球）",
            "4×50米接力",
            "不设跨栏项目",
            "不设长跑项目",
        ]
    }

    /// 少年组（11-14岁）跑项设置
    pub fn youth_events(&self) -> Vec<&'static str> {
        vec![
            "100米跑",
            "200米跑",
            "400米跑",
            "800米跑",
            "跳高、跳远",
            "铅球（3kg/4kg）",
            "垒球掷远",
            "4×100米接力",
        ]
    }

    /// 青少年组（15-18岁）项目设置
    pub fn junior_events(&self) -> Vec<&'static str> {
        vec![
            "100米、200米、400米",
            "800米、1500米",
            "110米栏（男）/100米栏（女）",
            "跳高、跳远、三级跳远",
            "铅球（男5kg/女4kg）",
            "标枪（男600g/女500g）",
            "铁饼（男1.5kg/女1kg）",
            "4×100米、4×400米接力",
        ]
    }

    /// 青年组（19-22岁）项目设置
    pub fn young_adult_events(&self) -> Vec<&'static str> {
        vec![
            "接近成人项目设置",
            "逐渐过渡到标准器材",
            "5000米可参赛",
            "十项/七项全能",
            "马拉松不推荐",
            "重量训练需专业指导",
        ]
    }

    /// 跨栏规格（按年龄组）
    pub fn hurdles_specifications(&self, age_group: YouthAgeGroup) -> Vec<&'static str> {
        match age_group {
            YouthAgeGroup::Children => {
                vec!["不设跨栏项目", "可使用软式障碍物练习"]
            }
            YouthAgeGroup::Youth => {
                vec![
                    "男子100米栏: 栏高0.762米",
                    "女子100米栏: 栏高0.762米",
                    "栏间距: 8.5米",
                    "栏架数量: 10个",
                ]
            }
            YouthAgeGroup::Junior => {
                vec![
                    "男子110米栏: 栏高0.914米",
                    "女子100米栏: 栏高0.838米",
                    "栏间距接近成人标准",
                    "栏架数量: 10个",
                ]
            }
            YouthAgeGroup::YoungAdult => {
                vec![
                    "男子110米栏: 栏高1.067米（成人标准）",
                    "女子100米栏: 栏高0.84米（成人标准）",
                    "栏间距: 成人标准",
                    "栏架数量: 10个",
                ]
            }
        }
    }

    /// 投掷器械重量调整
    pub fn throwing_implements(&self) -> Vec<&'static str> {
        vec![
            "少儿组铅球: 2kg（男/女）",
            "少年组铅球: 3kg（男）、2.5kg（女）",
            "青少年组铅球: 5kg（男）、4kg（女）",
            "青年组铅球: 接近成人重量",
            "标枪: 500g-700g（按年龄递增）",
            "铁饼: 0.75kg-1.5kg（按年龄递增）",
        ]
    }

    /// 安全保护措施
    pub fn safety_measures(&self) -> Vec<&'static str> {
        vec![
            "赛前体检: 必须提供健康证明",
            "热身时间: 至少30分钟",
            "比赛监督: 每组配备安全员",
            "医疗站: 必须配备医护人员",
            "急救设备: AED、担架、急救包",
            "天气监测: 高温/雷电天气暂停",
            "补水站: 长跑项目必须设置",
            "退赛机制: 允许运动员随时退出",
        ]
    }

    /// 参赛资格要求
    pub fn eligibility_requirements(&self) -> Vec<&'static str> {
        vec![
            "年龄证明: 出生证明或身份证",
            "学籍证明: 在校学生证明",
            "健康证明: 近期体检报告",
            "家长同意书: 未满18岁必须提供",
            "保险: 意外伤害保险",
            "注册: 体育协会注册（如需要）",
        ]
    }

    /// 训练建议
    pub fn training_guidelines(&self) -> Vec<&'static str> {
        vec![
            "多样化训练: 避免过早专项化",
            "训练时长: 每周不超过10小时",
            "休息恢复: 保证充足睡眠",
            "营养指导: 科学饮食搭配",
            "心理辅导: 关注心理健康",
            "学业平衡: 学习与训练兼顾",
            "长期规划: 分阶段目标设置",
        ]
    }

    /// 比赛服装要求
    pub fn uniform_requirements(&self) -> Vec<&'static str> {
        vec![
            "运动短裤和背心",
            "比赛号码布: 前后各一块",
            "跑鞋: 鞋钉长度不超过9mm",
            "禁止: 首饰、手表（非计时）",
            "禁止: 可能伤人的配件",
            "队服统一: 团体项目",
        ]
    }

    /// 犯规判定
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "抢跑: 两次抢跑取消资格",
            "跑道违规: 踩线、串道",
            "起跑犯规: 助跑违规",
            "投掷犯规: 越线投掷",
            "接力犯规: 掉棒、提前交接",
            "跳远犯规: 越线起跳",
            "跳高犯规: 横杆掉落、碰落",
            "不当行为: 不尊重裁判、对手",
        ]
    }

    /// 成绩记录规则
    pub fn record_rules(&self) -> Vec<&'static str> {
        vec![
            "手计时: 精确到0.1秒",
            "电子计时: 精确到0.01秒",
            "风速测定: 短跑项目必须",
            "录像回放: 争议判罚依据",
            "成绩公示: 赛后及时公布",
            "申诉流程: 赛后30分钟内",
        ]
    }
}

impl Default for YouthAthleticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for YouthAthleticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("youth_athletics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【青少年田径规则】\n\n\
            年龄分组标准:\n{}\n\n\
            少儿组（7-10岁）项目:\n{}\n\n\
            少年组（11-14岁）项目:\n{}\n\n\
            青少年组（15-18岁）项目:\n{}\n\n\
            投掷器械重量调整:\n{}\n\n\
            安全保护措施:\n{}",
            self.age_classifications()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.children_events()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.youth_events()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.junior_events()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.throwing_implements()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_measures()
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
    fn test_youth_athletics_basic() {
        let rules = YouthAthleticsRules::new();
        assert_eq!(rules.metadata().name, "青少年田径规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_age_groups() {
        let rules = YouthAthleticsRules::new();
        let classifications = rules.age_classifications();
        assert!(classifications.len() >= 4);
        assert!(classifications.iter().any(|c| c.contains("少儿组")));
        assert!(classifications.iter().any(|c| c.contains("少年组")));
        assert!(classifications.iter().any(|c| c.contains("青少年组")));
    }

    #[test]
    fn test_children_events() {
        let rules = YouthAthleticsRules::new();
        let events = rules.children_events();
        assert!(events.iter().any(|e| e.contains("60米")));
        assert!(events.iter().any(|e| e.contains("不设跨栏")));
    }

    #[test]
    fn test_youth_events() {
        let rules = YouthAthleticsRules::new();
        let events = rules.youth_events();
        assert!(events.iter().any(|e| e.contains("铅球")));
        assert!(events.iter().any(|e| e.contains("接力")));
    }

    #[test]
    fn test_junior_events() {
        let rules = YouthAthleticsRules::new();
        let events = rules.junior_events();
        assert!(events.iter().any(|e| e.contains("栏")));
        assert!(events.iter().any(|e| e.contains("三级跳远")));
    }

    #[test]
    fn test_hurdles_specifications() {
        let rules = YouthAthleticsRules::new();

        // 少儿组不设跨栏
        let children_hurdles = rules.hurdles_specifications(YouthAgeGroup::Children);
        assert!(children_hurdles.iter().any(|h| h.contains("不设跨栏")));

        // 少年组跨栏
        let youth_hurdles = rules.hurdles_specifications(YouthAgeGroup::Youth);
        assert!(youth_hurdles.iter().any(|h| h.contains("栏高")));

        // 青少年组跨栏
        let junior_hurdles = rules.hurdles_specifications(YouthAgeGroup::Junior);
        assert!(junior_hurdles.iter().any(|h| h.contains("栏高")));
    }

    #[test]
    fn test_safety_measures() {
        let rules = YouthAthleticsRules::new();
        let safety = rules.safety_measures();
        assert!(safety.iter().any(|s| s.contains("体检")));
        assert!(safety.iter().any(|s| s.contains("医疗")));
        assert!(safety.len() >= 6);
    }

    #[test]
    fn test_throwing_implements() {
        let rules = YouthAthleticsRules::new();
        let implements = rules.throwing_implements();
        assert!(implements.iter().any(|i| i.contains("铅球")));
        assert!(implements.iter().any(|i| i.contains("标枪")));
        assert!(implements.len() >= 5);
    }

    #[test]
    fn test_category() {
        let rules = YouthAthleticsRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_age_group_enum() {
        assert_eq!(YouthAgeGroup::Children.name(), "少儿组");
        assert_eq!(YouthAgeGroup::Youth.age_range(), "11-14岁");
        assert_eq!(YouthAgeGroup::Junior.name(), "青少年组");
    }
}
