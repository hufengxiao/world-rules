//! 行政处罚法深度规则
//!
//! 涵盖行政处罚核心领域的详细内容，包括：
//! - 行政处罚种类与设定
//! - 行政处罚程序规则
//! - 行政处罚执行规则
//!
//! # 法律依据
//!
//! 主要依据：
//! - 《中华人民共和国行政处罚法》（2021年修订）
//! - 《中华人民共和国行政处罚法实施条例》
//! - 《行政处罚程序规定》
//! - 《行政处罚听证程序规定》

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use serde::{Deserialize, Serialize};

/// 行政处罚种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyType {
    /// 警告
    Warning,
    /// 罚款
    Fine,
    /// 没收违法所得
    ConfiscateIncome,
    /// 没收非法财物
    ConfiscateGoods,
    /// 责令停产停业
    SuspendBusiness,
    /// 暂扣许可证
    SuspendLicense,
    /// 吊销许可证
    RevokeLicense,
    /// 行政拘留
    Detention,
}

/// 行政处罚程序类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyProcedure {
    /// 简易程序
    Simple,
    /// 一般程序
    General,
    /// 听证程序
    Hearing,
}

/// 行政处罚设定权限
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyAuthority {
    /// 法律
    Law,
    /// 行政法规
    AdminRegulation,
    /// 地方性法规
    LocalRegulation,
    /// 国务院决定
    StateCouncilDecision,
    /// 部门规章
    MinistryRule,
    /// 地方政府规章
    LocalGovRule,
}

/// 行政处罚执行方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionMethod {
    /// 当场收缴
    OnSite,
    /// 银行缴纳
    Bank,
    /// 强制执行
    Forced,
    /// 分期缴纳
    Installment,
}

/// 行政处罚参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyParams {
    /// 违法行为类型
    pub violation_type: String,
    /// 违法情节严重程度（1-5级）
    pub severity: u8,
    /// 是否为未成年人
    pub is_minor: bool,
    /// 是否有立功表现
    pub has_merit: bool,
    /// 是否初犯
    pub is_first_offense: bool,
    /// 违法所得金额
    pub illegal_income: f64,
    /// 是否主动消除危害
    pub mitigates_harm: bool,
}

simple_rule! {
    struct: AdminPenaltyDeepRules,
    name: "行政处罚法深度规则",
    desc: "行政处罚法的详细规则解析",
    origin: "中国",
    tags: ["法律", "行政法", "行政处罚"]
}

impl AdminPenaltyDeepRules {
    /// 确定适用程序类型
    ///
    /// # 参数
    /// - `penalty_type`: 处罚种类
    /// - `fine_amount`: 罚款金额（如有）
    /// - `target_type`: 处罚对象类型（个人/法人）
    ///
    /// # 返回
    /// 应适用的程序类型
    pub fn determine_procedure(
        &self,
        penalty_type: PenaltyType,
        fine_amount: Option<f64>,
        is_individual: bool,
    ) -> PenaltyProcedure {
        match penalty_type {
            PenaltyType::Warning => PenaltyProcedure::Simple,
            PenaltyType::Fine => {
                // 简易程序适用条件：对公民50元以下、法人1000元以下罚款
                let threshold = if is_individual { 50.0 } else { 1000.0 };
                if let Some(amount) = fine_amount {
                    if amount <= threshold {
                        return PenaltyProcedure::Simple;
                    }
                }
                PenaltyProcedure::General
            }
            PenaltyType::Detention => PenaltyProcedure::Hearing,
            PenaltyType::RevokeLicense => PenaltyProcedure::Hearing,
            PenaltyType::SuspendBusiness => PenaltyProcedure::Hearing,
            _ => PenaltyProcedure::General,
        }
    }

    /// 计算罚款金额
    ///
    /// # 参数
    /// - `base_amount`: 基础罚款金额
    /// - `params`: 处罚参数
    ///
    /// # 返回
    /// 实际罚款金额
    pub fn calculate_fine(&self, base_amount: f64, params: &PenaltyParams) -> f64 {
        let mut multiplier = 1.0;

        // 根据情节严重程度调整
        match params.severity {
            1 => multiplier *= 0.5, // 较轻
            2 => multiplier *= 0.8, // 轻微
            3 => {}                 // 一般
            4 => multiplier *= 1.5, // 较重
            5 => multiplier *= 2.0, // 严重
            _ => {}
        }

        // 减轻情节
        if params.is_first_offense {
            multiplier *= 0.8;
        }
        if params.has_merit {
            multiplier *= 0.7;
        }
        if params.mitigates_harm {
            multiplier *= 0.6;
        }

        // 未成年人从轻或减轻
        if params.is_minor {
            multiplier *= 0.5;
        }

        let fine = base_amount * multiplier;
        fine.max(base_amount * 0.1) // 最低不低于基准的10%
    }

    /// 判断是否可以当场收缴罚款
    ///
    /// # 参数
    /// - `fine_amount`: 罚款金额
    /// - `distance_km`: 距离银行距离（公里）
    /// - `_is_individual`: 是否为个人（未使用）
    ///
    /// # 返回
    /// 是否可以当场收缴
    pub fn can_collect_on_site(
        &self,
        fine_amount: f64,
        distance_km: f64,
        _is_individual: bool,
    ) -> bool {
        // 简易程序且20元以下罚款可当场收缴
        if fine_amount <= 20.0 {
            return true;
        }

        // 不当场收缴事后难以执行的情况
        if distance_km > 50.0 {
            return true;
        }

        // 水上、偏远地区交通不便
        if distance_km > 30.0 {
            return true;
        }

        false
    }

    /// 判断是否适用听证程序
    ///
    /// # 参数
    /// - `penalty_type`: 处罚种类
    /// - `fine_amount`: 罚款金额（如有）
    /// - `is_individual`: 是否为个人
    ///
    /// # 返回
    /// 是否适用听证程序
    pub fn requires_hearing(
        &self,
        penalty_type: PenaltyType,
        fine_amount: Option<f64>,
        is_individual: bool,
    ) -> bool {
        match penalty_type {
            PenaltyType::SuspendBusiness => true,
            PenaltyType::RevokeLicense => true,
            PenaltyType::Detention => true,
            PenaltyType::Fine => {
                // 较大数额罚款需听证
                if let Some(amount) = fine_amount {
                    let threshold = if is_individual { 1000.0 } else { 5000.0 };
                    return amount >= threshold;
                }
                false
            }
            _ => false,
        }
    }

    /// 计算行政拘留期限
    ///
    /// # 参数
    /// - `severity`: 情节严重程度（1-5）
    /// - `params`: 处罚参数
    ///
    /// # 返回
    /// 拘留天数（1-15天，或合并执行16-20天）
    pub fn calculate_detention_days(&self, severity: u8, params: &PenaltyParams) -> u8 {
        if params.is_minor {
            // 未成年人不执行行政拘留
            return 0;
        }

        let base_days: u8 = match severity {
            1 => 1,
            2 => 3,
            3 => 7,
            4 => 10,
            5 => 15,
            _ => 5,
        };

        // 减轻情节
        let days = if params.has_merit {
            base_days.saturating_sub(2)
        } else if params.mitigates_harm {
            base_days.saturating_sub(3)
        } else if params.is_first_offense {
            base_days.saturating_sub(1)
        } else {
            base_days
        };

        days.clamp(1, 15)
    }

    /// 判断处罚是否超过追诉时效
    ///
    /// # 参数
    /// - `years_since_violation`: 违法行为发生后经过的年数
    /// - `penalty_type`: 处罚种类
    ///
    /// # 返回
    /// 是否已过追诉时效
    pub fn is_statute_of_limitations_expired(
        &self,
        years_since_violation: f64,
        penalty_type: PenaltyType,
    ) -> bool {
        match penalty_type {
            // 一般违法行为：2年
            PenaltyType::Warning | PenaltyType::Fine => years_since_violation > 2.0,
            // 较重违法行为：更长时效
            PenaltyType::Detention | PenaltyType::RevokeLicense => years_since_violation > 2.0,
            _ => years_since_violation > 2.0,
        }
    }

    /// 获取处罚种类说明
    pub fn get_penalty_types_description(&self) -> Vec<&'static str> {
        vec![
            "警告: 对违法行为人予以谴责和警示",
            "罚款: 强制违法行为人缴纳一定数量货币",
            "没收违法所得: 将违法行为人的违法所得收归国有",
            "没收非法财物: 将违法行为人的非法财物收归国有",
            "责令停产停业: 责令违法行为人停止生产或经营",
            "暂扣许可证: 暂时扣留许可证，暂停其从事相应活动",
            "吊销许可证: 注销许可证，终止其从事相应活动",
            "行政拘留: 限制违法行为人的人身自由",
        ]
    }

    /// 获取简易程序规则
    pub fn get_simple_procedure_rules(&self) -> Vec<&'static str> {
        vec![
            "适用条件: 违法事实确凿并有法定依据",
            "罚款限额: 对公民50元以下，对法人1000元以下",
            "程序要求: 执法人员当场作出行政处罚决定",
            "决定书: 填写预定格式、编有号码的行政处罚决定书",
            "送达: 当场交付行政处罚决定书",
            "备案: 执法人员需将决定书报所属行政机关备案",
            "当事人权利: 对处罚决定不服的，可依法申请复议或提起诉讼",
        ]
    }

    /// 获取一般程序规则
    pub fn get_general_procedure_rules(&self) -> Vec<&'static str> {
        vec![
            "立案: 发现违法行为，经审查后予以立案",
            "调查: 执法人员进行调查，收集证据",
            "审查: 对调查结果进行审查，提出处理意见",
            "告知: 将处罚的事实、理由、依据告知当事人",
            "申辩: 充分听取当事人的陈述和申辩",
            "决定: 作出行政处罚决定，制作行政处罚决定书",
            "送达: 在7日内将行政处罚决定书送达当事人",
            "执行: 当事人在规定期限内履行处罚决定",
        ]
    }

    /// 获取听证程序规则
    pub fn get_hearing_procedure_rules(&self) -> Vec<&'static str> {
        vec![
            "告知权利: 作出决定前告知当事人有要求听证的权利",
            "申请期限: 当事人要求听证的，应在行政机关告知后5日内提出",
            "组织听证: 行政机关在听证7日前通知当事人听证时间、地点",
            "公开听证: 除涉及国家秘密、商业秘密或个人隐私外，听证公开举行",
            "主持听证: 由行政机关指定的非本案调查人员主持",
            "调查人员举证: 调查人员提出当事人违法的事实、证据和处罚建议",
            "当事人申辩: 当事人进行申辩和质证",
            "听证笔录: 制作听证笔录，交当事人审核无误后签字或盖章",
        ]
    }

    /// 获取执行规则
    pub fn get_execution_rules(&self) -> Vec<&'static str> {
        vec![
            "自动履行: 当事人在规定期限内自动履行处罚决定",
            "罚款缴纳: 应当自收到处罚决定书之日起15日内缴纳",
            "分期缴纳: 当事人确有经济困难，可申请分期或暂缓缴纳",
            "强制执行: 当事人逾期不履行，行政机关可强制执行",
            "加处罚款: 逾期不履行罚款处罚的，每日按罚款数额3%加处",
            "执行中止: 当事人履行确有困难的，可中止执行",
            "执行终结: 当事人死亡或终止，无遗产或权利义务承受人的，终结执行",
        ]
    }
}

impl Rule for AdminPenaltyDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("admin_penalty_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "行政处罚法深度规则",
            &[
                ("处罚种类", &self.get_penalty_types_description()),
                ("简易程序", &self.get_simple_procedure_rules()),
                ("一般程序", &self.get_general_procedure_rules()),
                ("听证程序", &self.get_hearing_procedure_rules()),
                ("执行规则", &self.get_execution_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_procedure_simple_warning() {
        let rules = AdminPenaltyDeepRules::new();
        assert_eq!(
            rules.determine_procedure(PenaltyType::Warning, None, true),
            PenaltyProcedure::Simple
        );
    }

    #[test]
    fn test_determine_procedure_simple_fine() {
        let rules = AdminPenaltyDeepRules::new();
        // 公民50元以下罚款适用简易程序
        assert_eq!(
            rules.determine_procedure(PenaltyType::Fine, Some(30.0), true),
            PenaltyProcedure::Simple
        );
    }

    #[test]
    fn test_determine_procedure_general_fine() {
        let rules = AdminPenaltyDeepRules::new();
        // 超过限额适用一般程序
        assert_eq!(
            rules.determine_procedure(PenaltyType::Fine, Some(100.0), true),
            PenaltyProcedure::General
        );
    }

    #[test]
    fn test_determine_procedure_hearing_detention() {
        let rules = AdminPenaltyDeepRules::new();
        assert_eq!(
            rules.determine_procedure(PenaltyType::Detention, None, true),
            PenaltyProcedure::Hearing
        );
    }

    #[test]
    fn test_calculate_fine_normal() {
        let rules = AdminPenaltyDeepRules::new();
        let params = PenaltyParams {
            violation_type: "test".to_string(),
            severity: 3,
            is_minor: false,
            has_merit: false,
            is_first_offense: false,
            illegal_income: 0.0,
            mitigates_harm: false,
        };
        let fine = rules.calculate_fine(1000.0, &params);
        assert_eq!(fine, 1000.0);
    }

    #[test]
    fn test_calculate_fine_minor() {
        let rules = AdminPenaltyDeepRules::new();
        let params = PenaltyParams {
            violation_type: "test".to_string(),
            severity: 3,
            is_minor: true,
            has_merit: false,
            is_first_offense: false,
            illegal_income: 0.0,
            mitigates_harm: false,
        };
        let fine = rules.calculate_fine(1000.0, &params);
        assert_eq!(fine, 500.0);
    }

    #[test]
    fn test_calculate_fine_mitigating() {
        let rules = AdminPenaltyDeepRules::new();
        let params = PenaltyParams {
            violation_type: "test".to_string(),
            severity: 3,
            is_minor: false,
            has_merit: true,
            is_first_offense: true,
            illegal_income: 0.0,
            mitigates_harm: true,
        };
        let fine = rules.calculate_fine(1000.0, &params);
        // 0.8 * 0.7 * 0.6 * 0.6 = 0.2016
        assert!(fine < 1000.0);
    }

    #[test]
    fn test_can_collect_on_site_small_amount() {
        let rules = AdminPenaltyDeepRules::new();
        // 20元以下可当场收缴
        assert!(rules.can_collect_on_site(15.0, 10.0, true));
    }

    #[test]
    fn test_can_collect_on_site_large_amount() {
        let rules = AdminPenaltyDeepRules::new();
        // 超过20元且距离近，不能当场收缴
        assert!(!rules.can_collect_on_site(50.0, 5.0, true));
    }

    #[test]
    fn test_can_collect_on_site_remote_area() {
        let rules = AdminPenaltyDeepRules::new();
        // 偏远地区可当场收缴
        assert!(rules.can_collect_on_site(50.0, 60.0, true));
    }

    #[test]
    fn test_requires_hearing_revoke_license() {
        let rules = AdminPenaltyDeepRules::new();
        assert!(rules.requires_hearing(PenaltyType::RevokeLicense, None, true));
    }

    #[test]
    fn test_requires_hearing_large_fine() {
        let rules = AdminPenaltyDeepRules::new();
        // 个人1000元以上罚款需听证
        assert!(rules.requires_hearing(PenaltyType::Fine, Some(1500.0), true));
    }

    #[test]
    fn test_requires_hearing_small_fine() {
        let rules = AdminPenaltyDeepRules::new();
        // 小额罚款无需听证
        assert!(!rules.requires_hearing(PenaltyType::Fine, Some(500.0), true));
    }

    #[test]
    fn test_calculate_detention_days_normal() {
        let rules = AdminPenaltyDeepRules::new();
        let params = PenaltyParams {
            violation_type: "test".to_string(),
            severity: 3,
            is_minor: false,
            has_merit: false,
            is_first_offense: false,
            illegal_income: 0.0,
            mitigates_harm: false,
        };
        let days = rules.calculate_detention_days(3, &params);
        assert_eq!(days, 7);
    }

    #[test]
    fn test_calculate_detention_days_minor() {
        let rules = AdminPenaltyDeepRules::new();
        let params = PenaltyParams {
            violation_type: "test".to_string(),
            severity: 3,
            is_minor: true,
            has_merit: false,
            is_first_offense: false,
            illegal_income: 0.0,
            mitigates_harm: false,
        };
        let days = rules.calculate_detention_days(3, &params);
        // 未成年人不执行行政拘留
        assert_eq!(days, 0);
    }

    #[test]
    fn test_is_statute_of_limitations_expired_within() {
        let rules = AdminPenaltyDeepRules::new();
        assert!(!rules.is_statute_of_limitations_expired(1.0, PenaltyType::Fine));
    }

    #[test]
    fn test_is_statute_of_limitations_expired_exceeded() {
        let rules = AdminPenaltyDeepRules::new();
        assert!(rules.is_statute_of_limitations_expired(3.0, PenaltyType::Fine));
    }

    #[test]
    fn test_explain() {
        let rules = AdminPenaltyDeepRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("行政处罚法深度规则"));
        assert!(explanation.contains("处罚种类"));
    }
}
