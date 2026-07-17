//! 产品质量法深度规则
//!
//! 涵盖产品质量核心领域的详细内容，包括：
//! - 产品质量标准与认证
//! - 产品缺陷认定
//! - 产品责任承担
//!
//! # 法律依据
//!
//! 主要依据：
//! - 《中华人民共和国产品质量法》（2018年修正）
//! - 《中华人民共和国民法典》侵权责任编
//! - 《缺陷消费品召回管理规定》
//! - 《认证认可条例》

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use serde::{Deserialize, Serialize};

/// 产品质量标准类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityStandard {
    /// 国家标准（GB）
    National,
    /// 行业标准
    Industry,
    /// 地方标准
    Local,
    /// 企业标准
    Enterprise,
    /// 国际标准（ISO、IEC等）
    International,
}

/// 产品认证类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationType {
    /// 强制性产品认证（CCC）
    Compulsory,
    /// 自愿性产品认证
    Voluntary,
    /// 质量管理体系认证（ISO 9001）
    QualityManagement,
    /// 环境管理体系认证（ISO 14001）
    EnvironmentalManagement,
    /// 食品安全管理体系认证
    FoodSafety,
}

/// 产品缺陷类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefectType {
    /// 设计缺陷
    Design,
    /// 制造缺陷
    Manufacturing,
    /// 警示缺陷
    Warning,
    /// 包装缺陷
    Packaging,
}

/// 产品责任主体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponsibleParty {
    /// 生产者
    Producer,
    /// 销售者
    Seller,
    /// 仓储者
    Warehouse,
    /// 运输者
    Transporter,
}

/// 产品质量验证参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductQualityParams {
    /// 是否有质量检验合格证明
    pub has_quality_certificate: bool,
    /// 是否有中文标识
    pub has_chinese_label: bool,
    /// 是否有生产厂名厂址
    pub has_manufacturer_info: bool,
    /// 是否有生产日期
    pub has_production_date: bool,
    /// 限期使用产品是否有有效期限
    pub has_expiry_date: bool,
    /// 是否有警示说明
    pub has_warning_label: bool,
    /// 是否符合国家标准
    pub meets_national_standard: bool,
    /// 是否存在缺陷
    pub has_defect: bool,
}

simple_rule! {
    struct: ProductQualityDeepRules,
    name: "产品质量法深度规则",
    desc: "产品质量法的详细规则解析",
    origin: "中国",
    tags: ["法律", "经济法", "产品质量"]
}

impl ProductQualityDeepRules {
    /// 验证产品标识合规性
    ///
    /// # 参数
    /// - `params`: 产品质量参数
    ///
    /// # 返回
    /// (是否合规, 不合规项列表)
    pub fn validate_product_label(
        &self,
        params: &ProductQualityParams,
    ) -> (bool, Vec<&'static str>) {
        let mut issues = Vec::new();

        if !params.has_quality_certificate {
            issues.push("缺少产品质量检验合格证明");
        }
        if !params.has_chinese_label {
            issues.push("缺少中文标识");
        }
        if !params.has_manufacturer_info {
            issues.push("缺少生产厂名厂址");
        }

        (issues.is_empty(), issues)
    }

    /// 判断产品是否存在不合理危险
    ///
    /// # 参数
    /// - `defect_type`: 缺陷类型
    /// - `has_national_standard`: 是否有国家标准
    /// - `meets_standard`: 是否符合标准
    ///
    /// # 返回
    /// 是否存在不合理危险
    pub fn has_unreasonable_danger(
        &self,
        defect_type: DefectType,
        has_national_standard: bool,
        meets_standard: bool,
    ) -> bool {
        if has_national_standard && !meets_standard {
            return true;
        }

        match defect_type {
            DefectType::Design => true,
            DefectType::Manufacturing => true,
            DefectType::Warning => true,
            DefectType::Packaging => true,
        }
    }

    /// 确定产品责任主体
    ///
    /// # 参数
    /// - `defect_origin`: 缺陷来源
    /// - `is_producer_fault`: 是否生产者责任
    /// - `is_seller_fault`: 是否销售者责任
    ///
    /// # 返回
    /// 责任主体列表
    pub fn determine_responsible_parties(
        &self,
        defect_origin: ResponsibleParty,
        is_producer_fault: bool,
        is_seller_fault: bool,
    ) -> Vec<ResponsibleParty> {
        let mut parties = Vec::new();

        if is_producer_fault {
            parties.push(ResponsibleParty::Producer);
        }
        if is_seller_fault {
            parties.push(ResponsibleParty::Seller);
        }

        if parties.is_empty() {
            parties.push(defect_origin);
        }

        parties
    }

    /// 计算产品责任赔偿
    ///
    /// # 参数
    /// - `defect_type`: 缺陷类型
    /// - `has_personal_injury`: 是否造成人身损害
    /// - `property_damage`: 财产损害金额
    ///
    /// # 返回
    /// 赔偿金额
    pub fn calculate_damages(
        &self,
        defect_type: DefectType,
        has_personal_injury: bool,
        property_damage: f64,
    ) -> f64 {
        let mut total = property_damage;

        if has_personal_injury {
            // 人身损害赔偿包括医疗费、护理费、误工费等
            total += 10000.0; // 示例基础金额
        }

        match defect_type {
            DefectType::Design => total,
            DefectType::Manufacturing => total,
            DefectType::Warning => total * 0.8, // 警示缺陷可能责任较轻
            DefectType::Packaging => total * 0.9,
        }
    }

    /// 判断是否需要召回
    ///
    /// # 参数
    /// - `defect_type`: 缺陷类型
    /// - `risk_level`: 风险等级（1-5，5最高）
    ///
    /// # 返回
    /// 是否需要召回
    pub fn requires_recall(&self, defect_type: DefectType, risk_level: u32) -> bool {
        risk_level >= 3 || matches!(defect_type, DefectType::Design | DefectType::Manufacturing)
    }

    /// 产品质量标准规则
    pub fn quality_standards_rules(&self) -> Vec<&'static str> {
        vec![
            "国家标准适用: 产品质量应当符合保障人体健康和人身财产安全的国家标准行业标准",
            "产品质量要求: 产品不存在危及人身财产安全的不合理危险",
            "产品质量检验: 产品出厂前应当经过检验合格附有产品质量检验合格证明",
            "产品标识要求: 产品或者其包装上的标识必须真实并符合法定要求",
            "中文标识: 产品标识必须有中文标明的产品名称生产厂厂名和厂址",
            "限期产品标识: 限期使用的产品应当在显著位置清晰地标明生产日期和安全使用期或失效日期",
            "危险产品警示: 使用不当可能危及人身财产安全的产品应当有警示标志或中文警示说明",
            "特殊产品标识: 易碎易燃易爆有毒有腐蚀性有放射性等危险物品应当有警示标志和中文警示说明",
            "产品质量承诺: 生产者生产的产品应当具有应当具备的使用性能但是对产品存在使用性能的瑕疵作出说明的除外",
            "产品质量担保: 销售者应当建立并执行进货检查验收制度验明产品合格证明和其他标识",
        ]
    }

    /// 产品认证规则
    pub fn certification_rules(&self) -> Vec<&'static str> {
        vec![
            "强制性产品认证: 列入目录的产品必须经国务院市场监督管理部门认证合格取得认证证书并加施认证标志",
            "CCC认证范围: 电线电缆电路开关保护装置低压电器电动工具汽车安全玻璃等",
            "认证申请: 认证申请人应当向认证机构提交书面申请并提供相关材料",
            "认证审查: 认证机构应当对申请认证的产品进行审查包括对工厂质量保证能力的检查",
            "认证证书: 认证证书应当明确认证范围和有效期",
            "认证标志: 获得认证的产品可以在产品及其包装上使用认证标志",
            "认证监督: 认证机构应当对获证产品及其生产企业进行跟踪检查",
            "认证撤销: 认证产品不再符合认证要求的认证机构应当暂停或者撤销认证证书",
            "认证责任: 认证机构应当对其认证的产品负责",
            "认证法律责任: 未取得认证擅自出厂销售进口的责令停止生产销售没收违法产品并处罚款",
        ]
    }

    /// 产品缺陷认定规则
    pub fn defect_determination_rules(&self) -> Vec<&'static str> {
        vec![
            "缺陷定义: 产品存在危及人身财产安全的不合理危险构成产品缺陷",
            "设计缺陷: 产品设计存在缺陷导致产品存在不合理危险",
            "制造缺陷: 产品制造过程中产生缺陷导致产品存在不合理危险",
            "警示缺陷: 产品警示说明不充分导致产品存在不合理危险",
            "缺陷判断标准: 产品存在不合理危险的应当认定存在缺陷",
            "国家标准优先: 产品不符合保障人体健康和人身财产安全的国家标准行业标准的认定为存在缺陷",
            "消费者合理期待: 产品不符合消费者合理期待的安全标准的可能认定为存在缺陷",
            "风险分析: 应当综合考虑产品用途使用环境消费者群体等因素判断是否存在缺陷",
            "缺陷证明: 受害人应当证明产品存在缺陷损害是由缺陷造成的",
            "缺陷抗辩: 生产者能够证明产品投入流通时缺陷尚不存在的可以免责",
        ]
    }

    /// 产品责任承担规则
    pub fn product_liability_rules(&self) -> Vec<&'static str> {
        vec![
            "生产者责任: 因产品存在缺陷造成人身缺陷产品以外的其他财产损害的生产者应当承担赔偿责任",
            "销售者责任: 销售者不能指明缺陷产品的生产者也不能指明缺陷产品的供货人的销售者应当承担赔偿责任",
            "连带责任: 消费者或者其他受害人因产品缺陷造成损害可以向生产者要求赔偿也可以向销售者要求赔偿",
            "追偿权: 属于生产者责任的销售者赔偿后有权向生产者追偿属于销售者责任的生产者赔偿后有权向销售者追偿",
            "人身损害赔偿: 造成人身伤害的应当赔偿医疗费护理费交通费等为治疗和康复支出的合理费用以及误工减少的收入",
            "残疾赔偿: 造成残疾的还应当赔偿残疾生活辅助具费和残疾赔偿金",
            "死亡赔偿: 造成死亡的还应当赔偿丧葬费和死亡赔偿金",
            "精神损害赔偿: 侵害他人人身权益造成他人精神损害的被侵权人可以请求精神损害赔偿",
           "财产损害赔偿: 侵害他人财产的应当按照损失发生时的市场价格或者其他方式计算财产损失",
            "诉讼时效: 因产品存在缺陷造成损害要求赔偿的诉讼时效期间为二年自当事人知道或者应当知道其权益受到损害时起计算",
        ]
    }

    /// 产品召回规则
    pub fn recall_rules(&self) -> Vec<&'static str> {
        vec![
            "召回义务: 生产者发现其生产的产品存在缺陷有危及人身财产安全危险的应当立即停止生产销售并及时召回",
            "召回报告: 生产者应当立即向市场监督管理部门报告并告知消费者",
            "召回公告: 生产者应当通过报刊广播电视网络等便于公众知晓的方式发布召回公告",
            "召回措施: 生产者应当对召回的产品采取修正补充修理更换退货等措施",
            "召回费用: 因产品召回产生的必要费用由生产者承担",
            "销售者义务: 销售者发现其销售的产品存在缺陷有危及人身财产安全危险的应当立即停止销售并报告",
            "召回监督: 市场监督管理部门应当对生产者召回实施情况进行监督",
            "召回评估: 市场监督管理部门可以组织专家对召回效果进行评估",
            "召回档案: 生产者应当建立召回档案保存召回记录",
            "召回法律责任: 未按规定实施召回的处以上一年度销售额百分之五以上百分之十以下罚款",
        ]
    }

    /// 产品质量监督规则
    pub fn supervision_rules(&self) -> Vec<&'static str> {
        vec![
            "监督体制: 国务院市场监督管理部门主管全国产品质量监督工作",
            "监督检查: 市场监督管理部门应当对产品质量进行监督检查",
            "抽查制度: 国家对产品质量实行以抽查为主要方式的监督检查制度",
            "抽查范围: 对可能危及人体健康和人身财产安全的产品影响国计民生的重要产品以及消费者反映有质量问题的产品进行抽查",
            "抽查程序: 监督抽查工作由市场监督管理部门规划和组织",
            "抽查检验: 检验抽取样品的数量不得超过检验的合理需要",
            "抽查结果: 监督抽查的结果应当公布",
            "质量认证: 国家根据国际通用的质量管理标准推行企业质量体系认证制度",
            "质量奖励: 国家鼓励企业采用先进的科学技术和科学管理方法提高产品质量",
            "举报制度: 任何单位和个人有权对产品质量违法行为向市场监督管理部门举报",
        ]
    }
}

impl Rule for ProductQualityDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("product_quality_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "产品质量法深度规则",
            &[
                ("质量标准规则", &self.quality_standards_rules()),
                ("产品认证规则", &self.certification_rules()),
                ("缺陷认定规则", &self.defect_determination_rules()),
                ("产品责任规则", &self.product_liability_rules()),
                ("产品召回规则", &self.recall_rules()),
                ("质量监督规则", &self.supervision_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_quality_deep_rules() {
        let rules = ProductQualityDeepRules::new();
        assert_eq!(rules.metadata().name, "产品质量法深度规则");
        assert!(!rules.quality_standards_rules().is_empty());
        assert!(!rules.certification_rules().is_empty());
        assert!(!rules.defect_determination_rules().is_empty());
        assert!(!rules.product_liability_rules().is_empty());
        assert!(!rules.recall_rules().is_empty());
        assert!(!rules.supervision_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_quality_standards_count() {
        let rules = ProductQualityDeepRules::new();
        assert_eq!(rules.quality_standards_rules().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = ProductQualityDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("product_quality_deep"));
    }

    #[test]
    fn test_validate_product_label_compliant() {
        let rules = ProductQualityDeepRules::new();
        let params = ProductQualityParams {
            has_quality_certificate: true,
            has_chinese_label: true,
            has_manufacturer_info: true,
            has_production_date: true,
            has_expiry_date: true,
            has_warning_label: true,
            meets_national_standard: true,
            has_defect: false,
        };
        let (compliant, issues) = rules.validate_product_label(&params);
        assert!(compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_product_label_non_compliant() {
        let rules = ProductQualityDeepRules::new();
        let params = ProductQualityParams {
            has_quality_certificate: false,
            has_chinese_label: false,
            has_manufacturer_info: false,
            has_production_date: true,
            has_expiry_date: true,
            has_warning_label: true,
            meets_national_standard: true,
            has_defect: false,
        };
        let (compliant, issues) = rules.validate_product_label(&params);
        assert!(!compliant);
        assert_eq!(issues.len(), 3);
    }

    #[test]
    fn test_has_unreasonable_danger_with_standard() {
        let rules = ProductQualityDeepRules::new();
        // 有国家标准但不符合
        assert!(rules.has_unreasonable_danger(DefectType::Design, true, false));
        // 有国家标准且符合
        assert!(!rules.has_unreasonable_danger(DefectType::Design, true, true));
    }

    #[test]
    fn test_determine_responsible_parties_producer() {
        let rules = ProductQualityDeepRules::new();
        let parties = rules.determine_responsible_parties(ResponsibleParty::Producer, true, false);
        assert_eq!(parties.len(), 1);
        assert_eq!(parties[0], ResponsibleParty::Producer);
    }

    #[test]
    fn test_determine_responsible_parties_both() {
        let rules = ProductQualityDeepRules::new();
        let parties = rules.determine_responsible_parties(ResponsibleParty::Producer, true, true);
        assert_eq!(parties.len(), 2);
    }

    #[test]
    fn test_calculate_damages_design() {
        let rules = ProductQualityDeepRules::new();
        let damages = rules.calculate_damages(DefectType::Design, false, 1000.0);
        assert_eq!(damages, 1000.0);
    }

    #[test]
    fn test_calculate_damages_with_injury() {
        let rules = ProductQualityDeepRules::new();
        let damages = rules.calculate_damages(DefectType::Manufacturing, true, 1000.0);
        assert!(damages > 1000.0);
    }

    #[test]
    fn test_requires_recall_high_risk() {
        let rules = ProductQualityDeepRules::new();
        // 高风险需要召回
        assert!(rules.requires_recall(DefectType::Design, 5));
    }

    #[test]
    fn test_requires_recall_low_risk() {
        let rules = ProductQualityDeepRules::new();
        // 低风险不需要召回
        assert!(!rules.requires_recall(DefectType::Packaging, 1));
    }
}
