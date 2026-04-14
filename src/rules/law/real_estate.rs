//! 房地产法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 房地产法规则
pub struct RealEstateLawRules {
    metadata: RuleMetadata,
}

impl RealEstateLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "房地产法规则",
                "中国房地产法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "房地产".into()]),
        }
    }

    /// 土地所有权制度
    pub fn land_ownership(&self) -> Vec<&'static str> {
        vec![
            "国家所有: 城市土地国有",
            "集体所有: 农村土地集体所有",
            "土地使用权: 有偿使用制度",
            "土地出让: 国家出让土地",
            "土地划拨: 无偿划拨使用",
            "土地流转: 农村土地流转",
            "土地征收: 国家征收土地",
            "土地收回: 国家收回土地",
        ]
    }

    /// 房屋所有权制度
    pub fn housing_ownership(&self) -> Vec<&'static str> {
        vec![
            "房屋所有权: 私有房屋权利",
            "房屋使用权: 使用房屋权利",
            "房屋处分权: 买卖转让权利",
            "共有房屋: 共有人权利义务",
            "房屋继承: 继承取得房屋",
            "房屋赠与: 赠与取得房屋",
            "房屋买卖: 买卖取得房屋",
            "房屋登记: 产权登记制度",
        ]
    }

    /// 房地产开发规则
    pub fn real_estate_development(&self) -> Vec<&'static str> {
        vec![
            "房地产开发许可",
            "土地使用权取得",
            "规划许可: 建设规划审批",
            "施工许可: 施工许可证",
            "预售许可: 商品房预售许可",
            "开发资质: 开发企业资质",
            "竣工验收: 项目验收",
            "交付使用: 房屋交付标准",
        ]
    }

    /// 房地产交易规则
    pub fn real_estate_transaction(&self) -> Vec<&'static str> {
        vec![
            "房屋买卖合同: 合同要件",
            "房屋租赁合同: 租赁规则",
            "房屋抵押: 抵押贷款",
            "房地产中介: 中介服务",
            "交易税费: 契税增值税等",
            "交易登记: 产权变更登记",
            "交易公示: 信息公示要求",
            "交易限制: 限制交易情形",
        ]
    }

    /// 房地产登记制度
    pub fn registration_system(&self) -> Vec<&'static str> {
        vec![
            "不动产登记: 统一登记制度",
            "登记类型: 首次登记变更登记",
            "登记程序: 申请审查登簿",
            "登记效力: 登记公示公信",
            "登记查询: 登记信息查询",
            "登记错误: 更正登记",
            "异议登记: 登记异议处理",
            "预告登记: 预登记制度",
        ]
    }

    /// 商品房销售规则
    pub fn commercial_housing_sales(&self) -> Vec<&'static str> {
        vec![
            "预售条件: 预售许可条件",
            "现售条件: 现售房屋条件",
            "销售广告: 广告真实性",
            "面积计算: 建筑面积规则",
            "面积误差: 面积差异处理",
            "交付标准: 交付质量要求",
            "保修责任: 房屋保修期限",
            "违约责任: 销售违约处理",
        ]
    }

    /// 物业管理制度
    pub fn property_management(&self) -> Vec<&'static str> {
        vec![
            "物业服务合同: 服务约定",
            "物业服务标准: 服务质量要求",
            "物业费收取: 费用收取规则",
            "业主委员会: 业主自治组织",
            "业主大会: 业主决策机构",
            "公共维修基金: 维修资金",
            "共有部分管理: 公共区域管理",
            "物业管理监督: 监管制度",
        ]
    }

    /// 房地产税费
    pub fn real_estate_taxes(&self) -> Vec<&'static str> {
        vec![
            "契税: 3%-5%",
            "增值税: 交易增值税",
            "个人所得税: 交易所得税",
            "房产税: 房产持有税",
            "土地增值税: 土地增值税",
            "印花税: 合同印花税",
            "城镇土地使用税",
            "城市维护建设税",
        ]
    }
}

impl Default for RealEstateLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RealEstateLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("real_estate")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【房地产法规则】\n\n土地所有权:\n{}\n\n房地产交易:\n{}\n\n登记制度:\n{}\n",
            self.land_ownership().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.real_estate_transaction().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.registration_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_estate_law_rules() {
        let rules = RealEstateLawRules::new();
        assert!(!rules.land_ownership().is_empty());
        assert!(!rules.registration_system().is_empty());
    }
}