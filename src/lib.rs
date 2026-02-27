use std::collections::HashMap;

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
    pub fn stocks(&self) -> &[Stock] {
        &self.stocks
    }

    /// Muestra una sugerencia de rebalancio a partir de un portafolio.
    ///
    /// La forma de rebalanceo que voy a aplicar es la siguiente:
    /// 1. Se suman los stocks del portafolio segun su precio actual para tener una idea de cuanto
    ///    dinero requerimos.
    /// 2. Se hacen proporciones objetivo para cada stock segun lo asignado; esto nos dice cuanto
    ///     de ese stock vender, cuando comprar.
    /// 3. Debido a que estamos trabajando con stocks que no necesariamente van a cuadrar
    ///    perfectamente en proporciones de 40% o similares, utilizare una estrategia conservadora:
    ///    venderemos o compraremos la mayor cantidad de stock posible hasta llegar a la proporcion
    ///    objetivo sin pasarnos. Esto seguramente resulta en un saldo excedente dentro de la
    ///    cartera del usuario/cliente.
    pub fn rebalance_portfolio(&self) -> RebalanceSuggestion {
        let mut suggestion = RebalanceSuggestion::default();

        // obtenemos el total de cada stock
        let balances: HashMap<&str, f64> =
            self.stocks().iter().fold(HashMap::new(), |mut acc, stock| {
                *acc.entry(stock.name()).or_insert(0.0) += stock.current_price();
                acc
            });

        // total en la cartera
        let total_balance = balances.values().sum::<f64>();

        // nuestras sugerencias parten por vender cualquier stock que no este en nuestro allocation

        // luego, por cada stock que si tenemos, tenemos que vender o comprar hasta alcanzar
        // nuestro objetivo

        // finalmente, compramos cualquier stock que nos haga falta

        suggestion
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

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Default)]
pub struct RebalanceSuggestion {
    to_buy: Vec<(usize, Stock)>,
    to_sell: Vec<(usize, Stock)>,
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

        if stocks.iter().any(|stock| stock.0 <= 0.0) {
            return Err("Al menos uno de los stocks provistos tiene valor 0 o negativo.".into());
        }

        Ok(Self { targets: stocks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests de Validación de PortfolioTarget ---

    #[test]
    fn test_target_sum_must_be_100() {
        // Debería fallar si la suma es 90% o 110%

        let target_one = PortfolioTarget::try_from_vec(vec![
            (
                45f64,
                Stock {
                    name: "META".into(),
                    current_price: 0f64,
                },
            ),
            (
                45f64,
                Stock {
                    name: "APPL".into(),
                    current_price: 0f64,
                },
            ),
        ]);

        assert!(target_one.is_err());

        let target_two = PortfolioTarget::try_from_vec(vec![
            (
                40f64,
                Stock {
                    name: "META".into(),
                    current_price: 0f64,
                },
            ),
            (
                70f64,
                Stock {
                    name: "APPL".into(),
                    current_price: 0f64,
                },
            ),
        ]);

        assert!(target_two.is_err());
    }

    #[test]
    fn test_target_with_negative_allocation() {
        // ¿Qué pasa si alguien intenta pasar un -10%?
        // Tu try_from_vec debería validar que cada elemento sea > 0.
        let target_one = PortfolioTarget::try_from_vec(vec![
            (
                45f64,
                Stock {
                    name: "META".into(),
                    current_price: 0f64,
                },
            ),
            (
                -10f64,
                Stock {
                    name: "APPL".into(),
                    current_price: 0f64,
                },
            ),
        ]);

        assert!(target_one.is_err());
    }

    // --- Tests de Lógica de Rebalanceo ---

    #[test]
    fn test_rebalance_already_perfectly_balanced() {
        // Escenario: Tienes 40€ de META y 60€ de APPL, y tu target es 40/60.
        // Resultado esperado: Sugerencias vacías (to_buy y to_sell deben estar vacíos).
        unimplemented!();
    }

    #[test]
    fn test_rebalance_sell_entire_position() {
        // Escenario: Tienes 100% de una acción que YA NO está en el PortfolioTarget.
        // Resultado esperado: to_sell debe contener todas esas acciones.
        unimplemented!();
    }

    #[test]
    fn test_rebalance_buy_from_zero() {
        // Escenario: Tienes 100€ en efectivo (o en una acción que vas a vender)
        // y quieres comprar una nueva acción que no tenías.
        // Resultado esperado: to_buy debe contener la cantidad correcta de la nueva acción.
        unimplemented!();
    }

    #[test]
    fn test_rebalance_with_indivisible_stocks() {
        // Este es el más importante para tu estrategia "conservadora".
        // Escenario: Tienes 100€ totales. Target es 50% META. META cuesta 30€.
        // Cálculo: 50% de 100€ es 50€. Con 50€ solo puedes comprar 1 META (30€).
        // Si compras 2 (60€), te pasas del 50%.
        // Resultado esperado: to_buy debe sugerir 1 unidad, no 1.66 ni 2.
        unimplemented!();
    }

    #[test]
    fn test_rebalance_empty_portfolio() {
        // Escenario: El vector de stocks está vacío.
        // Resultado esperado: No debe crashear, debe devolver sugerencias vacías
        // o manejar el total de 0.0.
        unimplemented!();
    }

    #[test]
    fn test_rebalance_drastic_price_change() {
        // Escenario: Tenías un portafolio balanceado, pero el precio de META subió al doble.
        // Resultado esperado: Debe sugerir vender META para recuperar la proporción original.
        unimplemented!();
    }
}
