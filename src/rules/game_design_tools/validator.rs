//! 游戏规则验证器
//!
//! 提供游戏规则验证功能，检查规则的完整性、一致性和可执行性。
//!
//! # 示例
//!
//! ```rust
//! use world_rules::rules::game_design_tools::validator::*;
//!
//! let validator = GameRuleValidator::new();
//! let report = validator.validate_basic("围棋", "古老棋类游戏", vec!["黑先白后", "落子无悔"]);
//! assert!(report.is_valid);
//! ```

use serde::{Deserialize, Serialize};

/// 游戏规则验证器
///
/// 验证游戏规则的完整性、一致性和可执行性。
///
/// # 示例
///
/// ```rust
/// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
///
/// let validator = GameRuleValidator::new();
/// assert!(validator.strict_mode);
/// ```
#[derive(Debug, Clone)]
pub struct GameRuleValidator {
    /// 严格模式（检查更多细节）
    pub strict_mode: bool,
    /// 必需规则列表
    pub required_rules: Vec<String>,
    /// 规则冲突检测配置
    pub conflict_detection: bool,
}

/// 验证报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// 游戏名称
    pub game_name: String,
    /// 是否有效
    pub is_valid: bool,
    /// 总体分数（0.0-1.0）
    pub score: f64,
    /// 完整性检查结果
    pub completeness: CompletenessResult,
    /// 一致性检查结果
    pub consistency: ConsistencyResult,
    /// 发现的错误
    pub errors: Vec<ValidationError>,
    /// 警告信息
    pub warnings: Vec<String>,
}

/// 完整性检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletenessResult {
    /// 是否完整
    pub is_complete: bool,
    /// 缺失的规则
    pub missing_rules: Vec<String>,
    /// 缺失的属性
    pub missing_attributes: Vec<String>,
    /// 完整性分数
    pub score: f64,
}

/// 一致性检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyResult {
    /// 是否一致
    pub is_consistent: bool,
    /// 冲突列表
    pub conflicts: Vec<RuleConflict>,
    /// 一致性分数
    pub score: f64,
}

/// 规则冲突
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConflict {
    /// 冲突规则1
    pub rule1: String,
    /// 冲突规则2
    pub rule2: String,
    /// 冲突描述
    pub description: String,
    /// 严重程度
    pub severity: ConflictSeverity,
}

/// 冲突严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictSeverity {
    /// 低（建议修复）
    Low,
    /// 中（应该修复）
    Medium,
    /// 高（必须修复）
    High,
    /// 严重（阻止验证通过）
    Critical,
}

/// 验证错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 错误位置
    pub location: Option<String>,
    /// 错误类型
    pub error_type: ValidationErrorType,
}

/// 验证错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationErrorType {
    /// 语法错误
    Syntax,
    /// 逻辑错误
    Logic,
    /// 缺失错误
    Missing,
    /// 冲突错误
    Conflict,
    /// 格式错误
    Format,
}

impl Default for GameRuleValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl GameRuleValidator {
    /// 创建新的规则验证器
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new();
    /// assert!(validator.strict_mode);
    /// ```
    pub fn new() -> Self {
        Self {
            strict_mode: true,
            required_rules: vec![
                "开始条件".to_string(),
                "结束条件".to_string(),
                "胜负判定".to_string(),
                "玩家行动".to_string(),
            ],
            conflict_detection: true,
        }
    }

    /// 设置严格模式
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new().with_strict_mode(false);
    /// assert!(!validator.strict_mode);
    /// ```
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// 设置必需规则
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new()
    ///     .with_required_rules(vec!["开始", "结束"]);
    /// assert_eq!(validator.required_rules.len(), 2);
    /// ```
    pub fn with_required_rules(mut self, rules: Vec<&str>) -> Self {
        self.required_rules = rules.iter().map(|s| s.to_string()).collect();
        self
    }

    /// 基本验证
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new();
    /// let report = validator.validate_basic("围棋", "古老棋类游戏", vec!["黑先白后"]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn validate_basic(
        &self,
        game_name: &str,
        description: &str,
        rules: Vec<&str>,
    ) -> ValidationReport {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut missing_rules = Vec::new();

        // 检查必需规则
        for required in &self.required_rules {
            let found = rules.iter().any(|r| r.contains(required));
            if !found {
                missing_rules.push(required.clone());
                if self.strict_mode {
                    errors.push(ValidationError {
                        code: "MISSING_RULE".to_string(),
                        message: format!("缺少必需规则: {}", required),
                        location: None,
                        error_type: ValidationErrorType::Missing,
                    });
                } else {
                    warnings.push(format!("建议添加规则: {}", required));
                }
            }
        }

        // 检查描述
        if description.is_empty() {
            warnings.push("游戏描述为空".to_string());
        }

        // 检查规则数量
        if rules.is_empty() {
            errors.push(ValidationError {
                code: "NO_RULES".to_string(),
                message: "没有定义任何规则".to_string(),
                location: None,
                error_type: ValidationErrorType::Missing,
            });
        }

        // 计算分数
        let completeness_score = if self.required_rules.is_empty() {
            1.0
        } else {
            1.0 - (missing_rules.len() as f64 / self.required_rules.len() as f64)
        };

        let is_valid = errors.is_empty() || !self.strict_mode;

        ValidationReport {
            game_name: game_name.to_string(),
            is_valid,
            score: completeness_score,
            completeness: CompletenessResult {
                is_complete: missing_rules.is_empty(),
                missing_rules,
                missing_attributes: Vec::new(),
                score: completeness_score,
            },
            consistency: ConsistencyResult {
                is_consistent: true,
                conflicts: Vec::new(),
                score: 1.0,
            },
            errors,
            warnings,
        }
    }

    /// 验证规则一致性
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new();
    /// let report = validator.validate_consistency("围棋", vec![
    ///     ("规则1", "玩家A先手"),
    ///     ("规则2", "玩家B先手"),
    /// ]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn validate_consistency(
        &self,
        game_name: &str,
        rules: Vec<(&str, &str)>,
    ) -> ValidationReport {
        let mut conflicts = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();

        // 检测冲突（简单字符串匹配）
        if self.conflict_detection && rules.len() >= 2 {
            for i in 0..rules.len() {
                for j in (i + 1)..rules.len() {
                    let (name1, content1) = &rules[i];
                    let (name2, content2) = &rules[j];

                    // 检测相互矛盾的内容
                    if self.are_contradictory(content1, content2) {
                        let conflict = RuleConflict {
                            rule1: name1.to_string(),
                            rule2: name2.to_string(),
                            description: format!("规则 '{}' 和 '{}' 存在矛盾", name1, name2),
                            severity: ConflictSeverity::High,
                        };
                        conflicts.push(conflict);
                    }
                }
            }
        }

        // 添加冲突错误
        for conflict in &conflicts {
            if conflict.severity == ConflictSeverity::High
                || conflict.severity == ConflictSeverity::Critical
            {
                errors.push(ValidationError {
                    code: "RULE_CONFLICT".to_string(),
                    message: conflict.description.clone(),
                    location: Some(format!("{} vs {}", conflict.rule1, conflict.rule2)),
                    error_type: ValidationErrorType::Conflict,
                });
            }
        }

        let is_valid = errors.is_empty();
        let consistency_score = if conflicts.is_empty() {
            1.0
        } else {
            1.0 - (conflicts.len() as f64 / rules.len().max(1) as f64)
        };

        ValidationReport {
            game_name: game_name.to_string(),
            is_valid,
            score: consistency_score,
            completeness: CompletenessResult {
                is_complete: true,
                missing_rules: Vec::new(),
                missing_attributes: Vec::new(),
                score: 1.0,
            },
            consistency: ConsistencyResult {
                is_consistent: conflicts.is_empty(),
                conflicts,
                score: consistency_score,
            },
            errors,
            warnings,
        }
    }

    /// 验证规则格式
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new();
    /// let report = validator.validate_format("围棋", vec![
    ///     ("回合时间", "每回合60秒"),
    /// ]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn validate_format(&self, game_name: &str, rules: Vec<(&str, &str)>) -> ValidationReport {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for (name, content) in &rules {
            // 检查规则名称
            if name.is_empty() {
                errors.push(ValidationError {
                    code: "EMPTY_RULE_NAME".to_string(),
                    message: "规则名称为空".to_string(),
                    location: None,
                    error_type: ValidationErrorType::Format,
                });
            }

            // 检查规则内容
            if content.is_empty() {
                warnings.push(format!("规则 '{}' 内容为空", name));
            }

            // 检查格式（严格模式）
            if self.strict_mode && !content.ends_with('.') && !content.ends_with('。') {
                warnings.push(format!("规则 '{}' 建议以句号结尾", name));
            }
        }

        let is_valid = errors.is_empty();
        let format_score = if rules.is_empty() {
            1.0
        } else {
            1.0 - (errors.len() as f64 / rules.len() as f64)
        };

        ValidationReport {
            game_name: game_name.to_string(),
            is_valid,
            score: format_score,
            completeness: CompletenessResult {
                is_complete: true,
                missing_rules: Vec::new(),
                missing_attributes: Vec::new(),
                score: 1.0,
            },
            consistency: ConsistencyResult {
                is_consistent: true,
                conflicts: Vec::new(),
                score: 1.0,
            },
            errors,
            warnings,
        }
    }

    /// 全面验证
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new();
    /// let report = validator.validate_full("围棋", "古老棋类游戏", vec![
    ///     ("黑先白后", "黑方先行"),
    ///     ("落子无悔", "落子后不能更改"),
    /// ]);
    /// assert!(report.score >= 0.0);
    /// ```
    pub fn validate_full(
        &self,
        game_name: &str,
        description: &str,
        rules: Vec<(&str, &str)>,
    ) -> ValidationReport {
        let rule_names: Vec<&str> = rules.iter().map(|(name, _)| *name).collect();

        // 基本验证
        let basic_report = self.validate_basic(game_name, description, rule_names);

        // 一致性验证
        let consistency_report = self.validate_consistency(game_name, rules.clone());

        // 格式验证
        let format_report = self.validate_format(game_name, rules);

        // 合并结果
        let total_score =
            (basic_report.score + consistency_report.score + format_report.score) / 3.0;
        let mut all_errors = basic_report.errors.clone();
        all_errors.extend(consistency_report.errors);
        all_errors.extend(format_report.errors);

        let mut all_warnings = basic_report.warnings.clone();
        all_warnings.extend(consistency_report.warnings);
        all_warnings.extend(format_report.warnings);

        let is_valid = all_errors.is_empty() || !self.strict_mode;

        ValidationReport {
            game_name: game_name.to_string(),
            is_valid,
            score: total_score,
            completeness: basic_report.completeness,
            consistency: consistency_report.consistency,
            errors: all_errors,
            warnings: all_warnings,
        }
    }

    /// 检测两个规则是否矛盾（简化版本）
    fn are_contradictory(&self, rule1: &str, rule2: &str) -> bool {
        // 检测明显矛盾的词汇对
        let contradictions = [
            ("先手", "后手"),
            ("开始", "结束"),
            ("增加", "减少"),
            ("允许", "禁止"),
            ("必须", "不能"),
        ];

        for (word1, word2) in &contradictions {
            if (rule1.contains(word1) && rule2.contains(word2))
                || (rule1.contains(word2) && rule2.contains(word1))
            {
                return true;
            }
        }

        false
    }
}

impl ValidationReport {
    /// 生成报告（Markdown格式）
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::rules::game_design_tools::validator::GameRuleValidator;
    ///
    /// let validator = GameRuleValidator::new();
    /// let report = validator.validate_basic("围棋", "游戏", vec!["规则"]);
    /// let md = report.generate_report();
    /// assert!(md.contains("围棋"));
    /// ```
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("# {} 规则验证报告\n\n", self.game_name));
        report.push_str(&format!(
            "**验证结果**: {} (分数: {:.2})\n\n",
            if self.is_valid {
                "✅ 通过"
            } else {
                "❌ 失败"
            },
            self.score
        ));

        // 完整性
        report.push_str("## 完整性检查\n\n");
        report.push_str(&format!(
            "- **状态**: {}\n",
            if self.completeness.is_complete {
                "✅ 完整"
            } else {
                "⚠️ 不完整"
            }
        ));
        report.push_str(&format!("- **分数**: {:.2}\n", self.completeness.score));

        if !self.completeness.missing_rules.is_empty() {
            report.push_str("\n**缺失规则**:\n");
            for rule in &self.completeness.missing_rules {
                report.push_str(&format!("- {}\n", rule));
            }
        }
        report.push('\n');

        // 一致性
        report.push_str("## 一致性检查\n\n");
        report.push_str(&format!(
            "- **状态**: {}\n",
            if self.consistency.is_consistent {
                "✅ 一致"
            } else {
                "⚠️ 存在冲突"
            }
        ));
        report.push_str(&format!("- **分数**: {:.2}\n", self.consistency.score));

        if !self.consistency.conflicts.is_empty() {
            report.push_str("\n**冲突列表**:\n");
            for conflict in &self.consistency.conflicts {
                report.push_str(&format!(
                    "- {} vs {}: {}\n",
                    conflict.rule1, conflict.rule2, conflict.description
                ));
            }
        }
        report.push('\n');

        // 错误
        if !self.errors.is_empty() {
            report.push_str("## 错误\n\n");
            for error in &self.errors {
                report.push_str(&format!(
                    "- **{}**: {} ({:?})\n",
                    error.code, error.message, error.error_type
                ));
            }
            report.push('\n');
        }

        // 警告
        if !self.warnings.is_empty() {
            report.push_str("## 警告\n\n");
            for warning in &self.warnings {
                report.push_str(&format!("- ⚠️ {}\n", warning));
            }
            report.push('\n');
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = GameRuleValidator::new();
        assert!(validator.strict_mode);
        assert!(!validator.required_rules.is_empty());
    }

    #[test]
    fn test_basic_validation() {
        let validator = GameRuleValidator::new();
        let report = validator.validate_basic("围棋", "古老棋类游戏", vec!["开始条件", "结束条件"]);

        assert!(report.score >= 0.0);
        assert_eq!(report.game_name, "围棋");
    }

    #[test]
    fn test_validation_with_missing_rules() {
        let validator = GameRuleValidator::new().with_strict_mode(true);
        let report = validator.validate_basic("测试游戏", "描述", vec![]);

        assert!(!report.completeness.missing_rules.is_empty());
    }

    #[test]
    fn test_consistency_validation() {
        let validator = GameRuleValidator::new();
        let report = validator
            .validate_consistency("游戏", vec![("规则1", "玩家A先手"), ("规则2", "玩家B先手")]);

        assert!(!report.consistency.conflicts.is_empty());
    }

    #[test]
    fn test_format_validation() {
        let validator = GameRuleValidator::new().with_strict_mode(true);
        let report = validator.validate_format("游戏", vec![("回合时间", "每回合60秒")]);

        assert!(report.score >= 0.0);
    }

    #[test]
    fn test_full_validation() {
        let validator = GameRuleValidator::new();
        let report = validator.validate_full(
            "围棋",
            "古老棋类游戏",
            vec![("黑先白后", "黑方先行"), ("落子无悔", "落子后不能更改")],
        );

        assert!(report.score >= 0.0);
        assert!(report.score <= 1.0);
    }

    #[test]
    fn test_report_generation() {
        let validator = GameRuleValidator::new();
        let report = validator.validate_basic("围棋", "游戏", vec!["规则"]);
        let md = report.generate_report();

        assert!(md.contains("围棋"));
        assert!(md.contains("验证报告"));
    }

    #[test]
    fn test_non_strict_mode() {
        let validator = GameRuleValidator::new().with_strict_mode(false);
        let report = validator.validate_basic("游戏", "", vec![]);

        assert!(report.is_valid);
    }

    #[test]
    fn test_contradiction_detection() {
        let validator = GameRuleValidator::new();

        assert!(validator.are_contradictory("玩家A先手", "玩家A后手"));
        assert!(!validator.are_contradictory("玩家A行动", "玩家B行动"));
    }
}
