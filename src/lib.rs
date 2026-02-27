/// Problema original:
///
/// Construct a simple Portfolio class that has a collection of Stocks. Assume each Stock has a "Current Price"
/// method that receives the last available price. Also, the Portfolio class has a collection of “allocated” Stocks
/// that represents the distribution of the Stocks the Portfolio is aiming (i.e. 40% META, 60% APPL)
///
/// Provide a portfolio rebalance method to know which Stocks should be sold and which ones should be bought to
/// have a balanced Portfolio based on the portfolio’s allocation..
///
/// Add documentation/comments to understand your thinking process and solution

#[derive(Debug)]
pub struct Portfolio {
    stocks: Vec<Stock>,
    allocation: PortfolioTarget,
}

impl Portfolio {
    pub fn stocks(&self) -> &Vec<Stock> {
        &self.stocks
    }
}

/// Clase que representa un stock.
#[derive(Debug)]
pub struct Stock {
    name: String, // E.J: META, APPL, ETC.
    current_price: f64,
}

impl Stock {
    /// Getter simple.
    pub fn current_price(&self) -> f64 {
        self.current_price
    }
}

/// Representa los stocks que el cliente quiere obtener.
///
/// Por ejemplo, (40% META, 60% APPL); La razon de crear esta clase es verificar procurar que no
/// hayan estados irrepresentables; por ejemplo, stocks de menos de 100%, o de mas de 100%;
/// queremos evitar que los programadores que usen nuestra clase de portafolio puedan, por
/// accidente, asignar algo sin sentido como (50% META, 75% APPL), o (-30% META), etc.
#[derive(Debug)]
pub struct PortfolioTarget {
    targets: Vec<(f64, Stock)>,
}

impl PortfolioTarget {
    /// Genera un nuevo target con un solo stock, que representa un portafolio objetivo de 100% de
    /// ese stock.
    fn new(stock: Stock) -> Self {
        Self {
            targets: vec![(100f64, stock)],
        }
    }

    fn try_from_vec(stocks: Vec<(f64, Stock)>) -> Result<Self, String> {
        if stocks.iter().map(|stock| stock.0).sum::<f64>() != 100f64 {
            return Err("Los stocks objetivos no suman un 100%".into());
        }

        Ok(Self { targets: stocks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        todo!()
    }
}
