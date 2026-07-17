//! 校园篮球规则
//!
//! 针对中小学的校园篮球运动规则，包括年龄分组、场地规格、比赛时间等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 校园篮球年龄组别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampusBasketballAgeGroup {
    /// 小学低年级组（1-3年级）
    PrimaryJunior,
    /// 小学高年级组（4-6年级）
    PrimarySenior,
    /// 初中组
    JuniorHigh,
    /// 高中组
    SeniorHigh,
}

impl CampusBasketballAgeGroup {
    /// 获取年龄组名称
    pub fn name(&self) -> &'static str {
        match self {
            CampusBasketballAgeGroup::PrimaryJunior => "小学低年级组",
            CampusBasketballAgeGroup::PrimarySenior => "小学高年级组",
            CampusBasketballAgeGroup::JuniorHigh => "初中组",
            CampusBasketballAgeGroup::SeniorHigh => "高中组",
        }
    }

    /// 获取年级范围
    pub fn grade_range(&self) -> &'static str {
        match self {
            CampusBasketballAgeGroup::PrimaryJunior => "1-3年级",
            CampusBasketballAgeGroup::PrimarySenior => "4-6年级",
            CampusBasketballAgeGroup::JuniorHigh => "7-9年级",
            CampusBasketballAgeGroup::SeniorHigh => "10-12年级",
        }
    }
}

/// 校园篮球规则
pub struct CampusBasketballRules {
    metadata: RuleMetadata,
}

impl CampusBasketballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("校园篮球规则", "中小学篮球运动规则和年级分组")
                .with_origin("教育部/中国篮协")
                .with_tags(vec![
                    "体育".into(),
                    "篮球".into(),
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
    pub fn field_dimensions(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "长度: 22米",
                    "宽度: 15米",
                    "三分线距离: 无",
                    "罚球线距离: 3.5米",
                    "篮筐高度: 2.60米",
                    "球场标记: 清晰可见",
                    "禁区: 无或简化",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "长度: 24米",
                    "宽度: 17米",
                    "三分线距离: 5.0米",
                    "罚球线距离: 4.0米",
                    "篮筐高度: 2.75米",
                    "中圈半径: 1.2米",
                    "禁区: 4.9米×5.8米",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "长度: 26米",
                    "宽度: 18米",
                    "三分线距离: 6.25米",
                    "罚球线距离: 4.6米",
                    "篮筐高度: 3.05米",
                    "中圈半径: 1.6米",
                    "禁区: 4.9米×5.8米",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "长度: 28米",
                    "宽度: 15米",
                    "三分线距离: 6.75米",
                    "罚球线距离: 4.6米",
                    "篮筐高度: 3.05米",
                    "中圈半径: 1.8米",
                    "禁区: 4.9米×5.8米",
                    "符合FIBA标准",
                ]
            }
        }
    }

    /// 球员人数规则
    pub fn team_composition(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "场上人数: 4人",
                    "替补人数: 无限制",
                    "轮换鼓励: 建议频繁轮换",
                    "上场时间: 平均分配",
                    "报名人数: 8-12人",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "场上人数: 5人",
                    "替补人数: 最多7人",
                    "轮换建议: 每节至少轮换",
                    "报名人数: 10-12人",
                    "每个人都要上场",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "场上人数: 5人",
                    "替补人数: 最多7人",
                    "轮换规则: 常规轮换",
                    "报名人数: 10-12人",
                    "技术犯规替补限制",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "场上人数: 5人",
                    "替补人数: 最多7人",
                    "轮换策略: 教练决定",
                    "报名人数: 12人",
                    "符合标准比赛规定",
                ]
            }
        }
    }

    /// 比赛时间规则
    pub fn match_duration(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "比赛总时长: 20分钟",
                    "节次: 2节×10分钟",
                    "节间休息: 5分钟",
                    "暂停次数: 每队每节1次",
                    "暂停时长: 30秒",
                    "不停表: 比赛不停表",
                    "最后1分钟: 适当停表",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "比赛总时长: 28分钟",
                    "节次: 4节×7分钟",
                    "节间休息: 2分钟",
                    "半场休息: 8分钟",
                    "暂停次数: 每队每半场2次",
                    "暂停时长: 30秒",
                    "最后2分钟停表",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "比赛总时长: 32分钟",
                    "节次: 4节×8分钟",
                    "节间休息: 2分钟",
                    "半场休息: 10分钟",
                    "暂停次数: 每队每半场2次",
                    "暂停时长: 60秒",
                    "最后2分钟停表",
                    "加时赛: 4分钟",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "比赛总时长: 40分钟",
                    "节次: 4节×10分钟",
                    "节间休息: 2分钟",
                    "半场休息: 15分钟",
                    "暂停次数: 每队每半场3次",
                    "暂停时长: 60秒",
                    "最后2分钟停表",
                    "加时赛: 5分钟",
                    "符合FIBA标准",
                ]
            }
        }
    }

    /// 得分规则
    pub fn scoring_rules(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "罚球得分: 1分",
                    "两分球: 禁区内投篮",
                    "三分线: 不设三分线",
                    "技术犯规罚球: 1次",
                    "得分记录: 简化记录",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "罚球得分: 1分",
                    "两分球: 禁区内投篮",
                    "三分球: 2分（简化）",
                    "技术犯规罚球: 1次",
                    "得分记录: 正式记录",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "罚球得分: 1分",
                    "两分球: 两分区内投篮",
                    "三分球: 三分线外投篮",
                    "技术犯规罚球: 2次",
                    "得分记录: 正式记录",
                    "符合标准规则",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "罚球得分: 1分",
                    "两分球: 两分区内投篮",
                    "三分球: 三分线外投篮",
                    "技术犯规罚球: 2次",
                    "得分记录: 正式记录",
                    "完全符合FIBA规则",
                ]
            }
        }
    }

    /// 犯规规则
    pub fn foul_rules(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "个人犯规上限: 无限制",
                    "罚出场规则: 不适用",
                    "技术犯规: 教育为主",
                    "犯规罚球: 累计6次后",
                    "犯规记录: 不严格记录",
                    "教育性犯规说明",
                    "教练及时纠正",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "个人犯规上限: 5次",
                    "罚出场规则: 第5次犯规罚出",
                    "技术犯规: 记录并警告",
                    "全队犯规上限: 每节4次",
                    "罚球规则: 第5次起罚球",
                    "犯规记录: 正式记录",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "个人犯规上限: 5次",
                    "罚出场规则: 第5次犯规罚出",
                    "技术犯规: 正式处罚",
                    "全队犯规上限: 每节4次",
                    "罚球规则: 第5次起罚球",
                    "违反体育道德犯规: 2次罚出",
                    "犯规记录: 正式记录",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "个人犯规上限: 5次",
                    "罚出场规则: 第5次犯规罚出",
                    "技术犯规: 正式处罚",
                    "全队犯规上限: 每节4次",
                    "罚球规则: 第5次起罚球",
                    "违反体育道德犯规: 2次罚出",
                    "取消比赛资格犯规: 立即罚出",
                    "符合FIBA标准",
                ]
            }
        }
    }

    /// 进攻时限规则
    pub fn shot_clock_rules(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "进攻时限: 无限制",
                    "简化规则: 不设24秒",
                    "鼓励进攻: 积极投篮",
                    "教育目的: 培养意识",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "进攻时限: 建议性指导",
                    "建议时间: 30秒内投篮",
                    "简化规则: 可不强制执行",
                    "培养快攻意识",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "进攻时限: 30秒",
                    "24秒规则: 开始引入",
                    "教育目的: 培养快攻",
                    "记录简单: 教练提醒",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "进攻时限: 24秒",
                    "回表规则: 抢篮板14秒",
                    "符合FIBA标准",
                    "正式记录和执行",
                ]
            }
        }
    }

    /// 换人规则
    pub fn substitution_rules(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "换人次数: 不限制",
                    "换人时机: 任何死球时",
                    "轮换鼓励: 频繁轮换",
                    "上场时间: 每人至少一半时间",
                    "教育目的: 参与为主",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "换人次数: 不限制",
                    "换人时机: 死球或暂停时",
                    "轮换要求: 每人至少上场一节",
                    "平衡上场时间",
                    "记录: 建议记录",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "换人次数: 不限制",
                    "换人时机: 死球时向记录台申请",
                    "换人流程: 正式流程",
                    "换人时间: 30秒内完成",
                    "记录: 正式记录",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "换人次数: 不限制",
                    "换人时机: 死球时向记录台申请",
                    "换人流程: 符合FIBA标准",
                    "换人时间: 尽快完成",
                    "记录: 正式记录",
                ]
            }
        }
    }

    /// 篮筐和球规格
    pub fn basket_and_ball_specs(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "篮筐高度: 2.60米",
                    "篮球大小: 4号或5号",
                    "篮球周长: 69-71厘米",
                    "篮球重量: 450-500克",
                    "篮筐直径: 45厘米",
                    "篮网: 软质篮网",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "篮筐高度: 2.75米",
                    "篮球大小: 5号或6号",
                    "篮球周长: 70-72厘米",
                    "篮球重量: 500-550克",
                    "篮筐直径: 45厘米",
                    "篮网: 标准篮网",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "篮筐高度: 3.05米",
                    "篮球大小: 6号或7号",
                    "篮球周长: 74-78厘米",
                    "篮球重量: 560-650克",
                    "篮筐直径: 45厘米",
                    "篮网: 标准篮网",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "篮筐高度: 3.05米",
                    "篮球大小: 7号（标准）",
                    "篮球周长: 75-78厘米",
                    "篮球重量: 600-650克",
                    "篮筐直径: 45厘米",
                    "篮网: 标准篮网",
                    "符合FIBA标准",
                ]
            }
        }
    }

    /// 安全与保护规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛场地: 平整无障碍",
            "球员装备: 运动服装",
            "禁止佩戴: 眼镜（运动眼镜除外）",
            "禁止佩戴: 首饰、手表",
            "鞋类要求: 运动鞋",
            "医疗准备: 急救箱",
            "裁判配备: 专业裁判",
            "比赛监督: 教师/教练在场",
            "饮水设施: 充足饮水",
            "天气要求: 室内或适宜天气",
            "热身要求: 赛前热身",
            "拉伸要求: 赛后拉伸",
            "犯规保护: 严惩危险犯规",
            "冲突处理: 立即隔离",
        ]
    }

    /// 教育与发展目标
    pub fn educational_goals(&self) -> Vec<&'static str> {
        vec![
            "培养兴趣: 快乐篮球",
            "基础技能: 运球、传球、投篮",
            "团队协作: 配合意识",
            "规则意识: 遵守规则",
            "体育精神: 尊重对手",
            "健康第一: 避免过度竞争",
            "全面发展: 身心健康",
            "公平竞争: 机会均等",
            "技能提升: 循序渐进",
            "比赛经验: 实战锻炼",
        ]
    }

    /// 裁判规则
    pub fn referee_rules(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "裁判人数: 1-2名",
                    "裁判级别: 体育教师或入门级",
                    "判罚尺度: 教育为主",
                    "犯规说明: 耐心解释",
                    "暂停使用: 允许教练指导",
                    "简化手势: 基本手势",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "裁判人数: 2名",
                    "裁判级别: 校级或区级",
                    "判罚尺度: 规范判罚",
                    "手势标准: 标准手势",
                    "记录台: 简化记录",
                    "计时员: 教师兼任",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "裁判人数: 2名",
                    "裁判级别: 区级或市级",
                    "判罚尺度: 正式判罚",
                    "手势标准: FIBA标准",
                    "记录台: 正式记录",
                    "计时员: 专人负责",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "裁判人数: 2-3名",
                    "裁判级别: 市级或国家级",
                    "判罚尺度: 完全符合FIBA",
                    "手势标准: FIBA标准",
                    "记录台: 完整配置",
                    "计时员: 专人负责",
                    "技术代表: 重要比赛",
                ]
            }
        }
    }

    /// 特殊规则调整
    pub fn special_adjustments(&self, age_group: CampusBasketballAgeGroup) -> Vec<&'static str> {
        match age_group {
            CampusBasketballAgeGroup::PrimaryJunior => {
                vec![
                    "不走步放宽: 允许轻微走步",
                    "双带放宽: 児童可重来",
                    "防守限制: 仅限半场防守",
                    "联防限制: 不允许联防",
                    "快攻鼓励: 允许快攻",
                    "暂停延长: 需要时延长",
                    "比分淡化: 不强调比分",
                ]
            }
            CampusBasketballAgeGroup::PrimarySenior => {
                vec![
                    "规则简化: 基本规则执行",
                    "防守限制: 可全场防守",
                    "联防引入: 可简单联防",
                    "暂停使用: 教练可叫暂停",
                    "记录简化: 基本记录",
                    "比分为辅: 强调参与",
                ]
            }
            CampusBasketballAgeGroup::JuniorHigh => {
                vec![
                    "规则正式: 标准规则执行",
                    "防守自由: 允许各种防守",
                    "联防自由: 允许联防",
                    "暂停规则: 符合规则",
                    "记录完整: 正式记录",
                    "比赛正规: 正规比赛",
                ]
            }
            CampusBasketballAgeGroup::SeniorHigh => {
                vec![
                    "规则完全: FIBA标准",
                    "防守自由: 完全自由",
                    "战术自由: 允许复杂战术",
                    "暂停规则: FIBA标准",
                    "记录完整: 完整记录",
                    "比赛正规: 完全正规",
                ]
            }
        }
    }
}

impl Default for CampusBasketballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CampusBasketballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("campus_basketball")
    }

    fn explain(&self) -> String {
        "【校园篮球规则】\n\n\
            青少年篮球运动的年龄分组与规则体系\n\n\
            年龄分组:\n\
            - 小学低年级组 (1-3年级)\n\
            - 小学高年级组 (4-6年级)\n\
            - 初中组 (7-9年级)\n\
            - 高中组 (10-12年级)\n\n\
            核心原则:\n\
            1. 年龄适宜性: 规则随年龄调整\n\
            2. 安全第一: 避免危险动作\n\
            3. 教育导向: 培养篮球兴趣\n\
            4. 公平参与: 每人都要上场\n\
            5. 技能发展: 循序渐进提升\n\
            6. 团队精神: 培养协作意识"
            .to_string()
    }

    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
        match ctx {
            ValidateContext::Generic(context) if context == "campus_basketball" => Ok(true),
            ValidateContext::Generic(context) if context == "basketball" => Ok(true),
            ValidateContext::Generic(context) if context == "sports" => Ok(true),
            _ => Ok(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campus_basketball_rules_creation() {
        let rules = CampusBasketballRules::new();
        assert!(!rules.age_classifications().is_empty());
        assert_eq!(rules.age_classifications().len(), 6);
    }

    #[test]
    fn test_age_group_names() {
        assert_eq!(
            CampusBasketballAgeGroup::PrimaryJunior.name(),
            "小学低年级组"
        );
        assert_eq!(
            CampusBasketballAgeGroup::PrimarySenior.name(),
            "小学高年级组"
        );
        assert_eq!(CampusBasketballAgeGroup::JuniorHigh.name(), "初中组");
        assert_eq!(CampusBasketballAgeGroup::SeniorHigh.name(), "高中组");
    }

    #[test]
    fn test_grade_ranges() {
        assert_eq!(
            CampusBasketballAgeGroup::PrimaryJunior.grade_range(),
            "1-3年级"
        );
        assert_eq!(
            CampusBasketballAgeGroup::PrimarySenior.grade_range(),
            "4-6年级"
        );
        assert_eq!(
            CampusBasketballAgeGroup::JuniorHigh.grade_range(),
            "7-9年级"
        );
        assert_eq!(
            CampusBasketballAgeGroup::SeniorHigh.grade_range(),
            "10-12年级"
        );
    }

    #[test]
    fn test_field_dimensions() {
        let rules = CampusBasketballRules::new();

        // 小学低年级组球场规格
        let primary_junior = rules.field_dimensions(CampusBasketballAgeGroup::PrimaryJunior);
        assert!(!primary_junior.is_empty());
        assert!(primary_junior.iter().any(|s| s.contains("篮筐高度")));

        // 高中组球场规格
        let senior_high = rules.field_dimensions(CampusBasketballAgeGroup::SeniorHigh);
        assert!(!senior_high.is_empty());
        assert!(senior_high.iter().any(|s| s.contains("FIBA标准")));
    }

    #[test]
    fn test_team_composition() {
        let rules = CampusBasketballRules::new();

        // 小学低年级组球员人数
        let primary_junior = rules.team_composition(CampusBasketballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|s| s.contains("4人")));

        // 高中组球员人数
        let senior_high = rules.team_composition(CampusBasketballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|s| s.contains("5人")));
    }

    #[test]
    fn test_match_duration() {
        let rules = CampusBasketballRules::new();

        // 小学低年级组比赛时间
        let primary_junior = rules.match_duration(CampusBasketballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|s| s.contains("20分钟")));

        // 高中组比赛时间
        let senior_high = rules.match_duration(CampusBasketballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|s| s.contains("40分钟")));
    }

    #[test]
    fn test_basket_height() {
        let rules = CampusBasketballRules::new();

        // 小学低年级篮筐高度
        let primary_junior = rules.basket_and_ball_specs(CampusBasketballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|s| s.contains("2.60米")));

        // 高中组篮筐高度
        let senior_high = rules.basket_and_ball_specs(CampusBasketballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|s| s.contains("3.05米")));
    }

    #[test]
    fn test_foul_rules() {
        let rules = CampusBasketballRules::new();

        // 小学低年级犯规规则
        let primary_junior = rules.foul_rules(CampusBasketballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|s| s.contains("无限制")));

        // 高中组犯规规则
        let senior_high = rules.foul_rules(CampusBasketballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|s| s.contains("5次")));
    }

    #[test]
    fn test_safety_rules() {
        let rules = CampusBasketballRules::new();
        let safety = rules.safety_rules();

        assert!(!safety.is_empty());
        assert!(safety.iter().any(|s| s.contains("急救箱")));
        assert!(safety.iter().any(|s| s.contains("运动鞋")));
    }

    #[test]
    fn test_educational_goals() {
        let rules = CampusBasketballRules::new();
        let goals = rules.educational_goals();

        assert!(!goals.is_empty());
        assert!(goals.iter().any(|s| s.contains("快乐篮球")));
        assert!(goals.iter().any(|s| s.contains("团队协作")));
    }

    #[test]
    fn test_rule_trait() {
        let rules = CampusBasketballRules::new();

        assert!(rules
            .validate(&ValidateContext::Generic("campus_basketball".to_string()))
            .is_ok());
        assert!(rules
            .validate(&ValidateContext::Generic("basketball".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_referee_rules() {
        let rules = CampusBasketballRules::new();

        // 小学低年级裁判规则
        let primary_junior = rules.referee_rules(CampusBasketballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|s| s.contains("教育为主")));

        // 高中组裁判规则
        let senior_high = rules.referee_rules(CampusBasketballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|s| s.contains("FIBA标准")));
    }

    #[test]
    fn test_shot_clock_rules() {
        let rules = CampusBasketballRules::new();

        // 小学低年级进攻时限
        let primary_junior = rules.shot_clock_rules(CampusBasketballAgeGroup::PrimaryJunior);
        assert!(primary_junior.iter().any(|s| s.contains("无限制")));

        // 高中组进攻时限
        let senior_high = rules.shot_clock_rules(CampusBasketballAgeGroup::SeniorHigh);
        assert!(senior_high.iter().any(|s| s.contains("24秒")));
    }
}
