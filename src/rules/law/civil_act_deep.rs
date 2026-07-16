//! 民事法律行为深度规则
//!
//! 涵盖民事法律行为的详细判定逻辑，包括：
//! - 成立条件判定（意思表示、行为能力、标的）
//! - 生效条件判定（法定条件、约定条件）
//! - 效力状态判定（有效、无效、可撤销、效力待定）
//! - 无效与可撤销的具体情形

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilActDeepRules,
    name: "民事法律行为深度规则",
    desc: "民事法律行为的详细判定规则",
    origin: "中国",
    tags: ["法律", "民法", "民事法律行为", "法律行为效力"]
}

/// 民事法律行为成立状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActEstablishmentStatus {
    /// 成立：满足成立要件
    Established,
    /// 未成立：缺少必要要件
    NotEstablished(String),
}

/// 民事法律行为效力状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActValidityStatus {
    /// 有效：满足全部生效要件
    Valid,
    /// 无效：违反法律强制性规定或公序良俗
    Invalid(InvalidReason),
    /// 可撤销：存在撤销事由
    Revocable(RevocableReason),
    /// 效力待定：需他人追认
    Pending(PendingReason),
}

/// 无效原因
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidReason {
    /// 无民事行为能力人实施
    NoCivilCapacity,
    /// 虚假意思表示
    FalseIntent,
    /// 违反法律强制性规定
    ViolateMandatoryLaw,
    /// 违背公序良俗
    ViolatePublicOrder,
    /// 恶意串通损害他人
    MaliciousConspiracy,
}

/// 可撤销原因
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevocableReason {
    /// 重大误解
    GrossMisunderstanding,
    /// 欺诈
    Fraud,
    /// 胁迫
    Duress,
    /// 显失公平
    Unconscionability,
}

/// 效力待定原因
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingReason {
    /// 限制民事行为能力人超出能力范围
    LimitedCapacityBeyond,
    /// 无权代理
    UnauthorizedAgency,
    /// 无权处分
    UnauthorizedDisposition,
}

/// 民事行为能力等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapacityLevel {
    /// 无民事行为能力（不满8周岁或不能辨认自己行为）
    None,
    /// 限制民事行为能力（8周岁以上不满18周岁，或不能完全辨认）
    Limited,
    /// 完全民事行为能力（18周岁以上）
    Full,
}

/// 民事法律行为类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActType {
    /// 单方行为（如遗嘱、抛弃）
    Unilateral,
    /// 双方行为（如合同）
    Bilateral,
    /// 多方行为（如决议）
    Multilateral,
}

/// 效力判定输入参数
#[derive(Debug, Clone)]
pub struct ValidityCheckParams {
    /// 行为能力等级
    pub capacity: CapacityLevel,
    /// 是否为虚假意思表示
    pub is_false_intent: bool,
    /// 是否违反法律强制性规定
    pub violate_mandatory_law: bool,
    /// 是否违背公序良俗
    pub violate_public_order: bool,
    /// 是否恶意串通
    pub malicious_conspiracy: bool,
    /// 是否存在重大误解
    pub gross_misunderstanding: bool,
    /// 是否存在欺诈
    pub fraud: bool,
    /// 是否存在胁迫
    pub duress: bool,
    /// 是否显失公平
    pub unconscionability: bool,
    /// 是否为无权代理
    pub is_unauthorized_agency: bool,
    /// 是否超出限制行为能力范围
    pub act_beyond_limited_capacity: bool,
}

impl Default for ValidityCheckParams {
    fn default() -> Self {
        Self {
            capacity: CapacityLevel::Full,
            is_false_intent: false,
            violate_mandatory_law: false,
            violate_public_order: false,
            malicious_conspiracy: false,
            gross_misunderstanding: false,
            fraud: false,
            duress: false,
            unconscionability: false,
            is_unauthorized_agency: false,
            act_beyond_limited_capacity: false,
        }
    }
}

impl CivilActDeepRules {
    /// 判定民事法律行为是否成立
    ///
    /// 成立要件：
    /// 1. 当事人存在
    /// 2. 意思表示存在
    /// 3. 标的确定且可能
    pub fn check_establishment(
        &self,
        has_parties: bool,
        has_intent_expression: bool,
        has_subject: bool,
        subject_possible: bool,
    ) -> ActEstablishmentStatus {
        if !has_parties {
            return ActEstablishmentStatus::NotEstablished("缺少当事人".to_string());
        }
        if !has_intent_expression {
            return ActEstablishmentStatus::NotEstablished("缺少意思表示".to_string());
        }
        if !has_subject {
            return ActEstablishmentStatus::NotEstablished("缺少标的".to_string());
        }
        if !subject_possible {
            return ActEstablishmentStatus::NotEstablished("标的自始客观不能".to_string());
        }
        ActEstablishmentStatus::Established
    }

    /// 判定民事法律行为的效力状态
    ///
    /// 效力判定顺序：
    /// 1. 先判定是否无效
    /// 2. 再判定是否可撤销
    /// 3. 再判定是否效力待定
    /// 4. 最后判定是否有效
    pub fn check_validity(&self, params: &ValidityCheckParams) -> ActValidityStatus {
        // 1. 判定无效情形
        if params.capacity == CapacityLevel::None {
            return ActValidityStatus::Invalid(InvalidReason::NoCivilCapacity);
        }
        if params.is_false_intent {
            return ActValidityStatus::Invalid(InvalidReason::FalseIntent);
        }
        if params.violate_mandatory_law {
            return ActValidityStatus::Invalid(InvalidReason::ViolateMandatoryLaw);
        }
        if params.violate_public_order {
            return ActValidityStatus::Invalid(InvalidReason::ViolatePublicOrder);
        }
        if params.malicious_conspiracy {
            return ActValidityStatus::Invalid(InvalidReason::MaliciousConspiracy);
        }

        // 2. 判定可撤销情形
        if params.gross_misunderstanding {
            return ActValidityStatus::Revocable(RevocableReason::GrossMisunderstanding);
        }
        if params.fraud {
            return ActValidityStatus::Revocable(RevocableReason::Fraud);
        }
        if params.duress {
            return ActValidityStatus::Revocable(RevocableReason::Duress);
        }
        if params.unconscionability {
            return ActValidityStatus::Revocable(RevocableReason::Unconscionability);
        }

        // 3. 判定效力待定情形
        if params.act_beyond_limited_capacity {
            return ActValidityStatus::Pending(PendingReason::LimitedCapacityBeyond);
        }
        if params.is_unauthorized_agency {
            return ActValidityStatus::Pending(PendingReason::UnauthorizedAgency);
        }

        // 4. 满足全部要件，行为有效
        ActValidityStatus::Valid
    }

    /// 判定撤销权是否已过除斥期间
    ///
    /// 撤销权行使期限：自知道或应当知道撤销事由之日起1年
    /// 重大误解的撤销权：自知道或应当知道之日起90日
    pub fn check_revocation_period(&self, reason: &RevocableReason, days_since_known: u32) -> bool {
        match reason {
            RevocableReason::GrossMisunderstanding => days_since_known <= 90,
            _ => days_since_known <= 365,
        }
    }

    /// 判定限制民事行为能力人行为的效力
    ///
    /// 限制民事行为能力人实施的纯获利益的民事法律行为有效
    /// 限制民事行为能力人实施的与其年龄、智力相适应的民事法律行为有效
    /// 其他行为需法定代理人追认
    pub fn check_limited_capacity_act(
        &self,
        is_pure_benefit: bool,
        is_age_appropriate: bool,
    ) -> ActValidityStatus {
        if is_pure_benefit {
            return ActValidityStatus::Valid;
        }
        if is_age_appropriate {
            return ActValidityStatus::Valid;
        }
        ActValidityStatus::Pending(PendingReason::LimitedCapacityBeyond)
    }

    /// 判定无权代理行为的效力
    ///
    /// 无权代理行为未经追认的，对被代理人不发生效力
    /// 相对人有理由相信行为人有代理权的，构成表见代理，行为有效
    pub fn check_unauthorized_agency(
        &self,
        ratified: bool,
        apparent_agency: bool,
    ) -> ActValidityStatus {
        if ratified {
            return ActValidityStatus::Valid;
        }
        if apparent_agency {
            return ActValidityStatus::Valid;
        }
        ActValidityStatus::Pending(PendingReason::UnauthorizedAgency)
    }

    /// 判定附条件民事法律行为的效力
    ///
    /// 附生效条件的：条件成就时生效
    /// 附解除条件的：条件成就时失效
    pub fn check_conditional_act(
        &self,
        condition_fulfilled: bool,
        is_resolving_condition: bool,
    ) -> ActValidityStatus {
        if condition_fulfilled {
            if is_resolving_condition {
                // 附解除条件，条件成就则失效
                ActValidityStatus::Invalid(InvalidReason::ViolateMandatoryLaw) // 用一个标记表示失效
            } else {
                // 附生效条件，条件成就则生效
                ActValidityStatus::Valid
            }
        } else {
            // 条件未成就，暂时不发生效力
            ActValidityStatus::Pending(PendingReason::UnauthorizedAgency) // 待条件成就
        }
    }

    /// 民事法律行为成立规则详解
    pub fn establishment_rules(&self) -> Vec<&'static str> {
        vec![
            "成立要件一：当事人存在，民事法律行为必须有具体的当事人",
            "成立要件二：意思表示存在，当事人必须作出明确的意思表示",
            "成立要件三：标的确定，民事法律行为必须有确定的内容",
            "成立要件四：标的可能，标的必须自始可能实现",
            "意思表示方式：明示方式（书面、口头）和默示方式（行为推定）",
            "意思表示生效：无相对人的自完成时生效；有相对人的自到达时生效",
            "单方行为成立：仅由一方意思表示即可成立，如遗嘱、抛弃",
            "双方行为成立：须双方意思表示一致才能成立，如合同",
            "多方行为成立：须多方意思表示一致才能成立，如决议",
            "实践行为成立：除意思表示外还需交付标的物才能成立",
        ]
    }

    /// 民事法律行为生效规则详解
    pub fn effect_rules(&self) -> Vec<&'static str> {
        vec![
            "生效要件一：行为人具有相应的民事行为能力",
            "生效要件二：意思表示真实",
            "生效要件三：不违反法律、行政法规的强制性规定",
            "生效要件四：不违背公序良俗",
            "法定生效条件：法律规定应当办理批准、登记等手续生效的，依照其规定",
            "约定生效条件：当事人可以约定附条件或附期限",
            "附生效条件：条件成就时生效，条件未成就前不具有效力",
            "附解除条件：条件成就时失效，条件未成就前具有效力",
            "附生效期限：期限届至时生效",
            "附终止期限：期限届满时失效",
        ]
    }

    /// 无效民事法律行为规则详解
    pub fn invalid_rules(&self) -> Vec<&'static str> {
        vec![
            "无效情形一：无民事行为能力人实施的民事法律行为无效",
            "无效情形二：双方以虚假意思表示实施的民事法律行为无效",
            "无效情形三：违反法律、行政法规的强制性规定的民事法律行为无效",
            "无效情形四：违背公序良俗的民事法律行为无效",
            "无效情形五：行为人与相对人恶意串通损害他人合法权益的民事法律行为无效",
            "无效后果：民事法律行为无效后，行为人因该行为取得的财产应当返还",
            "无效后果：不能返还或没有必要返还的，应当折价补偿",
            "无效后果：有过错的一方应当赔偿对方由此所受到的损失",
            "无效后果：各方都有过错的，应当各自承担相应的责任",
            "无效性质：无效的民事法律行为自始无效、当然无效、确定无效",
        ]
    }

    /// 可撤销民事法律行为规则详解
    pub fn revocable_rules(&self) -> Vec<&'static str> {
        vec![
            "可撤销情形一：基于重大误解实施的民事法律行为",
            "可撤销情形二：一方以欺诈手段使对方在违背真实意思的情况下实施",
            "可撤销情形三：第三人实施欺诈行为使对方在违背真实意思情况下实施",
            "可撤销情形四：一方或第三人以胁迫手段使对方在违背真实意思情况下实施",
            "可撤销情形五：一方利用对方处于危困状态、缺乏判断能力等情形致使成立时显失公平",
            "撤销权主体：重大误解的撤销权由误解方行使",
            "撤销权主体：欺诈的撤销权由受欺诈方行使",
            "撤销权主体：胁迫的撤销权由受胁迫方行使",
            "撤销权主体：显失公平的撤销权由受损害方行使",
            "撤销权期间：自知道或应当知道撤销事由之日起一年内行使",
            "重大误解期间：自知道或应当知道之日起九十日内行使",
            "最长期间：自民事法律行为发生之日起五年内没有行使撤销权的，撤销权消灭",
        ]
    }

    /// 效力待定民事法律行为规则详解
    pub fn pending_rules(&self) -> Vec<&'static str> {
        vec![
            "效力待定情形一：限制民事行为能力人实施的超出其行为能力范围的民事法律行为",
            "效力待定情形二：行为人没有代理权、超越代理权或代理权终止后实施的代理行为",
            "效力待定情形三：无权处分他人财产的民事法律行为",
            "追认权主体：限制民事行为能力人的法定代理人",
            "追认权主体：无权代理行为的被代理人",
            "追认期限：相对人可以催告法定代理人或被代理人在三十日内追认",
            "沉默效果：法定代理人或被代理人未作表示的，视为拒绝追认",
            "善意相对人撤销权：在被追认前，善意相对人有权撤销",
            "纯获利益例外：限制民事行为能力人实施的纯获利益的民事法律行为有效",
            "年龄智力适应例外：限制民事行为能力人实施的与其年龄、智力相适应的民事法律行为有效",
        ]
    }

    /// 部分无效规则详解
    pub fn partial_invalid_rules(&self) -> Vec<&'static str> {
        vec![
            "部分无效原则：民事法律行为部分无效不影响其他部分效力的，其他部分仍然有效",
            "无效条款独立：无效的条款不影响合同其他条款的效力",
            "解决争议条款有效：合同无效、被撤销或终止的，不影响合同中有关解决争议方法的条款的效力",
            "结算清理条款有效：合同的权利义务关系终止不影响合同中结算和清理条款的效力",
            "格式条款无效：提供格式条款一方不合理地免除或减轻其责任、加重对方责任的条款无效",
            "格式条款无效：提供格式条款一方排除对方主要权利的条款无效",
            "免责条款无效：造成对方人身损害的免责条款无效",
            "免责条款无效：因故意或重大过失造成对方财产损失的免责条款无效",
        ]
    }

    /// 意思表示解释规则详解
    pub fn interpretation_rules(&self) -> Vec<&'static str> {
        vec![
            "解释原则：有相对人的意思表示的解释，应当按照所使用的词句结合相关条款确定",
            "解释原则：应当结合行为的性质和目的、习惯以及诚信原则确定意思表示的含义",
            "解释原则：无相对人的意思表示的解释，不能完全拘泥于所使用的词句",
            "误载不害真意：行为人表达有误但不影响真实意思的，应当以真实意思为准",
            "补充解释：当事人对意思表示的内容没有约定或约定不明确的，可以补充解释",
            "习惯解释：意思表示的内容可以参照交易习惯确定",
            "诚信解释：意思表示的解释应当遵循诚信原则",
            "不利于提供方解释：对格式条款有两种以上解释的，应当作出不利于提供方的解释",
        ]
    }
}

impl Rule for CivilActDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_act_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民事法律行为深度规则",
            &[
                ("成立规则详解", &self.establishment_rules()),
                ("生效规则详解", &self.effect_rules()),
                ("无效行为规则", &self.invalid_rules()),
                ("可撤销行为规则", &self.revocable_rules()),
                ("效力待定规则", &self.pending_rules()),
                ("部分无效规则", &self.partial_invalid_rules()),
                ("意思表示解释规则", &self.interpretation_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_act_deep_rules_creation() {
        let rules = CivilActDeepRules::new();
        assert_eq!(rules.metadata().name, "民事法律行为深度规则");
    }

    #[test]
    fn test_establishment_all_conditions_met() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_establishment(true, true, true, true);
        assert_eq!(status, ActEstablishmentStatus::Established);
    }

    #[test]
    fn test_establishment_missing_parties() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_establishment(false, true, true, true);
        assert!(matches!(status, ActEstablishmentStatus::NotEstablished(_)));
    }

    #[test]
    fn test_establishment_missing_intent() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_establishment(true, false, true, true);
        assert!(matches!(status, ActEstablishmentStatus::NotEstablished(_)));
    }

    #[test]
    fn test_establishment_subject_impossible() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_establishment(true, true, true, false);
        assert!(matches!(status, ActEstablishmentStatus::NotEstablished(_)));
    }

    #[test]
    fn test_validity_no_civil_capacity() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            capacity: CapacityLevel::None,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Invalid(InvalidReason::NoCivilCapacity)
        ));
    }

    #[test]
    fn test_validity_false_intent() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            is_false_intent: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Invalid(InvalidReason::FalseIntent)
        ));
    }

    #[test]
    fn test_validity_violate_mandatory_law() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            violate_mandatory_law: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Invalid(InvalidReason::ViolateMandatoryLaw)
        ));
    }

    #[test]
    fn test_validity_gross_misunderstanding() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            gross_misunderstanding: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Revocable(RevocableReason::GrossMisunderstanding)
        ));
    }

    #[test]
    fn test_validity_fraud() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            fraud: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Revocable(RevocableReason::Fraud)
        ));
    }

    #[test]
    fn test_validity_duress() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            duress: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Revocable(RevocableReason::Duress)
        ));
    }

    #[test]
    fn test_validity_limited_capacity_beyond() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            capacity: CapacityLevel::Limited,
            act_beyond_limited_capacity: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Pending(PendingReason::LimitedCapacityBeyond)
        ));
    }

    #[test]
    fn test_validity_valid_act() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams::default();
        let status = rules.check_validity(&params);
        assert_eq!(status, ActValidityStatus::Valid);
    }

    #[test]
    fn test_revocation_period_normal() {
        let rules = CivilActDeepRules::new();
        let result = rules.check_revocation_period(&RevocableReason::Fraud, 100);
        assert!(result);
    }

    #[test]
    fn test_revocation_period_expired() {
        let rules = CivilActDeepRules::new();
        let result = rules.check_revocation_period(&RevocableReason::Fraud, 400);
        assert!(!result);
    }

    #[test]
    fn test_revocation_period_gross_misunderstanding() {
        let rules = CivilActDeepRules::new();
        // 重大误解的撤销权期间为90日
        let result = rules.check_revocation_period(&RevocableReason::GrossMisunderstanding, 80);
        assert!(result);

        let result = rules.check_revocation_period(&RevocableReason::GrossMisunderstanding, 100);
        assert!(!result);
    }

    #[test]
    fn test_limited_capacity_pure_benefit() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_limited_capacity_act(true, false);
        assert_eq!(status, ActValidityStatus::Valid);
    }

    #[test]
    fn test_limited_capacity_age_appropriate() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_limited_capacity_act(false, true);
        assert_eq!(status, ActValidityStatus::Valid);
    }

    #[test]
    fn test_limited_capacity_beyond() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_limited_capacity_act(false, false);
        assert!(matches!(
            status,
            ActValidityStatus::Pending(PendingReason::LimitedCapacityBeyond)
        ));
    }

    #[test]
    fn test_unauthorized_agency_ratified() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_unauthorized_agency(true, false);
        assert_eq!(status, ActValidityStatus::Valid);
    }

    #[test]
    fn test_unauthorized_agency_apparent() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_unauthorized_agency(false, true);
        assert_eq!(status, ActValidityStatus::Valid);
    }

    #[test]
    fn test_unauthorized_agency_pending() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_unauthorized_agency(false, false);
        assert!(matches!(
            status,
            ActValidityStatus::Pending(PendingReason::UnauthorizedAgency)
        ));
    }

    #[test]
    fn test_conditional_act_condition_fulfilled() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_conditional_act(true, false);
        assert_eq!(status, ActValidityStatus::Valid);
    }

    #[test]
    fn test_conditional_act_condition_not_fulfilled() {
        let rules = CivilActDeepRules::new();
        let status = rules.check_conditional_act(false, false);
        // 条件未成就时，行为暂时不生效
        assert!(matches!(status, ActValidityStatus::Pending(_)));
    }

    #[test]
    fn test_all_rules_methods_not_empty() {
        let rules = CivilActDeepRules::new();
        assert!(!rules.establishment_rules().is_empty());
        assert!(!rules.effect_rules().is_empty());
        assert!(!rules.invalid_rules().is_empty());
        assert!(!rules.revocable_rules().is_empty());
        assert!(!rules.pending_rules().is_empty());
        assert!(!rules.partial_invalid_rules().is_empty());
        assert!(!rules.interpretation_rules().is_empty());
    }

    #[test]
    fn test_explain() {
        let rules = CivilActDeepRules::new();
        let explanation = rules.explain();
        assert!(!explanation.is_empty());
        assert!(explanation.contains("成立规则详解"));
        assert!(explanation.contains("生效规则详解"));
    }

    #[test]
    fn test_category() {
        let rules = CivilActDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("civil_act_deep"));
    }

    #[test]
    fn test_validity_malicious_conspiracy() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            malicious_conspiracy: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Invalid(InvalidReason::MaliciousConspiracy)
        ));
    }

    #[test]
    fn test_validity_unconscionability() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            unconscionability: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Revocable(RevocableReason::Unconscionability)
        ));
    }

    #[test]
    fn test_validity_violate_public_order() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            violate_public_order: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Invalid(InvalidReason::ViolatePublicOrder)
        ));
    }

    #[test]
    fn test_validity_unauthorized_agency_flag() {
        let rules = CivilActDeepRules::new();
        let params = ValidityCheckParams {
            is_unauthorized_agency: true,
            ..Default::default()
        };
        let status = rules.check_validity(&params);
        assert!(matches!(
            status,
            ActValidityStatus::Pending(PendingReason::UnauthorizedAgency)
        ));
    }

    #[test]
    fn test_rules_count() {
        let rules = CivilActDeepRules::new();
        assert_eq!(rules.establishment_rules().len(), 10);
        assert_eq!(rules.effect_rules().len(), 10);
        assert_eq!(rules.invalid_rules().len(), 10);
        assert_eq!(rules.revocable_rules().len(), 12);
        assert_eq!(rules.pending_rules().len(), 10);
        assert_eq!(rules.partial_invalid_rules().len(), 8);
        assert_eq!(rules.interpretation_rules().len(), 8);
    }
}
