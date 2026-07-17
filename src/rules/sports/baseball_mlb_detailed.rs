//! MLB详细规则
//!
//! 美国职业棒球大联盟（Major League Baseball）完整规则体系，
//! 包括比赛规则、场地规格、投球规则、击球规则、跑垒规则、选秀、薪资等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// MLB比赛时间规则
#[derive(Debug, Clone)]
pub struct MlbGameTimingRules {
    metadata: RuleMetadata,
}

impl MlbGameTimingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB比赛时间规则", "MLB局数、时间、延赛等规则")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 局数规则
    pub fn innings_rules(&self) -> Vec<&'static str> {
        vec![
            "标准比赛为9局",
            "每局分上半局和下半局",
            "上半局客队进攻，下半局主队进攻",
            "每局3个出局数交换攻守",
            "9局结束后比分高者获胜",
            "如平局进入加赛局",
        ]
    }

    /// 加赛规则
    pub fn extra_innings_rules(&self) -> Vec<&'static str> {
        vec![
            "加赛局从第10局开始",
            "2020年新规则: 每局从二垒有人开始",
            "旨在缩短比赛时间",
            "比赛持续直到分出胜负",
            "加赛局无上限",
            "季后赛加赛规则略有不同",
        ]
    }

    /// 比赛时间规则
    pub fn time_rules(&self) -> Vec<&'static str> {
        vec![
            "棒球无时间限制",
            "比赛时长通常2.5-3小时",
            "2023年引入投球计时器",
            "投球计时器: 15秒（空垒）或20秒（有人）",
            "打击者需在计时器剩余8秒时进入打击区",
            "超时可能导致自动坏球或好球",
        ]
    }

    /// 延赛和暂停规则
    pub fn delay_rules(&self) -> Vec<&'static str> {
        vec![
            "因天气可暂停比赛",
            "比赛暂停需记录局数、比分、出局数、垒上情况",
            "比赛恢复后从暂停状态继续",
            "因天气无法继续可宣布比赛有效",
            "正式比赛: 至少完成4.5局（主队领先）或5局",
            "非正式比赛需重赛",
        ]
    }
}

impl Default for MlbGameTimingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbGameTimingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_game_timing")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB比赛时间规则",
            &[
                ("局数规则", &self.innings_rules()),
                ("加赛规则", &self.extra_innings_rules()),
                ("时间规则", &self.time_rules()),
                ("延赛规则", &self.delay_rules()),
            ],
        )
    }
}

/// MLB场地规格规则
#[derive(Debug, Clone)]
pub struct MlbFieldRules {
    metadata: RuleMetadata,
}

impl MlbFieldRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB场地规格", "MLB场地尺寸、标记和规格")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 球场尺寸
    pub fn field_dimensions(&self) -> Vec<&'static str> {
        vec![
            "本垒板到外野围墙距离: 最小325英尺（两侧）",
            "本垒板到中外野距离: 最小400英尺",
            "本垒板为五角形，17英寸宽",
            "投手板距离本垒板60英尺6英寸",
            "垒包距离90英尺",
            "球场形状不规则，各球场有特色",
        ]
    }

    /// 内野规格
    pub fn infield_dimensions(&self) -> Vec<&'static str> {
        vec![
            "内野为90英尺正方形",
            "四个垒包：一垒、二垒、三垒、本垒",
            "一垒和三垒在内野边线内",
            "二垒完全在内野内",
            "本垒板位于投手板正对方向",
            "投手丘高度10英寸",
            "投手丘直径18英尺",
        ]
    }

    /// 外野规格
    pub fn outfield_dimensions(&self) -> Vec<&'static str> {
        vec![
            "外野指内野以外的区域",
            "外野围墙高度各球场不同",
            "外野边线标明界内和界外",
            "本垒打线: 飞越围墙的界内球",
            "场地本垒打（Ground rule double）: 球卡在围墙",
            "外野草地和人工草皮规则不同",
        ]
    }

    /// 场地标记
    pub fn field_markings(&self) -> Vec<&'static str> {
        vec![
            "边线: 本垒板到外野围墙的线",
            "边线延长线用于判定界内界外",
            "打击区: 本垒板两侧的矩形区域",
            "捕手区: 本垒板后方的区域",
            "教练区: 一垒和三垒边",
            "休息区: 球队席位",
        ]
    }
}

impl Default for MlbFieldRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbFieldRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_field")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB场地规格",
            &[
                ("球场尺寸", &self.field_dimensions()),
                ("内野规格", &self.infield_dimensions()),
                ("外野规格", &self.outfield_dimensions()),
                ("场地标记", &self.field_markings()),
            ],
        )
    }
}

/// MLB投球规则
#[derive(Debug, Clone)]
pub struct MlbPitchingRules {
    metadata: RuleMetadata,
}

impl MlbPitchingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB投球规则", "MLB投球动作、违规投球、投手替换")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 投球动作
    pub fn pitching_motion(&self) -> Vec<&'static str> {
        vec![
            "投手必须站在投手板上",
            "投球时必须有明确的投球动作",
            "投球前可做牵制动作",
            "投手可向垒上传球（牵制）",
            "投球动作必须符合规则",
            "违规投球可能导致判罚",
        ]
    }

    /// 投球计数
    pub fn pitch_count(&self) -> Vec<&'static str> {
        vec![
            "好球（Strike）: 通过好球区的投球",
            "坏球（Ball）: 未通过好球区的投球",
            "好球区: 打击者膝盖到腋下的区域",
            "3个好球: 三振出局",
            "4个坏球: 打击者保送一垒",
            "界外球不计入第3个好球",
        ]
    }

    /// 投手替换
    pub fn pitching_change(&self) -> Vec<&'static str> {
        vec![
            "投手替换需向裁判报告",
            "新投手必须面对至少1名打击者",
            "受伤除外可提前换下",
            "投手可多次进入比赛（非先发）",
            "牛棚投手需提前热身",
            "左投和右投配合使用",
        ]
    }

    /// 违规投球
    pub fn illegal_pitch(&self) -> Vec<&'static str> {
        vec![
            "违规投球（Balk）: 投手违规动作",
            "投手做出投球动作但未投球",
            "投手向垒上跑者传球但未传",
            "违规投球判一坏球",
            "跑者可因违规投球前进一垒",
            "违规投球是技术性判罚",
        ]
    }

    /// 投球种类
    pub fn pitch_types(&self) -> Vec<&'static str> {
        vec![
            "快速球（Fastball）: 最高速投球",
            "曲球（Curveball）: 下坠球",
            "滑球（Slider）: 横向移动球",
            "变速球（Changeup）: 慢速球",
            "卡特球（Cutter）: 内角快速球",
            "伸卡球（Sinker）: 下沉快速球",
            "指叉球（Splitter）: 急速下坠球",
        ]
    }
}

impl Default for MlbPitchingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbPitchingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_pitching")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB投球规则",
            &[
                ("投球动作", &self.pitching_motion()),
                ("投球计数", &self.pitch_count()),
                ("投手替换", &self.pitching_change()),
                ("违规投球", &self.illegal_pitch()),
                ("投球种类", &self.pitch_types()),
            ],
        )
    }
}

/// MLB击球规则
#[derive(Debug, Clone)]
pub struct MlbBattingRules {
    metadata: RuleMetadata,
}

impl MlbBattingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB击球规则", "MLB打击动作、好球坏球、本垒打")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 打击动作
    pub fn batting_motion(&self) -> Vec<&'static str> {
        vec![
            "打击者必须站在打击区内",
            "打击者可使用左右打击区",
            "打击者必须握棒",
            "打击者可挥棒或观察",
            "打击者可尝试触击（Bunt）",
            "打击者跑向一垒必须直线跑",
        ]
    }

    /// 出局方式
    pub fn out_types(&self) -> Vec<&'static str> {
        vec![
            "三振（Strikeout）: 3个好球",
            "接杀（Fly out）: 界内飞球被接住",
            "滚地出局（Ground out）: 界内滚地球传到一垒",
            "封杀（Force out）: 跑者被迫前进被触垒",
            "触杀（Tag out）: 跑者被球触身",
            "界外飞球接杀（Foul out）: 界外飞球被接住",
        ]
    }

    /// 安打类型
    pub fn hit_types(&self) -> Vec<&'static str> {
        vec![
            "一垒安打（Single）: 打者上一垒",
            "二垒安打（Double）: 打者上二垒",
            "三垒安打（Triple）: 打者上三垒",
            "本垒打（Home run）: 飞越围墙",
            "场地二安打（Ground rule double）: 球卡在围墙",
            "全垒打（Grand slam）: 满垒本垒打",
        ]
    }

    /// 保送规则
    pub fn walk_rules(&self) -> Vec<&'static str> {
        vec![
            "四坏球保送（Walk）: 打者上一垒",
            "触身球（Hit by pitch）: 打者上一垒",
            "故意四坏球（Intentional walk）: 投手主动保送",
            "保送后跑者被迫前进",
            "满垒保送直接得分",
            "保送不计入打击率",
        ]
    }

    /// 本垒打规则
    pub fn home_run_rules(&self) -> Vec<&'static str> {
        vec![
            "本垒打: 界内飞球飞越围墙",
            "打者需依次踩过所有垒包",
            "跑者可跟随打者得分",
            "场外本垒打: 球飞出球场",
            "场地本垒打: 球在场地内但无法处理",
            "本垒打是最重要得分方式",
        ]
    }
}

impl Default for MlbBattingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbBattingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_batting")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB击球规则",
            &[
                ("打击动作", &self.batting_motion()),
                ("出局方式", &self.out_types()),
                ("安打类型", &self.hit_types()),
                ("保送规则", &self.walk_rules()),
                ("本垒打", &self.home_run_rules()),
            ],
        )
    }
}

/// MLB跑垒规则
#[derive(Debug, Clone)]
pub struct MlbBaseRunningRules {
    metadata: RuleMetadata,
}

impl MlbBaseRunningRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB跑垒规则", "MLB跑垒、盗垒、得分规则")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 跑垒规则
    pub fn base_running(&self) -> Vec<&'static str> {
        vec![
            "跑者必须依次踩过垒包",
            "跑者可超越前一垒但必须返回",
            "跑者必须在球被击出后才能跑",
            "跑者可在球未被击出时盗垒",
            "跑者必须避免干扰防守",
            "跑者跑过一垒可直跑向休息区",
        ]
    }

    /// 盗垒规则
    pub fn stolen_base(&self) -> Vec<&'static str> {
        vec![
            "盗垒: 投球时跑者提前起跑",
            "盗垒成功: 跑者安全上下一垒",
            "盗垒失败: 跑者出局",
            "盗垒时机由投手动作决定",
            "捕手传球准确度影响盗垒成功",
            "盗垒是重要进攻战术",
        ]
    }

    /// 得分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "得分: 跑者踩过本垒板",
            "得分必须按顺序踩过所有垒包",
            "得分后比赛继续进行",
            "得分无效情况: 出局为第3出局",
            "飞球接杀后跑者可得分（踩垒后）",
            "最高得分: 一局15分以上罕见",
        ]
    }

    /// 跑者出局
    pub fn runner_out(&self) -> Vec<&'static str> {
        vec![
            "封杀: 跑者被迫前进被触垒",
            "触杀: 跑者被球触身",
            "飞球接杀后离垒过早",
            "干扰防守: 跑者故意干扰",
            "跑出跑垒线: 逃避触杀",
            "超跑: 跑者超越前一跑者",
        ]
    }

    /// 牵制球
    pub fn pickoff(&self) -> Vec<&'static str> {
        vec![
            "牵制球: 投手向垒上传球",
            "牵制球旨在抓盗垒跑者",
            "跑者需回到垒包",
            "跑者可尝试回垒或前进",
            "牵制球成功率较低",
            "牵制球是防守战术",
        ]
    }
}

impl Default for MlbBaseRunningRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbBaseRunningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_base_running")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB跑垒规则",
            &[
                ("跑垒规则", &self.base_running()),
                ("盗垒规则", &self.stolen_base()),
                ("得分规则", &self.scoring_rules()),
                ("跑者出局", &self.runner_out()),
                ("牵制球", &self.pickoff()),
            ],
        )
    }
}

/// MLB选秀规则
#[derive(Debug, Clone)]
pub struct MlbDraftRules {
    metadata: RuleMetadata,
}

impl MlbDraftRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB选秀规则", "MLB选秀制度、选秀顺序、签约规则")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 选秀基本规则
    pub fn draft_basics(&self) -> Vec<&'static str> {
        vec![
            "选秀共20轮",
            "2024年起缩减为20轮",
            "参选资格: 高中毕业或大学球员",
            "选秀大会通常在7月举行",
            "未签约球员可重新参选",
            "国际球员有单独签约窗口",
        ]
    }

    /// 选秀顺序
    pub fn draft_order(&self) -> Vec<&'static str> {
        vec![
            "选秀顺序按战绩倒序排列",
            "战绩最差球队获得状元签",
            "季后赛球队选秀顺位靠后",
            "补偿选秀权: 失去自由球员的球队",
            "选秀权可以交易",
            "选秀权价值基于签约奖金",
        ]
    }

    /// 签约规则
    pub fn signing_rules(&self) -> Vec<&'static str> {
        vec![
            "球队有签约奖金池限制",
            "超过签约池需缴纳奢侈税",
            "严重超签可能失去未来选秀权",
            "签约截止日期: 通常在7月中旬",
            "未签约球员可重新参选",
            "大学球员签约后放弃大学资格",
        ]
    }

    /// 新秀合同
    pub fn rookie_contract(&self) -> Vec<&'static str> {
        vec![
            "新秀合同: 通常为小联盟合同",
            "签约后分配到小联盟球队",
            "小联盟球员薪资较低",
            "表现优秀可升入大联盟",
            "大联盟首次签约为仲裁前合同",
            "仲裁后可成为自由球员",
        ]
    }

    /// 国际球员签约
    pub fn international_signing(&self) -> Vec<&'static str> {
        vec![
            "国际球员: 非美国、加拿大、波多黎各球员",
            "国际球员签约窗口: 1月-12月",
            "球队有国际签约池限制",
            "国际球员年龄限制: 至少16岁",
            "日本和韩国球员需通过入札制度",
            "古巴球员需符合美国政策",
        ]
    }
}

impl Default for MlbDraftRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbDraftRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_draft")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB选秀规则",
            &[
                ("选秀基本", &self.draft_basics()),
                ("选秀顺序", &self.draft_order()),
                ("签约规则", &self.signing_rules()),
                ("新秀合同", &self.rookie_contract()),
                ("国际签约", &self.international_signing()),
            ],
        )
    }
}

/// MLB薪资规则
#[derive(Debug, Clone)]
pub struct MlbSalaryRules {
    metadata: RuleMetadata,
}

impl MlbSalaryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB薪资规则", "MLB工资、奢侈税、自由球员规则")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 工资规则
    pub fn salary_rules(&self) -> Vec<&'static str> {
        vec![
            "MLB无硬工资帽",
            "奢侈税线: 约2.3亿美元（2024）",
            "超过奢侈税线需缴纳奢侈税",
            "奢侈税采用阶梯式计算",
            "连续超税线球队税率更高",
            "奢侈税收入分配给小球市球队",
        ]
    }

    /// 仲裁制度
    pub fn arbitration_rules(&self) -> Vec<&'static str> {
        vec![
            "仲裁资格: 大联盟3-6年经验球员",
            "仲裁前球队有续约权",
            "仲裁双方提交薪资方案",
            "仲裁委员会选择一个方案",
            "仲裁通常在2月进行",
            "仲裁后合同为一年合同",
        ]
    }

    /// 自由球员
    pub fn free_agency(&self) -> Vec<&'static str> {
        vec![
            "自由球员: 大联盟6年以上经验球员",
            "自由球员可签约任何球队",
            "原球队可提供合格报价",
            "接受合格报价: 一年合同",
            "拒绝合格报价: 签约新球队",
            "签约自由球员可能失去选秀权",
        ]
    }

    /// 合同结构
    pub fn contract_structure(&self) -> Vec<&'static str> {
        vec![
            "MLB合同通常是全额保障",
            "合同可包含激励奖金",
            "合同可包含选择权（Options）",
            "球队选择权: 球队决定是否执行",
            "球员选择权: 球员决定是否执行",
            "合同可包含不可交易条款",
        ]
    }

    /// 底薪规则
    pub fn minimum_salary(&self) -> Vec<&'static str> {
        vec![
            "大联盟底薪: 约74万美元（2024）",
            "小联盟球员薪资较低",
            "底薪每年根据集体谈判协议调整",
            "仲裁前球员薪资通常高于底薪",
            "春训期间球员薪资另计",
            "底薪是大联盟最低保障",
        ]
    }
}

impl Default for MlbSalaryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbSalaryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_salary")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB薪资规则",
            &[
                ("工资规则", &self.salary_rules()),
                ("仲裁制度", &self.arbitration_rules()),
                ("自由球员", &self.free_agency()),
                ("合同结构", &self.contract_structure()),
                ("底薪规则", &self.minimum_salary()),
            ],
        )
    }
}

/// MLB季后赛规则
#[derive(Debug, Clone)]
pub struct MlbPlayoffRules {
    metadata: RuleMetadata,
}

impl MlbPlayoffRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB季后赛规则", "MLB季后赛赛制、世界大赛规则")
                .with_origin("MLB官方规则")
                .with_tags(vec!["体育".into(), "棒球".into(), "MLB".into()]),
        }
    }

    /// 季后赛资格
    pub fn playoff_qualification(&self) -> Vec<&'static str> {
        vec![
            "每联盟6支球队进入季后赛",
            "3个分区冠军",
            "3个外卡球队（战绩最好的非分区冠军）",
            "分区冠军自动进入分区系列赛",
            "外卡球队参加外卡系列赛",
            "共12支球队进入季后赛",
        ]
    }

    /// 季后赛赛制
    pub fn playoff_format(&self) -> Vec<&'static str> {
        vec![
            "外卡系列赛: 3局2胜",
            "分区系列赛: 5局3胜",
            "联盟冠军赛: 7局4胜",
            "世界大赛: 7局4胜",
            "主场优势由战绩决定",
            "世界大赛主场优势由全明星赛结果决定",
        ]
    }

    /// 世界大赛规则
    pub fn world_series_rules(&self) -> Vec<&'static str> {
        vec![
            "世界大赛: MLB总冠军赛",
            "世界大赛采用7局4胜制",
            "主场优势由全明星赛结果决定",
            "世界大赛MVP评选",
            "世界大赛是MLB最高荣誉",
            "世界大赛历史悠久（1903年开始）",
        ]
    }

    /// 季后赛特殊规则
    pub fn playoff_special_rules(&self) -> Vec<&'static str> {
        vec![
            "季后赛名单: 26人（2022年起）",
            "投手数量限制",
            "季后赛名单需在系列赛开始前确定",
            "受伤可替换但需医疗证明",
            "季后赛无加赛局限制",
            "比赛必须分出胜负",
        ]
    }

    /// 全明星赛规则
    pub fn all_star_rules(&self) -> Vec<&'static str> {
        vec![
            "全明星赛: 7月举行",
            "球迷投票选出先发阵容",
            "教练选出替补阵容",
            "美联vs国联",
            "全明星赛决定世界大赛主场优势",
            "全明星赛MVP评选",
        ]
    }
}

impl Default for MlbPlayoffRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbPlayoffRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_playoff")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MLB季后赛规则",
            &[
                ("季后赛资格", &self.playoff_qualification()),
                ("季后赛赛制", &self.playoff_format()),
                ("世界大赛", &self.world_series_rules()),
                ("特殊规则", &self.playoff_special_rules()),
                ("全明星赛", &self.all_star_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_timing_rules() {
        let rules = MlbGameTimingRules::new();
        assert!(!rules.innings_rules().is_empty());
        assert!(!rules.extra_innings_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("timing".to_string()))
            .is_ok());
    }

    #[test]
    fn test_field_rules() {
        let rules = MlbFieldRules::new();
        assert!(!rules.field_dimensions().is_empty());
        assert!(!rules.infield_dimensions().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("field".to_string()))
            .is_ok());
    }

    #[test]
    fn test_pitching_rules() {
        let rules = MlbPitchingRules::new();
        assert!(!rules.pitching_motion().is_empty());
        assert!(!rules.pitch_count().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("pitching".to_string()))
            .is_ok());
    }

    #[test]
    fn test_batting_rules() {
        let rules = MlbBattingRules::new();
        assert!(!rules.batting_motion().is_empty());
        assert!(!rules.out_types().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("batting".to_string()))
            .is_ok());
    }

    #[test]
    fn test_base_running_rules() {
        let rules = MlbBaseRunningRules::new();
        assert!(!rules.base_running().is_empty());
        assert!(!rules.stolen_base().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("running".to_string()))
            .is_ok());
    }

    #[test]
    fn test_draft_rules() {
        let rules = MlbDraftRules::new();
        assert!(!rules.draft_basics().is_empty());
        assert!(!rules.draft_order().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("draft".to_string()))
            .is_ok());
    }

    #[test]
    fn test_salary_rules() {
        let rules = MlbSalaryRules::new();
        assert!(!rules.salary_rules().is_empty());
        assert!(!rules.arbitration_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("salary".to_string()))
            .is_ok());
    }

    #[test]
    fn test_playoff_rules() {
        let rules = MlbPlayoffRules::new();
        assert!(!rules.playoff_qualification().is_empty());
        assert!(!rules.world_series_rules().is_empty());
        assert!(rules
            .validate(&ValidateContext::Generic("playoff".to_string()))
            .is_ok());
    }

    #[test]
    fn test_rule_categories() {
        assert!(matches!(
            MlbGameTimingRules::new().category(),
            RuleCategory::Sports { .. }
        ));
        assert!(matches!(
            MlbPitchingRules::new().category(),
            RuleCategory::Sports { .. }
        ));
    }

    #[test]
    fn test_explain_methods() {
        let rules = MlbBattingRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("打击动作"));
        assert!(explanation.contains("本垒打"));
    }

    #[test]
    fn test_detailed_rules() {
        let rules = MlbDetailedRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("mlb".to_string()))
            .is_ok());
        let all_rules = rules.all_rules();
        assert_eq!(all_rules.len(), 8);
    }
}

/// MLB综合详细规则（整合所有子规则）
#[derive(Debug, Clone)]
pub struct MlbDetailedRules {
    metadata: RuleMetadata,
    game_timing: MlbGameTimingRules,
    field: MlbFieldRules,
    pitching: MlbPitchingRules,
    batting: MlbBattingRules,
    base_running: MlbBaseRunningRules,
    draft: MlbDraftRules,
    salary: MlbSalaryRules,
    playoff: MlbPlayoffRules,
}

impl MlbDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("MLB详细规则", "美国职业棒球大联盟完整规则体系")
                .with_origin("MLB官方规则")
                .with_tags(vec![
                    "体育".into(),
                    "棒球".into(),
                    "MLB".into(),
                    "职业联赛".into(),
                ]),
            game_timing: MlbGameTimingRules::new(),
            field: MlbFieldRules::new(),
            pitching: MlbPitchingRules::new(),
            batting: MlbBattingRules::new(),
            base_running: MlbBaseRunningRules::new(),
            draft: MlbDraftRules::new(),
            salary: MlbSalaryRules::new(),
            playoff: MlbPlayoffRules::new(),
        }
    }

    /// 获取所有子规则
    pub fn all_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(MlbGameTimingRules::new()),
            Box::new(MlbFieldRules::new()),
            Box::new(MlbPitchingRules::new()),
            Box::new(MlbBattingRules::new()),
            Box::new(MlbBaseRunningRules::new()),
            Box::new(MlbDraftRules::new()),
            Box::new(MlbSalaryRules::new()),
            Box::new(MlbPlayoffRules::new()),
        ]
    }
}

impl Default for MlbDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MlbDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mlb_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        let mut explanation = String::new();
        explanation.push_str("=== MLB详细规则 ===\n\n");
        explanation.push_str(&self.game_timing.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.field.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.pitching.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.batting.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.base_running.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.draft.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.salary.explain());
        explanation.push_str("\n\n");
        explanation.push_str(&self.playoff.explain());
        explanation
    }
}

// 保留旧名称以兼容现有代码
pub type BaseballMlbDetailedRules = MlbDetailedRules;
