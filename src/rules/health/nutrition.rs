//! 营养规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 营养规则
pub struct NutritionRules {
    metadata: RuleMetadata,
}

impl NutritionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "营养规则",
                "健康营养基础知识"
            )
            .with_origin("营养学")
            .with_tags(vec!["健康".into(), "营养".into()]),
        }
    }

    /// 三大营养素
    pub fn macronutrients(&self) -> Vec<&'static str> {
        vec![
            "碳水化合物: 主要能量来源，占总热量50-65%",
            "蛋白质: 组织修复和生长，占总热量10-15%",
            "脂肪: 能量储存和细胞功能，占总热量20-30%",
            "每克碳水约4千卡热量",
            "每克蛋白质约4千卡热量",
            "每克脂肪约9千卡热量",
        ]
    }

    /// 维生素
    pub fn vitamins(&self) -> Vec<&'static str> {
        vec![
            "维生素A: 视力、免疫，来源于胡萝卜、肝脏",
            "维生素B族: 能量代谢，来源于谷物、肉类",
            "维生素C: 免疫、抗氧化，来源于水果蔬菜",
            "维生素D: 骨骼健康，来源于阳光照射、鱼类",
            "维生素E: 抗氧化，来源于坚果、植物油",
            "维生素K: 血液凝固，来源于绿叶蔬菜",
        ]
    }

    /// 矿物质
    pub fn minerals(&self) -> Vec<&'static str> {
        vec![
            "钙: 骨骼牙齿健康，来源于奶制品、豆腐",
            "铁: 血红蛋白合成，来源于红肉、菠菜",
            "锌: 免疫功能，来源于海鲜、肉类",
            "镁: 肌肉神经功能，来源于坚果、全谷物",
            "钾: 心脏功能，来源于香蕉、土豆",
            "钠: 水盐平衡，来源于食盐",
        ]
    }

    /// 膳食建议
    pub fn dietary_guidelines(&self) -> Vec<&'static str> {
        vec![
            "食物多样化，每天12种以上食物",
            "每天摄入300-500克蔬菜",
            "每天摄入200-350克水果",
            "每天摄入300克奶制品",
            "每天摄入50-150克全谷物",
            "限制添加糖摄入(<25克/天)",
            "限制钠摄入(<5克盐/天)",
            "限制饱和脂肪摄入",
        ]
    }

    /// 饮水建议
    pub fn water_intake(&self) -> Vec<&'static str> {
        vec![
            "成人每天饮水1500-1700毫升",
            "运动时增加饮水量",
            "少量多次饮水",
            "不要等到口渴再喝水",
            "避免含糖饮料",
            "早晨起床后喝一杯水",
        ]
    }

    /// 健康饮食原则
    pub fn healthy_eating_principles(&self) -> Vec<&'static str> {
        vec![
            "均衡饮食: 各类食物合理搭配",
            "适量饮食: 控制总热量摄入",
            "多样化: 不挑食不偏食",
            "定时定量: 规律进餐",
            "细嚼慢咽: 有助消化",
            "少油少盐少糖: 减少慢性病风险",
        ]
    }
}

impl Default for NutritionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NutritionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nutrition")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【营养规则】\n\n\
            三大营养素:\n{}\n\n\
            膳食建议:\n{}\n\n\
            饮水建议:\n{}\n",
            self.macronutrients().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.dietary_guidelines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.water_intake().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutrition_rules() {
        let rules = NutritionRules::new();
        assert!(!rules.macronutrients().is_empty());
    }
}