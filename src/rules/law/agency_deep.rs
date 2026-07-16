//! 代理深度规则
//!
//! 实现代理制度的详细规则验证，包括：
//! - 有权代理规则（委托代理、法定代理、指定代理）
//! - 无权代理规则（无代理权、超越代理权、代理权终止）
//! - 表见代理规则（权利外观、合理信赖）

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use serde::{Deserialize, Serialize};

/// 代理类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgencyType {
    /// 委托代理：按照被代理人的委托行使代理权
    Entrusted,
    /// 法定代理：依照法律规定行使代理权
    Statutory,
    /// 指定代理：按照人民法院或有权机关的指定行使代理权
    Designated,
}

/// 代理权状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgencyAuthorityStatus {
    /// 有权代理：代理权有效存在
    Authorized,
    /// 无权代理：无代理权
    NoAuthority,
    /// 超越代理权
    Exceeded,
    /// 代理权终止
    Terminated,
}

/// 无权代理效力状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnauthorizedAgencyStatus {
    /// 经追认后有效
    Ratified,
    /// 构成表见代理，有效
    ApparentAgency,
    /// 未经追认，对被代理人不发生效力
    NotEffective(String),
    /// 相对人善意撤销权
    GoodFaithRevocable,
}

/// 表见代理构成要件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApparentAgencyElements {
    /// 是否存在权利外观
    pub has_authority_appearance: bool,
    /// 相对人是否善意
    pub counterparty_good_faith: bool,
    /// 相对人是否有合理信赖
    pub reasonable_reliance: bool,
    /// 被代理人是否有过错
    pub principal_fault: bool,
}

/// 代理验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyValidation {
    /// 代理类型
    pub agency_type: AgencyType,
    /// 代理权状态
    pub authority_status: AgencyAuthorityStatus,
    /// 是否有效
    pub is_valid: bool,
    /// 验证消息
    pub message: String,
}

simple_rule! {
    struct: AgencyDeepRules,
    name: "代理深度规则",
    desc: "代理制度的详细规则验证",
    origin: "中国",
    tags: ["法律", "民法", "代理"]
}

impl AgencyDeepRules {
    /// 验证有权代理
    ///
    /// # Arguments
    /// * `agency_type` - 代理类型
    /// * `within_scope` - 是否在代理权限范围内
    /// * `valid_authority` - 代理权是否有效存在
    /// * `proper_exercise` - 是否正当行使代理权
    ///
    /// # Returns
    /// 返回代理验证结果
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::agency_deep::{AgencyDeepRules, AgencyType};
    ///
    /// let rules = AgencyDeepRules::new();
    /// let result = rules.validate_authorized_agency(
    ///     AgencyType::Entrusted,
    ///     true,
    ///     true,
    ///     true,
    /// );
    /// assert!(result.is_valid);
    /// ```
    pub fn validate_authorized_agency(
        &self,
        agency_type: AgencyType,
        within_scope: bool,
        valid_authority: bool,
        proper_exercise: bool,
    ) -> AgencyValidation {
        let authority_status = if valid_authority && within_scope {
            AgencyAuthorityStatus::Authorized
        } else if !valid_authority {
            AgencyAuthorityStatus::NoAuthority
        } else {
            AgencyAuthorityStatus::Exceeded
        };

        let is_valid = valid_authority && within_scope && proper_exercise;

        let message = if is_valid {
            match agency_type {
                AgencyType::Entrusted => "委托代理有效，代理行为对被代理人发生效力".to_string(),
                AgencyType::Statutory => "法定代理有效，代理行为对被代理人发生效力".to_string(),
                AgencyType::Designated => "指定代理有效，代理行为对被代理人发生效力".to_string(),
            }
        } else if !valid_authority {
            "代理权不存在，属于无权代理".to_string()
        } else if !within_scope {
            "超越代理权限，属于无权代理".to_string()
        } else {
            "代理权行使不当，可能承担损害赔偿责任".to_string()
        };

        AgencyValidation {
            agency_type,
            authority_status,
            is_valid,
            message,
        }
    }

    /// 验证无权代理
    ///
    /// 无权代理包括：
    /// - 行为人没有代理权
    /// - 超越代理权
    /// - 代理权终止后实施代理行为
    ///
    /// # Arguments
    /// * `ratified` - 是否经被代理人追认
    /// * `apparent_agency` - 是否构成表见代理
    /// * `counterparty_good_faith` - 相对人是否善意
    ///
    /// # Returns
    /// 返回无权代理效力状态
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::agency_deep::{AgencyDeepRules, UnauthorizedAgencyStatus};
    ///
    /// let rules = AgencyDeepRules::new();
    /// // 经追认后有效
    /// let status = rules.validate_unauthorized_agency(true, false, true);
    /// assert!(matches!(status, UnauthorizedAgencyStatus::Ratified));
    ///
    /// // 构成表见代理
    /// let status = rules.validate_unauthorized_agency(false, true, true);
    /// assert!(matches!(status, UnauthorizedAgencyStatus::ApparentAgency));
    /// ```
    pub fn validate_unauthorized_agency(
        &self,
        ratified: bool,
        apparent_agency: bool,
        counterparty_good_faith: bool,
    ) -> UnauthorizedAgencyStatus {
        if ratified {
            return UnauthorizedAgencyStatus::Ratified;
        }

        if apparent_agency {
            return UnauthorizedAgencyStatus::ApparentAgency;
        }

        if counterparty_good_faith {
            return UnauthorizedAgencyStatus::GoodFaithRevocable;
        }

        UnauthorizedAgencyStatus::NotEffective(
            "未经追认的无权代理，对被代理人不发生效力，由行为人承担责任".to_string(),
        )
    }

    /// 验证表见代理
    ///
    /// 表见代理构成要件：
    /// 1. 行为人无代理权
    /// 2. 存在权利外观（被代理人行为造成）
    /// 3. 相对人善意且有合理信赖
    ///
    /// # Arguments
    /// * `elements` - 表见代理构成要件
    ///
    /// # Returns
    /// 返回是否构成表见代理及原因说明
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::agency_deep::{AgencyDeepRules, ApparentAgencyElements};
    ///
    /// let rules = AgencyDeepRules::new();
    /// let elements = ApparentAgencyElements {
    ///     has_authority_appearance: true,
    ///     counterparty_good_faith: true,
    ///     reasonable_reliance: true,
    ///     principal_fault: true,
    /// };
    /// let (is_apparent, msg) = rules.validate_apparent_agency(&elements);
    /// assert!(is_apparent);
    /// ```
    pub fn validate_apparent_agency(
        &self,
        elements: &ApparentAgencyElements,
    ) -> (bool, String) {
        if !elements.has_authority_appearance {
            return (false, "不存在权利外观，不构成表见代理".to_string());
        }

        if !elements.counterparty_good_faith {
            return (false, "相对人非善意，不构成表见代理".to_string());
        }

        if !elements.reasonable_reliance {
            return (false, "相对人无合理信赖，不构成表见代理".to_string());
        }

        if !elements.principal_fault {
            return (false, "被代理人无过错，不构成表见代理".to_string());
        }

        (
            true,
            "构成表见代理，代理行为有效，对被代理人发生效力".to_string(),
        )
    }

    /// 检查追认期限
    ///
    /// 相对人可以催告被代理人在30日内追认
    /// 被代理人未作表示的，视为拒绝追认
    ///
    /// # Arguments
    /// * `days` - 经过天数
    /// * `has_response` - 被代理人是否作出表示
    ///
    /// # Returns
    /// 返回追认状态
    pub fn check_ratification_period(&self, days: u32, has_response: bool) -> (bool, String) {
        if has_response {
            (true, "被代理人已作出追认或拒绝表示".to_string())
        } else if days > 30 {
            (false, "超过30日追认期限，视为拒绝追认".to_string())
        } else {
            (
                true,
                format!("追认期限内（剩余{}日），等待被代理人追认", 30 - days),
            )
        }
    }

    /// 获取代理类型说明
    ///
    /// # Arguments
    /// * `agency_type` - 代理类型
    ///
    /// # Returns
    /// 返回代理类型的详细说明
    pub fn describe_agency_type(&self, agency_type: AgencyType) -> Vec<String> {
        match agency_type {
            AgencyType::Entrusted => vec![
                "委托代理：按照被代理人的委托行使代理权".to_string(),
                "委托代理可以采用书面形式或口头形式".to_string(),
                "书面委托代理应载明代理人的姓名或名称、代理事项、权限和期间".to_string(),
                "委托代理可以是有偿的，也可以是无偿的".to_string(),
                "委托人可以随时解除委托，但应赔偿因此造成的损失".to_string(),
            ],
            AgencyType::Statutory => vec![
                "法定代理：依照法律规定行使代理权".to_string(),
                "父母是未成年子女的法定代理人".to_string(),
                "配偶、父母、成年子女可以是丧失行为能力成年人的法定代理人".to_string(),
                "法定代理人可以委托他人代理，但应经被代理人同意".to_string(),
                "法定代理权的消灭：被代理人取得或恢复完全民事行为能力".to_string(),
            ],
            AgencyType::Designated => vec![
                "指定代理：按照人民法院或有权机关的指定行使代理权".to_string(),
                "指定代理适用于监护人争议无法确定的情形".to_string(),
                "指定代理人对被代理人负责，不得损害被代理人利益".to_string(),
                "指定代理权的消灭：被代理人取得或恢复完全民事行为能力".to_string(),
                "指定代理权的消灭：指定机关取消指定".to_string(),
            ],
        }
    }

    /// 获取代理终止情形
    ///
    /// # Arguments
    /// * `agency_type` - 代理类型
    ///
    /// # Returns
    /// 返回代理终止情形列表
    pub fn get_agency_termination_cases(&self, agency_type: AgencyType) -> Vec<String> {
        let common_cases = vec![
            "代理期间届满或代理事务完成".to_string(),
            "被代理人取消委托或代理人辞去委托".to_string(),
            "代理人丧失民事行为能力".to_string(),
            "代理人或被代理人死亡".to_string(),
            "作为代理人或被代理人的法人终止".to_string(),
        ];

        match agency_type {
            AgencyType::Entrusted => {
                let mut cases = common_cases;
                cases.push("委托代理可以随时解除".to_string());
                cases
            }
            AgencyType::Statutory => vec![
                "被代理人取得或恢复完全民事行为能力".to_string(),
                "代理人丧失民事行为能力".to_string(),
                "代理人或被代理人死亡".to_string(),
                "指定监护人或代理人的机关取消指定".to_string(),
            ],
            AgencyType::Designated => vec![
                "被代理人取得或恢复完全民事行为能力".to_string(),
                "指定机关取消指定".to_string(),
                "代理人丧失民事行为能力".to_string(),
                "代理人或被代理人死亡".to_string(),
            ],
        }
    }

    /// 获取有权代理规则详解
    pub fn authorized_agency_rules(&self) -> Vec<&'static str> {
        vec![
            "有权代理条件：代理权有效存在；在代理权限内行使；以被代理人名义实施",
            "委托代理设立：被代理人授权行为；授权应明确代理事项、权限和期间",
            "委托代理形式：书面形式优先，可附授权委托书；口头形式也可成立",
            "法定代理依据：法律规定直接产生，如父母对未成年子女的代理权",
            "法定代理范围：代理被代理人实施民事法律行为，保护被代理人利益",
            "指定代理情形：监护人有争议时，由人民法院或有权机关指定",
            "共同代理：数人为同一事项代理人的，应当共同行使代理权",
            "转代理：需经被代理人同意或追认，紧急情况下可为保护被代理人利益转代理",
            "代理行使限制：不得以被代理人名义与自己实施民事法律行为",
            "代理行使限制：不得以被代理人名义与自己同时代理的其他人实施民事法律行为",
        ]
    }

    /// 获取无权代理规则详解
    pub fn unauthorized_agency_rules(&self) -> Vec<&'static str> {
        vec![
            "无权代理情形一：行为人没有代理权而以他人名义实施代理行为",
            "无权代理情形二：行为人超越代理权范围实施代理行为",
            "无权代理情形三：代理权终止后，行为人仍以被代理人名义实施代理行为",
            "无权代理效力：未经被代理人追认的，对被代理人不发生效力",
            "追认权：被代理人有权追认无权代理行为，追认后行为有效",
            "追认期限：相对人可催告被代理人在30日内追认",
            "沉默效果：被代理人未作表示的，视为拒绝追认",
            "善意相对人撤销权：追认前，善意相对人有权撤销",
            "恶意相对人：相对人知道或应当知道行为人无代理权的，不得撤销",
            "无权代理人责任：行为人应对善意相对人承担履行义务或赔偿责任",
            "无权代理人例外：相对人知道或应当知道行为人无代理权的，双方按过错承担责任",
        ]
    }

    /// 获取表见代理规则详解
    pub fn apparent_agency_rules(&self) -> Vec<&'static str> {
        vec![
            "表见代理定义：行为人无代理权，但相对人有理由相信其有代理权的，代理行为有效",
            "表见代理要件一：行为人无代理权",
            "表见代理要件二：存在代理权外观，如持有盖章的空白合同书",
            "表见代理要件三：相对人善意且有合理信赖",
            "表见代理要件四：被代理人有过错，造成了权利外观",
            "表见代理后果：代理行为有效，对被代理人发生效力",
            "典型情形一：被代理人将证明文件交给他人，他人以此证明代理身份",
            "典型情形二：代理人超越代理权，但代理证书未载明限制",
            "典型情形三：代理关系终止后，被代理人未收回代理证书",
            "典型情形四：被代理人知道他人以其名义活动而不否认",
            "被代理人追偿：被代理人承担表见代理后果后，可向行为人追偿",
        ]
    }

    /// 获取代理责任规则详解
    pub fn agency_liability_rules(&self) -> Vec<&'static str> {
        vec![
            "代理人责任：代理人不履行职责造成被代理人损害的，应承担民事责任",
            "代理人与第三人串通：损害被代理人利益的，代理人和第三人负连带责任",
            "违法代理：代理人知道或应当知道代理事项违法仍然代理的，与被代理人负连带责任",
            "无权代理责任：未经追认的无权代理，由行为人承担民事责任",
            "善意相对人保护：善意相对人有权请求行为人履行债务或赔偿损失",
            "恶意相对人责任：相对人明知无代理权而与之实施行为的，按过错承担责任",
            "代理人与被代理人的连带责任：代理人知道代理事项违法仍代理的",
            "代理人与第三人的连带责任：代理人与第三人恶意串通损害被代理人的",
        ]
    }

    /// 获取代理深度规则列表
    pub fn get_deep_rules(&self) -> Vec<&'static str> {
        let mut rules = Vec::new();
        rules.extend(self.authorized_agency_rules().iter().copied());
        rules.extend(self.unauthorized_agency_rules().iter().copied());
        rules.extend(self.apparent_agency_rules().iter().copied());
        rules
    }
}

impl Rule for AgencyDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("agency_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "代理深度规则",
            &[
                ("有权代理规则", &self.authorized_agency_rules()),
                ("无权代理规则", &self.unauthorized_agency_rules()),
                ("表见代理规则", &self.apparent_agency_rules()),
                ("代理责任规则", &self.agency_liability_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agency_deep_rules_creation() {
        let rules = AgencyDeepRules::new();
        assert_eq!(rules.metadata().name, "代理深度规则");
    }

    #[test]
    fn test_validate_authorized_agency_valid() {
        let rules = AgencyDeepRules::new();
        let result = rules.validate_authorized_agency(
            AgencyType::Entrusted,
            true,
            true,
            true,
        );
        assert!(result.is_valid);
        assert_eq!(result.authority_status, AgencyAuthorityStatus::Authorized);
    }

    #[test]
    fn test_validate_authorized_agency_no_authority() {
        let rules = AgencyDeepRules::new();
        let result = rules.validate_authorized_agency(
            AgencyType::Entrusted,
            true,
            false,
            true,
        );
        assert!(!result.is_valid);
        assert_eq!(result.authority_status, AgencyAuthorityStatus::NoAuthority);
    }

    #[test]
    fn test_validate_authorized_agency_exceeded() {
        let rules = AgencyDeepRules::new();
        let result = rules.validate_authorized_agency(
            AgencyType::Entrusted,
            false,
            true,
            true,
        );
        assert!(!result.is_valid);
        assert_eq!(result.authority_status, AgencyAuthorityStatus::Exceeded);
    }

    #[test]
    fn test_validate_unauthorized_agency_ratified() {
        let rules = AgencyDeepRules::new();
        let status = rules.validate_unauthorized_agency(true, false, true);
        assert!(matches!(status, UnauthorizedAgencyStatus::Ratified));
    }

    #[test]
    fn test_validate_unauthorized_agency_apparent() {
        let rules = AgencyDeepRules::new();
        let status = rules.validate_unauthorized_agency(false, true, true);
        assert!(matches!(status, UnauthorizedAgencyStatus::ApparentAgency));
    }

    #[test]
    fn test_validate_unauthorized_agency_good_faith() {
        let rules = AgencyDeepRules::new();
        let status = rules.validate_unauthorized_agency(false, false, true);
        assert!(matches!(status, UnauthorizedAgencyStatus::GoodFaithRevocable));
    }

    #[test]
    fn test_validate_unauthorized_agency_not_effective() {
        let rules = AgencyDeepRules::new();
        let status = rules.validate_unauthorized_agency(false, false, false);
        assert!(matches!(status, UnauthorizedAgencyStatus::NotEffective(_)));
    }

    #[test]
    fn test_validate_apparent_agency_success() {
        let rules = AgencyDeepRules::new();
        let elements = ApparentAgencyElements {
            has_authority_appearance: true,
            counterparty_good_faith: true,
            reasonable_reliance: true,
            principal_fault: true,
        };
        let (is_apparent, msg) = rules.validate_apparent_agency(&elements);
        assert!(is_apparent);
        assert!(msg.contains("构成表见代理"));
    }

    #[test]
    fn test_validate_apparent_agency_no_appearance() {
        let rules = AgencyDeepRules::new();
        let elements = ApparentAgencyElements {
            has_authority_appearance: false,
            counterparty_good_faith: true,
            reasonable_reliance: true,
            principal_fault: true,
        };
        let (is_apparent, _) = rules.validate_apparent_agency(&elements);
        assert!(!is_apparent);
    }

    #[test]
    fn test_validate_apparent_agency_bad_faith() {
        let rules = AgencyDeepRules::new();
        let elements = ApparentAgencyElements {
            has_authority_appearance: true,
            counterparty_good_faith: false,
            reasonable_reliance: true,
            principal_fault: true,
        };
        let (is_apparent, _) = rules.validate_apparent_agency(&elements);
        assert!(!is_apparent);
    }

    #[test]
    fn test_check_ratification_period_within() {
        let rules = AgencyDeepRules::new();
        let (valid, _) = rules.check_ratification_period(10, false);
        assert!(valid);
    }

    #[test]
    fn test_check_ratification_period_exceeded() {
        let rules = AgencyDeepRules::new();
        let (valid, _) = rules.check_ratification_period(31, false);
        assert!(!valid);
    }

    #[test]
    fn test_check_ratification_period_responded() {
        let rules = AgencyDeepRules::new();
        let (valid, _) = rules.check_ratification_period(10, true);
        assert!(valid);
    }

    #[test]
    fn test_describe_agency_type_entrusted() {
        let rules = AgencyDeepRules::new();
        let desc = rules.describe_agency_type(AgencyType::Entrusted);
        assert!(!desc.is_empty());
        assert!(desc[0].contains("委托代理"));
    }

    #[test]
    fn test_describe_agency_type_statutory() {
        let rules = AgencyDeepRules::new();
        let desc = rules.describe_agency_type(AgencyType::Statutory);
        assert!(!desc.is_empty());
        assert!(desc[0].contains("法定代理"));
    }

    #[test]
    fn test_describe_agency_type_designated() {
        let rules = AgencyDeepRules::new();
        let desc = rules.describe_agency_type(AgencyType::Designated);
        assert!(!desc.is_empty());
        assert!(desc[0].contains("指定代理"));
    }

    #[test]
    fn test_get_agency_termination_cases() {
        let rules = AgencyDeepRules::new();
        let cases = rules.get_agency_termination_cases(AgencyType::Entrusted);
        assert!(!cases.is_empty());
    }

    #[test]
    fn test_authorized_agency_rules() {
        let rules = AgencyDeepRules::new();
        let rules_list = rules.authorized_agency_rules();
        assert_eq!(rules_list.len(), 10);
    }

    #[test]
    fn test_unauthorized_agency_rules() {
        let rules = AgencyDeepRules::new();
        let rules_list = rules.unauthorized_agency_rules();
        assert!(rules_list.len() >= 10);
    }

    #[test]
    fn test_apparent_agency_rules() {
        let rules = AgencyDeepRules::new();
        let rules_list = rules.apparent_agency_rules();
        assert_eq!(rules_list.len(), 12);
    }

    #[test]
    fn test_agency_liability_rules() {
        let rules = AgencyDeepRules::new();
        let rules_list = rules.agency_liability_rules();
        assert!(!rules_list.is_empty());
    }

    #[test]
    fn test_get_deep_rules() {
        let rules = AgencyDeepRules::new();
        let deep_rules = rules.get_deep_rules();
        assert!(!deep_rules.is_empty());
    }

    #[test]
    fn test_category() {
        let rules = AgencyDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("agency_deep"));
    }

    #[test]
    fn test_explain() {
        let rules = AgencyDeepRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("有权代理规则"));
        assert!(explanation.contains("无权代理规则"));
        assert!(explanation.contains("表见代理规则"));
    }
}