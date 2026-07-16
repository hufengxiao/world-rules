//! 竞争法深度规则
//!
//! 涵盖竞争法核心领域的详细内容，包括：
//! - 反垄断法深度规则
//! - 反不正当竞争法深度规则
//!
//! # 法律依据
//!
//! 主要依据：
//! - 《中华人民共和国反垄断法》（2022年修正）
//! - 《中华人民共和国反不正当竞争法》（2019年修正）
//! - 《国务院关于经营者集中申报标准的规定》
//! - 《禁止垄断协议规定》
//! - 《禁止滥用市场支配地位行为规定》

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CompetitionLawDeepRules,
    name: "竞争法深度规则",
    desc: "竞争法核心领域的详细规则解析",
    origin: "中国",
    tags: ["法律", "竞争法", "反垄断", "反不正当竞争"]
}

impl CompetitionLawDeepRules {
    /// 垄断协议认定规则
    ///
    /// 涵盖各类垄断协议的认定标准和法律后果
    pub fn monopoly_agreement_rules(&self) -> Vec<&'static str> {
        vec![
            "横向垄断协议-固定价格: 具有竞争关系的经营者达成固定或变更商品价格协议，推定具有排除、限制竞争效果",
            "横向垄断协议-限制产量: 禁止限制商品生产数量或销售数量，包括限制产量、销售量、库存量等",
            "横向垄断协议-分割市场: 禁止分割销售市场或原材料采购市场，包括划分销售区域、客户群体、产品类型",
            "横向垄断协议-限制创新: 禁止限制购买新技术、新设备或限制开发新技术、新产品",
            "横向垄断协议-联合抵制: 禁止联合抵制交易，即经营者联合起来不与特定经营者交易",
            "纵向垄断协议-转售价格维持: 禁止经营者与交易相对人达成固定向第三人转售价格或限定最低转售价格协议",
            "纵向垄断协议-限定交易: 禁止限定交易相对人仅与经营者或其指定的经营者交易",
            "轴辐协议: 经营者组织、协助其他经营者达成垄断协议，或为达成垄断协议提供实质性帮助",
            "垄断协议豁免: 能够证明协议不会严重限制相关市场竞争且能使消费者分享利益的可豁免",
            "宽大制度: 经营者主动报告垄断协议并提供重要证据的，可减轻或免除处罚",
        ]
    }

    /// 市场支配地位认定规则
    ///
    /// 依据市场份额、控制市场能力等因素综合认定
    pub fn market_dominance_determination(&self) -> Vec<&'static str> {
        vec![
            "市场份额推定: 一个经营者在相关市场市场份额达1/2，可推定具有市场支配地位",
            "市场份额推定-双寡头: 两个经营者合计市场份额达2/3，可推定具有市场支配地位",
            "市场份额推定-三寡头: 三个经营者合计市场份额达3/4，可推定具有市场支配地位",
            "市场份额不足: 市场份额不足1/10的经营者，一般不推定具有市场支配地位",
            "控制市场能力: 经营者在相关市场控制原材料采购或销售市场的能力",
            "控制价格能力: 经营者控制商品价格、数量或其他交易条件的能力",
            "财力和技术条件: 经营者的财力和技术条件对市场竞争的影响",
            "市场进入难易度: 其他经营者进入相关市场的难易程度",
            "交易依赖程度: 交易相对人对经营者的依赖程度",
            "综合认定方法: 应当依据多因素综合认定市场支配地位",
        ]
    }

    /// 滥用市场支配地位规则
    ///
    /// 具体滥用行为的认定标准和正当理由判断
    pub fn abuse_of_dominance_rules(&self) -> Vec<&'static str> {
        vec![
            "不公平高价: 以不公平的高价销售商品，无正当理由的，构成滥用市场支配地位",
            "不公平低价: 以不公平的低价购买商品，无正当理由的，构成滥用市场支配地位",
            "掠夺性定价: 以低于成本的价格销售商品，无正当理由的，构成滥用市场支配地位",
            "拒绝交易: 拒绝与交易相对人交易，无正当理由的，构成滥用市场支配地位",
            "限定交易: 限定交易相对人仅与其或其指定的经营者交易，无正当理由的构成滥用",
            "搭售商品: 违背购买者意愿搭售商品或附加不合理交易条件，构成滥用市场支配地位",
            "差别待遇: 对条件相同的交易相对人在交易条件上实行差别待遇，无正当理由的构成滥用",
            "正当理由认定: 正当理由包括满足产品安全要求、保护知识产权、特定交易相对人信用等",
            "技术创新例外: 为促进技术创新、研发新产品等目的的合理行为可构成正当理由",
            "法律责任: 滥用市场支配地位的，处以上一年度销售额1%-10%的罚款",
        ]
    }

    /// 经营者集中审查规则
    ///
    /// 集中申报、审查程序和法律效果
    pub fn business_concentration_review(&self) -> Vec<&'static str> {
        vec![
            "申报标准-营业额: 参与集中的经营者全球营业额超100亿且至少两经营者中国营业额超4亿需申报",
            "申报标准-市场份额: 参与集中的经营者中国营业额超20亿且至少两经营者超4亿需申报",
            "申报主体: 经营者集中达到申报标准的，参与集中的所有经营者均有申报义务",
            "申报材料: 应当提交集中对市场竞争影响的分析报告、集中协议、财务报告等材料",
            "初步审查: 反垄断执法机构应在30日内对申报进行初步审查，作出是否进一步审查决定",
            "进一步审查: 决定进一步审查的，应在90日内完成审查，特殊情况可延长60日",
            "禁止集中决定: 集中可能具有排除、限制竞争效果的，应当禁止集中",
            "附加条件批准: 能够证明集中对竞争产生的有利影响大于不利影响或符合公共利益的，可附加条件批准",
            "未经申报集中: 经营者集中达到申报标准但未经申报即实施的，责令停止集中并处罚款",
            "违法实施集中: 违反禁止集中决定或附加条件实施集中的，责令停止违法行为并处罚款",
        ]
    }

    /// 行政性垄断规则
    ///
    /// 行政机关滥用行政权力排除、限制竞争的规则
    pub fn administrative_monopoly_rules(&self) -> Vec<&'static str> {
        vec![
            "地区封锁-商品流通: 禁止行政机关对外地商品设定歧视性收费、价格或标准阻碍流通",
            "地区封锁-服务准入: 禁止行政机关对外地服务设定歧视性资质要求或评审标准阻碍准入",
            "行业垄断: 禁止行政机关通过设置准入壁垒、歧视性待遇等方式实施行业垄断",
            "强制交易: 禁止行政机关限定或变相限定经营者经营、购买、使用特定商品",
            "强制垄断行为: 禁止行政机关强制经营者达成垄断协议或滥用市场支配地位",
            "制定垄断政策: 猪止行政机关制定含有排除、限制竞争内容的规定",
            "公平竞争审查: 行政机关制定涉及市场主体经济活动的政策应当进行公平竞争审查",
            "行政垄断调查: 反垄断执法机构发现行政垄断的，可向有关上级机关提出处理建议",
            "法律责任: 行政垄断由上级机关责令改正，对直接责任人员依法给予处分",
            "举报权利: 任何单位和个人有权向反垄断执法机构举报行政垄断行为",
        ]
    }

    /// 反垄断执法程序规则
    ///
    /// 执法机构的调查、裁决程序
    pub fn antimonopoly_enforcement_procedure(&self) -> Vec<&'static str> {
        vec![
            "举报受理: 反垄断执法机构应当依法受理对涉嫌垄断行为的举报",
            "立案调查: 反垄断执法机构对涉嫌垄断行为可以立案调查",
            "调查措施-询问: 执法机构有权询问被调查经营者、利害关系人，要求说明情况",
            "调查措施-查阅: 执法机构有权查阅、复制被调查经营者的有关单据、协议等文件",
            "调查措施-查封: 执法机构有权查封、扣押相关证据，必要时可查询经营者银行账户",
            "调查期限: 反垄断执法机构应当自立案之日起90日内完成调查",
            "陈述申辩: 被调查经营者有权进行陈述和申辩，执法机构应当听取意见",
            "听证程序: 作出较大数额罚款决定的，应当告知当事人有要求听证的权利",
            "行政复议: 当事人对行政处罚决定不服的，可以依法申请行政复议",
            "行政诉讼: 当事人对行政复议决定不服的，可以依法提起行政诉讼",
        ]
    }

    /// 反不正当竞争-市场混淆行为
    ///
    /// 擅自使用他人有影响的标识导致市场混淆的行为
    pub fn market_confusion_rules(&self) -> Vec<&'static str> {
        vec![
            "混淆行为定义: 经营者实施混淆行为，引人误认为是他人商品或与他人存在特定联系",
            "擅自使用标识: 擅自使用与他人有一定影响的商品名称、包装、装潢等相同或近似的标识",
            "擅自使用企业名称: 擅自使用他人有一定影响的企业名称、社会组织名称、姓名",
            "擅自使用域名: 擅自使用他人有一定影响的域名主体部分、网站名称、页面设计",
            "混淆认定标准: 是否足以引人误认或误认为是他人商品或与他人存在特定联系",
            "有一定影响认定: 应当综合考虑商品销售时间、区域、销售额、广告宣传等因素",
            "法律责任-停止侵权: 责令停止违法行为，没收违法商品",
            "法律责任-罚款: 违法经营额5万元以上的，处违法经营额5倍以下罚款",
            "法律责任-小额罚款: 违法经营额不足5万元的，处25万元以下罚款",
            "民事责任: 受害人可向法院起诉要求停止侵害、赔偿损失",
        ]
    }

    /// 反不正当竞争-商业贿赂规则
    ///
    /// 商业贿赂行为的认定和法律后果
    pub fn commercial_bribery_rules(&self) -> Vec<&'static str> {
        vec![
            "商业贿赂定义: 经营者为谋取交易机会或竞争优势，采用财物或其他手段贿赂相关单位和个人",
            "受贿主体: 交易相对方的工作人员、受委托办理相关事务的单位或个人、利用影响力的人",
            "财物范围: 现金、实物、房产、汽车、有价证券等财产性利益",
            "其他手段: 旅游、娱乐、房车使用、购物卡等非财产性利益",
            "合法界限: 经营者销售或购买商品时以明示方式给对方折扣、给中间人佣金的合法",
            "入账要求: 给付折扣、佣金必须如实入账，接受折扣、佣金的经营者也必须如实入账",
            "行业惯例例外: 按照商业惯例赠送小额广告礼品的，一般不认定为商业贿赂",
            "法律责任-罚款: 处10万元以上300万元以下罚款",
            "法律责任-没收: 没收违法所得，情节严重的吊销营业执照",
            "刑事责任: 构成犯罪的，依法追究刑事责任",
        ]
    }

    /// 反不正当竞争-虚假宣传规则
    ///
    /// 虚假或引人误解的宣传行为的认定
    pub fn false_advertising_rules(&self) -> Vec<&'static str> {
        vec![
            "虚假宣传定义: 经营者对其商品的性能、功能、质量、销售状况、用户评价等作虚假或引人误解的宣传",
            "虚假内容: 商品不存在、商品性能与实际不符、使用虚构的科研成果等虚假内容",
            "引人误解: 使用歧义性语言、片面宣传、对比宣传等方式造成相关公众误解",
            "用户评价造假: 采用虚构交易、编造用户评价等方式进行虚假宣传",
            "刷单炒信: 通过组织虚假交易、虚构评价等方式提升商业信誉",
            "代言责任: 广告代言人明知或应知广告虚假仍作推荐、证明的，承担连带责任",
            "平台责任: 电商平台知道或应当知道平台内经营者虚假宣传未采取措施的，承担相应责任",
            "法律责任-罚款: 处20万元以上100万元以下罚款，情节严重的处100万元以上200万元以下罚款",
            "法律责任-吊销: 情节严重的可以吊销营业执照",
            "消费者救济: 消费者因虚假宣传购买商品的，有权要求退货退款和赔偿损失",
        ]
    }

    /// 反不正当竞争-侵犯商业秘密规则
    ///
    /// 商业秘密的保护和侵权认定
    pub fn trade_secret_infringement_rules(&self) -> Vec<&'static str> {
        vec![
            "商业秘密定义: 不为公众所知悉、具有商业价值并经权利人采取保密措施的技术或经营信息",
            "秘密性: 不为所属领域相关人员普遍知悉和容易获得",
            "价值性: 具有现实的或潜在的商业价值，能带来竞争优势",
            "保密措施: 权利人采取了与商业秘密价值相适应的合理保密措施",
            "侵权行为-盗窃: 以盗窃、贿赂、欺诈、胁迫或其他不正当手段获取权利人商业秘密",
            "侵权行为-披露: 披露、使用或允许他人使用以不正当手段获取的商业秘密",
            "侵权行为-违约: 违反保密约定或保密要求，披露、使用或允许他人使用商业秘密",
            "侵权行为-第三人: 第三人明知或应知商业秘密来源不正当仍获取、使用或披露",
            "法律责任-罚款: 处10万元以上50万元以下罚款，情节严重的处50万元以上300万元以下罚款",
            "民事赔偿: 侵犯商业秘密给权利人造成损失的，应当承担损害赔偿责任",
        ]
    }

    /// 反不正当竞争-互联网专条规则
    ///
    /// 互联网领域特有的不正当竞争行为
    pub fn internet_unfair_competition_rules(&self) -> Vec<&'static str> {
        vec![
            "流量劫持: 未经其他经营者同意，在其提供的网络产品中插入链接，强制跳转",
            "恶意不兼容: 未经同意干扰、破坏其他经营者网络产品或服务的正常运行",
            "数据爬取: 未经同意抓取其他经营者数据并实质性替代其网络产品或服务",
            "插入链接: 在其他经营者网络产品中插入链接，影响用户选择",
            "误导欺骗: 通过插入链接、强制跳转等方式，误导、欺骗用户",
            "强制跳转: 未经用户同意强制跳转，影响用户体验",
            "不兼容认定: 应当综合考虑行为正当性、对市场秩序影响、消费者权益保护等因素",
            "数据抓取例外: 为维护公共利益、保护消费者合法权益等目的的抓取可构成合理使用",
            "法律责任-罚款: 处10万元以上50万元以下罚款，情节严重的处50万元以上300万元以下罚款",
            "民事责任: 给其他经营者造成损害的，应当承担赔偿责任",
        ]
    }
}

impl Rule for CompetitionLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("competition_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "竞争法深度规则",
            &[
                ("垄断协议认定规则", &self.monopoly_agreement_rules()),
                (
                    "市场支配地位认定规则",
                    &self.market_dominance_determination(),
                ),
                ("滥用市场支配地位规则", &self.abuse_of_dominance_rules()),
                (
                    "经营者集中审查规则",
                    &self.business_concentration_review(),
                ),
                ("行政性垄断规则", &self.administrative_monopoly_rules()),
                (
                    "反垄断执法程序规则",
                    &self.antimonopoly_enforcement_procedure(),
                ),
                (
                    "反不正当竞争-市场混淆行为",
                    &self.market_confusion_rules(),
                ),
                (
                    "反不正当竞争-商业贿赂规则",
                    &self.commercial_bribery_rules(),
                ),
                (
                    "反不正当竞争-虚假宣传规则",
                    &self.false_advertising_rules(),
                ),
                (
                    "反不正当竞争-侵犯商业秘密规则",
                    &self.trade_secret_infringement_rules(),
                ),
                (
                    "反不正当竞争-互联网专条规则",
                    &self.internet_unfair_competition_rules(),
                ),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_competition_law_deep_rules() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.metadata().name, "竞争法深度规则");
        assert!(!rules.monopoly_agreement_rules().is_empty());
        assert!(!rules.market_dominance_determination().is_empty());
        assert!(!rules.abuse_of_dominance_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_monopoly_agreement_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.monopoly_agreement_rules().len(), 10);
    }

    #[test]
    fn test_market_dominance_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.market_dominance_determination().len(), 10);
    }

    #[test]
    fn test_abuse_of_dominance_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.abuse_of_dominance_rules().len(), 10);
    }

    #[test]
    fn test_business_concentration_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.business_concentration_review().len(), 10);
    }

    #[test]
    fn test_administrative_monopoly_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.administrative_monopoly_rules().len(), 10);
    }

    #[test]
    fn test_enforcement_procedure_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.antimonopoly_enforcement_procedure().len(), 10);
    }

    #[test]
    fn test_market_confusion_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.market_confusion_rules().len(), 10);
    }

    #[test]
    fn test_commercial_bribery_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.commercial_bribery_rules().len(), 10);
    }

    #[test]
    fn test_false_advertising_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.false_advertising_rules().len(), 10);
    }

    #[test]
    fn test_trade_secret_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.trade_secret_infringement_rules().len(), 10);
    }

    #[test]
    fn test_internet_rules_count() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(rules.internet_unfair_competition_rules().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CompetitionLawDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("competition_law_deep")
        );
    }

    #[test]
    fn test_validate() {
        let rules = CompetitionLawDeepRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        assert!(rules.validate(&ctx).unwrap());
    }
}