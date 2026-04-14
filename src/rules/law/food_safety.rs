//! 食品安全法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 食品安全法规则
pub struct FoodSafetyLawRules {
    metadata: RuleMetadata,
}

impl FoodSafetyLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "食品安全法规则",
                "中国食品安全法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "食品安全".into()]),
        }
    }

    /// 食品安全原则
    pub fn food_safety_principles(&self) -> Vec<&'static str> {
        vec![
            "预防为主原则: 预防食品安全风险",
            "风险管理原则: 风险评估管理",
            "全程控制原则: 全链条监管",
            "社会共治原则: 多方参与治理",
            "信息公开原则: 安全信息公开",
            "责任追究原则: 严格责任追究",
            "科学监管原则: 科学依据监管",
            "消费者保护原则: 消费者权益优先",
        ]
    }

    /// 食品生产经营许可
    pub fn food_business_permit(&self) -> Vec<&'static str> {
        vec![
            "食品生产许可: 生产许可证",
            "食品经营许可: 经营许可证",
            "餐饮服务许可: 餐饮许可",
            "许可条件: 场所设备人员",
            "许可程序: 申请审查发证",
            "许可变更: 变更审批",
            "许可延续: 续期申请",
            "许可注销: 注销程序",
        ]
    }

    /// 食品安全标准
    pub fn food_safety_standards(&self) -> Vec<&'static str> {
        vec![
            "国家标准: 食品安全国标",
            "地方标准: 地方补充标准",
            "企业标准: 企业内控标准",
            "限量标准: 污染物限量",
            "添加剂标准: 添加剂使用规定",
            "标签标准: 标签标识要求",
            "检测方法标准: 检测方法规定",
            "标准强制执行: 强制执行效力",
        ]
    }

    /// 食品生产经营要求
    pub fn food_production_requirements(&self) -> Vec<&'static str> {
        vec![
            "场所要求: 生产场所条件",
            "设备要求: 生产设备配置",
            "人员要求: 人员健康培训",
            "原料要求: 原料采购验收",
            "过程要求: 生产过程控制",
            "包装要求: 包装材料规范",
            "储存运输: 储运条件控制",
            "记录要求: 生产记录保存",
        ]
    }

    /// 食品标签规则
    pub fn food_labeling_rules(&self) -> Vec<&'static str> {
        vec![
            "名称标识: 食品真实名称",
            "配料表: 配料成分标识",
            "净含量: 重量体积标识",
            "生产者信息: 生产者名称地址",
            "生产日期: 生产日期标识",
            "保质期: 保质期限标识",
            "储存条件: 储存方法说明",
            "营养成分表: 营养成分标识",
        ]
    }

    /// 食品添加剂管理
    pub fn food_additive_management(&self) -> Vec<&'static str> {
        vec![
            "添加剂品种: 允许使用品种",
            "使用范围: 使用范围限制",
            "使用限量: 最大使用量",
            "残留限量: 最大残留量",
            "添加剂标识: 标识标明要求",
            "添加剂记录: 使用记录保存",
            "新品种审批: 新品种申请",
            "禁止添加剂: 禁止使用种类",
        ]
    }

    /// 食品检验制度
    pub fn food_inspection(&self) -> Vec<&'static str> {
        vec![
            "抽样检验: 抽样检测制度",
            "委托检验: 委托检验机构",
            "检验机构资质: 检验资质要求",
            "检验方法规范: 方法标准执行",
            "检验结果处理: 结果处置规定",
            "检验费用承担: 费用承担规则",
            "复检制度: 复检申请程序",
            "检验信息公开: 结果公布",
        ]
    }

    /// 食品安全责任
    pub fn food_safety_liability(&self) -> Vec<&'static str> {
        vec![
            "行政责任: 罚款吊销许可",
            "民事责任: 损害赔偿十倍赔偿",
            "刑事责任: 严重违法入刑",
            "主体责任: 经营者主体责任",
            "连带责任: 相关方连带",
            "惩罚性赔偿: 惩罚性赔偿规定",
            "举证责任: 部分举证倒置",
            "消费者索赔: 索赔权利保护",
        ]
    }
}

impl Default for FoodSafetyLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FoodSafetyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("food_safety")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【食品安全法规则】\n\n安全原则:\n{}\n\n经营许可:\n{}\n\n安全标准:\n{}\n",
            self.food_safety_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.food_business_permit().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.food_safety_standards().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_food_safety_law_rules() {
        let rules = FoodSafetyLawRules::new();
        assert!(!rules.food_safety_principles().is_empty());
        assert!(!rules.food_business_permit().is_empty());
    }
}