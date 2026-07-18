//! # 规则知识图谱模块
//!
//! 提供规则实体抽取、关系抽取、图谱构建和推理功能。
//!
//! ## 功能特性
//!
//! - **规则实体抽取**: 从规则定义中提取关键实体
//! - **规则关系抽取**: 提取实体之间的关系
//! - **规则图谱构建**: 构建知识图谱数据结构
//! - **规则推理引擎**: 基于知识图谱进行推理
//!
//! ## 示例
//!
//! ```rust
//! use world_rules::knowledge_graph::{EntityExtractor, KnowledgeGraph, EntityType};
//!
//! // 创建实体抽取器
//! let extractor = EntityExtractor::new();
//!
//! // 从文本中抽取实体
//! let text = "球员在比赛中犯规会被出示黄牌";
//! let entities = extractor.extract(text);
//!
//! // 构建知识图谱
//! let mut graph = KnowledgeGraph::new("体育规则");
//! graph.add_entities(entities);
//! ```

pub mod entity;
pub mod relation;
pub mod graph;
pub mod inference;

pub use entity::{Entity, EntityExtractor, EntityType};
pub use relation::{Relation, RelationExtractor, RelationType};
pub use graph::{KnowledgeGraph, GraphNode, GraphEdge, GraphStats};
pub use inference::{InferenceEngine, InferenceResult, InferenceRule};

/// 知识图谱模块的公共接口
pub mod prelude {
    pub use crate::knowledge_graph::{
        Entity, EntityExtractor, EntityType,
        Relation, RelationExtractor, RelationType,
        KnowledgeGraph, GraphNode, GraphEdge, GraphStats,
        InferenceEngine, InferenceResult, InferenceRule,
    };
}