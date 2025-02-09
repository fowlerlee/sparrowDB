// use arrow::datatypes::{DataType, Field, Schema};
// use datafusion::common::DFSchema;
// use datafusion::logical_expr::{col, lit};
// use datafusion::sql::unparser::Unparser;
// use datafusion::{
//     assert_batches_eq,
//     error::Result,
//     prelude::{ParquetReadOptions, SessionContext},
// };
// use datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate;
// use datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery;
// use datafusion_optimizer::optimizer::{Optimizer, OptimizerContext};
// use sqlparser::dialect::GenericDialect;
// use sqlparser::parser::Parser;

// #[test]
// fn query_plan_integration_test() {
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         let _sql = "SELECT a, b, 123, myfunc(b) \
//                FROM table_1 \
//                WHERE a > b AND b < 100 \
//                ORDER BY a DESC, b";
//         let sql2 = "SELECT a \
//                FROM table_1 \
//                WHERE a < 10";
//         let dialect = GenericDialect {};

//         let ast = Parser::parse_sql(&dialect, sql2).unwrap();

//         // println!("AST: {:?}", ast[0]);
//         let mut config = OptimizerContext::default();
//         let rule = std::sync::Arc::new(CommonSubexprEliminate::new());
//         let rule2 = std::sync::Arc::new(DecorrelatePredicateSubquery::new());
//         let optimizer = Optimizer::with_rules(vec![rule, rule2]);
//         // let optimized_plan = optimizer.optimize(&ast[0], &config, observe)?;
//     });
// }
