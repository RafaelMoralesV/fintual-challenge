use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
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
    pub fn rebalance_portfolio<'a>(&'a self) -> RebalanceSuggestion<'a> {
        let mut suggestion = RebalanceSuggestion::default();

        let mut current_units: HashMap<&str, usize> = HashMap::new();
        for stock in self.stocks() {
            *current_units.entry(stock.name()).or_insert(0) += 1;
        }

        let total_balance: Decimal = self.stocks().iter().map(|s| s.current_price()).sum();

        // no tenemos nada en el portafolio.
        if total_balance.is_zero() {
            return suggestion;
        }

        // cualquier stock que no existe en nuestra asignacion se sugiere eliminar completamente
        for (name, &units) in &current_units {
            if !self.allocation.contains_key(name) {
                suggestion.to_sell.insert(name, units);
            }
        }

        for (ratio, target_stock) in self.allocation.targets().iter() {
            let name = target_stock.name();
            let price_per_unit = target_stock.current_price();

            // nuestro maximo dinero objetivo
            let target_money = total_balance * (ratio / dec!(100.0));

            // esta es la cantidad maxima que podriamos tener (segun nuestra estrategia conservadora)
            let target_units = (target_money / price_per_unit)
                .trunc()
                .to_usize() // Esto no deberia fallar pq estamos truncando un numero mayor a cero
                .unwrap_or(0);

            // esta es la cantidad que tenemos
            let held_units = *current_units.get(name).unwrap_or(&0);

            if target_units > held_units {
                // sugerimos comprar la diferencia
                suggestion.to_buy.insert(name, target_units - held_units);
            } else if target_units < held_units {
                // sugerimos vender la diferencia
                suggestion.to_sell.insert(name, held_units - target_units);
            }
        }

        suggestion
    }
}

/// Clase que representa un stock.
#[derive(Debug, Clone)]
pub struct Stock {
    name: String, // E.J: META, APPL, ETC.
    current_price: Decimal,
}

impl Stock {
    pub fn new(name: &str, price: Decimal) -> Self {
        Self {
            name: name.into(),

            // Por hoy, voy a confiar que el precio es correcto nomas, pero deberia haber un constructor capaz
            // de evitar enviar un precio con algun valor negativo por ejemplo.
            current_price: price,
        }
    }

    /// Getter simple.
    pub fn current_price(&self) -> Decimal {
        self.current_price
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Default)]
pub struct RebalanceSuggestion<'a> {
    /// Mappea un stock (idenficado por su nombre) a una cantidad a comprar.
    pub to_buy: HashMap<&'a str, usize>,

    /// Mappea un stock (idenficado por su nombre) a una cantidad a vender.
    pub to_sell: HashMap<&'a str, usize>,
}

/// Representa los stocks que el cliente quiere obtener.
///
/// Por ejemplo, (40% META, 60% APPL); La razon de crear esta clase es verificar procurar que no
/// hayan estados irrepresentables; por ejemplo, stocks de menos de 100%, o de mas de 100%;
/// queremos evitar que los programadores que usen nuestra clase de portafolio puedan, por
/// accidente, asignar algo sin sentido como (50% META, 75% APPL), o (-30% META), etc.
#[derive(Debug)]
pub struct PortfolioTarget {
    targets: Vec<(Decimal, Stock)>,
}

impl PortfolioTarget {
    /// Genera un nuevo target con un solo stock, que representa un portafolio objetivo de 100% de
    /// ese stock.
    pub fn new(stock: Stock) -> Self {
        Self {
            targets: vec![(dec!(100), stock)],
        }
    }

    pub fn try_from_vec(stocks: Vec<(Decimal, Stock)>) -> Result<Self, String> {
        if stocks.iter().map(|stock| stock.0).sum::<Decimal>() != dec!(100) {
            return Err("Los stocks objetivos no suman un 100%".into());
        }

        if stocks.iter().any(|stock| stock.0 <= Decimal::ZERO) {
            return Err("Al menos uno de los stocks provistos tiene valor 0 o negativo.".into());
        }

        Ok(Self { targets: stocks })
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.targets.iter().any(|stock| stock.1.name() == name)
    }

    pub fn targets(&self) -> &[(Decimal, Stock)] {
        &self.targets
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
            (dec!(45.0), Stock::new("META", Decimal::ZERO)),
            (dec!(45.0), Stock::new("APPL", Decimal::ZERO)),
        ]);

        assert!(target_one.is_err());

        let target_two = PortfolioTarget::try_from_vec(vec![
            (dec!(40.0), Stock::new("META", Decimal::ZERO)),
            (dec!(70.0), Stock::new("APPL", Decimal::ZERO)),
        ]);

        assert!(target_two.is_err());
    }

    #[test]
    fn test_target_with_negative_allocation() {
        // ¿Qué pasa si alguien intenta pasar un -10%?
        // Tu try_from_vec debería validar que cada elemento sea > 0.
        let target_one = PortfolioTarget::try_from_vec(vec![
            (dec!(45.0), Stock::new("META", Decimal::ZERO)),
            (dec!(-10.0), Stock::new("APPL", Decimal::ZERO)),
        ]);

        assert!(target_one.is_err());
    }

    // --- Tests de Lógica de Rebalanceo ---

    #[test]
    fn test_rebalance_already_perfectly_balanced() {
        // Escenario: Tienes 40€ de META y 60€ de APPL, y tu target es 40/60.
        // Resultado esperado: Sugerencias vacías (to_buy y to_sell deben estar vacíos).
        let target = PortfolioTarget::try_from_vec(vec![
            (dec!(40.0), Stock::new("META", dec!(10.0))),
            (dec!(60.0), Stock::new("APPL", dec!(15.0))),
        ])
        .unwrap();

        let mut stocks = Vec::new();
        for _ in 0..4 {
            stocks.push(Stock::new("META", dec!(10.0)));
        }
        for _ in 0..4 {
            stocks.push(Stock::new("APPL", dec!(15.0)));
        }

        let portfolio = Portfolio {
            stocks,
            allocation: target,
        };
        let suggestion = portfolio.rebalance_portfolio();

        assert!(suggestion.to_buy.is_empty());
        assert!(suggestion.to_sell.is_empty());
    }

    #[test]
    fn test_rebalance_sell_entire_position() {
        // Escenario: Tienes 100% de una acción que YA NO está en el PortfolioTarget.
        // Resultado esperado: to_sell debe contener todas esas acciones.
        let target = PortfolioTarget::new(Stock::new("META", dec!(100.0)));
        let portfolio = Portfolio {
            stocks: vec![
                Stock::new("GOOG", dec!(50.0)),
                Stock::new("GOOG", dec!(50.0)),
            ],
            allocation: target,
        };

        let suggestion = portfolio.rebalance_portfolio();

        // Debe vender las 2 de GOOG y comprar 1 de META
        assert_eq!(*suggestion.to_sell.get("GOOG").unwrap(), 2);
        assert_eq!(*suggestion.to_buy.get("META").unwrap(), 1);
    }

    #[test]
    fn test_rebalance_buy_from_zero() {
        // Escenario: Tienes 100€ en efectivo (o en una acción que vas a vender)
        // y quieres comprar una nueva acción que no tenías.
        // Resultado esperado: to_buy debe contener la cantidad correcta de la nueva acción.
        let meta_target = Stock::new("META", dec!(25.0));
        let target = PortfolioTarget::new(meta_target);

        let portfolio = Portfolio {
            stocks: vec![
                Stock::new("CASH", dec!(1.0)); 100 // 100 unidades de 1€
            ],
            allocation: target,
        };

        let suggestion = portfolio.rebalance_portfolio();

        // nos deberia sugerir vender todo
        assert_eq!(
            *suggestion.to_sell.get("CASH").expect("Debería vender CASH"),
            100
        );

        // nos deberia sugerir comprar todo lo que podamos de META
        assert_eq!(
            *suggestion.to_buy.get("META").expect("Debería comprar META"),
            4
        );
    }

    #[test]
    fn test_rebalance_with_indivisible_stocks() {
        // Este es el más importante para tu estrategia "conservadora".
        // Escenario: Tienes 100€ totales. Target es 50% META. META cuesta 30€.
        // Cálculo: 50% de 100€ es 50€. Con 50€ solo puedes comprar 1 META (30€).
        // Si compras 2 (60€), te pasas del 50%.
        // Resultado esperado: to_buy debe sugerir 1 unidad, no 1.66 ni 2.
        let target = PortfolioTarget::try_from_vec(vec![
            (dec!(50.0), Stock::new("META", dec!(30.0))),
            (dec!(50.0), Stock::new("CASH", dec!(1.0))), // Relleno para el 100%
        ])
        .unwrap();

        let portfolio = Portfolio {
            stocks: vec![Stock::new("OTHER", dec!(100.0))],
            allocation: target,
        };

        let suggestion = portfolio.rebalance_portfolio();

        // Verificamos que no intenta comprar 2 (que costarían 60€, pasando el target de 50€)
        assert_eq!(*suggestion.to_buy.get("META").unwrap(), 1);
    }

    #[test]
    fn test_rebalance_empty_portfolio() {
        // Escenario: El vector de stocks está vacío.
        // Resultado esperado: No debe crashear, debe devolver sugerencias vacías
        // o manejar el total de 0.0.
        let target = PortfolioTarget::new(Stock::new("META", dec!(100.0)));
        let portfolio = Portfolio {
            stocks: vec![],
            allocation: target,
        };

        let suggestion = portfolio.rebalance_portfolio();
        assert!(suggestion.to_buy.is_empty());
        assert!(suggestion.to_sell.is_empty());
    }
}
