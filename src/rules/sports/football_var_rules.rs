//! VAR技术规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// VAR审查类型
#[derive(Debug, Clone, Copy)]
pub enum VARReviewType {
    /// 进球审查
    GoalReview,
    /// 点球审查
    PenaltyReview,
    /// 红牌审查
    RedCardReview,
    /// 身份错误审查
    IdentityReview,
}

/// VAR技术规则详解
pub struct FootballVARRules {
    metadata: RuleMetadata,
}

impl FootballVARRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("VAR技术规则详解", "视频助理裁判技术的完整规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "VAR".into()]),
        }
    }

    /// VAR应用场景
    pub fn applicable_scenarios(&self) -> Vec<&'static str> {
        vec![
            "进球判定",
            "点球判定",
            "红牌判定",
            "身份错误纠正",
            "四种场景限定",
        ]
    }

    /// VAR审查流程
    pub fn review_process(&self) -> Vec<&'static str> {
        vec![
            "VAR监测比赛",
            "发现潜在错误",
            "通知主裁判",
            "主裁判决定审查",
            "视频回放分析",
            "最终裁定",
        ]
    }

    /// 进球审查内容
    pub fn goal_review_checklist(&self) -> Vec<&'static str> {
        vec![
            "进球有效性",
            "越位判定",
            "犯规判定",
            "手球判定",
            "球是否过线",
            "干扰 goalkeeper",
        ]
    }

    /// 点球审查内容
    pub fn penalty_review_checklist(&self) -> Vec<&'static str> {
        vec![
            "犯规是否发生",
            "犯规位置",
            "犯规严重性",
            "防守方犯规",
            "进攻方犯规",
            "虚假犯规",
        ]
    }

    /// 红牌审查内容
    pub fn red_card_review_checklist(&self) -> Vec<&'static str> {
        vec![
            "犯规严重性",
            "暴力行为",
            "犯规意图",
            "犯规程度",
            "是否阻止明显进球",
            "是否应该降为黄牌",
        ]
    }

    /// VAR裁判角色
    pub fn var_roles(&self) -> Vec<&'static str> {
        vec![
            "VAR(视频助理裁判)",
            "AVAR(助理VAR)",
            "VAR裁判助理",
            "视频操作员",
            "主裁判",
            "现场裁判",
        ]
    }

    /// VAR技术设备
    pub fn technology_equipment(&self) -> Vec<&'static str> {
        vec![
            "多个摄像头角度",
            "高清回放设备",
            "实时传输系统",
            "越位线绘制",
            "视频分析软件",
            "通讯系统",
        ]
    }

    /// VAR时间限制
    pub fn time_constraints(&self) -> Vec<&'static str> {
        vec![
            "尽量不影响比赛流畅",
            "快速审查",
            "主裁判决定时间",
            "通常不超过几分钟",
            "复杂情况可能较长",
            "暂停时间记录",
        ]
    }

    /// VAR争议
    pub fn controversies(&self) -> Vec<&'static str> {
        vec![
            "主观判定争议",
            "技术精度争议",
            "比赛流畅影响",
            "时间延迟争议",
            "裁判权威争议",
            "观众接受度",
        ]
    }

    /// VAR改进措施
    pub fn improvements(&self) -> Vec<&'static str> {
        vec![
            "半自动越位系统",
            "更清晰规则",
            "裁判培训",
            "技术升级",
            "观众信息提供",
            "透明度提升",
        ]
    }

    /// 计算VAR审查时间(模拟)
    pub fn estimate_review_time(&self, review_type: VARReviewType) -> u8 {
        match review_type {
            VARReviewType::GoalReview => 45,     // 进球审查约45秒
            VARReviewType::PenaltyReview => 60,  // 点球审查约60秒
            VARReviewType::RedCardReview => 90,  // 红牌审查约90秒
            VARReviewType::IdentityReview => 30, // 身份审查约30秒
        }
    }
}

impl Default for FootballVARRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballVARRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_var")
    }

    fn explain(&self) -> String {
        format!(
            "【VAR技术规则详解】\n\n\
            应用场景:\n{}\n\n\
            审查流程:\n{}\n\n\
            进球审查内容:\n{}\n\n\
            点球审查内容:\n{}\n\n\
            红牌审查内容:\n{}\n",
            self.applicable_scenarios()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.review_process()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.goal_review_checklist()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.penalty_review_checklist()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.red_card_review_checklist()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_applicable_scenarios() {
        let rules = FootballVARRules::new();
        let scenarios = rules.applicable_scenarios();
        assert!(scenarios.contains(&"进球判定"));
        assert!(scenarios.contains(&"点球判定"));
    }

    #[test]
    fn test_review_process() {
        let rules = FootballVARRules::new();
        let process = rules.review_process();
        assert!(process.contains(&"VAR监测比赛"));
        assert!(process.contains(&"最终裁定"));
    }

    #[test]
    fn test_goal_review_checklist() {
        let rules = FootballVARRules::new();
        let checklist = rules.goal_review_checklist();
        assert!(checklist.contains(&"进球有效性"));
        assert!(checklist.contains(&"越位判定"));
    }

    #[test]
    fn test_estimate_review_time() {
        let rules = FootballVARRules::new();

        // 进球审查时间
        let goal_time = rules.estimate_review_time(VARReviewType::GoalReview);
        assert!(goal_time >= 30 && goal_time <= 60);

        // 点球审查时间
        let penalty_time = rules.estimate_review_time(VARReviewType::PenaltyReview);
        assert!(penalty_time >= 40 && penalty_time <= 80);

        // 红牌审查时间
        let red_time = rules.estimate_review_time(VARReviewType::RedCardReview);
        assert!(red_time >= 60 && red_time <= 120);
    }

    #[test]
    fn test_var_roles() {
        let rules = FootballVARRules::new();
        let roles = rules.var_roles();
        assert!(roles.contains(&"VAR(视频助理裁判)"));
        assert!(roles.contains(&"主裁判"));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballVARRules::new();
        assert_eq!(rules.metadata().name, "VAR技术规则详解");
        assert_eq!(rules.category(), RuleCategory::sports("football_var"));
    }
}
