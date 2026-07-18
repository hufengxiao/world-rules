//! 电子竞技裁判通用规则
//!
//! 涵盖各类电子竞技项目的裁判职责、判罚标准、争议处理等通用规则，
//! 适用于 Valorant、Overwatch、PUBG、Fortnite 等电竞项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 电子竞技裁判职责规则
#[derive(Debug, Clone)]
pub struct EsportsRefereeDutiesRules {
    metadata: RuleMetadata,
}

impl EsportsRefereeDutiesRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("电子竞技裁判职责规则", "电子竞技比赛裁判职责和工作规范")
                .with_origin("IESF国际电子竞技联合会")
                .with_tags(vec!["电竞".into(), "裁判".into(), "规则".into()]),
        }
    }

    /// 主裁判职责
    pub fn head_referee_duties(&self) -> Vec<&'static str> {
        vec![
            "负责比赛整体流程和重大判罚",
            "协调助理裁判和技术裁判工作",
            "处理争议和申诉",
            "决定比赛暂停和恢复",
            "签发比赛结果确认",
            "处理突发事件",
            "与赛事方沟通协调",
            "提交比赛报告",
        ]
    }

    /// 助理裁判职责
    pub fn assistant_referee_duties(&self) -> Vec<&'static str> {
        vec![
            "协助主裁判执行比赛规则",
            "记录比赛数据和统计",
            "观察比赛中的违规行为",
            "协助处理技术问题",
            "维护比赛秩序",
            "协助装备检查",
            "记录暂停和违规",
            "辅助争议处理",
        ]
    }

    /// 技术裁判职责
    pub fn technical_referee_duties(&self) -> Vec<&'static str> {
        vec![
            "检查比赛设备和网络",
            "处理技术故障和问题",
            "验证选手装备合规性",
            "协助解决游戏Bug",
            "维护比赛服务器",
            "处理重连和断线问题",
            "记录技术问题",
            "技术问题报告",
        ]
    }

    /// 裁判培训要求
    pub fn training_requirements(&self) -> Vec<&'static str> {
        vec![
            "电竞规则知识培训",
            "游戏机制深入理解",
            "争议处理培训",
            "沟通技巧培训",
            "应急处理培训",
            "职业道德培训",
            "技术设备操作培训",
            "心理素质培训",
        ]
    }
}

impl Default for EsportsRefereeDutiesRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EsportsRefereeDutiesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("esports_referee_duties")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电子竞技裁判职责规则",
            &[
                ("主裁判职责", &self.head_referee_duties()),
                ("助理裁判职责", &self.assistant_referee_duties()),
                ("技术裁判职责", &self.technical_referee_duties()),
                ("培训要求", &self.training_requirements()),
            ],
        )
    }
}

/// 电子竞技判罚标准规则
#[derive(Debug, Clone)]
pub struct EsportsPenaltyRules {
    metadata: RuleMetadata,
}

impl EsportsPenaltyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("电子竞技判罚标准规则", "电子竞技比赛判罚标准和处罚措施")
                .with_origin("IESF国际电子竞技联合会")
                .with_tags(vec!["电竞".into(), "裁判".into(), "规则".into()]),
        }
    }

    /// 作弊行为判罚
    pub fn cheating_penalties(&self) -> Vec<&'static str> {
        vec![
            "使用外挂/作弊软件：永久禁赛",
            "利用游戏漏洞：取消比赛成绩",
            "篡改游戏数据：永久禁赛",
            "使用未授权硬件：取消资格",
            "代打行为：永久禁赛",
            "账号共享：取消比赛资格",
            "假赛行为：永久禁赛+罚款",
            "跨队串通：取消双方资格",
        ]
    }

    /// 技术违规判罚
    pub fn technical_violations(&self) -> Vec<&'static str> {
        vec![
            "未授权软件：警告或取消资格",
            "设备改装：取消装备使用资格",
            "网络干扰：技术暂停",
            "游戏设置违规：强制重置",
            "非法通讯：警告或判负",
            "屏幕录制/直播：取消资格",
            "通讯设备使用：警告或判负",
            "延迟报到：警告或判负",
        ]
    }

    /// 行为违规判罚
    pub fn behavioral_violations(&self) -> Vec<&'static str> {
        vec![
            "不当言论：警告、罚款或禁赛",
            "肢体冲突：禁赛或永久禁赛",
            "挑衅对手：警告或罚款",
            "侮辱裁判：警告或禁赛",
            "干扰比赛：警告或判负",
            "拒绝采访：警告或罚款",
            "违反着装规定：警告或罚款",
            "未按规定时间到场：警告或判负",
        ]
    }

    /// 处罚程序
    pub fn penalty_procedure(&self) -> Vec<&'static str> {
        vec![
            "当场警告：轻微违规",
            "回合判负：利用Bug或技术违规",
            "比赛判负：严重违规或多次违规",
            "取消资格：作弊或极端行为",
            "追加处罚：赛后审查后决定",
            "申诉程序：24小时内提交",
            "证据要求：需提供视频或日志",
            "最终裁决：裁判长决定",
        ]
    }

    /// 罚款标准
    pub fn fine_standards(&self) -> Vec<&'static str> {
        vec![
            "轻微违规：$100-500",
            "一般违规：$500-2000",
            "严重违规：$2000-10000",
            "作弊行为：$10000+永久禁赛",
            "假赛行为：$50000+永久禁赛",
            "不当言论：$1000-5000",
            "延迟报到：$200-1000",
            "违反着装：$100-500",
        ]
    }
}

impl Default for EsportsPenaltyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EsportsPenaltyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("esports_penalty")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电子竞技判罚标准规则",
            &[
                ("作弊行为判罚", &self.cheating_penalties()),
                ("技术违规判罚", &self.technical_violations()),
                ("行为违规判罚", &self.behavioral_violations()),
                ("处罚程序", &self.penalty_procedure()),
                ("罚款标准", &self.fine_standards()),
            ],
        )
    }
}

/// 电子竞技争议处理规则
#[derive(Debug, Clone)]
pub struct EsportsDisputeResolutionRules {
    metadata: RuleMetadata,
}

impl EsportsDisputeResolutionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("电子竞技争议处理规则", "电子竞技比赛争议处理和申诉程序")
                .with_origin("IESF国际电子竞技联合会")
                .with_tags(vec!["电竞".into(), "裁判".into(), "规则".into()]),
        }
    }

    /// 争议类型
    pub fn dispute_types(&self) -> Vec<&'static str> {
        vec![
            "游戏Bug争议",
            "技术故障争议",
            "判罚争议",
            "装备争议",
            "身份争议",
            "赛程争议",
            "积分争议",
            "资格争议",
        ]
    }

    /// 申诉程序
    pub fn appeal_procedure(&self) -> Vec<&'static str> {
        vec![
            "申诉提交：比赛结束后24小时内",
            "申诉材料：书面申诉+证据",
            "申诉费用：$500保证金",
            "初审：裁判组审查",
            "听证会：必要时举行",
            "裁决：裁判长最终决定",
            "结果通知：5个工作日内",
            "再次申诉：不可再次申诉",
        ]
    }

    /// 证据要求
    pub fn evidence_requirements(&self) -> Vec<&'static str> {
        vec![
            "视频证据：比赛录像片段",
            "日志文件：游戏日志和系统日志",
            "截图证据：违规行为截图",
            "聊天记录：游戏内聊天记录",
            "技术报告：技术人员出具",
            "证人证词：其他选手或观众",
            "专家意见：游戏专家分析",
            "数据统计：比赛数据分析",
        ]
    }

    /// 技术问题处理
    pub fn technical_issue_handling(&self) -> Vec<&'static str> {
        vec![
            "游戏崩溃：回合重赛（影响结果时）",
            "网络断线：技术暂停等待重连",
            "设备故障：技术暂停更换设备",
            "服务器问题：暂停比赛等待修复",
            "选手掉线：等待最多5分钟",
            "无法重连：回合判负或比赛延期",
            "Bug利用：回合判负",
            "恶意断线：比赛判负",
        ]
    }

    /// 判罚争议处理
    pub fn penalty_dispute_handling(&self) -> Vec<&'static str> {
        vec![
            "当场解释：裁判说明判罚理由",
            "申诉窗口：比赛结束后24小时",
            "证据提交：需提供充分证据",
            "裁判组审查：裁判组集体讨论",
            "裁判长裁决：最终决定权",
            "维持原判：申诉失败",
            "改判决定：申诉成功",
            "申诉结果通知：5个工作日",
        ]
    }
}

impl Default for EsportsDisputeResolutionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EsportsDisputeResolutionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("esports_dispute_resolution")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电子竞技争议处理规则",
            &[
                ("争议类型", &self.dispute_types()),
                ("申诉程序", &self.appeal_procedure()),
                ("证据要求", &self.evidence_requirements()),
                ("技术问题处理", &self.technical_issue_handling()),
                ("判罚争议处理", &self.penalty_dispute_handling()),
            ],
        )
    }
}

/// 电子竞技比赛管理规则
#[derive(Debug, Clone)]
pub struct EsportsMatchManagementRules {
    metadata: RuleMetadata,
}

impl EsportsMatchManagementRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("电子竞技比赛管理规则", "电子竞技比赛流程管理和组织规范")
                .with_origin("IESF国际电子竞技联合会")
                .with_tags(vec!["电竞".into(), "裁判".into(), "规则".into()]),
        }
    }

    /// 赛前准备
    pub fn pre_match_preparation(&self) -> Vec<&'static str> {
        vec![
            "设备检查：提前30分钟完成",
            "选手验证：身份和装备检查",
            "游戏设置：验证游戏版本",
            "网络测试：确保连接稳定",
            "选手入场：提前10分钟",
            "战术准备：赛前5分钟",
            "热身时间：赛前3分钟",
            "比赛开始：准时开始",
        ]
    }

    /// 比赛流程
    pub fn match_flow(&self) -> Vec<&'static str> {
        vec![
            "选手就位：指定座位",
            "设备确认：选手确认设备正常",
            "游戏加载：等待所有选手加载",
            "比赛开始：裁判宣布开始",
            "比赛进行：裁判监督",
            "暂停管理：裁判批准暂停",
            "比赛结束：确认比赛结果",
            "赛后检查：选手确认并签字",
        ]
    }

    /// 暂停管理
    pub fn pause_management(&self) -> Vec<&'static str> {
        vec![
            "战术暂停：每队每场2次，每次60秒",
            "技术暂停：裁判判定，无限次",
            "医疗暂停：健康问题，最长5分钟",
            "暂停请求：仅教练或队长",
            "暂停时机：回合结束或死球",
            "暂停记录：记录暂停原因和时间",
            "暂停后恢复：裁判宣布恢复",
            "恶意暂停：可能受处罚",
        ]
    }

    /// 赛后流程
    pub fn post_match_procedure(&self) -> Vec<&'static str> {
        vec![
            "结果确认：选手确认比赛结果",
            "数据保存：保存比赛录像",
            "设备归还：选手归还设备",
            "采访安排：胜者接受采访",
            "成绩上报：上报比赛结果",
            "奖金发放：按规定发放",
            "赛后报告：裁判撰写报告",
            "争议记录：记录争议和处理",
        ]
    }

    /// 应急处理
    pub fn emergency_handling(&self) -> Vec<&'static str> {
        vec![
            "突发断电：暂停比赛",
            "火灾警报：疏散选手和观众",
            "医疗急救：暂停比赛，急救",
            "设备故障：更换设备",
            "网络中断：等待恢复",
            "选手伤病：医疗暂停",
            "观众骚乱：安保介入",
            "天气影响：室内比赛不受影响",
        ]
    }
}

impl Default for EsportsMatchManagementRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EsportsMatchManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("esports_match_management")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电子竞技比赛管理规则",
            &[
                ("赛前准备", &self.pre_match_preparation()),
                ("比赛流程", &self.match_flow()),
                ("暂停管理", &self.pause_management()),
                ("赛后流程", &self.post_match_procedure()),
                ("应急处理", &self.emergency_handling()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referee_duties_rules() {
        let rules = EsportsRefereeDutiesRules::new();
        assert!(!rules.head_referee_duties().is_empty());
        assert!(!rules.assistant_referee_duties().is_empty());
        assert!(!rules.technical_referee_duties().is_empty());
        assert!(!rules.training_requirements().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_penalty_rules() {
        let rules = EsportsPenaltyRules::new();
        assert!(!rules.cheating_penalties().is_empty());
        assert!(!rules.technical_violations().is_empty());
        assert!(!rules.behavioral_violations().is_empty());
        assert!(!rules.penalty_procedure().is_empty());
        assert!(!rules.fine_standards().is_empty());
    }

    #[test]
    fn test_dispute_resolution_rules() {
        let rules = EsportsDisputeResolutionRules::new();
        assert!(!rules.dispute_types().is_empty());
        assert!(!rules.appeal_procedure().is_empty());
        assert!(!rules.evidence_requirements().is_empty());
        assert!(!rules.technical_issue_handling().is_empty());
        assert!(!rules.penalty_dispute_handling().is_empty());
    }

    #[test]
    fn test_match_management_rules() {
        let rules = EsportsMatchManagementRules::new();
        assert!(!rules.pre_match_preparation().is_empty());
        assert!(!rules.match_flow().is_empty());
        assert!(!rules.pause_management().is_empty());
        assert!(!rules.post_match_procedure().is_empty());
        assert!(!rules.emergency_handling().is_empty());
    }

    #[test]
    fn test_metadata() {
        let rules = EsportsRefereeDutiesRules::new();
        assert_eq!(rules.metadata().name, "电子竞技裁判职责规则");
        assert!(rules.metadata().tags.contains(&"电竞".to_string()));
    }

    #[test]
    fn test_category() {
        let rules = EsportsRefereeDutiesRules::new();
        let category = rules.category();
        assert!(category.to_string().contains("esports"));
    }

    #[test]
    fn test_validate() {
        let rules = EsportsRefereeDutiesRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        let result = rules.validate(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_explain_format() {
        let rules = EsportsRefereeDutiesRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("电子竞技裁判职责规则"));
        assert!(explanation.contains("主裁判职责"));
    }

    #[test]
    fn test_cheating_penalty_severity() {
        let rules = EsportsPenaltyRules::new();
        let penalties = rules.cheating_penalties();
        assert!(penalties.iter().any(|r| r.contains("永久禁赛")));
    }

    #[test]
    fn test_appeal_deadline() {
        let rules = EsportsDisputeResolutionRules::new();
        let procedure = rules.appeal_procedure();
        assert!(procedure.iter().any(|r| r.contains("24小时")));
    }

    #[test]
    fn test_pause_limit() {
        let rules = EsportsMatchManagementRules::new();
        let pause_rules = rules.pause_management();
        assert!(pause_rules.iter().any(|r| r.contains("2次")));
    }
}
