


mod common;
#[cfg(test)]
mod tests {
    use crate::common;

    use actix_web::{test};
    use testcontainers::clients::Cli;
    use stocks_consumer::stock_functions::{save_stock, calculate_stock_summary};
    use stocks_consumer::persistence::repository::{get_stock_summary_by_symbol};
    #[test]
    async fn save_stocks() {
        let docker = Cli::default();
        let (_pg_container, pool) = common::setup(&docker);
        let result = 2 + 2;
        assert_eq!(result, 4);
        let symbo = "COST".to_string();
        let strocks_to_buy = 12;
        let strocks_to_sale = 2;
        save_stock(
            symbo.to_string(),
            12,
            "123.23".to_string(),
            "0.3".to_string(),
            "buy".to_string(),
            &mut pool.get().expect("Can't get DB connection")
        );
        save_stock(
            symbo.to_string(),
            2,
            "123.23".to_string(),
            "0.3".to_string(),
            "sell".to_string(),
            &mut pool.get().expect("Can't get DB connection")
        );
        calculate_stock_summary(
            symbo.to_string(),
            2,
            "123.23".to_string(),
            "0.3".to_string(),
            &mut pool.get().expect("Can't get DB connection")
        );

        let stock_summary_by_symbol = get_stock_summary_by_symbol(
            symbo.to_string(), &mut pool.get().expect("Can't get DB connection")
        );
        match stock_summary_by_symbol {
            None => {}
            _ => {
                assert_eq!(strocks_to_buy - strocks_to_sale, stock_summary_by_symbol.unwrap().first().unwrap().shares);
            },
        }
    }

    #[test]
    async fn validate_symbol() {
        let valid_symbol = "COST".to_string();
        let result = common_utils::get_stock_from_nasdaq(valid_symbol);
        assert_eq!(true, result.success);
        let invalid_symbol = "COST1223434".to_string();
        let result = common_utils::get_stock_from_nasdaq(invalid_symbol);
        assert_eq!(false, result.success);
    }
}
