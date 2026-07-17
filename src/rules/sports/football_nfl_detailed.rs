//! NFL详细规则
//!
//! 美国职业橄榄球大联盟（National Football League）完整规则体系，
//! 包括比赛规则、场地规格、得分规则、进攻防守规则、选秀、薪资帽等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// NFL比赛时间规则
#[derive(Debug, Clone)]
pub struct NflGameTimingRules {
    metadata: RuleMetadata,
}

impl NflGameTimingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL比赛时间规则", "NFL比赛时间、节次、暂停等规则")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 比赛时间规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛分为4节，每节15分钟",
            "常规时间共60分钟",
            "上半场（第1-2节）和下半场（第3-4节）",
            "第1节和第2节之间休息2分钟（警告时间）",
            "中场休息12分钟",
            "第3节和第4节之间休息2分钟（警告时间）",
            "如常规时间平局，进入加时赛（10分钟）",
        ]
    }

    /// 暂停规则
    pub fn timeout_rules(&self) -> Vec<&'static str> {
        vec![
            "每队每半场3次暂停",
            "官方暂停（电视暂停）自动触发",
            "两分钟警告（2-minute warning）自动暂停",
            "受伤暂停不计入暂停次数",
            "暂停时间: 1分50秒",
            "教练可以挑战裁判判罚（需有暂停机会）",
            "挑战成功不消耗暂停，挑战失败扣除暂停",
        ]
    }

    /// 比赛时钟规则
    pub fn play_clock_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻方有40秒开始下一次进攻（play clock）",
            "裁判暂停后，play clock为25秒",
            "未能在规定时间内开球将被罚5码",
            "比赛时钟在边界出界时暂停",
            "比赛时钟在不完整传球时暂停",
            "比赛时钟在得分后暂停",
            "两分钟警告后时钟管理更严格",
        ]
    }

    /// 加时赛规则
    pub fn overtime_rules(&self) -> Vec<&'static str> {
        vec![
            "常规赛加时赛: 10分钟突然死亡法",
            "季后赛加时赛: 无时间限制",
            "先得分的球队获胜（突然死亡）",
            "2017年规则修改: 双方都有机会进攻",
            "如果先攻方只踢射门，后攻方有机会得分",
            "如果先攻方达阵，直接获胜",
            "如果10分钟内无人得分，比赛平局",
        ]
    }
}

impl Default for NflGameTimingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflGameTimingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_game_timing")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL比赛时间规则",
            &[
                ("比赛时间", &self.timing_rules()),
                ("暂停规则", &self.timeout_rules()),
                ("比赛时钟", &self.play_clock_rules()),
                ("加时赛", &self.overtime_rules()),
            ],
        )
    }
}

/// NFL场地规格规则
#[derive(Debug, Clone)]
pub struct NflFieldRules {
    metadata: RuleMetadata,
}

impl NflFieldRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL场地规格", "NFL场地尺寸、标记和规格")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 场地尺寸
    pub fn field_dimensions(&self) -> Vec<&'static str> {
        vec![
            "场地长度: 120码（109.73米）",
            "场地宽度: 53 1/3码（48.76米）",
            "端区长度: 10码（9.14米）",
            "比赛场地长度: 100码（91.44米）",
            "每10码有一个码线标记",
            "每5码有一条横跨场地的线",
            "场地表面: 天然草皮或人造草皮",
        ]
    }

    /// 场地标记
    pub fn field_markings(&self) -> Vec<&'static str> {
        vec![
            "码线: 每10码标明码数（10, 20, 30, 40, 50等）",
            "码数标记在场边和场地中央",
            "hash marks: 场地中央的标记线",
            "hash marks距离边线70英尺9英寸",
            "每码有两条平行的线",
            "端线: 场地两端的线",
            "边线: 场地两侧的线",
            "球门柱: 位于端区后方",
        ]
    }

    /// 球门规格
    pub fn goal_post_specs(&self) -> Vec<&'static str> {
        vec![
            "球门柱高度: 30英尺（9.14米）",
            "横杆高度: 10英尺（3.05米）",
            "横杆长度: 18英尺6英寸（5.64米）",
            "球门柱位于端区后方",
            "球门柱必须为黄色",
            "球门柱必须包覆保护材料",
        ]
    }

    /// 球场设施
    pub fn stadium_facilities(&self) -> Vec<&'static str> {
        vec![
            "更衣室设施必须符合标准",
            "医疗设施必须齐全",
            "照明系统必须满足电视转播要求",
            "观众座位必须满足NFL要求",
            "安全设施必须齐全",
            "媒体设施必须符合NFL要求",
        ]
    }
}

impl Default for NflFieldRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflFieldRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_field")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL场地规格",
            &[
                ("场地尺寸", &self.field_dimensions()),
                ("场地标记", &self.field_markings()),
                ("球门规格", &self.goal_post_specs()),
                ("球场设施", &self.stadium_facilities()),
            ],
        )
    }
}

/// NFL得分规则
#[derive(Debug, Clone)]
pub struct NflScoringRules {
    metadata: RuleMetadata,
}

impl NflScoringRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL得分规则", "NFL达阵、射门、附加分、安全分等规则")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 达阵规则
    pub fn touchdown_rules(&self) -> Vec<&'static str> {
        vec![
            "达阵（Touchdown）: 6分",
            "达阵条件: 球员在对方端区内控制球",
            "接球达阵: 球员在端区内完成接球",
            "跑球达阵: 球员带球进入端区",
            "达阵后可尝试附加分",
            "达阵是最高分值得分方式",
        ]
    }

    /// 附加分规则
    pub fn extra_point_rules(&self) -> Vec<&'static str> {
        vec![
            "附加分（1分）: 在15码线踢射门",
            "附加分（2分）: 在2码线尝试达阵",
            "防守方可以回攻得分（2分）",
            "2015年规则修改: 附加分距离增加",
            "附加分成功率约95%（1分）",
            "2分转换成功率约50%",
        ]
    }

    /// 射门规则
    pub fn field_goal_rules(&self) -> Vec<&'static str> {
        vec![
            "射门（Field Goal）: 3分",
            "射门必须在4档进攻时尝试",
            "射门距离越长越困难",
            "最长射门记录: 66码",
            "射门不中，球权转换",
            "射门必须从开球线后进行",
        ]
    }

    /// 安全分规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全分（Safety）: 2分",
            "安全分条件: 进攻方在本方端区被擒抱",
            "安全分条件: 进攻方在本方端区犯规",
            "安全分后，失分方自由踢球开球",
            "安全分较为罕见",
            "故意安全分（Intentional Safety）策略使用",
        ]
    }

    /// 其他得分规则
    pub fn other_scoring(&self) -> Vec<&'static str> {
        vec![
            "防守方达阵: 抄截或掉球回攻达阵",
            "自由球回攻达阵: 开球或射门不中后回攻",
            "防守方附加分回攻: 2分",
            "单人得分记录: 6次达阵（36分）",
            "单队得分记录: 73分",
            "最大分差记录: 59分",
        ]
    }
}

impl Default for NflScoringRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflScoringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_scoring")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL得分规则",
            &[
                ("达阵规则", &self.touchdown_rules()),
                ("附加分", &self.extra_point_rules()),
                ("射门规则", &self.field_goal_rules()),
                ("安全分", &self.safety_rules()),
                ("其他得分", &self.other_scoring()),
            ],
        )
    }
}

/// NFL进攻规则
#[derive(Debug, Clone)]
pub struct NflOffenseRules {
    metadata: RuleMetadata,
}

impl NflOffenseRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL进攻规则", "NFL进攻次数、传球、跑球等规则")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 进攻次数规则
    pub fn down_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻方有4次进攻机会推进10码",
            "每推进10码重新获得4次进攻机会",
            "第4档进攻可选择弃踢、射门或强攻",
            "第4档强攻失败，球权转换",
            "第4档通常选择弃踢或射门",
            "新进攻机会从死球点开始",
        ]
    }

    /// 传球规则
    pub fn passing_rules(&self) -> Vec<&'static str> {
        vec![
            "四分卫（QB）是主要传球手",
            "向前传球必须在开球线后进行",
            "每档进攻只能有1次向前传球",
            "接球手必须双脚在界内完成接球",
            "接球后必须成为跑球员",
            "传球时犯规可能导致接球无效",
            "不完整传球导致下一档从原位置开始",
        ]
    }

    /// 跑球规则
    pub fn rushing_rules(&self) -> Vec<&'static str> {
        vec![
            "跑球员可以持球跑动",
            "跑球员可以被擒抱（tackle）",
            "擒抱后球位（spot）决定下一档位置",
            "跑球员可以向前传球（必须是合法传球手）",
            "跑球员可以接球",
            "跑球员可以阻挡",
            "跑球是重要进攻手段",
        ]
    }

    /// 开球规则
    pub fn snap_rules(&self) -> Vec<&'static str> {
        vec![
            "开球（Snap）开始每档进攻",
            "中锋（Center）将球传给四分卫",
            "开球必须是向后的传球",
            "开球时所有进攻球员必须在开球线后",
            "非法开球罚5码",
            "开球失败可能导致掉球",
        ]
    }

    /// 非法动作
    pub fn illegal_actions(&self) -> Vec<&'static str> {
        vec![
            "非法向前传球: 罚5码+失去档数",
            "非法接球手: 罚5码",
            "非法移动: 罚5码",
            "非法阻挡: 罚10码",
            "非法用手: 罚10码",
            "夹人（Holding）: 罚10码",
        ]
    }
}

impl Default for NflOffenseRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflOffenseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_offense")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL进攻规则",
            &[
                ("进攻次数", &self.down_rules()),
                ("传球规则", &self.passing_rules()),
                ("跑球规则", &self.rushing_rules()),
                ("开球规则", &self.snap_rules()),
                ("非法动作", &self.illegal_actions()),
            ],
        )
    }
}

/// NFL防守规则
#[derive(Debug, Clone)]
pub struct NflDefenseRules {
    metadata: RuleMetadata,
}

impl NflDefenseRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL防守规则", "NFL擒抱、抄截、掉球、擒杀等规则")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 擒抱规则
    pub fn tackling_rules(&self) -> Vec<&'static str> {
        vec![
            "擒抱（Tackle）: 阻止进攻球员前进",
            "擒抱时球位确定下一档位置",
            "擒抱必须符合安全规范",
            "禁止头盔对头盔撞击",
            "禁止擒抱头部和颈部",
            "违规擒抱可能导致罚款或禁赛",
            "擒抱是防守的核心技术",
        ]
    }

    /// 抄截规则
    pub fn interception_rules(&self) -> Vec<&'static str> {
        vec![
            "抄截（Interception）: 防守方接住传球",
            "抄截后防守方转为进攻方",
            "抄截球员可以回攻",
            "抄截回攻可达阵得分",
            "抄截改变球权",
            "抄截是重大防守成就",
        ]
    }

    /// 掉球规则
    pub fn fumble_rules(&self) -> Vec<&'static str> {
        vec![
            "掉球（Fumble）: 进攻球员失去控球",
            "掉球后双方都可争夺球权",
            "防守方获得掉球后转为进攻方",
            "掉球后进攻方可能保持球权",
            "掉球出界：球权归最后控球方",
            "掉球是重大失误",
        ]
    }

    /// 擒杀规则
    pub fn sack_rules(&self) -> Vec<&'static str> {
        vec![
            "擒杀（Sack）: 在开球线后擒抱四分卫",
            "擒杀导致码数损失",
            "擒杀是防守方重要成就",
            "擒杀后下一档从擒杀点开始",
            "擒杀可能导致掉球",
            "擒杀是重要防守统计",
        ]
    }

    /// 防守犯规
    pub fn defensive_penalties(&self) -> Vec<&'static str> {
        vec![
            "防守过线（Offside）: 罚5码",
            "非法接触（Illegal Contact）: 罚5码+自动首攻",
            "防守传球干扰: 罚球位+自动首攻",
            "粗暴对待四分卫: 罚15码",
            " Helmet-to-helmet撞击: 罚15码+可能禁赛",
            "非法夹人: 罚10码",
        ]
    }
}

impl Default for NflDefenseRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflDefenseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_defense")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL防守规则",
            &[
                ("擒抱规则", &self.tackling_rules()),
                ("抄截规则", &self.interception_rules()),
                ("掉球规则", &self.fumble_rules()),
                ("擒杀规则", &self.sack_rules()),
                ("防守犯规", &self.defensive_penalties()),
            ],
        )
    }
}

/// NFL选秀规则
#[derive(Debug, Clone)]
pub struct NflDraftRules {
    metadata: RuleMetadata,
}

impl NflDraftRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL选秀规则", "NFL选秀制度、选秀顺序、交易规则")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 选秀基本规则
    pub fn draft_basics(&self) -> Vec<&'static str> {
        vec![
            "选秀共7轮，每轮32个选秀权",
            "总共224个选秀权（不含补偿选秀权）",
            "参选年龄: 至少高中毕业3年",
            "参选球员需声明参选",
            "选秀大会通常在4月举行",
            "未选秀球员可签约自由球员",
        ]
    }

    /// 选秀顺序
    pub fn draft_order(&self) -> Vec<&'static str> {
        vec![
            "选秀顺序按战绩倒序排列",
            "战绩最差球队获得状元签",
            "超级碗冠军获得最后选秀权",
            "战绩相同球队通过附加赛决定",
            "附加赛: 赛程强度-对阵记录-抽签",
            "选秀权可以交易",
        ]
    }

    /// 补偿选秀权
    pub fn compensatory_picks(&self) -> Vec<&'static str> {
        vec![
            "补偿选秀权: 补偿失去自由球员的球队",
            "基于自由球员年薪、上场时间等计算",
            "补偿选秀权在第3-7轮末尾",
            "最多32个补偿选秀权",
            "2020年规则修改: 只有失去自由球员才获得",
            "补偿选秀权可交易（2021年起）",
        ]
    }

    /// 选秀权交易
    pub fn draft_pick_trading(&self) -> Vec<&'static str> {
        vec![
            "选秀权可以交易",
            "选秀权交易需在选秀大会前完成",
            "选秀权可以换球员或未来选秀权",
            "选秀权价值根据选秀价值表评估",
            "向上交易获得更高选秀权",
            "向下交易获得更多选秀权",
        ]
    }

    /// 新秀合同
    pub fn rookie_contract_rules(&self) -> Vec<&'static str> {
        vec![
            "首轮新秀合同: 4年+球队第5年选项",
            "次轮及以后新秀合同: 4年",
            "新秀薪资按选秀顺位确定",
            "新秀合同金额由集体谈判协议规定",
            "第5年选项需在第4年前执行",
            "首轮新秀合同全额保障",
        ]
    }
}

impl Default for NflDraftRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflDraftRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_draft")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL选秀规则",
            &[
                ("选秀基本", &self.draft_basics()),
                ("选秀顺序", &self.draft_order()),
                ("补偿选秀权", &self.compensatory_picks()),
                ("选秀权交易", &self.draft_pick_trading()),
                ("新秀合同", &self.rookie_contract_rules()),
            ],
        )
    }
}

/// NFL薪资帽规则
#[derive(Debug, Clone)]
pub struct NflSalaryCapRules {
    metadata: RuleMetadata,
}

impl NflSalaryCapRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL薪资帽规则", "NFL工资帽、奢侈税、合同规则")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 工资帽基本规则
    pub fn salary_cap_basics(&self) -> Vec<&'static str> {
        vec![
            "工资帽: 球队薪资总额上限",
            "2024赛季工资帽: 约2.55亿美元",
            "NFL采用硬工资帽制度",
            "硬工资帽不得突破",
            "工资帽每年根据收入计算",
            "工资帽与集体谈判协议相关",
        ]
    }

    /// 工资帽计算
    pub fn cap_calculation(&self) -> Vec<&'static str> {
        vec![
            "工资帽包括所有球员薪资",
            "工资帽按年度计算",
            "签字费按合同年限分摊",
            "激励奖金可能计入或不计入",
            "死钱（Dead Money）: 已释放球员的剩余薪资",
            "工资帽管理是总经理核心能力",
        ]
    }

    /// 合同结构
    pub fn contract_structure(&self) -> Vec<&'static str> {
        vec![
            "合同: 基本工资 + 签字费 + 激励奖金",
            "基本工资按赛季支付",
            "签字费在签约时支付",
            "合同通常有保障和非保障部分",
            "保障部分计入工资帽",
            "合同可重新协商",
        ]
    }

    /// 特殊条款
    pub fn special_clauses(&self) -> Vec<&'static str> {
        vec![
            "不可交易条款（No-trade clause）",
            "不可下放条款（No-waiver clause）",
            "跳出条款（Opt-out clause）",
            "激励条款（Incentives）",
            "名册奖金（Roster bonus）",
            "出场奖金（Per-game bonus）",
        ]
    }

    /// 自由球员规则
    pub fn free_agency(&self) -> Vec<&'static str> {
        vec![
            "非限制自由球员: 合同到期球员",
            "受限自由球员: 4年经验球员",
            "球队可对受限自由球员报价",
            "过渡标签（Transition tag）",
            "特权标签（Franchise tag）",
            "特权标签薪资按位置最高薪资计算",
        ]
    }
}

impl Default for NflSalaryCapRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflSalaryCapRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_salary_cap")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL薪资帽规则",
            &[
                ("工资帽基本", &self.salary_cap_basics()),
                ("工资帽计算", &self.cap_calculation()),
                ("合同结构", &self.contract_structure()),
                ("特殊条款", &self.special_clauses()),
                ("自由球员", &self.free_agency()),
            ],
        )
    }
}

/// NFL季后赛规则
#[derive(Debug, Clone)]
pub struct NflPlayoffRules {
    metadata: RuleMetadata,
}

impl NflPlayoffRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NFL季后赛规则", "NFL季后赛赛制、排名、超级碗")
                .with_origin("NFL官方规则")
                .with_tags(vec!["体育".into(), "橄榄球".into(), "NFL".into()]),
        }
    }

    /// 季后赛资格
    pub fn playoff_qualification(&self) -> Vec<&'static str> {
        vec![
            "每分区4支球队，共8个分区",
            "分区冠军自动进入季后赛（4支）",
            "外卡球队: 非分区冠军战绩最好的球队",
            "外卡球队每联盟3支",
            "每联盟共7支球队进入季后赛",
            "共14支球队进入季后赛",
        ]
    }

    /// 季后赛排名
    pub fn playoff_seeding(&self) -> Vec<&'static str> {
        vec![
            "第1种子: 战绩最好的分区冠军",
            "第2种子: 战绩第二好的分区冠军",
            "第3种子: 战绩第三好的分区冠军",
            "第4种子: 战绩第四好的分区冠军",
            "第5-7种子: 外卡球队按战绩排名",
            "第1种子获得首轮轮空（bye week）",
        ]
    }

    /// 季后赛赛制
    pub fn playoff_format(&self) -> Vec<&'static str> {
        vec![
            "外卡轮: 第2-7种子比赛",
            "分区轮: 外卡轮胜者vs第1种子",
            "联盟冠军赛: 分区轮胜者",
            "超级碗: 东西部联盟冠军对决",
            "季后赛单场淘汰制",
            "主场优势由种子排名决定",
        ]
    }

    /// 超级碗规则
    pub fn super_bowl_rules(&self) -> Vec<&'static str> {
        vec![
            "超级碗: NFL冠军赛",
            "超级碗在预定场地举行（中立场地）",
            "超级碗通常在2月第一个周日",
            "超级碗MVP评选",
            "超级碗是最高荣誉",
            "超级碗中场秀是重要活动",
        ]
    }

    /// 季后赛加时赛
    pub fn playoff_overtime(&self) -> Vec<&'static str> {
        vec![
            "季后赛加时赛无时间限制",
            "双方都有机会进攻",
            "如果先攻方达阵，比赛结束",
            "如果先攻方只踢射门，后攻方有机会",
            "如果双方打平，继续加时赛",
            "最终会分出胜负",
        ]
    }
}

impl Default for NflPlayoffRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NflPlayoffRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nfl_playoff")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NFL季后赛规则",
            &[
                ("季后赛资格", &self.playoff_qualification()),
                ("季后赛排名", &self.playoff_seeding()),
                ("季后赛赛制", &self.playoff_format()),
                ("超级碗规则", &self.super_bowl_rules()),
                ("季后赛加时", &self.playoff_overtime()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_timing_rules() {
        let rules = NflGameTimingRules::new();
        assert!(!rules.timing_rules().is_empty());
        assert!(!rules.timeout_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("timing".to_string()))
            .is_ok());
    }

    #[test]
    fn test_field_rules() {
        let rules = NflFieldRules::new();
        assert!(!rules.field_dimensions().is_empty());
        assert!(!rules.field_markings().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("field".to_string()))
            .is_ok());
    }

    #[test]
    fn test_scoring_rules() {
        let rules = NflScoringRules::new();
        assert!(!rules.touchdown_rules().is_empty());
        assert!(!rules.field_goal_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("scoring".to_string()))
            .is_ok());
    }

    #[test]
    fn test_offense_rules() {
        let rules = NflOffenseRules::new();
        assert!(!rules.down_rules().is_empty());
        assert!(!rules.passing_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("offense".to_string()))
            .is_ok());
    }

    #[test]
    fn test_defense_rules() {
        let rules = NflDefenseRules::new();
        assert!(!rules.tackling_rules().is_empty());
        assert!(!rules.interception_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("defense".to_string()))
            .is_ok());
    }

    #[test]
    fn test_draft_rules() {
        let rules = NflDraftRules::new();
        assert!(!rules.draft_basics().is_empty());
        assert!(!rules.draft_order().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("draft".to_string()))
            .is_ok());
    }

    #[test]
    fn test_salary_cap_rules() {
        let rules = NflSalaryCapRules::new();
        assert!(!rules.salary_cap_basics().is_empty());
        assert!(!rules.cap_calculation().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("salary".to_string()))
            .is_ok());
    }

    #[test]
    fn test_playoff_rules() {
        let rules = NflPlayoffRules::new();
        assert!(!rules.playoff_qualification().is_empty());
        assert!(!rules.super_bowl_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("playoff".to_string()))
            .is_ok());
    }

    #[test]
    fn test_rule_categories() {
        assert!(matches!(
            NflGameTimingRules::new().category(),
            RuleCategory::Sports { .. }
        ));
        assert!(matches!(
            NflScoringRules::new().category(),
            RuleCategory::Sports { .. }
        ));
    }

    #[test]
    fn test_explain_methods() {
        let rules = NflScoringRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("达阵"));
        assert!(explanation.contains("射门"));
    }
}
