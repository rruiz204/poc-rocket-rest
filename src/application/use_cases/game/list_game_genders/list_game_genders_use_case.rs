use super::{
  list_game_genders_query::ListGameGendersQuery,
  list_game_genders_response::ListGameGendersResponse,
};

pub struct ListGameGendersUseCase {}

impl ListGameGendersUseCase {
  pub fn new() -> Self {
    Self { }
  }

  pub async fn execute(&self, query: ListGameGendersQuery) -> Vec<ListGameGendersResponse> {
    println!("page: {}, limit: {}", query.page, query.limit);

    let genders: Vec<ListGameGendersResponse> = vec![
      ListGameGendersResponse::new(1, "Game Gender #1".to_string()),
      ListGameGendersResponse::new(2, "Game Gender #2".to_string()),
      ListGameGendersResponse::new(3, "Game Gender #3".to_string()),
    ];

    genders
  }
}