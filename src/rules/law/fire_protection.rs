//! 消防法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 消防法规则
pub struct FireProtectionLawRules {
    metadata: RuleMetadata,
}

impl FireProtectionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "消防法规则",
                "中国消防法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "消防".into()]),
        }
    }

    /// 消防安全责任
    pub fn fire_safety_responsibility(&self) -> Vec<&'static str> {
        vec![
            "单位消防安全责任",
            "消防安全责任人职责",
            "消防安全管理人职责",
            "员工消防安全义务",
            "消防值班人员责任",
            "消防设施维护责任",
            "消防检查整改责任",
            "火灾扑救配合责任",
        ]
    }

    /// 消防设施要求
    pub fn fire_facilities(&self) -> Vec<&'static str> {
        vec![
            "消防设施配备",
            "消防器材配置",
            "自动报警系统",
            "自动喷淋系统",
            "消防栓设置",
            "灭火器配置",
            "消防通道设置",
            "应急照明疏散",
        ]
    }

    /// 消防安全管理
    pub fn fire_safety_management(&self) -> Vec<&'static str> {
        vec![
            "消防检查制度",
            "隐患排查整改",
            "消防培训教育",
            "消防演练开展",
            "消防档案管理",
            "消防值班制度",
            "动火作业管理",
            "易燃易爆管理",
        ]
    }

    /// 建筑消防要求
    pub fn building_fire_requirements(&self) -> Vec<&'static str> {
        vec![
            "建筑消防设计",
            "防火分区设置",
            "防火门配置",
            "消防电梯设置",
            "疏散通道设计",
            "安全出口设置",
            "防火间距要求",
            "消防水源配置",
        ]
    }

    /// 消防行政许可
    pub fn fire_administrative_approval(&self) -> Vec<&'static str> {
        vec![
            "消防设计审核",
            "消防竣工验收",
            "消防开业检查",
            "消防许可申请",
            "消防验收程序",
            "消防备案制度",
            "消防许可变更",
            "消防许可撤销",
        ]
    }

    /// 火灾预防措施
    pub fn fire_prevention(&self) -> Vec<&'static str> {
        vec![
            "火灾风险评估",
            "火灾隐患排查",
            "易燃物品管理",
            "电器安全使用",
            "明火使用管控",
            "吸烟区域管理",
            "装饰材料防火",
            "仓储防火管理",
        ]
    }

    /// 火灾应急处理
    pub fn fire_emergency_response(&self) -> Vec<&'static str> {
        vec![
            "火灾报警程序",
            "火灾扑救组织",
            "人员疏散组织",
            "火灾扑救配合",
            "火灾扑救支援",
            "火灾现场保护",
            "火灾调查配合",
            "火灾善后处理",
        ]
    }

    /// 消防违法行为
    pub fn fire_violations(&self) -> Vec<&'static str> {
        vec![
            "消防设施缺失",
            "消防通道堵塞",
            "无证动火作业",
            "易燃物品违规",
            "消防值班缺失",
            "隐患未整改",
            "妨碍消防扑救",
            "虚假消防申报",
        ]
    }
}

impl Default for FireProtectionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FireProtectionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("fire_protection")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【消防法规则】\n\n安全责任:\n{}\n\n消防设施:\n{}\n\n安全管理:\n{}\n",
            self.fire_safety_responsibility().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fire_facilities().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fire_safety_management().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_protection_law_rules() {
        let rules = FireProtectionLawRules::new();
        assert!(!rules.fire_safety_responsibility().is_empty());
        assert!(!rules.fire_facilities().is_empty());
    }
}