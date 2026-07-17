//! 中超联赛详细规则
//!
//! 中国足球协会超级联赛（Chinese Football Association Super League）完整规则体系，
//! 包括比赛规则、赛事规则、球员注册、外援规则、转会规则、财务规则、纪律规则等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 中超比赛规则
#[derive(Debug, Clone)]
pub struct CslMatchRules {
    metadata: RuleMetadata,
}

impl CslMatchRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中超比赛规则", "中超比赛时间、换人、替补席等规则")
                .with_origin("中国足协官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "中超".into()]),
        }
    }

    /// 比赛时间规则
    pub fn match_timing(&self) -> Vec<&'static str> {
        vec![
            "比赛分为上下半场，各45分钟",
            "中场休息不超过15分钟",
            "比赛时间可根据比赛停顿补时",
            "补时由主裁判决定",
            "比赛时间延长需在死球状态",
            "加时赛仅在淘汰赛阶段进行",
            "赛季时间: 3月-11月（跨年赛季改革讨论中）",
        ]
    }

    /// 换人规则
    pub fn substitution_rules(&self) -> Vec<&'static str> {
        vec![
            "每队最多5个换人名额",
            "换人最多分3次进行（中场休息除外）",
            "脑震荡换人额外允许（不占用常规名额）",
            "替补席最多9名球员",
            "换人需在死球状态进行",
            "被换下球员不得再次上场",
            "U23球员换人有特殊规定",
        ]
    }

    /// U23政策
    pub fn u23_policy(&self) -> Vec<&'static str> {
        vec![
            "每场比赛必须有至少1名U23球员首发",
            "U23定义: 当年不满23岁",
            "U23球员累计出场时间有要求",
            "如U23球员被换下需由另一名U23替换",
            "违反U23规定可能导致判负",
            "政策调整频繁，需关注最新通知",
            "U23政策旨在促进年轻球员发展",
        ]
    }

    /// 球员装备规则
    pub fn kit_rules(&self) -> Vec<&'static str> {
        vec![
            "两队球衣颜色必须明显不同",
            "门将球衣颜色必须区别于双方球员",
            "球衣背后必须有清晰号码",
            "号码范围: 1-99",
            "场上队长需佩戴袖标",
            "严禁佩戴危险物品（首饰、手表等）",
            "球鞋必须符合安全标准",
            "球衣广告需符合中超商业规定",
        ]
    }
}

impl Default for CslMatchRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CslMatchRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("csl_match")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中超比赛规则",
            &[
                ("比赛时间", &self.match_timing()),
                ("换人规则", &self.substitution_rules()),
                ("U23政策", &self.u23_policy()),
                ("球员装备", &self.kit_rules()),
            ],
        )
    }
}

/// 中超赛事规则
#[derive(Debug, Clone)]
pub struct CslCompetitionRules {
    metadata: RuleMetadata,
}

impl CslCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中超赛事规则", "中超积分、排名、亚冠资格、降级规则")
                .with_origin("中国足协官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "中超".into()]),
        }
    }

    /// 积分规则
    pub fn points_system(&self) -> Vec<&'static str> {
        vec![
            "比赛获胜: 3分",
            "比赛平局: 1分",
            "比赛失利: 0分",
            "积分决定联赛排名",
            "积分相同则比较净胜球",
            "净胜球相同则比较总进球",
            "以上都相同则比较相互对战成绩",
        ]
    }

    /// 排名规则
    pub fn ranking_criteria(&self) -> Vec<&'static str> {
        vec![
            "首先比较积分（高者居前）",
            "积分相同比较净胜球",
            "净胜球相同比较总进球",
            "进球相同比较相互对战积分",
            "对战积分相同比较相互对战净胜球",
            "对战净胜球相同比较相互对战进球",
            "如仍相同则比较预备队联赛成绩",
            "最后可比较公平竞赛积分",
        ]
    }

    /// 亚冠资格
    pub fn acl_qualification(&self) -> Vec<&'static str> {
        vec![
            "中超冠军: 亚冠小组赛资格",
            "亚军: 亚冠小组赛资格",
            "第三名: 亚冠附加赛资格",
            "足协杯冠军: 亚冠小组赛资格",
            "如足协杯冠军已通过联赛获得资格，名额顺延",
            "亚足联技术积分排名影响名额分配",
            "技术积分基于俱乐部亚战成绩",
        ]
    }

    /// 降级规则
    pub fn relegation_rules(&self) -> Vec<&'static str> {
        vec![
            "倒数第2名（第15名）降级至中甲",
            "倒数第1名（第16名）降级至中甲",
            "降级球队失去中超转播分成",
            "降级球队可获得一定补偿",
            "如中超扩军，降级规则可能调整",
            "2023赛季后降级名额调整为2个",
        ]
    }

    /// 赛程安排
    pub fn schedule_rules(&self) -> Vec<&'static str> {
        vec![
            "每队30场联赛比赛（16支球队）",
            "每队主客场各15场",
            "赛季通常从3月开始，11月结束",
            "因国家队比赛可调整赛程",
            "为国家队集训让路，联赛可能暂停",
            "极端天气可延期比赛",
            "亚冠比赛日联赛通常不安排",
        ]
    }

    /// 联赛扩军
    pub fn league_expansion(&self) -> Vec<&'static str> {
        vec![
            "2024赛季起中超扩军至18支球队",
            "每队34场联赛比赛",
            "降级名额相应调整",
            "扩军旨在增加比赛场次",
            "提高联赛竞争性",
            "与亚足联赛历接轨",
        ]
    }
}

impl Default for CslCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CslCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("csl_competition")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中超赛事规则",
            &[
                ("积分系统", &self.points_system()),
                ("排名规则", &self.ranking_criteria()),
                ("亚冠资格", &self.acl_qualification()),
                ("降级规则", &self.relegation_rules()),
                ("赛程安排", &self.schedule_rules()),
                ("联赛扩军", &self.league_expansion()),
            ],
        )
    }
}

/// 中超球员注册规则
#[derive(Debug, Clone)]
pub struct CslPlayerRegistrationRules {
    metadata: RuleMetadata,
}

impl CslPlayerRegistrationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中超球员注册规则", "中超本土球员、外援、青训球员规则")
                .with_origin("中国足协官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "中超".into()]),
        }
    }

    /// 球员名单规则
    pub fn squad_registration(&self) -> Vec<&'static str> {
        vec![
            "每队最多报名30名球员",
            "必须包含至少4名U23球员",
            "门将必须至少2名",
            "注册窗口: 赛季前和夏季",
            "未注册球员不得参加中超比赛",
            "赛季中可补报名（伤病等）",
        ]
    }

    /// 外援规则
    pub fn foreign_player_rules(&self) -> Vec<&'static str> {
        vec![
            "每队最多报名5名外援",
            "每场比赛最多上场4名外援",
            "外援无亚洲外援特殊名额",
            "外援需持有国际转会证明（ITC）",
            "外援需通过体检和注册",
            "港澳台球员不计入外援名额",
            "归化球员政策另有规定",
        ]
    }

    /// 归化球员规则
    pub fn naturalized_player_rules(&self) -> Vec<&'static str> {
        vec![
            "归化球员: 外籍球员入籍中国",
            "血缘归化: 有中国血统的球员",
            "非血缘归化: 无中国血统的入籍球员",
            "归化球员政策调整频繁",
            "非血缘归化计入外援名额",
            "血缘归化视为本土球员",
            "国家队归化球员有特殊规定",
        ]
    }

    /// 青训球员规则
    pub fn youth_players(&self) -> Vec<&'static str> {
        vec![
            "青训球员定义: 本俱乐部培养",
            "青训补偿金制度",
            "联合培养机制",
            "青少年球员转会保护",
            "U21球员转会需符合国际足联规则",
            "中国足协青训标准认证",
            "青训投入计入俱乐部准入条件",
        ]
    }

    /// 球员准入条件
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "球员需持有有效合同",
            "球员需通过体能测试（YOYO测试）",
            "球员需持有健康证明",
            "球员需完成注册手续",
            "欠薪球员可申请仲裁解除合同",
            "纪律处罚期间不得参赛",
        ]
    }
}

impl Default for CslPlayerRegistrationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CslPlayerRegistrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("csl_player_registration")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中超球员注册规则",
            &[
                ("球员名单", &self.squad_registration()),
                ("外援规则", &self.foreign_player_rules()),
                ("归化球员", &self.naturalized_player_rules()),
                ("青训球员", &self.youth_players()),
                ("准入条件", &self.eligibility()),
            ],
        )
    }
}

/// 中超转会规则
#[derive(Debug, Clone)]
pub struct CslTransferRules {
    metadata: RuleMetadata,
}

impl CslTransferRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中超转会规则", "中超转会窗口、转会费、调节费规则")
                .with_origin("中国足协官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "中超".into()]),
        }
    }

    /// 转会窗口规则
    pub fn transfer_windows(&self) -> Vec<&'static str> {
        vec![
            "冬季转会窗口: 1-2月（约1个月）",
            "夏季转会窗口: 7-8月（约1个月）",
            "窗口外不得注册新球员",
            "自由球员可在窗口外签约",
            "门将转会可申请例外",
            "国际转会需持有ITC",
        ]
    }

    /// 转会费规则
    pub fn transfer_fees(&self) -> Vec<&'static str> {
        vec![
            "转会费由俱乐部协商确定",
            "转会费需在中国足协备案",
            "高额转会费需缴纳调节费",
            "签字费计入转会成本",
            "经纪人费也计入转会成本",
            "内部交易需公平定价",
        ]
    }

    /// 引援调节费（奢侈税）
    pub fn adjustment_fee(&self) -> Vec<&'static str> {
        vec![
            "引援调节费: 中国足协奢侈税",
            "外援转会费超过4500万人民币需缴纳等额调节费",
            "内援转会费超过2000万人民币需缴纳等额调节费",
            "调节费用于青少年足球发展",
            "调节费政策调整频繁",
            "部分俱乐部因调节费放弃引援",
            "调节费引发争议，政策可能改革",
        ]
    }

    /// 租借规则
    pub fn loan_rules(&self) -> Vec<&'static str> {
        vec![
            "每队最多租借入3名球员",
            "租借期最短: 两个注册窗口之间",
            "租借期最长: 一个赛季",
            "租借球员不得在对阵母队时上场",
            "U21租借不受限制",
            "租借需在中国足协备案",
        ]
    }

    /// 球员欠薪处理
    pub fn salary_arrears(&self) -> Vec<&'static str> {
        vec![
            "欠薪超过3个月球员可申请仲裁",
            "仲裁成功可解除合同",
            "欠薪俱乐部可能被扣分",
            "严重欠薪可能取消准入资格",
            "欠薪球员可自由转会",
            "中国足协设立仲裁委员会",
        ]
    }
}

impl Default for CslTransferRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CslTransferRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("csl_transfer")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中超转会规则",
            &[
                ("转会窗口", &self.transfer_windows()),
                ("转会费", &self.transfer_fees()),
                ("引援调节费", &self.adjustment_fee()),
                ("租借规则", &self.loan_rules()),
                ("欠薪处理", &self.salary_arrears()),
            ],
        )
    }
}

/// 中超财务规则
#[derive(Debug, Clone)]
pub struct CslFinancialRules {
    metadata: RuleMetadata,
}

impl CslFinancialRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中超财务规则", "中超工资帽、投资帽、财务监管")
                .with_origin("中国足协官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "中超".into()]),
        }
    }

    /// 工资帽
    pub fn salary_cap(&self) -> Vec<&'static str> {
        vec![
            "中超工资帽: 单个俱乐部顶薪限制",
            "顶薪: 税前300万人民币（国内球员）",
            "外援顶薪: 税前300万欧元",
            "工资帽政策2020年开始实施",
            "违反工资帽可能面临处罚",
            "工资帽政策调整频繁",
            "部分俱乐部面临降薪压力",
        ]
    }

    /// 投资帽
    pub fn investment_cap(&self) -> Vec<&'static str> {
        vec![
            "投资帽: 俱乐部年度投资上限",
            "中超俱乐部年度投资上限: 6亿人民币",
            "中甲俱乐部年度投资上限: 2亿人民币",
            "投资包括工资、转会费、运营等",
            "超过投资帽需缴纳调节费",
            "投资帽旨在控制俱乐部亏损",
            "政策实施后俱乐部投入减少",
        ]
    }

    /// 财务监管
    pub fn financial_supervision(&self) -> Vec<&'static str> {
        vec![
            "俱乐部需提交年度财务报告",
            "财务报告需经审计",
            "亏损俱乐部需制定整改计划",
            "连续亏损可能影响准入",
            "中国足协设立财务监管机构",
            "财务信息公开透明化",
            "严重财务问题可能取消注册资格",
        ]
    }

    /// 名称中性化
    pub fn club_naming(&self) -> Vec<&'static str> {
        vec![
            "俱乐部名称中性化政策: 2021年实施",
            "名称不得含有投资人名称",
            "名称需体现地域特色",
            "少数俱乐部保留传统名称",
            "名称变更需中国足协批准",
            "政策旨在俱乐部品牌独立",
            "部分俱乐部因更名损失商业价值",
        ]
    }
}

impl Default for CslFinancialRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CslFinancialRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("csl_financial")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中超财务规则",
            &[
                ("工资帽", &self.salary_cap()),
                ("投资帽", &self.investment_cap()),
                ("财务监管", &self.financial_supervision()),
                ("名称中性化", &self.club_naming()),
            ],
        )
    }
}

/// 中超VAR和裁判规则
#[derive(Debug, Clone)]
pub struct CslVARRules {
    metadata: RuleMetadata,
}

impl CslVARRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中超VAR规则", "中超VAR使用、裁判、判罚规则")
                .with_origin("中国足协官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "中超".into()]),
        }
    }

    /// VAR使用规则
    pub fn var_usage(&self) -> Vec<&'static str> {
        vec![
            "VAR: Video Assistant Referee",
            "VAR用于纠正明显错误",
            "四种可审查情况: 进球、点球、红牌、身份错误",
            "VAR仅在明显错误时介入",
            "主裁判可要求VAR审查",
            "VAR可建议主裁判审查",
            "最终决定权在主裁判",
            "2018赛季开始全面使用VAR",
        ]
    }

    /// VAR审查流程
    pub fn var_process(&self) -> Vec<&'static str> {
        vec![
            "VAR团队位于比赛现场或北京",
            "VAR团队: 1名VAR + 1名AVAR",
            "VAR实时监控所有比赛",
            "发现潜在错误时通知主裁判",
            "主裁判可进行On-Field Review",
            "审查时间尽量短",
            "审查结果通过耳机通知",
        ]
    }

    /// 裁判规则
    pub fn refereeing_rules(&self) -> Vec<&'static str> {
        vec![
            "主裁判负责比赛管理",
            "助理裁判协助判罚越位、边线",
            "第四官员管理替补席、换人",
            "VAR团队协助关键判罚",
            "裁判需持有中国足协执照",
            "裁判体能测试每赛季进行",
            "裁判评估影响晋升降级",
            "外籍裁判可执法重要比赛",
        ]
    }

    /// 裁判职业化
    pub fn professional_referees(&self) -> Vec<&'static str> {
        vec![
            "中国足协推进裁判职业化",
            "职业裁判享受薪酬保障",
            "职业裁判需全职投入",
            "职业裁判数量逐步增加",
            "裁判培训体系完善",
            "裁判引入考核淘汰机制",
            "优秀裁判可晋升国际级",
        ]
    }
}

impl Default for CslVARRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CslVARRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("csl_var")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中超VAR规则",
            &[
                ("VAR使用", &self.var_usage()),
                ("VAR流程", &self.var_process()),
                ("裁判规则", &self.refereeing_rules()),
                ("裁判职业化", &self.professional_referees()),
            ],
        )
    }
}

/// 中超纪律规则
#[derive(Debug, Clone)]
pub struct CslDisciplinaryRules {
    metadata: RuleMetadata,
}

impl CslDisciplinaryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中超纪律规则", "中超红黄牌、禁赛、罚款规则")
                .with_origin("中国足协官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "中超".into()]),
        }
    }

    /// 黄牌累积规则
    pub fn yellow_card_accumulation(&self) -> Vec<&'static str> {
        vec![
            "4张黄牌: 自动禁赛1场",
            "8张黄牌: 自动禁赛2场",
            "12张黄牌: 自动禁赛3场",
            "累积计算整个赛季",
            "黄牌在足协杯中单独计算",
            "黄牌罚款: 每张500元人民币",
            "严重违纪可能追罚",
        ]
    }

    /// 红牌禁赛规则
    pub fn red_card_suspension(&self) -> Vec<&'static str> {
        vec![
            "两黄变一红: 禁赛1场",
            "直接红牌: 禁赛至少1场",
            "暴力行为: 禁赛3场以上",
            "严重犯规: 禁赛3场以上",
            "辱骂裁判: 禁赛5场以上",
            "打架斗殴: 禁赛6-12个月",
            "禁赛可上诉",
            "中国足协纪律委员会裁决",
        ]
    }

    /// 赛后纪律行动
    pub fn retrospective_action(&self) -> Vec<&'static str> {
        vec![
            "赛后可追罚未被发现违规",
            "需在比赛后48小时内提出",
            "纪律委员会审议",
            "球员可因不当行为被追罚",
            "教练言论可能被追罚",
            "俱乐部可能被罚款",
            "严重违纪可能终身禁赛",
        ]
    }

    /// 罚款规则
    pub fn fines(&self) -> Vec<&'static str> {
        vec![
            "红牌: 自动罚款",
            "围攻裁判: 集体罚款",
            "延误比赛: 罚款",
            "球员冲突: 双方罚款",
            "球迷不当行为: 俱乐部罚款",
            "场地安全问题: 俱乐部罚款",
            "罚款金额根据违纪程度确定",
        ]
    }

    /// 赛风赛纪
    pub fn fair_play(&self) -> Vec<&'static str> {
        vec![
            "中国足协重视赛风赛纪",
            "俱乐部需签署赛风赛纪责任书",
            "违纪行为与俱乐部准入挂钩",
            "建立黑名单制度",
            "严重违纪可能取消注册资格",
            "裁判安全保护加强",
            "球迷行为规范",
        ]
    }
}

impl Default for CslDisciplinaryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CslDisciplinaryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("csl_disciplinary")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中超纪律规则",
            &[
                ("黄牌累积", &self.yellow_card_accumulation()),
                ("红牌禁赛", &self.red_card_suspension()),
                ("赛后追罚", &self.retrospective_action()),
                ("罚款规则", &self.fines()),
                ("赛风赛纪", &self.fair_play()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_rules() {
        let rules = CslMatchRules::new();
        assert!(!rules.match_timing().is_empty());
        assert!(!rules.substitution_rules().is_empty());
        assert!(!rules.u23_policy().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("match".to_string())).is_ok());
    }

    #[test]
    fn test_competition_rules() {
        let rules = CslCompetitionRules::new();
        assert!(!rules.points_system().is_empty());
        assert!(!rules.ranking_criteria().is_empty());
        assert!(!rules.acl_qualification().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("competition".to_string())).is_ok());
    }

    #[test]
    fn test_player_registration_rules() {
        let rules = CslPlayerRegistrationRules::new();
        assert!(!rules.squad_registration().is_empty());
        assert!(!rules.foreign_player_rules().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("registration".to_string())).is_ok());
    }

    #[test]
    fn test_transfer_rules() {
        let rules = CslTransferRules::new();
        assert!(!rules.transfer_windows().is_empty());
        assert!(!rules.adjustment_fee().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("transfer".to_string())).is_ok());
    }

    #[test]
    fn test_financial_rules() {
        let rules = CslFinancialRules::new();
        assert!(!rules.salary_cap().is_empty());
        assert!(!rules.investment_cap().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("financial".to_string())).is_ok());
    }

    #[test]
    fn test_var_rules() {
        let rules = CslVARRules::new();
        assert!(!rules.var_usage().is_empty());
        assert!(!rules.var_process().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("var".to_string())).is_ok());
    }

    #[test]
    fn test_disciplinary_rules() {
        let rules = CslDisciplinaryRules::new();
        assert!(!rules.yellow_card_accumulation().is_empty());
        assert!(!rules.red_card_suspension().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("disciplinary".to_string())).is_ok());
    }

    #[test]
    fn test_rule_categories() {
        assert!(matches!(
            CslMatchRules::new().category(),
            RuleCategory::Sports { .. }
        ));
        assert!(matches!(
            CslCompetitionRules::new().category(),
            RuleCategory::Sports { .. }
        ));
    }

    #[test]
    fn test_explain_methods() {
        let rules = CslMatchRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("比赛时间"));
        assert!(explanation.contains("U23政策"));
    }
}