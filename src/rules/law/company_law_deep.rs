//! 公司法深度规则 - 设立、治理、解散、清算

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CompanyLawDeepRules,
    name: "公司法深度规则",
    desc: "公司法的详细规则解析，涵盖设立、治理、解散、清算",
    origin: "中国",
    tags: ["法律", "商法", "公司法"]
}

impl CompanyLawDeepRules {
    /// 公司设立深度规则
    pub fn establishment_deep(&self) -> Vec<&'static str> {
        vec![
            "设立条件: 股东符合法定人数(有限公司50人以下，股份公司2-200发起人)",
            "公司章程: 需载明公司名称、住所、注册资本、经营范围、股东姓名等事项",
            "出资方式: 货币、实物、知识产权、土地使用权等非货币财产评估作价",
            "出资期限: 认缴出资额应在章程规定期限内缴足",
            "设立登记: 向公司登记机关申请设立登记，领取营业执照",
            "一人公司特殊要求: 一个自然人只能投资设立一个一人公司，该一人公司不能再投资设立新的一人公司",
            "国有独资公司: 国务院或地方政府授权的机构作为股东，不设股东会",
            "外商投资公司: 适用外商投资法规定，负面清单外实行准入前国民待遇",
        ]
    }

    /// 公司治理深度规则
    pub fn governance_deep(&self) -> Vec<&'static str> {
        vec![
            "股东会职权: 决定经营方针、投资计划、选举董监高、审议批准董事会监事会报告",
            "股东会召开: 定期会议按章程规定，临时会议代表10%以上表决权股东提议召开",
            "股东表决权: 按出资比例行使，章程另有规定除外",
            "董事会职权: 执行股东会决议、决定经营计划和投资方案、制定预算方案",
            "董事任期: 每届不超过三年，连选可以连任",
            "监事会职权: 检查公司财务、监督董高履职、提议召开临时股东会",
            "法定代表人: 董事长、执行董事或经理担任，章程规定",
            "高管义务: 勤勉义务、忠实义务，不得挪用资金、违规担保、同业竞争",
        ]
    }

    /// 公司解散深度规则
    pub fn dissolution_deep(&self) -> Vec<&'static str> {
        vec![
            "解散事由: 章程规定的营业期限届满、股东会决议解散、合并分立解散、依法被吊销营业执照",
            "强制解散: 公司经营管理发生严重困难，继续存续会使股东利益受到重大损失，持有10%以上表决权股东可请求法院解散",
            "解散决议: 有限责任公司经代表三分之二以上表决权的股东通过，股份有限公司经出席会议股东所持表决权三分之二以上通过",
            "清算组成立: 15日内成立清算组，有限公司股东组成，股份公司董事或股东大会确定人员",
            "清算组职权: 清理公司财产、通知公告债权人、处理未了结业务、清缴税款、清理债权债务",
            "债权人申报: 清算组应自成立之日起10日内通知债权人，60日内报纸公告，债权人45日内申报债权",
            "清算方案: 清算组制定清算方案，报股东会或法院确认",
            "清算报告: 清算结束后制作清算报告，报股东会或法院确认，申请注销登记",
        ]
    }

    /// 公司清算深度规则
    pub fn liquidation_deep(&self) -> Vec<&'static str> {
        vec![
            "清算财产: 公司全部财产，包括固定资产、流动资产、无形资产等",
            "清算费用: 清算组成员报酬、公告费用、诉讼费用、办公费用优先支付",
            "清偿顺序: 清算费用→职工工资社保→税款→普通债权",
            "剩余财产分配: 有限公司按股东出资比例分配，股份公司按股东持股比例分配",
            "未了结业务: 继续履行合同或依法解除，产生的债权债务纳入清算",
            "诉讼仲裁: 继续进行或中止审理，由清算组代表公司参加",
            "破产清算: 资产不足以清偿债务时应向法院申请破产",
            "注销登记: 清算结束后30日内申请注销登记，公告公司终止",
            "股东责任: 股东未履行清算义务导致公司财产贬值流失的，承担赔偿责任",
            "档案保管: 清算完毕后档案应当妥善保管，保管期限不得少于10年",
        ]
    }

    /// 股权转让深度规则
    pub fn equity_transfer_deep(&self) -> Vec<&'static str> {
        vec![
            "内部转让: 有限公司股东之间可以相互转让全部或部分股权",
            "对外转让: 须过半数其他股东同意，其他股东享有优先购买权",
            "通知义务: 应当将转让事项书面通知其他股东征求同意，30日内未答复视为同意",
            "优先购买权: 经股东同意转让的股权，在同等条件下其他股东有优先购买权",
            "继承取得: 自然人股东死亡后，合法继承人可以继承股东资格，章程另有规定除外",
            "股份公司股份转让: 自公司成立之日起一年内不得转让，董监高任职期间每年转让不得超过25%",
            "异议股东回购: 连续五年盈利且符合分配利润条件但不分配利润的，异议股东可请求公司回购",
            "股权出质: 股权可以出质，质权自办理出质登记时设立",
        ]
    }

    /// 公司并购深度规则
    pub fn merger_acquisition_deep(&self) -> Vec<&'static str> {
        vec![
            "吸收合并: 一个公司吸收其他公司，被吸收公司解散",
            "新设合并: 两个以上公司合并设立新公司，合并各方解散",
            "合并决议: 股东会决议，有限公司经代表三分之二以上表决权股东通过",
            "合并协议: 应当签订合并协议，编制资产负债表及财产清单",
            "通知公告: 合并决议之日起10日内通知债权人，30日内报纸公告",
            "债权人权利: 债权人可以要求公司清偿债务或提供担保",
            "存续公司责任: 存续公司承继合并前公司的债权债务",
            "分立规则: 公司分立财产作相应分割，资产负债表及财产清单",
        ]
    }
}

impl Rule for CompanyLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("company_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "公司法深度规则",
            &[
                ("公司设立规则", &self.establishment_deep()),
                ("公司治理规则", &self.governance_deep()),
                ("公司解散规则", &self.dissolution_deep()),
                ("公司清算规则", &self.liquidation_deep()),
                ("股权转让规则", &self.equity_transfer_deep()),
                ("公司并购规则", &self.merger_acquisition_deep()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_law_deep_rules() {
        let rules = CompanyLawDeepRules::new();
        assert_eq!(rules.metadata().name, "公司法深度规则");
        assert!(!rules.establishment_deep().is_empty());
        assert!(!rules.governance_deep().is_empty());
        assert!(!rules.dissolution_deep().is_empty());
        assert!(!rules.liquidation_deep().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_establishment_count() {
        let rules = CompanyLawDeepRules::new();
        assert_eq!(rules.establishment_deep().len(), 8);
    }

    #[test]
    fn test_governance_count() {
        let rules = CompanyLawDeepRules::new();
        assert_eq!(rules.governance_deep().len(), 8);
    }

    #[test]
    fn test_dissolution_count() {
        let rules = CompanyLawDeepRules::new();
        assert_eq!(rules.dissolution_deep().len(), 8);
    }

    #[test]
    fn test_liquidation_count() {
        let rules = CompanyLawDeepRules::new();
        assert_eq!(rules.liquidation_deep().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CompanyLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("company_law_deep"));
    }
}
