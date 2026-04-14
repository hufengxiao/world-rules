//! 信息科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 信息科学定律集合
pub struct InformationScienceLaws {
    metadata: RuleMetadata,
}

impl InformationScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "信息科学定律",
                "信息科学基本定律"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "信息".into()]),
        }
    }

    /// 信息理论定律
    pub fn theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("香农熵定律", "H = -Σp log p", "信息熵度量"),
            ("信息量定律", "I = -log p", "信息量计算"),
            ("信道容量定律", "C = max I(X;Y)", "信道传输能力"),
            ("信源编码定律", "无损压缩极限", "信源编码定理"),
            ("信道编码定律", "可靠传输极限", "信道编码定理"),
            ("互信息定律", "I(X;Y)", "变量信息关联"),
            ("信息传输定律", "传输效率", "信息传输规律"),
            ("冗余定律", "冗余信息", "信息冗余作用"),
        ]
    }

    /// 编码定律
    pub fn coding_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("编码定律", "编码方法", "信息编码原理"),
            ("压缩定律", "数据压缩", "数据压缩方法"),
            ("加密定律", "信息加密", "信息加密原理"),
            ("纠错定律", "错误纠正", "纠错编码方法"),
            (" Huffman编码定律", "最优编码", "Huffman编码"),
            ("算术编码定律", "高效编码", "算术编码方法"),
            ("信道编码定律", "可靠编码", "信道编码技术"),
        ]
    }

    /// 信息处理定律
    pub fn processing_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("信息采集定律", "采集方法", "信息采集原理"),
            ("信息存储定律", "存储技术", "信息存储方法"),
            ("信息检索定律", "检索效率", "信息检索原理"),
            ("信息处理定律", "处理方法", "信息处理技术"),
            ("信息分析定律", "分析方法", "信息分析技术"),
            ("信息可视化定律", "可视化", "信息呈现方法"),
            ("信息安全定律", "安全保护", "信息安全保障"),
        ]
    }

    /// 数据定律
    pub fn data_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("数据定律", "数据特性", "数据基本特性"),
            ("大数据定律", "大数据处理", "大数据分析方法"),
            ("数据挖掘定律", "挖掘方法", "数据挖掘技术"),
            ("数据质量定律", "质量标准", "数据质量要求"),
            ("数据完整性定律", "完整性", "数据完整性保障"),
            ("数据隐私定律", "隐私保护", "数据隐私保护"),
            ("数据治理定律", "治理方法", "数据治理原则"),
        ]
    }

    /// 信息类型
    pub fn information_types(&self) -> Vec<&'static str> {
        vec![
            "文本信息",
            "图像信息",
            "音频信息",
            "视频信息",
            "数值信息",
            "结构化信息",
            "非结构化信息",
            "元信息",
        ]
    }

    /// 信息技术
    pub fn technologies(&self) -> Vec<&'static str> {
        vec![
            "数据库技术",
            "数据挖掘",
            "机器学习",
            "自然语言处理",
            "计算机视觉",
            "信息检索",
            "知识图谱",
            "数据可视化",
        ]
    }
}

impl Default for InformationScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for InformationScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("information_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【信息科学定律】\n\n理论定律:\n{}\n\n编码定律:\n{}\n\n处理定律:\n{}\n",
            self.theory_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.coding_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.processing_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_information_science_laws() {
        let laws = InformationScienceLaws::new();
        assert!(!laws.theory_laws().is_empty());
        assert!(!laws.processing_laws().is_empty());
    }
}