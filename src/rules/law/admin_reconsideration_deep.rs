//! 行政复议法深度规则
//!
//! 涵盖行政复议核心领域的详细内容，包括：
//! - 行政复议申请与受理
//! - 行政复议审理与决定
//! - 行政复议执行与监督
//!
//! # 法律依据
//!
//! 主要依据：
//! - 《中华人民共和国行政复议法》（2023年修订）
//! - 《行政复议法实施条例》
//! - 《行政复议办案规程》

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use serde::{Deserialize, Serialize};

/// 行政复议范围类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconsiderationScope {
    /// 行政处罚
    Penalty,
    /// 行政许可
    License,
    /// 行政强制
    Coercion,
    /// 行政征收
    Levy,
    /// 行政给付
    Payment,
    /// 行政不作为
    Inaction,
    /// 其他具体行政行为
    Other,
}

/// 复议机关类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconsiderationAuthority {
    /// 本级人民政府
    LocalGov,
    /// 上一级主管部门
    UpperDept,
    /// 省级人民政府
    ProvincialGov,
    /// 国务院部门
    StateCouncil,
}

/// 复议申请方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationMethod {
    /// 书面申请
    Written,
    /// 口头申请
    Oral,
    /// 电子申请
    Electronic,
}

/// 复议决定类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    /// 维持决定
    Maintain,
    /// 责令履行
    OrderPerform,
    /// 撤销决定
    Revoke,
    /// 变更决定
    Modify,
    /// 确认违法
    ConfirmIllegal,
    /// 责令重作
    OrderRedo,
}

/// 复议程序状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcedureStatus {
    /// 申请受理
    Accepted,
    /// 审理中
    UnderReview,
    /// 中止
    Suspended,
    /// 终止
    Terminated,
    /// 已决定
    Decided,
}

/// 行政复议申请参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconsiderationParams {
    /// 被申请行政行为类型
    pub act_type: ReconsiderationScope,
    /// 申请期限（天）
    pub days_since_act: u32,
    /// 是否超过法定期限
    pub exceeds_time_limit: bool,
    /// 是否有正当理由延误
    pub has_valid_reason: bool,
    /// 申请人类型
    pub is_individual: bool,
    /// 是否涉及第三人
    pub involves_third_party: bool,
}

simple_rule! {
    struct: AdminReconsiderationDeepRules,
    name: "行政复议法深度规则",
    desc: "行政复议法的详细规则解析",
    origin: "中国",
    tags: ["法律", "行政法", "行政复议"]
}

impl AdminReconsiderationDeepRules {
    /// 确定复议机关
    ///
    /// # 参数
    /// - `act_type`: 被申请行政行为类型
    /// - `is_local_gov_act`: 是否为地方人民政府行为
    /// - `is_dept_act`: 是否为部门行为
    ///
    /// # 返回
    /// 复议机关类型
    pub fn determine_authority(
        &self,
        _act_type: ReconsiderationScope,
        is_local_gov_act: bool,
        is_dept_act: bool,
    ) -> ReconsiderationAuthority {
        if is_local_gov_act {
            ReconsiderationAuthority::UpperDept
        } else if is_dept_act {
            // 申请人可选择本级人民政府或上一级主管部门
            ReconsiderationAuthority::LocalGov
        } else {
            ReconsiderationAuthority::LocalGov
        }
    }

    /// 判断是否属于复议范围
    ///
    /// # 参数
    /// - `act_type`: 行政行为类型
    ///
    /// # 返回
    /// 是否属于复议范围
    pub fn is_within_scope(&self, act_type: ReconsiderationScope) -> bool {
        match act_type {
            ReconsiderationScope::Penalty => true,
            ReconsiderationScope::License => true,
            ReconsiderationScope::Coercion => true,
            ReconsiderationScope::Levy => true,
            ReconsiderationScope::Payment => true,
            ReconsiderationScope::Inaction => true,
            ReconsiderationScope::Other => false,
        }
    }

    /// 判断申请是否在有效期限内
    ///
    /// # 参数
    /// - `params`: 申请参数
    ///
    /// # 返回
    /// 是否在有效期限内
    pub fn is_within_time_limit(&self, params: &ReconsiderationParams) -> bool {
        if params.exceeds_time_limit && !params.has_valid_reason {
            return false;
        }
        true
    }

    /// 计算复议审理期限
    ///
    /// # 参数
    /// - `involves_third_party`: 是否涉及第三人
    /// - `requires_investigation`: 是否需要调查取证
    ///
    /// # 返回
    /// 审理期限（天）
    pub fn calculate_review_period(
        &self,
        involves_third_party: bool,
        requires_investigation: bool,
    ) -> u32 {
        let base_days = 60;

        let mut extension = 0;
        if involves_third_party {
            extension += 10;
        }
        if requires_investigation {
            extension += 15;
        }

        base_days + extension
    }

    /// 确定复议决定类型
    ///
    /// # 参数
    /// - `is_act_legal`: 被申请行为是否合法
    /// - `is_act_appropriate`: 被申请行为是否适当
    /// - `is_procedure_valid`: 程序是否合法
    /// - `has_evidence`: 是否有充分证据
    ///
    /// # 返回
    /// 复议决定类型
    pub fn determine_decision(
        &self,
        is_act_legal: bool,
        is_act_appropriate: bool,
        is_procedure_valid: bool,
        has_evidence: bool,
    ) -> DecisionType {
        if is_act_legal && is_act_appropriate && is_procedure_valid && has_evidence {
            return DecisionType::Maintain;
        }

        if !is_act_legal || !is_procedure_valid || !has_evidence {
            return DecisionType::Revoke;
        }

        if !is_act_appropriate {
            return DecisionType::Modify;
        }

        DecisionType::Maintain
    }

    /// 判断是否应中止复议
    ///
    /// # 参数
    /// - `applicant_deceased`: 申请人是否死亡
    /// - `applicant_incapacitated`: 申请人是否丧失行为能力
    /// - `pending_criminal_case`: 是否有待决刑事案件
    /// - `awaiting_other_decision`: 是否等待其他机关决定
    ///
    /// # 返回
    /// 是否应中止
    pub fn should_suspend(
        &self,
        applicant_deceased: bool,
        applicant_incapacitated: bool,
        pending_criminal_case: bool,
        awaiting_other_decision: bool,
    ) -> bool {
        applicant_deceased
            || applicant_incapacitated
            || pending_criminal_case
            || awaiting_other_decision
    }

    /// 判断是否应终止复议
    ///
    /// # 参数
    /// - `applicant_withdraws`: 申请人是否撤回申请
    /// - `applicant_no_successor`: 申请人死亡无继承人
    /// - `act_rescinded`: 被申请行为是否已被撤销
    /// - `request_fulfilled`: 请求是否已被满足
    ///
    /// # 返回
    /// 是否应终止
    pub fn should_terminate(
        &self,
        applicant_withdraws: bool,
        applicant_no_successor: bool,
        act_rescinded: bool,
        request_fulfilled: bool,
    ) -> bool {
        applicant_withdraws || applicant_no_successor || act_rescinded || request_fulfilled
    }

    /// 获取知悉权利期限
    ///
    /// # 参数
    /// - `_params`: 复议参数（未使用）
    ///
    /// # 返回
    /// 期限（天）
    pub fn calculate_awareness_period(&self, _params: &ReconsiderationParams) -> u32 {
        // 一般情况下60日内应知悉权利
        60
    }

    /// 获取复议范围说明
    pub fn get_scope_description(&self) -> Vec<&'static str> {
        vec![
            "行政处罚: 对行政机关作出的警告、罚款、没收、责令停产停业、暂扣或吊销许可证、行政拘留等处罚不服",
            "行政许可: 对行政机关作出的有关许可证、执照、资质证、资格证等证书变更、中止、撤销的决定不服",
            "行政强制: 对行政机关作出的限制人身自由或查封、扣押、冻结财产等行政强制措施不服",
            "行政征收: 对行政机关作出的征用决定或补偿决定不服",
            "行政给付: 认为行政机关未依法支付抚恤金、保险金或最低生活保障费",
            "行政不作为: 认为行政机关未依法履行保护人身权、财产权等合法权益的职责",
            "排除范围: 不服行政法规、规章、内部行政行为、行政调解等不可申请复议",
        ]
    }

    /// 获取申请与受理规则
    pub fn get_application_rules(&self) -> Vec<&'static str> {
        vec![
            "申请期限: 自知道或应当知道该行政行为之日起60日内",
            "申请方式: 可书面申请、口头申请或电子申请",
            "申请材料: 行政复议申请书、申请人身份证明、被申请行政行为文书等",
            "受理审查: 复议机关收到申请后5日内进行审查",
            "补正通知: 申请材料不齐全的，5日内一次告知申请人补正",
            "受理决定: 符合条件的，受理并书面通知申请人",
            "不予受理: 不符合受理条件的，决定不予受理并说明理由",
            "转送申请: 不属本机关管辖的，应转送有权管辖的复议机关",
        ]
    }

    /// 获取审理规则
    pub fn get_review_rules(&self) -> Vec<&'static str> {
        vec![
            "审理方式: 原则上采取书面审理，必要时可实地调查",
            "调查取证: 复议机关可向有关组织和人员调查情况、收集证据",
            "第三人参加: 同申请复议的行政行为有利害关系的，可申请参加复议",
            "举证责任: 被申请人对其行政行为的合法性、适当性承担举证责任",
            "延期审理: 有正当理由无法按时审理的，可延期",
            "和解调解: 在自愿合法基础上，可组织当事人和解",
            "听证审理: 对重大复杂案件，可举行听证",
            "审理期限: 一般应在受理之日起60日内作出决定",
        ]
    }

    /// 获取决定规则
    pub fn get_decision_rules(&self) -> Vec<&'static str> {
        vec![
            "维持决定: 具体行政行为认定事实清楚、证据确凿、适用依据正确、程序合法的",
            "撤销决定: 主要事实不清、证据不足、适用依据错误、违反法定程序的",
            "变更决定: 具体行政行为明显不当的",
            "确认违法: 具体行政行为违法但不具有可撤销内容的",
            "责令履行: 被申请人不履行法定职责的",
            "责令重作: 被申请人未依法履行法定职责的",
            "赔偿决定: 具体行政行为损害申请人合法权益的，可一并申请行政赔偿",
            "决定送达: 行政复议决定书一经送达即发生法律效力",
        ]
    }

    /// 获取执行规则
    pub fn get_execution_rules(&self) -> Vec<&'static str> {
        vec![
            "履行期限: 被申请人应在法定期限内履行行政复议决定",
            "强制执行: 被申请人不履行决定的，复议机关或上级行政机关责令限期履行",
            "诉讼权利: 对行政复议决定不服的，可在收到决定书之日起15日内提起行政诉讼",
            "终局决定: 法律规定由行政机关最终裁决的行政行为不可提起诉讼",
            "监督执行: 复议机关应监督检查行政复议决定的执行情况",
            "拒绝履行后果: 对拒不履行的，可给予行政处分",
            "国家赔偿: 因行政复议决定造成损害的，可申请国家赔偿",
        ]
    }
}

impl Rule for AdminReconsiderationDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("admin_reconsideration_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "行政复议法深度规则",
            &[
                ("复议范围", &self.get_scope_description()),
                ("申请与受理", &self.get_application_rules()),
                ("审理规则", &self.get_review_rules()),
                ("决定规则", &self.get_decision_rules()),
                ("执行规则", &self.get_execution_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_authority_local_gov() {
        let rules = AdminReconsiderationDeepRules::new();
        assert_eq!(
            rules.determine_authority(ReconsiderationScope::Penalty, true, false),
            ReconsiderationAuthority::UpperDept
        );
    }

    #[test]
    fn test_determine_authority_dept() {
        let rules = AdminReconsiderationDeepRules::new();
        assert_eq!(
            rules.determine_authority(ReconsiderationScope::Penalty, false, true),
            ReconsiderationAuthority::LocalGov
        );
    }

    #[test]
    fn test_is_within_scope_penalty() {
        let rules = AdminReconsiderationDeepRules::new();
        assert!(rules.is_within_scope(ReconsiderationScope::Penalty));
    }

    #[test]
    fn test_is_within_scope_license() {
        let rules = AdminReconsiderationDeepRules::new();
        assert!(rules.is_within_scope(ReconsiderationScope::License));
    }

    #[test]
    fn test_is_within_scope_other() {
        let rules = AdminReconsiderationDeepRules::new();
        assert!(!rules.is_within_scope(ReconsiderationScope::Other));
    }

    #[test]
    fn test_is_within_time_limit_valid() {
        let rules = AdminReconsiderationDeepRules::new();
        let params = ReconsiderationParams {
            act_type: ReconsiderationScope::Penalty,
            days_since_act: 30,
            exceeds_time_limit: false,
            has_valid_reason: false,
            is_individual: true,
            involves_third_party: false,
        };
        assert!(rules.is_within_time_limit(&params));
    }

    #[test]
    fn test_is_within_time_limit_expired() {
        let rules = AdminReconsiderationDeepRules::new();
        let params = ReconsiderationParams {
            act_type: ReconsiderationScope::Penalty,
            days_since_act: 90,
            exceeds_time_limit: true,
            has_valid_reason: false,
            is_individual: true,
            involves_third_party: false,
        };
        assert!(!rules.is_within_time_limit(&params));
    }

    #[test]
    fn test_is_within_time_limit_with_reason() {
        let rules = AdminReconsiderationDeepRules::new();
        let params = ReconsiderationParams {
            act_type: ReconsiderationScope::Penalty,
            days_since_act: 90,
            exceeds_time_limit: true,
            has_valid_reason: true,
            is_individual: true,
            involves_third_party: false,
        };
        assert!(rules.is_within_time_limit(&params));
    }

    #[test]
    fn test_calculate_review_period_base() {
        let rules = AdminReconsiderationDeepRules::new();
        let days = rules.calculate_review_period(false, false);
        assert_eq!(days, 60);
    }

    #[test]
    fn test_calculate_review_period_with_third_party() {
        let rules = AdminReconsiderationDeepRules::new();
        let days = rules.calculate_review_period(true, false);
        assert_eq!(days, 70); // 60 + 10
    }

    #[test]
    fn test_calculate_review_period_with_investigation() {
        let rules = AdminReconsiderationDeepRules::new();
        let days = rules.calculate_review_period(false, true);
        assert_eq!(days, 75); // 60 + 15
    }

    #[test]
    fn test_determine_decision_maintain() {
        let rules = AdminReconsiderationDeepRules::new();
        let decision = rules.determine_decision(true, true, true, true);
        assert_eq!(decision, DecisionType::Maintain);
    }

    #[test]
    fn test_determine_decision_revoke() {
        let rules = AdminReconsiderationDeepRules::new();
        let decision = rules.determine_decision(false, true, true, true);
        assert_eq!(decision, DecisionType::Revoke);
    }

    #[test]
    fn test_determine_decision_modify() {
        let rules = AdminReconsiderationDeepRules::new();
        let decision = rules.determine_decision(true, false, true, true);
        assert_eq!(decision, DecisionType::Modify);
    }

    #[test]
    fn test_should_suspend_deceased() {
        let rules = AdminReconsiderationDeepRules::new();
        assert!(rules.should_suspend(true, false, false, false));
    }

    #[test]
    fn test_should_suspend_no_reason() {
        let rules = AdminReconsiderationDeepRules::new();
        assert!(!rules.should_suspend(false, false, false, false));
    }

    #[test]
    fn test_should_terminate_withdraws() {
        let rules = AdminReconsiderationDeepRules::new();
        assert!(rules.should_terminate(true, false, false, false));
    }

    #[test]
    fn test_should_terminate_no_reason() {
        let rules = AdminReconsiderationDeepRules::new();
        assert!(!rules.should_terminate(false, false, false, false));
    }

    #[test]
    fn test_calculate_awareness_period() {
        let rules = AdminReconsiderationDeepRules::new();
        let params = ReconsiderationParams {
            act_type: ReconsiderationScope::Penalty,
            days_since_act: 0,
            exceeds_time_limit: false,
            has_valid_reason: false,
            is_individual: true,
            involves_third_party: false,
        };
        assert_eq!(rules.calculate_awareness_period(&params), 60);
    }

    #[test]
    fn test_explain() {
        let rules = AdminReconsiderationDeepRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("行政复议法深度规则"));
        assert!(explanation.contains("复议范围"));
    }
}
