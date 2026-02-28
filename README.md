# Fintual Coding Challenge

Este proyecto implementa un motor de rebalanceo de portafolios de inversión utilizando Rust. La solución se enfoca en la precisión financiera y el manejo de activos discretos (unidades enteras).

## Cómo compilar

Este proyecto utiliza el toolchain de Rust. Puede instalarse desde https://rustup.rs/

### Toolchain desde devenv

Utilizo `devenv.nix` para producir este toolchain de manera reproducible (ya que uso NixOS en mi pc personal 🤓). Puedes obtener el toolchain de rust ejecutando:

```bash
devenv shell
```

### Utilizando Cargo

Con el toolchain instalado, se puede probar el proyecto tal que:

```bash
# Compilar
cargo build

# ejecutar pruebas unitarias
cargo test
```

### Ejemplo de uso rápido

Escribí este código como si fuera una librería, por lo que no produce un output o un ejecutable. La forma de probarlo es agregarlo como dependencia a otro proyecto de rust. Para integrar esta lógica en una aplicación, el flujo básico sería algo tal que:

```rust
// definimos un objetivo
let target = PortfolioTarget::try_from_vec(vec![
    (dec!(40.0), Stock::new("META", dec!(150.0))),
    (dec!(60.0), Stock::new("AAPL", dec!(180.0))),
]).unwrap();

// definimos nuestro portafolio
let portfolio = Portfolio { stocks, allocation: target };

// obtenemos nuestras sugerencias de rebalanceo
let sugerencia = portfolio.rebalance_portfolio();

println!("Comprar: {:?}", sugerencia.to_buy);
println!("Vender: {:?}", sugerencia.to_sell);
```

## ¿ Por qué Rust ?

Rust como lenguaje destaca en lugares donde se necesita código impecable, robusto, explícito y confiable, y otorga garantías para sistemas donde un error numérico no es una opción.

Con esta elección logré eliminar errores en tiempo de ejecución utilizando el sistema de tipado fuerte del lenguaje, mantener una gran precision financiera en base al crate `rust_decimal` para evitar imprecisiones inherentes a los puntos flotantes, y generar invariantes de Dominio, donde es imposible generar estados inválidos.

## Estrategia

Se utilizó una estrategia conservadora para las sugerencias finales; el algoritmo sugiere un cambio total del portafolio, donde se asume que el capital para financiar la compra de stocks viene exclusivamente desde los stocks vendidos del portafolio.

La estrategia conservadora termina produciendo, por lo general, un pequeño excedente. Esto es debido a que, pese a que queremos que nuestro portafolio actual sea un 40% de META, el precio de este stock no necesariamente nos permite un valor exacto. La resolucion por la que se optó es tomar ese 40% como un 'máximo', por lo que, cualquier porcentaje bajo lo que no se haya logrado comprar se considera excedente.

## Estructuras

Se crearon cuatro estructuras para este ejercicio:

- `Stock`: entidad con precio actual y nombre
- `Portfolio`: Tambien lo llamo cartera del cliente, es un contenedor de activos.
- `PortfolioTarget`: representa la proporción de stocks que el cliente quiere; la estructura garantiza validez de datos.
- `RebalanceSuggestion`: el resultado del calculo, que indica cuantas acciones vender y cuantas acciones comprar con ese dinero.

## Recursos

Utilicé Gemini para resolver algunas dudas pequeñas de negocio y orientar mi respuesta final, asi como generar boilerplate para pruebas unitarias. La conversacion [se encuentra en este link](https://gemini.google.com/share/3bf568c334b3).

Esta misma conversacion resultó en el uso de `rust_decimal` para aritmetica decimal de alta precisión.
