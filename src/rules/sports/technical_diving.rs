//! 技术潜水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 技术潜水规则 (深潜/洞穴/沉船)
pub struct TechnicalDivingRules {
    metadata: RuleMetadata,
}

impl TechnicalDivingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("技术潜水规则", "技术潜水标准与规则")
                .with_origin("国际")
                .with_tags(vec![
                    "体育".into(),
                    "水上".into(),
                    "潜水".into(),
                    "技术".into(),
                ]),
        }
    }

    /// 潜水类型
    pub fn diving_types(&self) -> Vec<&'static str> {
        vec![
            "深潜: 40-100米深度潜水",
            "洞穴潜水: 水下洞穴探索",
            "沉船潜水: 沉船内部探索",
            "减压潜水: 减压停留要求",
            "密闭环境: 狭窄空间潜水",
        ]
    }

    /// 认证要求
    pub fn certification_requirements(&self) -> Vec<&'static str> {
        vec![
            "基础技术潜水: 进阶开放水域+50潜",
            "洞穴潜水: 专门的洞穴潜水认证",
            "沉船渗透: 沉船潜水专长",
            "三混气潜水: Trimix认证",
            "密闭循环呼吸器: CCR认证",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "双气瓶系统: 双瓶配置",
            "独立气源: 备用气源系统",
            "减压气体: 高氧和纯氧",
            "潜水电脑: 多气体电脑",
            "线轮和指南针: 导航工具",
            "备用灯光: 多个备用光源",
            "减压浮标: 减压停留标记",
        ]
    }

    /// 气体规划
    pub fn gas_planning(&self) -> Vec<&'static str> {
        vec![
            "三分之一规则: 1/3去程，1/3回程，1/3备用",
            "气体分析: 气体成分确认",
            "氧气限制: 最大氧分压1.4-1.6",
            "氮醉限制: END不超过30-40米",
            "减压计划: 完整减压表",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "三之一规则: 严格遵守气体管理",
            "团队潜水: 最少2人团队",
            "紧急计划: 制定应急程序",
            "备份计划: 多个退出方案",
            "检查点: 关键决策点评估",
        ]
    }

    /// 减压程序
    pub fn decompression_procedures(&self) -> Vec<&'static str> {
        vec![
            "减压停留: 按计划深度停留",
            "气体切换: 按计划切换高氧",
            "氧气窗口: 利用氧气加速减压",
            "安全缓冲: 增加减压时间",
            "紧急减压: 备用减压方案",
        ]
    }

    /// 团队程序
    pub fn team_procedures(&self) -> Vec<&'static str> {
        vec![
            "潜伴系统: 随时保持联系",
            "手势沟通: 确认信号",
            "线导潜水: 使用引导线",
            "位置确认: 定期位置检查",
            "紧急协议: 统一应急程序",
        ]
    }

    /// 风险管理
    pub fn risk_management(&self) -> Vec<&'static str> {
        vec![
            "风险评估: 潜水前评估",
            "装备检查: 完整装备检查",
            "环境评估: 条件评估",
            "个人能力: 不超越训练",
            "中止决策: 可随时中止",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "单独潜水: 禁止单独技术潜水",
            "超越训练: 不超认证深度",
            "忽视减压: 遵守减压计划",
            "装备省略: 不可省略备用装备",
            "恶劣条件: 不在恶劣条件下潜水",
        ]
    }

    /// 健康要求
    pub fn health_requirements(&self) -> Vec<&'static str> {
        vec![
            "技术潜水体检: 年度体检",
            "心血管评估: 心脏健康",
            "呼吸系统: 肺功能正常",
            "心理评估: 心理稳定",
            "压力测试: 压力应对能力",
        ]
    }

    /// 训练要求
    pub fn training_requirements(&self) -> Vec<&'static str> {
        vec![
            "渐进训练: 逐步增加难度",
            "导师指导: 经验导师带领",
            "模拟练习: 紧急情况训练",
            "经验积累: 充足潜水经历",
            "持续学习: 定期更新知识",
        ]
    }
}

impl Default for TechnicalDivingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TechnicalDivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("technical_diving")
    }

    fn explain(&self) -> String {
        format!(
            "【技术潜水规则】\n\n\
            潜水类型:\n{}\n\n\
            认证要求:\n{}\n\n\
            安全规则:\n{}\n\n\
            气体规划:\n{}",
            self.diving_types()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.certification_requirements()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.gas_planning()
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
    fn test_technical_diving_rules() {
        let rules = TechnicalDivingRules::new();
        assert_eq!(rules.metadata().name, "技术潜水规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_diving_types() {
        let rules = TechnicalDivingRules::new();
        let types = rules.diving_types();
        assert!(types.iter().any(|t| t.contains("深潜")));
        assert!(types.iter().any(|t| t.contains("洞穴")));
        assert!(types.iter().any(|t| t.contains("沉船")));
        assert!(types.len() >= 5);
    }

    #[test]
    fn test_certification_requirements() {
        let rules = TechnicalDivingRules::new();
        let certs = rules.certification_requirements();
        assert!(certs.iter().any(|c| c.contains("基础技术")));
        assert!(certs.iter().any(|c| c.contains("洞穴")));
        assert!(certs.iter().any(|c| c.contains("Trimix")));
    }

    #[test]
    fn test_equipment() {
        let rules = TechnicalDivingRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("双气瓶")));
        assert!(equipment.iter().any(|e| e.contains("减压")));
        assert!(equipment.iter().any(|e| e.contains("潜水电脑")));
    }

    #[test]
    fn test_gas_planning() {
        let rules = TechnicalDivingRules::new();
        let gas = rules.gas_planning();
        assert!(gas.iter().any(|g| g.contains("三分之一")));
        assert!(gas.iter().any(|g| g.contains("氧分压")));
        assert!(gas.iter().any(|g| g.contains("减压")));
    }

    #[test]
    fn test_safety_rules() {
        let rules = TechnicalDivingRules::new();
        let safety = rules.safety_rules();
        assert!(safety.iter().any(|s| s.contains("团队")));
        assert!(safety.iter().any(|s| s.contains("应急")));
        assert!(safety.iter().any(|s| s.contains("三之一")));
    }

    #[test]
    fn test_decompression_procedures() {
        let rules = TechnicalDivingRules::new();
        let deco = rules.decompression_procedures();
        assert!(deco.iter().any(|d| d.contains("停留")));
        assert!(deco.iter().any(|d| d.contains("气体切换")));
        assert!(deco.iter().any(|d| d.contains("氧气")));
    }

    #[test]
    fn test_team_procedures() {
        let rules = TechnicalDivingRules::new();
        let team = rules.team_procedures();
        assert!(team.iter().any(|t| t.contains("潜伴")));
        assert!(team.iter().any(|t| t.contains("线导")));
    }

    #[test]
    fn test_risk_management() {
        let rules = TechnicalDivingRules::new();
        let risk = rules.risk_management();
        assert!(risk.iter().any(|r| r.contains("风险评估")));
        assert!(risk.iter().any(|r| r.contains("中止")));
    }

    #[test]
    fn test_prohibited_actions() {
        let rules = TechnicalDivingRules::new();
        let prohibited = rules.prohibited_actions();
        assert!(prohibited.iter().any(|p| p.contains("单独潜水")));
        assert!(prohibited.iter().any(|p| p.contains("超越训练")));
    }

    #[test]
    fn test_health_requirements() {
        let rules = TechnicalDivingRules::new();
        let health = rules.health_requirements();
        assert!(health.iter().any(|h| h.contains("体检")));
        assert!(health.iter().any(|h| h.contains("心理")));
    }

    #[test]
    fn test_training_requirements() {
        let rules = TechnicalDivingRules::new();
        let training = rules.training_requirements();
        assert!(training.iter().any(|t| t.contains("渐进")));
        assert!(training.iter().any(|t| t.contains("导师")));
    }
}
