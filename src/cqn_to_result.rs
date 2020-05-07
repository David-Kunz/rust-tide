// use crate::cqn;
// use crate::cqn::SQL;
// use sqlx::sqlite::SqliteQueryAs;
// use sqlx::FromRow;
// use crate::State;

// pub async fn SELECT_to_result<T>(select: &cqn::SELECT, state: &State) -> Result<T,_>
// where T: Send
//  {
//     let res = sqlx::query_as::<_, T>(&select.to_sql())
//         .fetch_all(&state.pool)
//         .await;
//     res
// }
