//! 竞技屏气潜水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 竞技屏气潜水规则 (AIDA竞技)
pub struct ApneaDivingRules {
    metadata: RuleMetadata,
}

impl ApneaDivingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("竞技屏气潜水规则", "AIDA竞技屏气潜水竞赛规则")
                .with_origin("国际")
                .with_tags(vec![
                    "体育".into(),
                    "水上".into(),
                    "潜水".into(),
                    "竞技".into(),
                ]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "静态屏气(STA): 静止闭气时间记录",
            "动态屏气(DYN): 水平距离记录",
            "恒重下潜(CWT): 鳍泳下潜深度",
            "变重下潜(FIM): 绳索下潜深度",
            "无限制下潜(NLT): 最大深度挑战",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全潜水员: 每位运动员配安全潜水员",
            "医疗待命: 现场医疗团队",
            "深度限制: 根据训练等级限制",
            "救援程序: 标准化救援流程",
            "禁赛规则: 身体不适不得参赛",
        ]
    }

    /// 比赛程序
    pub fn competition_procedures(&self) -> Vec<&'static str> {
        vec![
            "宣布深度: 赛前宣布目标",
            "热身时间: 比赛前热身时段",
            "官方计时: 2分钟准备时间",
            "裁判监督: 水下和水面裁判",
            "成绩确认: 裁判签字确认",
        ]
    }

    /// 技术要求
    pub fn technical_requirements(&self) -> Vec<&'static str> {
        vec![
            "面镜: 禁止呼吸管",
            "脚蹼: 单蹼或双蹼",
            "配重: 固定配重限制",
            "绳索: 标准下潜绳",
            "底部盘: 深度标记确认",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "黑视/晕厥(BO): 取消比赛资格",
            "运动控制丧失(LMC): 成绩无效",
            "过早开始: 时间处罚",
            "触摸绳索: 恒重项目中犯规",
            "装备违规: 取消成绩",
        ]
    }

    /// 成绩判定
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "深度记录: 米为单位",
            "时间记录: 秒为单位",
            "距离记录: 米为单位",
            "白卡: 成绩有效",
            "红卡: 成绩无效",
        ]
    }

    /// 等级认证
    pub fn certification_levels(&self) -> Vec<&'static str> {
        vec![
            "初级潜水员: 基础自由潜水",
            "进阶潜水员: 20-30米深度",
            "自由潜水员: 30-40米深度",
            "竞技潜水员: 参加正式比赛",
            "教练等级: 教学认证",
        ]
    }

    /// 记录类型
    pub fn record_types(&self) -> Vec<&'static str> {
        vec![
            "世界记录: AIDA认证",
            "国家记录: 各国认证",
            "洲际记录: 区域认证",
            "个人最好: 个人记录",
            "比赛记录: 单场比赛记录",
        ]
    }

    /// 健康要求
    pub fn health_requirements(&self) -> Vec<&'static str> {
        vec![
            "医疗证明: 年度体检报告",
            "肺功能检查: 呼吸系统评估",
            "心脏检查: 心血管健康",
            "耳鼻喉检查: 压力平衡能力",
            "心理评估: 心理状态良好",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "单独潜水: 禁止单独训练",
            "过度换气: 危险呼吸技术",
            "隐瞒健康问题: 不诚实申报",
            "超越能力: 超出训练深度",
            "使用药物: 违禁物质使用",
        ]
    }
}

impl Default for ApneaDivingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ApneaDivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("apnea_diving")
    }

    fn explain(&self) -> String {
        format!(
            "【竞技屏气潜水规则】\n\n\
            比赛项目:\n{}\n\n\
            安全规则:\n{}\n\n\
            比赛程序:\n{}\n\n\
            技术要求:\n{}",
            self.competition_events()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.competition_procedures()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technical_requirements()
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
    fn test_apnea_diving_rules() {
        let rules = ApneaDivingRules::new();
        assert_eq!(rules.metadata().name, "竞技屏气潜水规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_competition_events() {
        let rules = ApneaDivingRules::new();
        let events = rules.competition_events();
        assert!(events.iter().any(|e| e.contains("静态屏气")));
        assert!(events.iter().any(|e| e.contains("恒重下潜")));
        assert!(events.iter().any(|e| e.contains("无限制")));
        assert!(events.len() >= 5);
    }

    #[test]
    fn test_safety_rules() {
        let rules = ApneaDivingRules::new();
        let safety = rules.safety_rules();
        assert!(safety.iter().any(|s| s.contains("安全潜水员")));
        assert!(safety.iter().any(|s| s.contains("医疗")));
    }

    #[test]
    fn test_competition_procedures() {
        let rules = ApneaDivingRules::new();
        let procedures = rules.competition_procedures();
        assert!(procedures.iter().any(|p| p.contains("宣布")));
        assert!(procedures.iter().any(|p| p.contains("裁判")));
    }

    #[test]
    fn test_technical_requirements() {
        let rules = ApneaDivingRules::new();
        let tech = rules.technical_requirements();
        assert!(tech.iter().any(|t| t.contains("面镜")));
        assert!(tech.iter().any(|t| t.contains("脚蹼")));
    }

    #[test]
    fn test_fouls() {
        let rules = ApneaDivingRules::new();
        let fouls = rules.fouls();
        assert!(fouls.iter().any(|f| f.contains("晕厥")));
        assert!(fouls.iter().any(|f| f.contains("LMC")));
    }

    #[test]
    fn test_scoring() {
        let rules = ApneaDivingRules::new();
        let scoring = rules.scoring();
        assert!(scoring.iter().any(|s| s.contains("白卡")));
        assert!(scoring.iter().any(|s| s.contains("红卡")));
    }

    #[test]
    fn test_certification_levels() {
        let rules = ApneaDivingRules::new();
        let levels = rules.certification_levels();
        assert!(levels.iter().any(|l| l.contains("初级")));
        assert!(levels.iter().any(|l| l.contains("竞技")));
    }

    #[test]
    fn test_record_types() {
        let rules = ApneaDivingRules::new();
        let records = rules.record_types();
        assert!(records.iter().any(|r| r.contains("世界记录")));
        assert!(records.iter().any(|r| r.contains("国家记录")));
    }

    #[test]
    fn test_health_requirements() {
        let rules = ApneaDivingRules::new();
        let health = rules.health_requirements();
        assert!(health.iter().any(|h| h.contains("医疗")));
        assert!(health.iter().any(|h| h.contains("心脏")));
    }

    #[test]
    fn test_prohibited_actions() {
        let rules = ApneaDivingRules::new();
        let prohibited = rules.prohibited_actions();
        assert!(prohibited.iter().any(|p| p.contains("单独潜水")));
        assert!(prohibited.iter().any(|p| p.contains("过度换气")));
    }
}
