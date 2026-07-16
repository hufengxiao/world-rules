//! 证券法深度规则 - 发行、交易、信息披露

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: SecuritiesLawDeepRules,
    name: "证券法深度规则",
    desc: "证券法的详细规则解析，涵盖发行、交易、信息披露",
    origin: "中国",
    tags: ["法律", "商法", "证券法"]
}

impl SecuritiesLawDeepRules {
    /// 证券发行深度规则
    pub fn issuance_deep(&self) -> Vec<&'static str> {
        vec![
            "注册制改革: 证券发行实行注册制，由证券交易所审核、证监会注册",
            "IPO条件: 发行人应是股份有限公司，具有持续经营能力，财务会计报告无保留意见",
            "主板上市: 公司股本总额不少于5000万元，公开发行的股份达到25%以上",
            "科创板上市: 预计市值不低于10亿元，最近两年净利润为正且累计不低于5000万元",
            "创业板上市: 预计市值不低于10亿元，最近一年净利润为正",
            "北交所上市: 在全国股转系统连续挂牌满12个月的创新层挂牌公司",
            "再融资规则: 配股比例不超过10配3，增发价格不低于定价基准日前20日均价80%",
            "优先股发行: 公司已发行的优先股不超过普通股股份总数的50%，筹资金额不超过发行前净资产50%",
            "公司债券发行: 净资产不低于人民币3000万元，累计债券余额不超过净资产40%",
        ]
    }

    /// 证券交易深度规则
    pub fn trading_deep(&self) -> Vec<&'static str> {
        vec![
            "交易场所: 证券交易所(上交所、深交所、北交所)、全国股转系统",
            "交易机制: 集中竞价交易、大宗交易、盘后固定价格交易",
            "交易时间: 每周一至周五，上午9:30-11:30，下午13:00-15:00",
            "涨跌幅限制: 主板10%、科创板和创业板20%、北交所30%、ST股票5%",
            "停牌制度: 重大事项停牌、股价异常波动停牌、媒体报道停牌",
            "融资融券: 投资者账户资产不低于50万元，满6个月交易经验",
            "转融通: 证券金融公司将自有或借入资金证券出借给证券公司",
            "做空机制: 融券卖出、股指期货、期权等做空工具",
            "内幕交易禁止: 内幕信息知情人在信息公开前不得买卖证券",
            "操纵市场禁止: 禁止连续买卖、约定交易、对倒等操纵行为",
        ]
    }

    /// 信息披露深度规则
    pub fn information_disclosure_deep(&self) -> Vec<&'static str> {
        vec![
            "定期报告: 年度报告(4个月内)、中期报告(2个月内)、季度报告(1个月内)",
            "临时报告: 发生可能对股价产生较大影响的重大事件时应披露",
            "重大事件: 经营方针重大变化、重大投资、重大债务违约、重大亏损",
            "业绩预告: 预计净利润为负、扭亏为盈、实现盈利且净利润同比变动50%以上应预告",
            "业绩快报: 上市公司可以在年度报告披露前发布业绩快报",
            "公平披露: 向所有投资者公开披露，不得向特定对象单独披露",
            "网络披露: 通过证券交易所网站和证监会指定网站披露",
            "澄清公告: 媒体报道与事实不符的应在2个工作日内澄清",
            "内幕信息登记: 建立内幕信息知情人登记制度，防止内幕交易",
        ]
    }

    /// 投资者保护深度规则
    pub fn investor_protection_deep(&self) -> Vec<&'static str> {
        vec![
            "投资者适当性管理: 证券公司应对投资者进行风险等级评估，提供适当产品",
            "风险揭示: 证券公司应充分揭示投资风险，投资者应签署风险揭示书",
            "投资者教育: 证券公司应开展投资者教育，提高投资者风险意识",
            "投资者投诉处理: 证券公司应建立健全投诉处理机制，及时处理投资者投诉",
            "先行赔付: 发行人因虚假陈述致使投资者损失的，可以设立先行赔付基金",
            "证券纠纷调解: 证券业协会设立调解中心，调解证券纠纷",
            "代表人诉讼: 投资者提起虚假陈述诉讼，人民法院可以采用代表人诉讼制度",
            "特别代表人诉讼: 投资者保护机构受50名以上投资者委托，可以作为代表人参加诉讼",
        ]
    }

    /// 证券违法行为深度规则
    pub fn violations_deep(&self) -> Vec<&'static str> {
        vec![
            "虚假陈述: 信息披露文件有虚假记载、误导性陈述或重大遗漏",
            "内幕交易: 内幕信息知情人利用内幕信息买卖证券或建议他人买卖",
            "操纵市场: 单独或合谋连续买卖、约定交易、对倒等方式操纵股价",
            "老鼠仓: 基金经理利用未公开信息买卖股票",
            "短线交易: 上市公司董监高买入后6个月内卖出或卖出后6个月内买入",
            "超比例减持: 大股东减持股份达到5%应报告公告",
            "违法处罚: 没收违法所得并处罚款，严重者移送司法机关",
            "民事赔偿: 投资者因违法行为遭受损失的，可以请求民事赔偿",
        ]
    }

    /// 证券监管深度规则
    pub fn regulation_deep(&self) -> Vec<&'static str> {
        vec![
            "监管机构: 中国证监会及其派出机构、证券交易所",
            "监管职权: 现场检查、调查取证、冻结账户、行政处罚",
            "自律监管: 证券交易所对上市公司和会员进行自律监管",
            "注册审核: 证券交易所审核发行申请，证监会注册",
            "持续监管: 对上市公司、证券公司、基金公司等持续监管",
            "行政执法: 证监会调查违法行为，作出行政处罚决定",
            "行政复议: 当事人对行政处罚不服可以申请行政复议",
            "行政诉讼: 当事人对行政复议决定不服可以提起行政诉讼",
        ]
    }
}

impl Rule for SecuritiesLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("securities_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "证券法深度规则",
            &[
                ("证券发行规则", &self.issuance_deep()),
                ("证券交易规则", &self.trading_deep()),
                ("信息披露规则", &self.information_disclosure_deep()),
                ("投资者保护规则", &self.investor_protection_deep()),
                ("证券违法行为规则", &self.violations_deep()),
                ("证券监管规则", &self.regulation_deep()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_securities_law_deep_rules() {
        let rules = SecuritiesLawDeepRules::new();
        assert_eq!(rules.metadata().name, "证券法深度规则");
        assert!(!rules.issuance_deep().is_empty());
        assert!(!rules.trading_deep().is_empty());
        assert!(!rules.information_disclosure_deep().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_issuance_count() {
        let rules = SecuritiesLawDeepRules::new();
        assert_eq!(rules.issuance_deep().len(), 9);
    }

    #[test]
    fn test_trading_count() {
        let rules = SecuritiesLawDeepRules::new();
        assert_eq!(rules.trading_deep().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = SecuritiesLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("securities_law_deep"));
    }
}