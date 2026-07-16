//! 诉讼时效深度规则
//!
//! 实现诉讼时效制度的详细规则验证，包括：
//! - 起算规则（权利受侵害之日、知道之日等）
//! - 中断规则（起诉、请求、承认等）
//! - 中止规则（不可抗力、障碍等）
//! - 延长规则（特殊情况）

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 诉讼时效类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitationType {
    /// 普通诉讼时效：3年
    General,
    /// 特殊诉讼时效：根据法律规定
    Special(u32),
    /// 最长保护期：20年
    Maximum,
}

/// 诉讼时效状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitationStatus {
    /// 时效进行中
    Running { remaining_days: u32 },
    /// 时效已届满
    Expired,
    /// 时效已中断
    Interrupted { reason: InterruptReason },
    /// 时效已中止
    Suspended { reason: SuspendReason },
}

/// 中断原因
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptReason {
    /// 权利人提起诉讼
    Litigation,
    /// 权利人申请仲裁
    Arbitration,
    /// 权利人向义务人提出履行请求
    Demand,
    /// 义务人同意履行义务
    Acknowledgment,
    /// 其他与提起诉讼或申请仲裁具有同等效力的情形
    EquivalentAction,
}

/// 中止原因
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuspendReason {
    /// 不可抗力
    ForceMajeure,
    /// 无民事行为能力人或限制民事行为能力人没有法定代理人
    NoLegalRepresentative,
    /// 法定代理人死亡、丧失代理权或丧失行为能力
    RepresentativeIncident,
    /// 继承开始后未确定继承人
    UndeterminedHeir,
    /// 权利人被义务人或其他人控制
    UnderControl,
    /// 其他导致权利人不能行使请求权的障碍
    OtherObstacle,
}

/// 诉讼时效起算参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitationStartParams {
    /// 权利受到侵害的日期
    pub injury_date: DateTime<Utc>,
    /// 权利人知道或应当知道权利受损的日期
    pub knowledge_date: Option<DateTime<Utc>>,
    /// 权利人知道或应当知道义务人的日期
    pub obligor_knowledge_date: Option<DateTime<Utc>>,
    /// 是否为分期履行债务
    pub is_installment: bool,
    /// 是否为未成年人受性侵害
    pub is_minor_victim: bool,
    /// 法定代理终止日期（针对未成年人受性侵害）
    pub legal_representation_end_date: Option<DateTime<Utc>>,
}

/// 诉讼时效验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitationResult {
    /// 是否在时效期间内
    pub within_limitation: bool,
    /// 剩余天数（如果未届满）
    pub remaining_days: Option<u32>,
    /// 时效状态
    pub status: LimitationStatus,
    /// 说明信息
    pub message: String,
}

simple_rule! {
    struct: StatuteOfLimitationsDeepRules,
    name: "诉讼时效深度规则",
    desc: "诉讼时效制度的详细规则验证",
    origin: "中国",
    tags: ["法律", "民法", "诉讼时效"]
}

impl StatuteOfLimitationsDeepRules {
    /// 计算诉讼时效起算点
    ///
    /// 起算规则：
    /// 1. 一般情况：自权利人知道或应当知道权利受到损害以及义务人之日起计算
    /// 2. 分期履行：自最后一期履行期限届满之日起计算
    /// 3. 未成年人受性侵害：自受害人年满18周岁之日起计算
    ///
    /// # Arguments
    /// * `params` - 起算参数
    ///
    /// # Returns
    /// 返回起算日期
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::statute_of_limitations_deep::{
    ///     StatuteOfLimitationsDeepRules, LimitationStartParams,
    /// };
    /// use chrono::{DateTime, Utc};
    ///
    /// let rules = StatuteOfLimitationsDeepRules::new();
    /// let params = LimitationStartParams {
    ///     injury_date: Utc::now(),
    ///     knowledge_date: Some(Utc::now()),
    ///     obligor_knowledge_date: Some(Utc::now()),
    ///     is_installment: false,
    ///     is_minor_victim: false,
    ///     legal_representation_end_date: None,
    /// };
    /// let start_date = rules.calculate_start_date(&params);
    /// ```
    pub fn calculate_start_date(&self, params: &LimitationStartParams) -> DateTime<Utc> {
        // 分期履行债务的特殊规则
        if params.is_installment {
            return params.injury_date;
        }

        // 未成年人受性侵害的特殊规则
        if params.is_minor_victim {
            if let Some(end_date) = params.legal_representation_end_date {
                return end_date;
            }
        }

        // 一般规则：自知道或应当知道权利受损以及义务人之日起计算
        match (params.knowledge_date, params.obligor_knowledge_date) {
            (Some(know_date), Some(obligor_date)) => {
                // 取较晚的日期作为起算点
                if know_date >= obligor_date {
                    know_date
                } else {
                    obligor_date
                }
            }
            (Some(know_date), None) => know_date,
            (None, Some(obligor_date)) => obligor_date,
            (None, None) => params.injury_date,
        }
    }

    /// 验证诉讼时效状态
    ///
    /// # Arguments
    /// * `limitation_type` - 时效类型
    /// * `start_date` - 起算日期
    /// * `current_date` - 当前日期
    /// * `interrupted` - 是否已中断
    /// * `suspended` - 是否已中止
    ///
    /// # Returns
    /// 返回时效验证结果
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::statute_of_limitations_deep::{
    ///     StatuteOfLimitationsDeepRules, LimitationType,
    /// };
    /// use chrono::{Duration, Utc};
    ///
    /// let rules = StatuteOfLimitationsDeepRules::new();
    /// let result = rules.validate_limitation(
    ///     LimitationType::General,
    ///     Utc::now() - Duration::days(365),
    ///     Utc::now(),
    ///     false,
    ///     false,
    /// );
    /// assert!(result.within_limitation);
    /// ```
    pub fn validate_limitation(
        &self,
        limitation_type: LimitationType,
        start_date: DateTime<Utc>,
        current_date: DateTime<Utc>,
        interrupted: bool,
        suspended: bool,
    ) -> LimitationResult {
        // 获取时效年数
        let limitation_years = match limitation_type {
            LimitationType::General => 3,
            LimitationType::Special(years) => years,
            LimitationType::Maximum => 20,
        };

        // 计算经过的天数
        let elapsed_days = (current_date - start_date).num_days() as u32;
        let total_days = limitation_years * 365;

        // 检查中断或中止状态
        if interrupted {
            return LimitationResult {
                within_limitation: true,
                remaining_days: Some(total_days),
                status: LimitationStatus::Interrupted {
                    reason: InterruptReason::Demand,
                },
                message: "诉讼时效已中断，重新起算".to_string(),
            };
        }

        if suspended {
            return LimitationResult {
                within_limitation: true,
                remaining_days: Some(total_days - elapsed_days),
                status: LimitationStatus::Suspended {
                    reason: SuspendReason::ForceMajeure,
                },
                message: "诉讼时效已中止，待中止事由消除后继续计算".to_string(),
            };
        }

        // 计算剩余天数
        let remaining_days = total_days.saturating_sub(elapsed_days);

        if remaining_days > 0 {
            LimitationResult {
                within_limitation: true,
                remaining_days: Some(remaining_days),
                status: LimitationStatus::Running { remaining_days },
                message: format!("诉讼时效进行中，剩余{}天", remaining_days),
            }
        } else {
            LimitationResult {
                within_limitation: false,
                remaining_days: None,
                status: LimitationStatus::Expired,
                message: "诉讼时效已届满，义务人可以提出不履行义务的抗辩".to_string(),
            }
        }
    }

    /// 检查时效中断情形
    ///
    /// 中断情形：
    /// 1. 权利人向义务人提出履行请求
    /// 2. 义务人同意履行义务
    /// 3. 权利人提起诉讼或申请仲裁
    ///
    /// # Arguments
    /// * `reason` - 中断原因
    /// * `occurred` - 中断事由是否发生
    ///
    /// # Returns
    /// 返回是否构成中断及新的起算点
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::statute_of_limitations_deep::{
    ///     StatuteOfLimitationsDeepRules, InterruptReason,
    /// };
    ///
    /// let rules = StatuteOfLimitationsDeepRules::new();
    /// let (interrupted, msg) = rules.check_interruption(InterruptReason::Demand, true);
    /// assert!(interrupted);
    /// ```
    pub fn check_interruption(&self, reason: InterruptReason, occurred: bool) -> (bool, String) {
        if !occurred {
            return (false, "未发生中断事由".to_string());
        }

        let msg = match reason {
            InterruptReason::Litigation => {
                "权利人提起诉讼，诉讼时效中断，自程序终结时重新计算".to_string()
            }
            InterruptReason::Arbitration => {
                "权利人申请仲裁，诉讼时效中断，自程序终结时重新计算".to_string()
            }
            InterruptReason::Demand => {
                "权利人提出履行请求，诉讼时效中断，自请求提出之日起重新计算".to_string()
            }
            InterruptReason::Acknowledgment => {
                "义务人同意履行，诉讼时效中断，自同意之日起重新计算".to_string()
            }
            InterruptReason::EquivalentAction => {
                "与起诉或申请仲裁具有同等效力的行为，诉讼时效中断".to_string()
            }
        };

        (true, msg)
    }

    /// 检查时效中止情形
    ///
    /// 中止条件：在诉讼时效期间的最后6个月内发生障碍
    ///
    /// # Arguments
    /// * `reason` - 中止原因
    /// * `elapsed_days` - 已经过的天数
    /// * `obstacle_resolved` - 障碍是否已消除
    ///
    /// # Returns
    /// 返回是否构成中止及剩余时效计算说明
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::statute_of_limitations_deep::{
    ///     StatuteOfLimitationsDeepRules, SuspendReason,
    /// };
    ///
    /// let rules = StatuteOfLimitationsDeepRules::new();
    /// let (suspended, msg) = rules.check_suspension(
    ///     SuspendReason::ForceMajeure,
    ///     1000, // 已经过的天数（约2.7年）
    ///     false,
    /// );
    /// assert!(suspended);
    /// ```
    pub fn check_suspension(
        &self,
        reason: SuspendReason,
        elapsed_days: u32,
        obstacle_resolved: bool,
    ) -> (bool, String) {
        // 中止只能在时效期间的最后6个月内发生
        let general_limitation_days = 3 * 365; // 普通时效1095天
        let last_6_months = general_limitation_days - 180; // 最后6个月开始于第915天

        if elapsed_days < last_6_months {
            return (
                false,
                "中止事由未发生在时效期间的最后6个月内，不构成中止".to_string(),
            );
        }

        if obstacle_resolved {
            return (
                true,
                "中止事由已消除，诉讼时效继续计算，剩余时效加上中止期间".to_string(),
            );
        }

        let msg = match reason {
            SuspendReason::ForceMajeure => {
                "因不可抗力导致诉讼时效中止，待不可抗力消除后继续计算".to_string()
            }
            SuspendReason::NoLegalRepresentative => {
                "无法定代理人导致诉讼时效中止，待法定代理人确定后继续计算".to_string()
            }
            SuspendReason::RepresentativeIncident => {
                "法定代理人丧失代理权导致诉讼时效中止".to_string()
            }
            SuspendReason::UndeterminedHeir => "继承开始后未确定继承人导致诉讼时效中止".to_string(),
            SuspendReason::UnderControl => {
                "权利人被控制导致诉讼时效中止，待控制解除后继续计算".to_string()
            }
            SuspendReason::OtherObstacle => {
                "其他障碍导致权利人不能行使请求权，诉讼时效中止".to_string()
            }
        };

        (true, msg)
    }

    /// 检查最长保护期
    ///
    /// 最长保护期：自权利受到损害之日起超过20年的，人民法院不予保护
    /// 有特殊情况的，人民法院可以根据权利人的申请决定延长
    ///
    /// # Arguments
    /// * `injury_date` - 权利受侵害日期
    /// * `current_date` - 当前日期
    /// * `has_special_circumstance` - 是否有特殊情况
    ///
    /// # Returns
    /// 返回是否超过最长保护期
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::statute_of_limitations_deep::StatuteOfLimitationsDeepRules;
    /// use chrono::{Duration, Utc};
    ///
    /// let rules = StatuteOfLimitationsDeepRules::new();
    /// let result = rules.check_maximum_period(
    ///     Utc::now() - Duration::days(365 * 10),
    ///     Utc::now(),
    ///     false,
    /// );
    /// assert!(result.0);
    /// ```
    pub fn check_maximum_period(
        &self,
        injury_date: DateTime<Utc>,
        current_date: DateTime<Utc>,
        has_special_circumstance: bool,
    ) -> (bool, String) {
        let elapsed_years = ((current_date - injury_date).num_days() / 365) as u32;

        if elapsed_years > 20 {
            if has_special_circumstance {
                (
                    true,
                    "虽超过20年最长保护期，但有特殊情况，可申请法院延长".to_string(),
                )
            } else {
                (false, "超过20年最长保护期，人民法院不予保护".to_string())
            }
        } else {
            (
                true,
                format!("未超过20年最长保护期（已过{}年）", elapsed_years),
            )
        }
    }

    /// 检查不适用诉讼时效的请求权
    ///
    /// 不适用诉讼时效的请求权：
    /// 1. 支付存款本金及利息请求权
    /// 2. 兑付国债、金融债券以及向不特定对象发行的企业债券本息请求权
    /// 3. 基于投资关系产生的缴付出资请求权
    /// 4. 其他依法不适用诉讼时效的请求权
    ///
    /// # Arguments
    /// * `claim_type` - 请求权类型
    ///
    /// # Returns
    /// 返回是否适用诉讼时效
    pub fn check_exemption(&self, claim_type: &str) -> (bool, String) {
        let exempt_claims = [
            "存款本金",
            "存款利息",
            "国债兑付",
            "金融债券兑付",
            "企业债券兑付",
            "缴付出资",
            "抚养费",
            "扶养费",
            "赡养费",
        ];

        for exempt in exempt_claims.iter() {
            if claim_type.contains(exempt) {
                return (false, format!("{}请求权不适用诉讼时效", claim_type));
            }
        }

        (true, format!("{}请求权适用诉讼时效", claim_type))
    }

    /// 诉讼时效起算规则详解
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "起算规则一：自权利人知道或应当知道权利受到损害之日起计算",
            "起算规则二：自权利人知道或应当知道义务人之日起计算",
            "起算规则三：分期履行债务，自最后一期履行期限届满之日起计算",
            "起算规则四：无民事行为能力人或限制民事行为能力人对法定代理人的请求权，自法定代理终止之日起计算",
            "起算规则五：未成年人受性侵害的损害赔偿请求权，自受害人年满18周岁之日起计算",
            "起算规则六：撤销权自权利人知道或应当知道撤销事由之日起计算",
            "起算规则七：继续性侵权行为的诉讼时效，自侵权行为终了之日起计算",
            "起算规则八：债务履行期限届满之日起计算债权时效",
        ]
    }

    /// 诉讼时效中断规则详解
    pub fn interrupt_rules(&self) -> Vec<&'static str> {
        vec![
            "中断情形一：权利人向义务人提出履行请求",
            "中断情形二：义务人同意履行义务",
            "中断情形三：权利人提起诉讼",
            "中断情形四：权利人申请仲裁",
            "中断情形五：与提起诉讼或申请仲裁具有同等效力的其他情形",
            "中断效果：诉讼时效中断，从中断时起，诉讼时效期间重新计算",
            "中断效力：诉讼时效中断，已经过的时效期间归于无效",
            "多次中断：诉讼时效可以多次中断，每次中断后重新起算",
            "起诉后撤诉：起诉后撤诉的，不发生中断效力",
            "连带债权：连带债权人之一发生中断事由，对其他连带债权人也发生效力",
        ]
    }

    /// 诉讼时效应中止规则详解
    pub fn suspend_rules(&self) -> Vec<&'static str> {
        vec![
            "中止情形一：不可抗力",
            "中止情形二：无民事行为能力人或限制民事行为能力人没有法定代理人",
            "中止情形三：法定代理人死亡、丧失代理权或丧失行为能力",
            "中止情形四：继承开始后未确定继承人",
            "中止情形五：权利人被义务人或其他人控制",
            "中止情形六：其他导致权利人不能行使请求权的障碍",
            "中止时间：在诉讼时效期间的最后6个月内发生",
            "中止效果：中止事由消除后，诉讼时效继续计算",
            "中止期间：中止期间不计入诉讼时效期间",
            "障碍消除：中止事由消除之日起满6个月，诉讼时效期间届满",
        ]
    }

    /// 诉讼时效延长规则详解
    pub fn extend_rules(&self) -> Vec<&'static str> {
        vec![
            "延长条件：有特殊情况，人民法院可以根据权利人的申请决定延长",
            "延长申请：权利人应当提出延长申请，并说明特殊情况",
            "特殊情况：权利人因障碍不能行使请求权，且障碍持续至时效届满后",
            "延长决定：人民法院根据具体情况决定是否延长及延长的期限",
            "延长程序：延长申请应当在时效届满后合理期限内提出",
            "延长限制：最长保护期20年的延长需特别充分的理由",
            "延长情形：自然灾害、战争等不可抗力导致的障碍",
            "延长情形：权利人丧失行为能力且无法定代理人",
        ]
    }

    /// 诉讼时效不适用规则详解
    pub fn exemption_rules(&self) -> Vec<&'static str> {
        vec![
            "不适用一：支付存款本金及利息请求权",
            "不适用二：兑付国债、金融债券以及向不特定对象发行的企业债券本息请求权",
            "不适用三：基于投资关系产生的缴付出资请求权",
            "不适用四：抚养费、扶养费、赡养费请求权",
            "不适用五：人格权请求权（停止侵害、排除妨碍、消除危险）",
            "不适用六：物权请求权（返还财产、恢复原状）",
            "不适用七：知识产权请求权（停止侵权、消除影响）",
            "不适用八：其他依法不适用诉讼时效的请求权",
        ]
    }

    /// 诉讼时效抗辩规则详解
    pub fn defense_rules(&self) -> Vec<&'static str> {
        vec![
            "抗辩权：诉讼时效届满后，义务人可以提出不履行义务的抗辩",
            "抗辩放弃：诉讼时效届满后，义务人同意履行的，不得再主张时效抗辩",
            "抗辩限制：义务人已自愿履行的，不得请求返还",
            "抗辩性质：诉讼时效抗辩需由义务人主动提出",
            "法院职责：人民法院不得主动适用诉讼时效的规定",
            "抗辩时间：诉讼时效抗辩应当在一审期间提出",
            "连带债务：连带债务人之一主张时效抗辩，对其他债务人不发生效力",
            "保证责任：主债务诉讼时效届满，保证人可以主张时效抗辩",
        ]
    }
}

impl Rule for StatuteOfLimitationsDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("statute_of_limitations_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "诉讼时效深度规则",
            &[
                ("起算规则详解", &self.start_rules()),
                ("中断规则详解", &self.interrupt_rules()),
                ("中止规则详解", &self.suspend_rules()),
                ("延长规则详解", &self.extend_rules()),
                ("不适用规则详解", &self.exemption_rules()),
                ("抗辩规则详解", &self.defense_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_statute_of_limitations_deep_rules_creation() {
        let rules = StatuteOfLimitationsDeepRules::new();
        assert_eq!(rules.metadata().name, "诉讼时效深度规则");
    }

    #[test]
    fn test_calculate_start_date_general() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let injury_date = Utc::now() - Duration::days(100);
        let knowledge_date = Utc::now() - Duration::days(50);

        let params = LimitationStartParams {
            injury_date,
            knowledge_date: Some(knowledge_date),
            obligor_knowledge_date: Some(knowledge_date),
            is_installment: false,
            is_minor_victim: false,
            legal_representation_end_date: None,
        };

        let start_date = rules.calculate_start_date(&params);
        assert!((start_date - knowledge_date).num_days().abs() < 1);
    }

    #[test]
    fn test_calculate_start_date_installment() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let injury_date = Utc::now() - Duration::days(100);

        let params = LimitationStartParams {
            injury_date,
            knowledge_date: Some(Utc::now()),
            obligor_knowledge_date: Some(Utc::now()),
            is_installment: true,
            is_minor_victim: false,
            legal_representation_end_date: None,
        };

        let start_date = rules.calculate_start_date(&params);
        assert!((start_date - injury_date).num_days().abs() < 1);
    }

    #[test]
    fn test_validate_limitation_within_period() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let start_date = Utc::now() - Duration::days(365); // 1年前

        let result = rules.validate_limitation(
            LimitationType::General,
            start_date,
            Utc::now(),
            false,
            false,
        );

        assert!(result.within_limitation);
        assert!(matches!(result.status, LimitationStatus::Running { .. }));
    }

    #[test]
    fn test_validate_limitation_expired() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let start_date = Utc::now() - Duration::days(4 * 365); // 4年前

        let result = rules.validate_limitation(
            LimitationType::General,
            start_date,
            Utc::now(),
            false,
            false,
        );

        assert!(!result.within_limitation);
        assert!(matches!(result.status, LimitationStatus::Expired));
    }

    #[test]
    fn test_validate_limitation_interrupted() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let start_date = Utc::now() - Duration::days(365);

        let result = rules.validate_limitation(
            LimitationType::General,
            start_date,
            Utc::now(),
            true, // 已中断
            false,
        );

        assert!(result.within_limitation);
        assert!(matches!(
            result.status,
            LimitationStatus::Interrupted { .. }
        ));
    }

    #[test]
    fn test_check_interruption_litigation() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let (interrupted, _) = rules.check_interruption(InterruptReason::Litigation, true);
        assert!(interrupted);
    }

    #[test]
    fn test_check_interruption_not_occurred() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let (interrupted, _) = rules.check_interruption(InterruptReason::Demand, false);
        assert!(!interrupted);
    }

    #[test]
    fn test_check_suspension_in_last_six_months() {
        let rules = StatuteOfLimitationsDeepRules::new();
        // 普通时效3年，最后6个月从第915天开始（1095-180）
        let elapsed_days = 950; // 在最后6个月内

        let (suspended, _) =
            rules.check_suspension(SuspendReason::ForceMajeure, elapsed_days, false);

        assert!(suspended);
    }

    #[test]
    fn test_check_suspension_not_in_last_six_months() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let elapsed_days = 100; // 不在最后6个月内

        let (suspended, _) =
            rules.check_suspension(SuspendReason::ForceMajeure, elapsed_days, false);

        assert!(!suspended);
    }

    #[test]
    fn test_check_maximum_period_within() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let injury_date = Utc::now() - Duration::days(10 * 365);

        let (valid, _) = rules.check_maximum_period(injury_date, Utc::now(), false);
        assert!(valid);
    }

    #[test]
    fn test_check_maximum_period_exceeded() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let injury_date = Utc::now() - Duration::days(25 * 365);

        let (valid, _) = rules.check_maximum_period(injury_date, Utc::now(), false);
        assert!(!valid);
    }

    #[test]
    fn test_check_exemption_deposit() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let (applicable, _) = rules.check_exemption("存款本金返还请求");
        assert!(!applicable);
    }

    #[test]
    fn test_check_exemption_general() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let (applicable, _) = rules.check_exemption("借款返还请求");
        assert!(applicable);
    }

    #[test]
    fn test_start_rules_not_empty() {
        let rules = StatuteOfLimitationsDeepRules::new();
        assert!(!rules.start_rules().is_empty());
    }

    #[test]
    fn test_interrupt_rules_not_empty() {
        let rules = StatuteOfLimitationsDeepRules::new();
        assert!(!rules.interrupt_rules().is_empty());
    }

    #[test]
    fn test_suspend_rules_not_empty() {
        let rules = StatuteOfLimitationsDeepRules::new();
        assert!(!rules.suspend_rules().is_empty());
    }

    #[test]
    fn test_category() {
        let rules = StatuteOfLimitationsDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("statute_of_limitations_deep")
        );
    }

    #[test]
    fn test_explain() {
        let rules = StatuteOfLimitationsDeepRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("诉讼时效深度规则"));
    }
}
