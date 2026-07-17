//! 行政许可法深度规则
//!
//! 涵盖行政许可核心领域的详细内容，包括：
//! - 行政许可申请与受理
//! - 行政许可审查与决定
//! - 行政许可监督检查
//!
//! # 法律依据
//!
//! 主要依据：
//! - 《中华人民共和国行政许可法》（2019年修正）
//! - 《行政许可标准化指引》
//! - 《行政许可实施办法》

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use serde::{Deserialize, Serialize};

/// 行政许可类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseType {
    /// 普通许可
    General,
    /// 特许许可
    Special,
    /// 认可许可
    Approval,
    /// 核准许可
    Verify,
    /// 登记许可
    Registration,
}

/// 申请状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationStatus {
    /// 待审查
    Pending,
    /// 受理
    Accepted,
    /// 补正
    Correction,
    /// 不予受理
    Rejected,
    /// 已批准
    Approved,
    /// 已拒绝
    Denied,
}

/// 审查结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewResult {
    /// 符合条件
    Qualified,
    /// 不符合条件
    Unqualified,
    /// 需要补正
    NeedCorrection,
    /// 需要听证
    NeedHearing,
}

/// 许可决定类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    /// 准予许可
    Grant,
    /// 不予许可
    Deny,
    /// 附条件许可
    Conditional,
    /// 临时许可
    Temporary,
}

/// 申请材料完整性检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialCheckResult {
    /// 是否完整
    pub is_complete: bool,
    /// 缺失材料列表
    pub missing_materials: Vec<String>,
    /// 是否符合形式要求
    pub meets_format: bool,
}

/// 行政许可申请参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseApplicationParams {
    /// 申请人类型（个人/法人）
    pub is_individual: bool,
    /// 申请事项
    pub application_type: String,
    /// 材料是否齐全
    pub materials_complete: bool,
    /// 是否符合法定形式
    pub meets_formal_requirements: bool,
    /// 是否需要听证
    pub requires_hearing: bool,
    /// 是否涉及公共利益
    pub involves_public_interest: bool,
    /// 是否涉及他人重大利益
    pub involves_others_interest: bool,
}

simple_rule! {
    struct: AdminLicenseDeepRules,
    name: "行政许可法深度规则",
    desc: "行政许可法的详细规则解析",
    origin: "中国",
    tags: ["法律", "行政法", "行政许可"]
}

impl AdminLicenseDeepRules {
    /// 检查申请材料完整性
    ///
    /// # 参数
    /// - `required_materials`: 必需材料列表
    /// - `submitted_materials`: 已提交材料列表
    ///
    /// # 返回
    /// 材料检查结果
    pub fn check_materials(
        &self,
        required_materials: &[&str],
        submitted_materials: &[&str],
    ) -> MaterialCheckResult {
        let missing: Vec<String> = required_materials
            .iter()
            .filter(|m| !submitted_materials.contains(*m))
            .map(|s| s.to_string())
            .collect();

        MaterialCheckResult {
            is_complete: missing.is_empty(),
            missing_materials: missing,
            meets_format: true,
        }
    }

    /// 确定申请处理方式
    ///
    /// # 参数
    /// - `params`: 申请参数
    ///
    /// # 返回
    /// 申请状态
    pub fn determine_application_status(&self, params: &LicenseApplicationParams) -> ApplicationStatus {
        if !params.materials_complete {
            return ApplicationStatus::Correction;
        }

        if !params.meets_formal_requirements {
            return ApplicationStatus::Correction;
        }

        ApplicationStatus::Accepted
    }

    /// 计算审查期限
    ///
    /// # 参数
    /// - `base_days`: 基础审查期限（天）
    /// - `requires_hearing`: 是否需要听证
    /// - `involves_multiple_departments`: 是否涉及多个部门
    ///
    /// # 返回
    /// 实际审查期限（天）
    pub fn calculate_review_period(
        &self,
        base_days: u32,
        requires_hearing: bool,
        involves_multiple_departments: bool,
    ) -> u32 {
        let mut days = base_days;

        if requires_hearing {
            days += 20; // 听证延长20天
        }

        if involves_multiple_departments {
            days += 10; // 多部门协调延长10天
        }

        days
    }

    /// 判断是否需要公告
    ///
    /// # 参数
    /// - `params`: 申请参数
    ///
    /// # 返回
    /// 是否需要公告
    pub fn requires_public_notice(&self, params: &LicenseApplicationParams) -> bool {
        params.involves_public_interest || params.requires_hearing
    }

    /// 判断是否需要听证
    ///
    /// # 参数
    /// - `params`: 申请参数
    ///
    /// # 返回
    /// 是否需要听证
    pub fn requires_hearing_procedure(&self, params: &LicenseApplicationParams) -> bool {
        params.requires_hearing || params.involves_others_interest
    }

    /// 确定许可决定类型
    ///
    /// # 参数
    /// - `review_result`: 审查结果
    /// - `params`: 申请参数
    ///
    /// # 返回
    /// 许可决定类型
    pub fn determine_decision(
        &self,
        review_result: ReviewResult,
        params: &LicenseApplicationParams,
    ) -> DecisionType {
        match review_result {
            ReviewResult::Qualified => {
                if params.involves_public_interest {
                    DecisionType::Conditional
                } else {
                    DecisionType::Grant
                }
            }
            ReviewResult::Unqualified => DecisionType::Deny,
            ReviewResult::NeedCorrection => DecisionType::Deny,
            ReviewResult::NeedHearing => DecisionType::Deny,
        }
    }

    /// 计算许可有效期
    ///
    /// # 参数
    /// - `license_type`: 许可类型
    /// - `is_temporary`: 是否为临时许可
    ///
    /// # 返回
    /// 有效期（月）
    pub fn calculate_validity_period(
        &self,
        license_type: LicenseType,
        is_temporary: bool,
    ) -> u32 {
        if is_temporary {
            return 6; // 临时许可最多6个月
        }

        match license_type {
            LicenseType::General => 60,      // 5年
            LicenseType::Special => 36,      // 3年
            LicenseType::Approval => 36,     // 3年
            LicenseType::Verify => 24,       // 2年
            LicenseType::Registration => 0,  // 登记类无期限
        }
    }

    /// 判断是否可以变更许可
    ///
    /// # 参数
    /// - `license_type`: 许可类型
    /// - `has_violation`: 是否有违法行为
    ///
    /// # 返回
    /// 是否可以变更
    pub fn can_modify_license(&self, license_type: LicenseType, has_violation: bool) -> bool {
        if has_violation {
            return false;
        }

        match license_type {
            LicenseType::Registration => false, // 登记类不可变更
            _ => true,
        }
    }

    /// 判断是否可以延续许可
    ///
    /// # 参数
    /// - `license_type`: 许可类型
    /// - `has_violation`: 是否有违法行为
    /// - `months_before_expiry`: 许可到期前月数
    ///
    /// # 返回
    /// 是否可以延续
    pub fn can_renew_license(
        &self,
        license_type: LicenseType,
        has_violation: bool,
        months_before_expiry: u32,
    ) -> bool {
        if has_violation {
            return false;
        }

        // 应在许可有效期届满30日前提出
        if months_before_expiry > 1 {
            return true;
        }

        match license_type {
            LicenseType::Registration => false,
            _ => true,
        }
    }

    /// 判断是否应撤销许可
    ///
    /// # 参数
    /// - `obtained_by_fraud`: 是否通过欺诈取得
    /// - `no_longer_qualified`: 是否不再具备条件
    /// - `has_serious_violation`: 是否有严重违法行为
    ///
    /// # 返回
    /// 是否应撤销
    pub fn should_revoke_license(
        &self,
        obtained_by_fraud: bool,
        no_longer_qualified: bool,
        has_serious_violation: bool,
    ) -> bool {
        obtained_by_fraud || no_longer_qualified || has_serious_violation
    }

    /// 获取许可类型说明
    pub fn get_license_types_description(&self) -> Vec<&'static str> {
        vec![
            "普通许可: 由行政机关确认自然人、法人或其他组织是否具备从事特定活动的条件",
            "特许许可: 由行政机关代表国家向公民、法人或其他组织转让特定权利",
            "认可许可: 由行政机关认定申请人是否具备特定技能或资格",
            "核准许可: 由行政机关对特定事项是否符合法定标准进行核实",
            "登记许可: 由行政机关确立特定主体资格或特定权利",
        ]
    }

    /// 获取申请与受理规则
    pub fn get_application_rules(&self) -> Vec<&'static str> {
        vec![
            "申请方式: 书面申请或电子申请",
            "材料提交: 提交符合法定形式和数量要求的申请材料",
            "即时受理: 申请材料齐全、符合法定形式的，应当当场受理",
            "补正通知: 材料不齐全或不符合形式的，当场或5日内一次告知需要补正的内容",
            "不予受理: 不属于本行政机关职权范围的，即时作出不予受理决定",
            "受理凭证: 受理行政许可申请，应出具加盖本行政机关印章的书面凭证",
            "申请撤回: 申请人在行政机关作出决定前可撤回申请",
            "申请变更: 被许可人要求变更许可事项的，应向作出许可决定的机关申请",
        ]
    }

    /// 获取审查与决定规则
    pub fn get_review_rules(&self) -> Vec<&'static str> {
        vec![
            "审查方式: 书面审查或实地核查",
            "审查期限: 一般20日内作出决定，经批准可延长10日",
            "听证程序: 法律法规规定应当听证或涉及重大公共利益的应听证",
            "招标拍卖: 有限自然资源开发、公共资源配置等应通过招标、拍卖等公平竞争方式",
            "集中办理: 行政许可需要多个部门办理的，应确定一个部门统一受理",
            "决定送达: 作出准予许可决定的，10日内向申请人颁发许可证",
            "不予许可说明: 作出不予许可决定的，应说明理由并告知救济权利",
            "期限扣除: 听证、招标、拍卖、检验、检测等时间不计算在审查期限内",
        ]
    }

    /// 获取监督检查规则
    pub fn get_supervision_rules(&self) -> Vec<&'static str> {
        vec![
            "监督制度: 行政机关应建立健全监督制度",
            "核查方式: 通过核查反映被许可人活动情况的材料",
            "实地检查: 必要时可实地检查被许可人从事许可活动的情况",
            "违法行为: 发现被许可人有违法行为的，应依法及时查处",
            "档案管理: 建立许可档案，妥善保管相关材料",
            "信息共享: 建立信息共享机制，提高行政效率",
            "社会监督: 公民、法人或其他组织可对违法从事许可活动的情况举报",
            "责任追究: 监督检查不得妨碍被许可人正常的生产经营活动",
        ]
    }
}

impl Rule for AdminLicenseDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("admin_license_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "行政许可法深度规则",
            &[
                ("许可类型", &self.get_license_types_description()),
                ("申请与受理", &self.get_application_rules()),
                ("审查与决定", &self.get_review_rules()),
                ("监督检查", &self.get_supervision_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_materials_complete() {
        let rules = AdminLicenseDeepRules::new();
        let result = rules.check_materials(
            &["身份证", "申请书", "营业执照"],
            &["身份证", "申请书", "营业执照"],
        );
        assert!(result.is_complete);
        assert!(result.missing_materials.is_empty());
    }

    #[test]
    fn test_check_materials_incomplete() {
        let rules = AdminLicenseDeepRules::new();
        let result = rules.check_materials(
            &["身份证", "申请书", "营业执照"],
            &["身份证", "申请书"],
        );
        assert!(!result.is_complete);
        assert_eq!(result.missing_materials.len(), 1);
    }

    #[test]
    fn test_determine_application_status_accepted() {
        let rules = AdminLicenseDeepRules::new();
        let params = LicenseApplicationParams {
            is_individual: true,
            application_type: "test".to_string(),
            materials_complete: true,
            meets_formal_requirements: true,
            requires_hearing: false,
            involves_public_interest: false,
            involves_others_interest: false,
        };
        assert_eq!(
            rules.determine_application_status(&params),
            ApplicationStatus::Accepted
        );
    }

    #[test]
    fn test_determine_application_status_correction() {
        let rules = AdminLicenseDeepRules::new();
        let params = LicenseApplicationParams {
            is_individual: true,
            application_type: "test".to_string(),
            materials_complete: false,
            meets_formal_requirements: true,
            requires_hearing: false,
            involves_public_interest: false,
            involves_others_interest: false,
        };
        assert_eq!(
            rules.determine_application_status(&params),
            ApplicationStatus::Correction
        );
    }

    #[test]
    fn test_calculate_review_period_base() {
        let rules = AdminLicenseDeepRules::new();
        let days = rules.calculate_review_period(20, false, false);
        assert_eq!(days, 20);
    }

    #[test]
    fn test_calculate_review_period_with_hearing() {
        let rules = AdminLicenseDeepRules::new();
        let days = rules.calculate_review_period(20, true, false);
        assert_eq!(days, 40); // 20 + 20
    }

    #[test]
    fn test_calculate_review_period_with_multiple() {
        let rules = AdminLicenseDeepRules::new();
        let days = rules.calculate_review_period(20, false, true);
        assert_eq!(days, 30); // 20 + 10
    }

    #[test]
    fn test_requires_public_notice_true() {
        let rules = AdminLicenseDeepRules::new();
        let params = LicenseApplicationParams {
            is_individual: true,
            application_type: "test".to_string(),
            materials_complete: true,
            meets_formal_requirements: true,
            requires_hearing: true,
            involves_public_interest: false,
            involves_others_interest: false,
        };
        assert!(rules.requires_public_notice(&params));
    }

    #[test]
    fn test_requires_public_notice_false() {
        let rules = AdminLicenseDeepRules::new();
        let params = LicenseApplicationParams {
            is_individual: true,
            application_type: "test".to_string(),
            materials_complete: true,
            meets_formal_requirements: true,
            requires_hearing: false,
            involves_public_interest: false,
            involves_others_interest: false,
        };
        assert!(!rules.requires_public_notice(&params));
    }

    #[test]
    fn test_determine_decision_grant() {
        let rules = AdminLicenseDeepRules::new();
        let params = LicenseApplicationParams {
            is_individual: true,
            application_type: "test".to_string(),
            materials_complete: true,
            meets_formal_requirements: true,
            requires_hearing: false,
            involves_public_interest: false,
            involves_others_interest: false,
        };
        assert_eq!(
            rules.determine_decision(ReviewResult::Qualified, &params),
            DecisionType::Grant
        );
    }

    #[test]
    fn test_determine_decision_deny() {
        let rules = AdminLicenseDeepRules::new();
        let params = LicenseApplicationParams {
            is_individual: true,
            application_type: "test".to_string(),
            materials_complete: true,
            meets_formal_requirements: true,
            requires_hearing: false,
            involves_public_interest: false,
            involves_others_interest: false,
        };
        assert_eq!(
            rules.determine_decision(ReviewResult::Unqualified, &params),
            DecisionType::Deny
        );
    }

    #[test]
    fn test_calculate_validity_period_general() {
        let rules = AdminLicenseDeepRules::new();
        assert_eq!(rules.calculate_validity_period(LicenseType::General, false), 60);
    }

    #[test]
    fn test_calculate_validity_period_temporary() {
        let rules = AdminLicenseDeepRules::new();
        assert_eq!(rules.calculate_validity_period(LicenseType::General, true), 6);
    }

    #[test]
    fn test_can_modify_license_normal() {
        let rules = AdminLicenseDeepRules::new();
        assert!(rules.can_modify_license(LicenseType::General, false));
    }

    #[test]
    fn test_can_modify_license_with_violation() {
        let rules = AdminLicenseDeepRules::new();
        assert!(!rules.can_modify_license(LicenseType::General, true));
    }

    #[test]
    fn test_can_renew_license_normal() {
        let rules = AdminLicenseDeepRules::new();
        assert!(rules.can_renew_license(LicenseType::General, false, 2));
    }

    #[test]
    fn test_can_renew_license_too_late() {
        let rules = AdminLicenseDeepRules::new();
        assert!(!rules.can_renew_license(LicenseType::General, false, 0));
    }

    #[test]
    fn test_should_revoke_license_fraud() {
        let rules = AdminLicenseDeepRules::new();
        assert!(rules.should_revoke_license(true, false, false));
    }

    #[test]
    fn test_should_revoke_license_no_reason() {
        let rules = AdminLicenseDeepRules::new();
        assert!(!rules.should_revoke_license(false, false, false));
    }

    #[test]
    fn test_explain() {
        let rules = AdminLicenseDeepRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("行政许可法深度规则"));
        assert!(explanation.contains("许可类型"));
    }
}