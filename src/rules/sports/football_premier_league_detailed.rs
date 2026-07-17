//! 英超联赛详细规则
//!
//! 英格兰足球超级联赛（Premier League）完整规则体系，包括比赛规则、赛事规则、
//! 球员注册、转会规则、财务规则、VAR规则、纪律规则等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 英超比赛规则
#[derive(Debug, Clone)]
pub struct PremierLeagueMatchRules {
    metadata: RuleMetadata,
}

impl PremierLeagueMatchRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超比赛规则", "英超比赛时间、换人、替补席等规则")
                .with_origin("Premier League官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "英超".into()]),
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
            "COVID-19期间临时5换人规则已永久化",
        ]
    }

    /// 替补席规则
    pub fn bench_rules(&self) -> Vec<&'static str> {
        vec![
            "替补席最多9名球员",
            "COVID-19前替补席为7名",
            "每队可报名最多25人参赛名单",
            "U21球员不占用25人名额限制",
            "替补球员必须赛前确认",
            "门将必须有替补",
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
        ]
    }
}

impl Default for PremierLeagueMatchRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PremierLeagueMatchRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("premier_league_match")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超比赛规则",
            &[
                ("比赛时间", &self.match_timing()),
                ("换人规则", &self.substitution_rules()),
                ("替补席", &self.bench_rules()),
                ("球员装备", &self.kit_rules()),
            ],
        )
    }
}

/// 英超赛事规则
#[derive(Debug, Clone)]
pub struct PremierLeagueCompetitionRules {
    metadata: RuleMetadata,
}

impl PremierLeagueCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超赛事规则", "英超积分、排名、欧战资格、降级规则")
                .with_origin("Premier League官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "英超".into()]),
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
            "如仍相同则在 neutral venue 进行附加赛",
        ]
    }

    /// 欧战资格
    pub fn european_qualification(&self) -> Vec<&'static str> {
        vec![
            "前4名: 欧冠小组赛资格",
            "第5名: 欧联杯小组赛资格",
            "足总杯冠军: 欧联杯资格（如未通过联赛获得欧战资格）",
            "联赛杯冠军: 欧会杯资格（如未通过联赛获得欧战资格）",
            "英超可能获得第5个欧冠席位（欧战积分排名）",
            "欧战资格可能因欧战成绩获得额外席位",
        ]
    }

    /// 降级规则
    pub fn relegation_rules(&self) -> Vec<&'static str> {
        vec![
            "倒数第3名（第18名）降级至英冠",
            "倒数第2名（第19名）降级至英冠",
            "倒数第1名（第20名）降级至英冠",
            "降级球队失去英超转播分成",
            "降级球队可获得降落伞款项（parachute payments）",
            "降落伞款项持续2-3年",
            "降落伞款项帮助降级球队适应财务",
        ]
    }

    /// 赛程安排
    pub fn schedule_rules(&self) -> Vec<&'static str> {
        vec![
            "每队38场联赛比赛",
            "每队主客场各19场",
            "赛季通常从8月开始，次年5月结束",
            "冬歇期: 2周（1月）",
            "节礼日（Boxing Day）传统比赛",
            "周中比赛安排灵活",
            "因欧战、杯赛可调整赛程",
        ]
    }
}

impl Default for PremierLeagueCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PremierLeagueCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("premier_league_competition")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超赛事规则",
            &[
                ("积分系统", &self.points_system()),
                ("排名规则", &self.ranking_criteria()),
                ("欧战资格", &self.european_qualification()),
                ("降级规则", &self.relegation_rules()),
                ("赛程安排", &self.schedule_rules()),
            ],
        )
    }
}

/// 英超球员注册规则
#[derive(Debug, Clone)]
pub struct PremierLeaguePlayerRegistrationRules {
    metadata: RuleMetadata,
}

impl PremierLeaguePlayerRegistrationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超球员注册规则", "英超本土球员、外籍球员、青训球员规则")
                .with_origin("Premier League官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "英超".into()]),
        }
    }

    /// 25人名单规则
    pub fn squad_registration(&self) -> Vec<&'static str> {
        vec![
            "每队最多注册25名球员",
            "25人中至少8名本土培养球员",
            "U21球员不占用25人名额",
            "U21定义: 当年满21岁前加盟球队",
            "注册窗口: 夏季（6-8月）和冬季（1月）",
            "未注册球员不得参加英超比赛",
        ]
    }

    /// 本土培养球员规则
    pub fn homegrown_players(&self) -> Vec<&'static str> {
        vec![
            "本土培养: 21岁前在英格兰或威尔士俱乐部注册3年",
            "本土培养不等于英格兰国籍",
            "外籍球员通过青训也可成为本土培养",
            "必须至少8名本土培养球员",
            "不足8人需削减25人名单名额",
            "本土培养定义与国际足联不同",
        ]
    }

    /// 外籍球员规则
    pub fn foreign_players(&self) -> Vec<&'static str> {
        vec![
            "英超无外籍球员名额限制",
            "外籍球员需获得工作许可证（GBE）",
            "GBE: Governing Body Endorsement",
            "GBE基于国家队出场、俱乐部级别等评分",
            "顶尖球员自动获得GBE",
            "年轻球员需满足特定条件",
            "EU球员 Brexit 后也需要GBE",
        ]
    }

    /// 青训球员规则
    pub fn youth_players(&self) -> Vec<&'static str> {
        vec![
            "U18球员需签订奖学金合同",
            "U18球员不得签订职业合同",
            "青训球员转会需符合 FIFA 规则",
            "跨境转会需年满16岁（EU）或18岁（非EU）",
            "青训补偿金制度",
            "联合培养机制",
            "英格兰精英球员计划（EPPP）",
        ]
    }
}

impl Default for PremierLeaguePlayerRegistrationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PremierLeaguePlayerRegistrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("premier_league_player_registration")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超球员注册规则",
            &[
                ("25人名单", &self.squad_registration()),
                ("本土培养", &self.homegrown_players()),
                ("外籍球员", &self.foreign_players()),
                ("青训球员", &self.youth_players()),
            ],
        )
    }
}

/// 英超转会规则
#[derive(Debug, Clone)]
pub struct PremierLeagueTransferRules {
    metadata: RuleMetadata,
}

impl PremierLeagueTransferRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超转会规则", "英超转会窗口、转会费、合同规则")
                .with_origin("Premier League官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "英超".into()]),
        }
    }

    /// 转会窗口规则
    pub fn transfer_windows(&self) -> Vec<&'static str> {
        vec![
            "夏季转会窗口: 6月14日 - 8月30日",
            "冬季转会窗口: 1月1日 - 1月31日",
            "转会窗口截止日期可能调整",
            "窗口外不得注册新球员",
            "自由球员可在窗口外签约",
            "紧急门将转会可申请例外",
        ]
    }

    /// 转会费规则
    pub fn transfer_fees(&self) -> Vec<&'static str> {
        vec![
            "转会费由俱乐部协商确定",
            "转会费可分期支付",
            "转会费可能包含浮动条款",
            "签字费计入转会成本",
            "经纪人费也计入转会成本",
            "内部交易需公平定价",
            "转会费透明化要求",
        ]
    }

    /// 合同规则
    pub fn contract_rules(&self) -> Vec<&'static str> {
        vec![
            "球员合同最长5年",
            "合同可提前续约",
            "合同到期后球员成为自由球员",
            "28岁以下球员合同保护期3年",
            "28岁以上球员合同保护期2年",
            "合同买断条款受限制",
            "韦伯斯特条款: 3年后可买断转会",
        ]
    }

    /// 租借规则
    pub fn loan_rules(&self) -> Vec<&'static str> {
        vec![
            "每队最多租借入4名球员",
            "租借期最短: 两个注册窗口之间",
            "租借期最长: 一个赛季",
            "短期租借: 最长93天（紧急门将）",
            "租借球员不得在对阵母队时上场",
            "U21租借不受限制",
            "国际租借有特殊限制",
        ]
    }

    /// 青训补偿
    pub fn training_compensation(&self) -> Vec<&'static str> {
        vec![
            "青训球员转会需支付培养补偿",
            "补偿金额根据俱乐部级别确定",
            "12-15岁: 低级别补偿",
            "16-23岁: 高级别补偿",
            " solidarity payment: 转会费5%",
            "培养补偿用于青少年足球发展",
        ]
    }
}

impl Default for PremierLeagueTransferRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PremierLeagueTransferRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("premier_league_transfer")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超转会规则",
            &[
                ("转会窗口", &self.transfer_windows()),
                ("转会费", &self.transfer_fees()),
                ("合同规则", &self.contract_rules()),
                ("租借规则", &self.loan_rules()),
                ("青训补偿", &self.training_compensation()),
            ],
        )
    }
}

/// 英超财务规则
#[derive(Debug, Clone)]
pub struct PremierLeagueFinancialRules {
    metadata: RuleMetadata,
}

impl PremierLeagueFinancialRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超财务规则", "英超盈利可持续性规则、财务监管")
                .with_origin("Premier League官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "英超".into()]),
        }
    }

    /// 盈利可持续性规则（PSR）
    pub fn profitability_sustainability(&self) -> Vec<&'static str> {
        vec![
            "盈利可持续性规则（Profitability and Sustainability Rules）",
            "原名: 盈利与可持续性规则（PSSD）",
            "3年亏损上限: 1.05亿英镑",
            "2025-26赛季开始实施新规则",
            "新规则更灵活，允许合理投资",
            "财务报告必须经审计",
            "违反PSR可能面临扣分",
        ]
    }

    /// 支出控制规则
    pub fn squad_cost_control(&self) -> Vec<&'static str> {
        vec![
            "球队成本控制规则（Squad Cost Control）",
            "工资、转会费、经纪人费占收入比例限制",
            "2025-26赛季开始实施",
            "顶级俱乐部上限: 85%",
            "欧战俱乐部受欧足联规则约束",
            "违规可能导致转会限制",
        ]
    }

    /// 转播收入分配
    pub fn broadcast_revenue(&self) -> Vec<&'static str> {
        vec![
            "国内转播收入平分: 50%",
            "名次奖金: 25%",
            "转播出场费: 25%",
            "海外转播收入平分",
            "总额约每队1亿英镑/赛季",
            "降级球队获得降落伞款项",
            "降落伞款项2年分期支付",
        ]
    }

    /// 商业收入规则
    pub fn commercial_revenue(&self) -> Vec<&'static str> {
        vec![
            "商业收入不设上限",
            "赞助合同需公平交易",
            "关联方交易需公平定价",
            "商业收入计入PSR计算",
            "球衣赞助、球场冠名等",
            "比赛日收入归俱乐部所有",
        ]
    }
}

impl Default for PremierLeagueFinancialRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PremierLeagueFinancialRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("premier_league_financial")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超财务规则",
            &[
                ("盈利可持续性", &self.profitability_sustainability()),
                ("支出控制", &self.squad_cost_control()),
                ("转播收入", &self.broadcast_revenue()),
                ("商业收入", &self.commercial_revenue()),
            ],
        )
    }
}

/// 英超VAR和裁判规则
#[derive(Debug, Clone)]
pub struct PremierLeagueVARRules {
    metadata: RuleMetadata,
}

impl PremierLeagueVARRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超VAR规则", "英超VAR使用、裁判、判罚规则")
                .with_origin("Premier League官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "英超".into()]),
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
        ]
    }

    /// VAR审查流程
    pub fn var_process(&self) -> Vec<&'static str> {
        vec![
            "VAR团队位于 Stockley Park",
            "VAR团队: 1名VAR + 1名AVAR",
            "VAR实时监控所有比赛",
            "发现潜在错误时通知主裁判",
            "主裁判可进行On-Field Review",
            "审查时间尽量短",
            "审查结果通过耳机通知",
        ]
    }

    /// VAR透明度
    pub fn var_transparency(&self) -> Vec<&'static str> {
        vec![
            "VAR决定需向球迷解释",
            "2023-24赛季开始实时公告",
            "球场大屏幕显示VAR决定",
            "赛后公布VAR音频",
            "Key Match Incidents报告",
            "独立裁判小组评估",
        ]
    }

    /// 裁判规则
    pub fn refereeing_rules(&self) -> Vec<&'static str> {
        vec![
            "主裁判负责比赛管理",
            "助理裁判协助判罚越位、边线",
            "第四官员管理替补席、换人",
            "VAR团队协助关键判罚",
            "裁判需持有FIFA执照",
            "裁判体能测试每赛季进行",
            "裁判评估影响晋升降级",
        ]
    }
}

impl Default for PremierLeagueVARRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PremierLeagueVARRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("premier_league_var")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超VAR规则",
            &[
                ("VAR使用", &self.var_usage()),
                ("VAR流程", &self.var_process()),
                ("VAR透明度", &self.var_transparency()),
                ("裁判规则", &self.refereeing_rules()),
            ],
        )
    }
}

/// 英超纪律规则
#[derive(Debug, Clone)]
pub struct PremierLeagueDisciplinaryRules {
    metadata: RuleMetadata,
}

impl PremierLeagueDisciplinaryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超纪律规则", "英超红黄牌、禁赛、罚款规则")
                .with_origin("Premier League官方规则")
                .with_tags(vec!["体育".into(), "足球".into(), "英超".into()]),
        }
    }

    /// 黄牌累积规则
    pub fn yellow_card_accumulation(&self) -> Vec<&'static str> {
        vec![
            "5张黄牌: 自动禁赛1场",
            "10张黄牌: 自动禁赛2场",
            "15张黄牌: 自动禁赛3场",
            "累积计算至赛季第32轮",
            "第32轮后重新计算",
            "黄牌在杯赛中单独计算",
            "友好赛黄牌不计入累积",
        ]
    }

    /// 红牌禁赛规则
    pub fn red_card_suspension(&self) -> Vec<&'static str> {
        vec![
            "两黄变一红: 禁赛1场",
            "直接红牌: 禁赛至少1场",
            "暴力行为: 禁赛3场",
            "严重犯规: 禁赛3场",
            "辱骂裁判: 禁赛2-12场",
            "咬人、吐口水: 禁赛6-12场",
            "禁赛可上诉",
        ]
    }

    /// 赛后纪律行动
    pub fn retrospective_action(&self) -> Vec<&'static str> {
        vec![
            "赛后可追罚未被发现违规",
            "需在比赛后48小时内提出",
            "独立监管小组审议",
            "球员可因不当行为被追罚",
            "教练言论可能被追罚",
            "俱乐部可能被罚款",
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
        ]
    }
}

impl Default for PremierLeagueDisciplinaryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PremierLeagueDisciplinaryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("premier_league_disciplinary")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超纪律规则",
            &[
                ("黄牌累积", &self.yellow_card_accumulation()),
                ("红牌禁赛", &self.red_card_suspension()),
                ("赛后追罚", &self.retrospective_action()),
                ("罚款规则", &self.fines()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_rules() {
        let rules = PremierLeagueMatchRules::new();
        assert!(!rules.match_timing().is_empty());
        assert!(!rules.substitution_rules().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("match".to_string())).is_ok());
    }

    #[test]
    fn test_competition_rules() {
        let rules = PremierLeagueCompetitionRules::new();
        assert!(!rules.points_system().is_empty());
        assert!(!rules.ranking_criteria().is_empty());
        assert!(!rules.european_qualification().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("competition".to_string())).is_ok());
    }

    #[test]
    fn test_player_registration_rules() {
        let rules = PremierLeaguePlayerRegistrationRules::new();
        assert!(!rules.squad_registration().is_empty());
        assert!(!rules.homegrown_players().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("registration".to_string())).is_ok());
    }

    #[test]
    fn test_transfer_rules() {
        let rules = PremierLeagueTransferRules::new();
        assert!(!rules.transfer_windows().is_empty());
        assert!(!rules.contract_rules().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("transfer".to_string())).is_ok());
    }

    #[test]
    fn test_financial_rules() {
        let rules = PremierLeagueFinancialRules::new();
        assert!(!rules.profitability_sustainability().is_empty());
        assert!(!rules.broadcast_revenue().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("financial".to_string())).is_ok());
    }

    #[test]
    fn test_var_rules() {
        let rules = PremierLeagueVARRules::new();
        assert!(!rules.var_usage().is_empty());
        assert!(!rules.var_process().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("var".to_string())).is_ok());
    }

    #[test]
    fn test_disciplinary_rules() {
        let rules = PremierLeagueDisciplinaryRules::new();
        assert!(!rules.yellow_card_accumulation().is_empty());
        assert!(!rules.red_card_suspension().is_empty());
        assert!(rules.validate(&ValidateContext::Generic("disciplinary".to_string())).is_ok());
    }

    #[test]
    fn test_rule_categories() {
        assert!(matches!(
            PremierLeagueMatchRules::new().category(),
            RuleCategory::Sports { .. }
        ));
        assert!(matches!(
            PremierLeagueCompetitionRules::new().category(),
            RuleCategory::Sports { .. }
        ));
    }

    #[test]
    fn test_explain_methods() {
        let rules = PremierLeagueMatchRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("比赛时间"));
        assert!(explanation.contains("换人规则"));
    }
}