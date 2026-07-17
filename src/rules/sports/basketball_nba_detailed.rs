//! NBA详细规则
//!
//! 美国职业篮球联赛(NBA)详细比赛规则，包括比赛时间、场地规格、犯规规则、
//! 选秀制度、工资帽、季后赛等完整规则体系。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// NBA比赛时间规则
#[derive(Debug, Clone)]
pub struct NbaGameTimingRules {
    metadata: RuleMetadata,
}

impl NbaGameTimingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA比赛时间规则", "NBA比赛时间、节次、暂停等规则")
                .with_origin("NBA官方规则")
                .with_tags(vec!["体育".into(), "篮球".into(), "NBA".into()]),
        }
    }

    /// 比赛时间规则
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛分为4节，每节12分钟",
            "常规时间共48分钟",
            "加时赛每节5分钟，直到分出胜负",
            "第1节与第2节之间休息2分钟",
            "第3节与第4节之间休息2分钟",
            "上半场与下半场之间休息15分钟",
            "加时赛之间休息2分钟",
        ]
    }

    /// 暂停规则
    pub fn timeout_rules(&self) -> Vec<&'static str> {
        vec![
            "每队常规时间有7次暂停",
            "每队每节最多4次暂停",
            "第四节最后3分钟每队最多2次暂停",
            "加时赛每队有2次暂停",
            "官方暂停：每节第一个强制暂停（7分钟）",
            "电视暂停：每节第二个强制暂停（3分钟）",
            "必须进入死球状态才能叫暂停",
            "控球方可以在前场叫20秒短暂停（每场1次）",
        ]
    }

    /// 24秒进攻规则
    pub fn shot_clock_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻方有24秒完成投篮",
            "投篮必须触及篮筐才算有效",
            "抢到进攻篮板后时钟重置为14秒",
            "防守方犯规后时钟重置为14秒",
            "如果剩余时间少于14秒，保持原时间",
            "踩线算违例，球权转换",
            "技术犯规后进攻方重新获得24秒",
        ]
    }

    /// 8秒规则
    pub fn eight_second_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻方必须在8秒内将球运过半场",
            "运球者必须双脚都过半场",
            "防守方犯规或叫暂停后重置为14秒",
            "抢断后从后场开始时重置为14秒",
        ]
    }

    /// 3秒规则
    pub fn three_second_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻球员不得在限制区停留超过3秒",
            "限制区为油漆区（16英尺×19英尺）",
            "当球离开禁区时重新计时",
            "投篮后重新计时",
            "防守三秒规则：防守者不能在禁区停留超过3秒",
            "防守三秒导致一罚一掷",
        ]
    }
}

impl Default for NbaGameTimingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaGameTimingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_game_timing")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA比赛时间规则",
            &[
                ("比赛时间", &self.timing_rules()),
                ("暂停规则", &self.timeout_rules()),
                ("24秒进攻", &self.shot_clock_rules()),
                ("8秒规则", &self.eight_second_rules()),
                ("3秒规则", &self.three_second_rules()),
            ],
        )
    }
}

/// NBA球场规格规则
#[derive(Debug, Clone)]
pub struct NbaCourtRules {
    metadata: RuleMetadata,
}

impl NbaCourtRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA球场规格", "NBA球场尺寸、标记和规格")
                .with_origin("NBA官方规则")
                .with_tags(vec!["体育".into(), "篮球".into(), "NBA".into()]),
        }
    }

    /// 球场尺寸
    pub fn court_dimensions(&self) -> Vec<&'static str> {
        vec![
            "球场长度: 94英尺（28.65米）",
            "球场宽度: 50英尺（15.24米）",
            "中圈半径: 6英尺（1.83米）",
            "三分线弧顶距离: 23英尺9英寸（7.24米）",
            "三分线底角距离: 22英尺（6.70米）",
            "罚球线距离: 15英尺（4.57米）",
            "限制区宽度: 16英尺（4.88米）",
            "限制区高度: 19英尺（5.79米）",
            "篮筐高度: 10英尺（3.05米）",
            "篮筐直径: 18英寸（45.72厘米）",
        ]
    }

    /// 球场标记
    pub fn court_markings(&self) -> Vec<&'static str> {
        vec![
            "中线将球场分为两个半场",
            "中圈位于球场中央",
            "限制区（油漆区）标记为不同颜色",
            "三分线形成弧形区域",
            "罚球线与限制区相连",
            "球场边线宽2英寸",
            "禁区内有进攻有理区半圆",
        ]
    }

    /// 篮板规格
    pub fn backboard_specs(&self) -> Vec<&'static str> {
        vec![
            "篮板宽度: 6英尺（1.83米）",
            "篮板高度: 3.5英尺（1.07米）",
            "篮板厚度: 2英寸（5.08厘米）",
            "篮板材料: 透明玻璃或类似材料",
            "篮板下沿距离地面: 9英尺（2.74米）",
        ]
    }
}

impl Default for NbaCourtRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaCourtRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_court")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA球场规格",
            &[
                ("球场尺寸", &self.court_dimensions()),
                ("球场标记", &self.court_markings()),
                ("篮板规格", &self.backboard_specs()),
            ],
        )
    }
}

/// NBA犯规规则
#[derive(Debug, Clone)]
pub struct NbaFoulRules {
    metadata: RuleMetadata,
}

impl NbaFoulRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA犯规规则", "NBA犯规类型和处罚规则")
                .with_origin("NBA官方规则")
                .with_tags(vec!["体育".into(), "篮球".into(), "NBA".into()]),
        }
    }

    /// 个人犯规限制
    pub fn personal_foul_limits(&self) -> Vec<&'static str> {
        vec![
            "球员个人犯规上限: 6次",
            "达到6次个人犯规后罚出场",
            "技术犯规计入个人犯规总数",
            "恶意犯规可能导致直接驱逐",
            "两次技术犯规直接罚出场",
        ]
    }

    /// 球队犯规规则
    pub fn team_foul_rules(&self) -> Vec<&'static str> {
        vec![
            "每节球队犯规累计达到4次后进入加罚状态",
            "加罚状态下每次非投篮犯规罚球2次",
            "最后一节最后2分钟任何犯规都罚球",
            "加时赛重新计算球队犯规",
            "进攻犯规不计入球队犯规",
            "技术犯规计入球队犯规总数",
        ]
    }

    /// 犯规类型
    pub fn foul_types(&self) -> Vec<&'static str> {
        vec![
            "普通犯规: 打手、推人、阻挡等",
            "投篮犯规: 对投篮球员犯规",
            "进攻犯规: 进攻方犯规",
            "技术犯规: 不当行为、延误比赛等",
            "恶意犯规: 不必要的身体接触",
            "违反体育道德犯规: 过度身体接触",
            "双方犯规: 双方同时犯规",
            "争球犯规: 争球时的犯规",
        ]
    }

    /// 罚球规则
    pub fn free_throw_rules(&self) -> Vec<&'static str> {
        vec![
            "两分投篮犯规: 罚球2次",
            "三分投篮犯规: 罚球3次",
            "投篮命中同时犯规: 加罚1次",
            "技术犯规: 罚球1次+球权",
            "恶意犯规: 罚球2次+球权",
            "罚球时其他球员站位规则",
            "罚球线后站位球员最多5人",
        ]
    }
}

impl Default for NbaFoulRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaFoulRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_foul")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA犯规规则",
            &[
                ("个人犯规", &self.personal_foul_limits()),
                ("球队犯规", &self.team_foul_rules()),
                ("犯规类型", &self.foul_types()),
                ("罚球规则", &self.free_throw_rules()),
            ],
        )
    }
}

/// NBA选秀规则
#[derive(Debug, Clone)]
pub struct NbaDraftRules {
    metadata: RuleMetadata,
}

impl NbaDraftRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA选秀规则", "NBA选秀制度、乐透抽签、选秀权交易")
                .with_origin("NBA官方规则")
                .with_tags(vec!["体育".into(), "篮球".into(), "NBA".into()]),
        }
    }

    /// 选秀基本规则
    pub fn draft_basics(&self) -> Vec<&'static str> {
        vec![
            "选秀共两轮，每轮30个选秀权",
            "总共60个选秀权",
            "参选年龄: 至少19岁",
            "参选年限: 高中毕业后至少1年",
            "国际球员年满22岁自动获得参选资格",
            "选秀大会通常在6月下旬举行",
            "球队在选秀前需提交保护名单",
        ]
    }

    /// 乐透抽签规则
    pub fn lottery_rules(&self) -> Vec<&'static str> {
        vec![
            "未进入季后赛的14支球队参与乐透抽签",
            "战绩最差球队获得状元签概率最高（14%）",
            "战绩第二差球队获得状元签概率12.5%",
            "战绩第三差球队获得状元签概率10.5%",
            "乐透抽签决定前3顺位",
            "第4-14顺位按战绩倒序排列",
            "抽签通常在5月举行",
            "抽签过程公开透明，由NBA官方监督",
        ]
    }

    /// 选秀权交易规则
    pub fn draft_pick_trading(&self) -> Vec<&'static str> {
        vec![
            "球队可以交易未来7年的选秀权",
            "选秀权交易需符合薪资匹配规则",
            "首轮选秀权可附带保护条款",
            "保护条款示例: 前5顺位保护",
            "保护顺位不获签则推迟到下年",
            "选秀权交易需在选秀大会前完成",
            "现金不能直接用于选秀权交易",
        ]
    }

    /// 新秀合同规则
    pub fn rookie_contract_rules(&self) -> Vec<&'static str> {
        vec![
            "首轮新秀合同: 2年保障+2年球队选项",
            "首轮新秀薪资按选秀顺位确定",
            "首轮新秀合同由CBA规定金额",
            "球队需在第3年赛前执行第3年选项",
            "球队需在第4年赛前执行第4年选项",
            "第5年为受限制自由球员年",
            "次轮新秀无保障合同",
            "次轮新秀可签2年或多年合同",
        ]
    }
}

impl Default for NbaDraftRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaDraftRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_draft")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA选秀规则",
            &[
                ("选秀基本", &self.draft_basics()),
                ("乐透抽签", &self.lottery_rules()),
                ("选秀权交易", &self.draft_pick_trading()),
                ("新秀合同", &self.rookie_contract_rules()),
            ],
        )
    }
}

/// NBA工资帽和奢侈税规则
#[derive(Debug, Clone)]
pub struct NbaSalaryCapRules {
    metadata: RuleMetadata,
}

impl NbaSalaryCapRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA工资帽规则", "NBA工资帽、奢侈税、特例条款")
                .with_origin("NBA官方规则")
                .with_tags(vec!["体育".into(), "篮球".into(), "NBA".into()]),
        }
    }

    /// 工资帽基本规则
    pub fn salary_cap_basics(&self) -> Vec<&'static str> {
        vec![
            "工资帽是球队薪资总额上限",
            "2024-25赛季工资帽约1.36亿美元",
            "NBA采用软工资帽制度",
            "软工资帽允许特例条款超过上限",
            "工资帽每年根据BRI计算",
            "BRI: 篮球相关收入",
            "工资帽在每年7月开始冻结",
        ]
    }

    /// 奢侈税规则
    pub fn luxury_tax_rules(&self) -> Vec<&'static str> {
        vec![
            "奢侈税线高于工资帽",
            "2024-25赛季奢侈税线约1.65亿美元",
            "超过奢侈税线需缴纳奢侈税",
            "奢侈税采用阶梯式计算",
            "首次超税线按1.5美元/美元计算",
            "连续多年超税线税率递增",
            "奢侈税收入由未超税球队分配",
            "土豪税（超级奢侈税）针对连续超税球队",
        ]
    }

    /// 特例条款
    pub fn exceptions(&self) -> Vec<&'static str> {
        vec![
            "伯德条款: 超帽续约自己的自由球员",
            "早伯德条款: 效力2年以上球员",
            "非伯德条款: 效力1年以下球员",
            "中产阶级条款: 约900万美元特例",
            "双年特例: 约300万美元特例",
            "伤病特例: 球员赛季报销后获得",
            "交易特例: 交易中获得薪资空间",
            "底薪特例: 签约底薪球员",
        ]
    }

    /// 最低薪资规则
    pub fn minimum_salary_rules(&self) -> Vec<&'static str> {
        vec![
            "每队必须达到工资帽90%",
            "未达到部分需平均分配给球员",
            "最低薪资根据球龄确定",
            "0年球龄最低薪资约110万美元",
            "10年以上球龄最低薪资约300万美元",
            "球队需至少有14名球员",
            "每队最多15名球员（常规赛）",
        ]
    }
}

impl Default for NbaSalaryCapRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaSalaryCapRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_salary_cap")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA工资帽规则",
            &[
                ("工资帽基本", &self.salary_cap_basics()),
                ("奢侈税", &self.luxury_tax_rules()),
                ("特例条款", &self.exceptions()),
                ("最低薪资", &self.minimum_salary_rules()),
            ],
        )
    }
}

/// NBA季后赛规则
#[derive(Debug, Clone)]
pub struct NbaPlayoffRules {
    metadata: RuleMetadata,
}

impl NbaPlayoffRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA季后赛规则", "NBA季后赛赛制、排名、主场优势")
                .with_origin("NBA官方规则")
                .with_tags(vec!["体育".into(), "篮球".into(), "NBA".into()]),
        }
    }

    /// 季后赛资格
    pub fn playoff_qualification(&self) -> Vec<&'static str> {
        vec![
            "东西部各8支球队进入季后赛",
            "分区前6名直接晋级",
            "第7-10名参加附加赛",
            "附加赛: 7vs8, 9vs10",
            "7vs8胜者获得第7种子",
            "9vs10胜者vs 7vs8败者，胜者获第8种子",
            "常规赛战绩决定排名",
            "分区冠军自动进入前4",
        ]
    }

    /// 季后赛赛制
    pub fn playoff_format(&self) -> Vec<&'static str> {
        vec![
            "季后赛采用7场4胜制",
            "主场优势: 更高排名球队多一个主场",
            "主场分配: 2-2-1-1-1",
            "总决赛采用2-3-2（2014年前）",
            "总决赛现在采用2-2-1-1-1",
            "每轮系列赛独立，不重新种子",
            "东西部冠军在总决赛相遇",
        ]
    }

    /// 排名规则
    pub fn seeding_rules(&self) -> Vec<&'static str> {
        vec![
            "首先比较胜率",
            "相互交锋记录",
            "分区胜率",
            "本联盟胜率",
            "对季后赛球队胜率",
            "对季后赛球队净胜分",
            "随机抽签（最后手段）",
        ]
    }

    /// 季后赛轮次
    pub fn playoff_rounds(&self) -> Vec<&'static str> {
        vec![
            "第一轮: 16强（1vs8, 2vs7, 3vs6, 4vs5）",
            "半决赛: 8强",
            "分区决赛: 4强（东西部决赛）",
            "总决赛: 东西部冠军",
            "总决赛MVP评选",
            "总冠军获得奥布莱恩杯",
        ]
    }
}

impl Default for NbaPlayoffRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaPlayoffRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_playoff")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA季后赛规则",
            &[
                ("季后赛资格", &self.playoff_qualification()),
                ("季后赛赛制", &self.playoff_format()),
                ("排名规则", &self.seeding_rules()),
                ("季后赛轮次", &self.playoff_rounds()),
            ],
        )
    }
}

/// NBA交易规则
#[derive(Debug, Clone)]
pub struct NbaTradeRules {
    metadata: RuleMetadata,
}

impl NbaTradeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA交易规则", "NBA球员交易规则和限制")
                .with_origin("NBA官方规则")
                .with_tags(vec!["体育".into(), "篮球".into(), "NBA".into()]),
        }
    }

    /// 交易基本规则
    pub fn trade_basics(&self) -> Vec<&'static str> {
        vec![
            "交易截止日通常在2月第二周星期四",
            "交易需在截止日前完成",
            "被交易球员需48小时内报到",
            "交易后60天内不能再次交易同一球员",
            "休赛期交易无时间限制",
            "交易需经NBA官方批准",
        ]
    }

    /// 薪资匹配规则
    pub fn salary_matching(&self) -> Vec<&'static str> {
        vec![
            "交易双方薪资需匹配",
            "接收方薪资差额不超过送出方薪资125%+10万",
            "或接收方薪资差额在650万以内",
            "交易特例可用于薪资匹配",
            "签约后3个月或12月15日前不能交易",
            "夏天签约球员通常12月15日后可交易",
            "受限自由球员签约后1年不能交易",
        ]
    }

    /// 交易限制
    pub fn trade_restrictions(&self) -> Vec<&'static str> {
        vec![
            "有合同球员不能交易换现金",
            "选秀权可交易（未来7年）",
            "交易特例不能用于签约自由球员",
            "受限自由球员母队有匹配权",
            "顶薪续约后1年内有交易否决权",
            "部分球员有交易否决权",
            "部分球员合同中有交易保证金",
        ]
    }

    /// 买断规则
    pub fn buyout_rules(&self) -> Vec<&'static str> {
        vec![
            "球队可买断球员合同",
            "买断金额协商确定",
            "买断后球员成为自由球员",
            "买断窗口期有截止日期",
            "季后赛资格认定截止日",
            "买断球员季后赛前签约可参赛",
            "买断薪资计入工资帽",
        ]
    }
}

impl Default for NbaTradeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaTradeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_trade")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA交易规则",
            &[
                ("交易基本", &self.trade_basics()),
                ("薪资匹配", &self.salary_matching()),
                ("交易限制", &self.trade_restrictions()),
                ("买断规则", &self.buyout_rules()),
            ],
        )
    }
}

/// NBA综合详细规则（整合所有子规则）
pub struct NbaDetailedRules {
    metadata: RuleMetadata,
    game_timing: NbaGameTimingRules,
    court: NbaCourtRules,
    foul: NbaFoulRules,
    draft: NbaDraftRules,
    salary_cap: NbaSalaryCapRules,
    playoff: NbaPlayoffRules,
    trade: NbaTradeRules,
}

impl NbaDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("NBA详细规则", "美国职业篮球联赛完整规则体系")
                .with_origin("NBA官方规则")
                .with_tags(vec![
                    "体育".into(),
                    "篮球".into(),
                    "NBA".into(),
                    "职业联赛".into(),
                ]),
            game_timing: NbaGameTimingRules::new(),
            court: NbaCourtRules::new(),
            foul: NbaFoulRules::new(),
            draft: NbaDraftRules::new(),
            salary_cap: NbaSalaryCapRules::new(),
            playoff: NbaPlayoffRules::new(),
            trade: NbaTradeRules::new(),
        }
    }

    /// 获取所有子规则
    pub fn all_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(NbaGameTimingRules::new()),
            Box::new(NbaCourtRules::new()),
            Box::new(NbaFoulRules::new()),
            Box::new(NbaDraftRules::new()),
            Box::new(NbaSalaryCapRules::new()),
            Box::new(NbaPlayoffRules::new()),
            Box::new(NbaTradeRules::new()),
        ]
    }
}

impl Default for NbaDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NbaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nba_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        let mut explanation = String::new();
        explanation.push_str("=== NBA详细规则 ===\n\n");
        explanation.push_str(&self.game_timing.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.court.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.foul.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.draft.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.salary_cap.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.playoff.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.trade.explain());
        explanation
    }
}

// 保留旧名称以兼容现有代码
pub type BasketballNbaDetailedRules = NbaDetailedRules;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_timing_rules() {
        let rules = NbaGameTimingRules::new();
        assert!(!rules.timing_rules().is_empty());
        assert!(!rules.timeout_rules().is_empty());
        assert!(!rules.shot_clock_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_court_rules() {
        let rules = NbaCourtRules::new();
        assert!(!rules.court_dimensions().is_empty());
        assert!(!rules.court_markings().is_empty());
        assert!(!rules.backboard_specs().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_foul_rules() {
        let rules = NbaFoulRules::new();
        assert!(!rules.personal_foul_limits().is_empty());
        assert!(!rules.team_foul_rules().is_empty());
        assert!(!rules.foul_types().is_empty());
        assert!(!rules.free_throw_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_draft_rules() {
        let rules = NbaDraftRules::new();
        assert!(!rules.draft_basics().is_empty());
        assert!(!rules.lottery_rules().is_empty());
        assert!(!rules.draft_pick_trading().is_empty());
        assert!(!rules.rookie_contract_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_salary_cap_rules() {
        let rules = NbaSalaryCapRules::new();
        assert!(!rules.salary_cap_basics().is_empty());
        assert!(!rules.luxury_tax_rules().is_empty());
        assert!(!rules.exceptions().is_empty());
        assert!(!rules.minimum_salary_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_playoff_rules() {
        let rules = NbaPlayoffRules::new();
        assert!(!rules.playoff_qualification().is_empty());
        assert!(!rules.playoff_format().is_empty());
        assert!(!rules.seeding_rules().is_empty());
        assert!(!rules.playoff_rounds().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_trade_rules() {
        let rules = NbaTradeRules::new();
        assert!(!rules.trade_basics().is_empty());
        assert!(!rules.salary_matching().is_empty());
        assert!(!rules.trade_restrictions().is_empty());
        assert!(!rules.buyout_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_nba_detailed_rules() {
        let rules = NbaDetailedRules::new();
        assert!(!rules.explain().is_empty());
        let all_rules = rules.all_rules();
        assert_eq!(all_rules.len(), 7);
    }

    #[test]
    fn test_compatibility() {
        // 测试旧名称兼容性
        let rules = BasketballNbaDetailedRules::new();
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_validate() {
        let rules = NbaDetailedRules::new();
        let ctx = ValidateContext::default();
        assert!(rules.validate(&ctx).unwrap());
    }

    #[test]
    fn test_metadata() {
        let rules = NbaDetailedRules::new();
        let meta = rules.metadata();
        assert_eq!(meta.name, "NBA详细规则");
    }

    #[test]
    fn test_category() {
        let rules = NbaDetailedRules::new();
        let cat = rules.category();
        assert!(matches!(cat, RuleCategory::Sports(_)));
    }
}
