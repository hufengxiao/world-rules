//! 体操项目详细规则
//!
//! 本模块提供体操运动的详细规则实现，涵盖竞技体操、艺术体操、蹦床等。
//! 符合国际体操联合会 (FIG) 标准。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 体操项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GymnasticsDiscipline {
    /// 竞技体操
    ArtisticGymnastics,
    /// 艺术体操
    RhythmicGymnastics,
    /// 蹦床
    Trampoline,
    /// 技巧体操
    AcrobaticGymnastics,
    /// 健美操
    AerobicGymnastics,
}

impl GymnasticsDiscipline {
    /// 获取项目名称
    pub fn name(&self) -> &'static str {
        match self {
            GymnasticsDiscipline::ArtisticGymnastics => "竞技体操",
            GymnasticsDiscipline::RhythmicGymnastics => "艺术体操",
            GymnasticsDiscipline::Trampoline => "蹦床",
            GymnasticsDiscipline::AcrobaticGymnastics => "技巧体操",
            GymnasticsDiscipline::AerobicGymnastics => "健美操",
        }
    }

    /// 是否为奥运会项目
    pub fn is_olympic(&self) -> bool {
        matches!(
            self,
            GymnasticsDiscipline::ArtisticGymnastics
                | GymnasticsDiscipline::RhythmicGymnastics
                | GymnasticsDiscipline::Trampoline
        )
    }
}

/// 竞技体操器械
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Apparatus {
    // 男子器械
    /// 自由体操
    FloorExercise,
    /// 鞍马
    PommelHorse,
    /// 吊环
    Rings,
    /// 跳马
    Vault,
    /// 双杠
    ParallelBars,
    /// 单杠
    HorizontalBar,
    // 女子器械
    /// 平衡木
    BalanceBeam,
    /// 高低杠
    UnevenBars,
}

impl Apparatus {
    /// 获取器械名称
    pub fn name(&self) -> &'static str {
        match self {
            Apparatus::FloorExercise => "自由体操",
            Apparatus::PommelHorse => "鞍马",
            Apparatus::Rings => "吊环",
            Apparatus::Vault => "跳马",
            Apparatus::ParallelBars => "双杠",
            Apparatus::HorizontalBar => "单杠",
            Apparatus::BalanceBeam => "平衡木",
            Apparatus::UnevenBars => "高低杠",
        }
    }

    /// 是否为男子器械
    pub fn is_men_apparatus(&self) -> bool {
        matches!(
            self,
            Apparatus::FloorExercise
                | Apparatus::PommelHorse
                | Apparatus::Rings
                | Apparatus::Vault
                | Apparatus::ParallelBars
                | Apparatus::HorizontalBar
        )
    }

    /// 是否为女子器械
    pub fn is_women_apparatus(&self) -> bool {
        matches!(
            self,
            Apparatus::FloorExercise
                | Apparatus::Vault
                | Apparatus::BalanceBeam
                | Apparatus::UnevenBars
        )
    }
}

/// 体操详细规则
pub struct GymnasticsDetailedRules {
    metadata: RuleMetadata,
}

impl GymnasticsDetailedRules {
    /// 创建新的体操详细规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("体操详细规则", "国际体操联合会标准体操比赛规则")
                .with_origin("FIG")
                .with_tags(vec!["体育".into(), "体操".into()]),
        }
    }

    // ==================== 竞技体操规则 ====================

    /// 竞技体操男子项目
    pub fn mens_apparatus(&self) -> Vec<&'static str> {
        vec![
            "自由体操 (FX)",
            "鞍马 (PH)",
            "吊环 (SR)",
            "跳马 (VT)",
            "双杠 (PB)",
            "单杠 (HB)",
        ]
    }

    /// 竞技体操女子项目
    pub fn womens_apparatus(&self) -> Vec<&'static str> {
        vec![
            "跳马 (VT)",
            "高低杠 (UB)",
            "平衡木 (BB)",
            "自由体操 (FX)",
        ]
    }

    /// 体操评分规则（2022-2024周期）
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "总分 = 难度分 (D) + 完成分 (E)",
            "难度分: 从0分起评，叠加难度值",
            "完成分: 从10分起评，扣减失误",
            "难度裁判: 2人，取平均分",
            "完成裁判: 6人，去掉最高最低取平均",
            "完成扣分: 0.1分（小错）、0.3分（中错）、0.5分（大错）",
            "跌落: 扣1.0分",
            "最低分: 难度分最低为0分",
        ]
    }

    /// 难度分级规则
    pub fn difficulty_requirements(&self) -> Vec<&'static str> {
        vec![
            "难度值: A(0.1)到J(1.0)共10级",
            "成套要求: 每项需满足组别要求",
            "男子自由操: 需包含纵轴、横轴动作",
            "鞍马: 需包含单环、双环动作",
            "吊环: 需包含力量、摆动动作",
            "跳马: 根据动作难度值评分",
            "双杠: 需包含摆动、静止动作",
            "单杠: 需包含飞行动作",
        ]
    }

    /// 完成分评分细则
    pub fn execution_scoring(&self) -> Vec<&'static str> {
        vec![
            "姿态扣分: 分腿、屈膝、勾脚",
            "高度扣分: 动作高度不足",
            "幅度扣分: 动作幅度不够",
            "节奏扣分: 动作节奏不稳",
            "落地扣分: 落地不稳、出界",
            "静止扣分: 静止动作时间不足",
            "技术扣分: 技术动作不规范",
            "艺术扣分: 艺术表现不足",
        ]
    }

    /// 自由体操规则
    pub fn floor_exercise_rules(&self) -> Vec<&'static str> {
        vec![
            "场地: 12米×12米",
            "时间限制: 男子50-70秒，女子70-90秒",
            "音乐: 女子需配音乐，男子无音乐",
            "边界: 越线扣0.5分",
            "最低要求: 满足组别要求",
            "男子要求: 纵轴转体、横轴空翻",
            "女子要求: 跳跃、转身、平衡、柔韧",
            "难度上限: 男子不超过6个技巧串",
        ]
    }

    /// 鞍马规则
    pub fn pommel_horse_rules(&self) -> Vec<&'static str> {
        vec![
            "器械高度: 1.05米",
            "马身长度: 1.60米",
            "马身宽度: 0.35米",
            "鞍环高度: 0.12米",
            "鞍环间距: 0.40-0.45米",
            "成套要求: 单环、双环、全旋",
            "静止动作: 最多停留2秒",
            "下法: 必须包含难度的下法",
        ]
    }

    /// 吊环规则
    pub fn rings_rules(&self) -> Vec<&'static str> {
        vec![
            "器械高度: 2.80米（环底）",
            "环距地面: 2.75米",
            "环间距: 0.50米",
            "成套要求: 力量、摆动动作",
            "静止要求: 每个静止2秒",
            "力量要求: 十字支撑、倒立等",
            "摆动要求: 大回环、下法",
            "下法: 必须有难度的下法",
        ]
    }

    /// 跳马规则
    pub fn vault_rules(&self) -> Vec<&'static str> {
        vec![
            "助跑长度: 最长25米",
            "跳马高度: 1.35米（男）、1.25米（女）",
            "跳跃次数: 团体赛1跳，单项决赛2跳",
            "分数计算: 单跳取该跳分数",
            "难度值: 根据动作组别确定",
            "动作组别: 5个组别",
            "落地要求: 双脚同时落地",
            "犯规: 踩线、触马犯规",
        ]
    }

    /// 双杠规则
    pub fn parallel_bars_rules(&self) -> Vec<&'static str> {
        vec![
            "器械高度: 2.00米",
            "杠长: 3.50米",
            "杠间距: 0.42-0.52米",
            "成套要求: 摆动、静止动作",
            "静止要求: 每个静止2秒",
            "支撑要求: 需包含支撑动作",
            "下法: 必须有难度的下法",
            "飞行: 可包含飞行动作",
        ]
    }

    /// 单杠规则
    pub fn horizontal_bar_rules(&self) -> Vec<&'static str> {
        vec![
            "器械高度: 2.78米",
            "杠长: 2.40米",
            "直径: 0.028米",
            "成套要求: 飞行动作、大回环",
            "飞行要求: 至少2个飞行动作",
            "握法: 正握、反握、混合握",
            "下法: 必须有难度的下法",
            "换握: 需包含换握动作",
        ]
    }

    /// 平衡木规则
    pub fn balance_beam_rules(&self) -> Vec<&'static str> {
        vec![
            "器械高度: 1.25米",
            "木长: 5.00米",
            "木宽: 0.10米",
            "时间限制: 70-90秒",
            "音乐: 需配音乐",
            "成套要求: 跳跃、转身、平衡、技巧",
            "连接要求: 舞蹈与技巧动作连接",
            "最低要求: 满足组别要求",
        ]
    }

    /// 高低杠规则
    pub fn uneven_bars_rules(&self) -> Vec<&'static str> {
        vec![
            "高杠高度: 2.50米",
            "低杠高度: 1.70米",
            "杠间距: 可调节",
            "成套要求: 飞行动作、换杠",
            "飞行要求: 至少2个飞行动作",
            "换杠: 需包含换杠动作",
            "下法: 必须有难度的下法",
            "连接: 飞行动作连接加分",
        ]
    }

    // ==================== 艺术体操规则 ====================

    /// 艺术体操器械
    pub fn rhythmic_apparatus(&self) -> Vec<&'static str> {
        vec![
            "绳 (Rope)",
            "圈 (Hoop)",
            "球 (Ball)",
            "棒 (Clubs)",
            "带 (Ribbon)",
        ]
    }

    /// 艺术体操评分规则
    pub fn rhythmic_scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "总分 = 难度分 (D) + 完成分 (E) + 艺术分 (A)",
            "难度分: 身体难度 + 器械难度",
            "完成分: 从10分起评",
            "艺术分: 音乐表达、艺术表现",
            "时间限制: 75-90秒",
            "音乐: 必须使用音乐",
            "场地: 13米×13米",
            "出界: 扣0.3分",
        ]
    }

    /// 艺术体操技术要求
    pub fn rhythmic_requirements(&self) -> Vec<&'static str> {
        vec![
            "身体难度: 跳跃、平衡、旋转、柔韧",
            "器械难度: 抛接、滚动、转动等",
            "器械使用: 必须全程使用器械",
            "身体姿态: 优美、流畅",
            "音乐表达: 与音乐节奏配合",
            "艺术表现: 情感表达",
            "创新: 鼓励创新动作",
            "合作: 团体赛需团队配合",
        ]
    }

    // ==================== 蹦床规则 ====================

    /// 蹦床比赛规则
    pub fn trampoline_rules(&self) -> Vec<&'static str> {
        vec![
            "成套动作: 10个连续动作",
            "时间限制: 无具体时间限制",
            "动作要求: 需包含难度的空翻",
            "评分: 难度分 + 完成分 + 时间分",
            "难度分: 根据动作难度值累加",
            "完成分: 从10分起评",
            "时间分: 飞行时间评分",
            "高度要求: 保持适当高度",
        ]
    }

    /// 蹦床器械规格
    pub fn trampoline_specs(&self) -> Vec<&'static str> {
        vec![
            "网面尺寸: 2.14米×4.28米",
            "框架高度: 1.155米",
            "弹簧数量: 约80-100个",
            "安全平台: 周围设置",
            "净空高度: 至少8米",
            "弹性: 符合FIG标准",
            "稳定性: 框架必须稳定",
            "安全垫: 厚度至少20厘米",
        ]
    }

    // ==================== 技巧体操规则 ====================

    /// 技巧体操规则
    pub fn acrobatic_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛形式: 配对（双人）、三人、四人",
            "性别组合: 男双、女双、混双、女三、男四",
            "动作类型: 平衡动作、动力动作",
            "成套时间: 平衡2分30秒，动力2分",
            "评分: 难度分 + 完成分 + 艺术分",
            "难度要求: 根据级别确定",
            "配合要求: 高度配合",
            "音乐: 需配音乐",
        ]
    }

    // ==================== 健美操规则 ====================

    /// 健美操规则
    pub fn aerobic_rules(&self) -> Vec<&'static str> {
        vec![
            "成套时间: 1分20秒-1分30秒",
            "动作要求: 连续复杂的高强度动作",
            "音乐: 需配音乐",
            "评分: 难度分 + 完成分 + 艺术分",
            "难度动作: 跳跃、俯卧撑、平衡等",
            "复杂性: 动作需连续复杂",
            "艺术表现: 音乐表达",
            "场地: 10米×10米",
        ]
    }

    // ==================== 通用规则 ====================

    /// 比赛装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "体操服: 紧身、无袖或有袖",
            "颜色: 纯色或图案",
            "装饰: 不允许过多装饰",
            "鞋子: 软底鞋或光脚",
            "护具: 可使用护腕、护膝",
            "发饰: 不得遮挡脸部",
            "首饰: 禁止佩戴",
            "号码: 需佩戴号码布",
        ]
    }

    /// 比赛流程规则
    pub fn competition_procedure(&self) -> Vec<&'static str> {
        vec![
            "检录: 赛前30分钟检录",
            "热身: 规定时间热身",
            "入场: 听到信号后入场",
            "展示: 向裁判示意",
            "开始: 裁判示意后开始",
            "结束: 动作结束后示意",
            "退场: 等待裁判示意后退场",
            "分数: 显示最终分数",
        ]
    }

    /// 犯规与扣分
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "超时: 扣0.5分",
            "出界: 扣0.3-0.5分",
            "跌落: 扣1.0分",
            "重复动作: 扣分",
            "动作缺失: 扣难度分",
            "静止超时: 扣0.3分",
            "服装违规: 扣0.3-0.5分",
            "干扰比赛: 取消资格",
        ]
    }

    /// 年龄组别
    pub fn age_categories(&self) -> Vec<&'static str> {
        vec![
            "幼儿组: 6-8岁",
            "儿童组: 9-11岁",
            "少年组: 12-14岁",
            "青年组: 15-17岁",
            "成年组: 18岁以上",
            "年龄限制: 根据比赛级别确定",
            "青年比赛: 世界青年体操锦标赛",
            "年龄证明: 需提供身份证明",
        ]
    }

    /// 世界纪录与认证
    pub fn world_records(&self) -> Vec<&'static str> {
        vec![
            "比赛级别: 需为FIG认证赛事",
            "裁判: 符合FIG认证标准",
            "器械: 符合FIG认证标准",
            "计时: 使用电子计时系统",
            "申请程序: 赛后提交申请",
            "审核: FIG技术委员会审核",
            "认证时间: 通常赛后2周内",
            "争议: 可向CAS申诉",
        ]
    }

    /// 反兴奋剂规定
    pub fn anti_doping_rules(&self) -> Vec<&'static str> {
        vec![
            "遵守WADA反兴奋剂条例",
            "禁药清单: WADA年度发布",
            "检测: 赛内和赛外均可检测",
            "治疗用药豁免: 需提前申请TUE",
            "行踪申报: 精英运动员需申报",
            "违规处罚: 禁赛2-4年",
            "申诉: CAS仲裁",
            "年龄造假: 视为兴奋剂违规",
        ]
    }
}

impl Default for GymnasticsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GymnasticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("gymnastics_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【体操详细规则】\n\
            国际体操联合会(FIG)标准规则\n\n\
            男子项目:\n{}\n\n\
            女子项目:\n{}\n\n\
            评分规则:\n{}\n\n\
            完成分评分:\n{}\n\n\
            犯规扣分:\n{}\n",
            self.mens_apparatus()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.womens_apparatus()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.execution_scoring()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.penalties()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gymnastics_detailed_rules_creation() {
        let rules = GymnasticsDetailedRules::new();
        assert_eq!(rules.metadata().name, "体操详细规则");
    }

    #[test]
    fn test_gymnastics_discipline_properties() {
        assert!(GymnasticsDiscipline::ArtisticGymnastics.is_olympic());
        assert!(!GymnasticsDiscipline::AcrobaticGymnastics.is_olympic());
    }

    #[test]
    fn test_apparatus_properties() {
        assert!(Apparatus::PommelHorse.is_men_apparatus());
        assert!(!Apparatus::PommelHorse.is_women_apparatus());
        assert!(Apparatus::BalanceBeam.is_women_apparatus());
        assert!(!Apparatus::BalanceBeam.is_men_apparatus());
    }

    #[test]
    fn test_mens_apparatus() {
        let rules = GymnasticsDetailedRules::new();
        let apparatus = rules.mens_apparatus();
        assert_eq!(apparatus.len(), 6);
    }

    #[test]
    fn test_womens_apparatus() {
        let rules = GymnasticsDetailedRules::new();
        let apparatus = rules.womens_apparatus();
        assert_eq!(apparatus.len(), 4);
    }

    #[test]
    fn test_scoring_rules() {
        let rules = GymnasticsDetailedRules::new();
        let scoring = rules.scoring_rules();
        assert!(!scoring.is_empty());
        assert!(scoring.iter().any(|s| s.contains("难度分")));
    }

    #[test]
    fn test_floor_exercise_rules() {
        let rules = GymnasticsDetailedRules::new();
        let rules_list = rules.floor_exercise_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("12米")));
    }

    #[test]
    fn test_explain() {
        let rules = GymnasticsDetailedRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("体操详细规则"));
        assert!(explanation.contains("FIG"));
    }
}