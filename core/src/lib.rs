use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Stats {
  pub level: u32,
  pub wood: u32,
  pub stone: u32,
  pub iron: u32,
  pub cost: u32,
  pub workforce: u32,
  pub maintenance: u32,
  pub production: u32,
  pub capacity: u32,
  pub custom: u32,
}

#[wasm_bindgen]
pub fn calc(config: &Config) -> Result<Vec<Stats>, JsError> {
  let capacity = config.max_level.try_into()?;
  let mut stats = Vec::with_capacity(capacity);

  if config.max_level < 1 {
    return Ok(stats);
  }

  let min_level = f64::from(config.min_level);
  let max_level = f64::from(config.max_level);

  let min_cost = f64::from(config.min_cost);
  let max_cost = f64::from(config.max_cost);
  let cost_growth = growth(min_cost, max_cost, min_level, max_level);

  let min_workforce = f64::from(config.min_workforce);
  let max_workforce = f64::from(config.max_workforce);
  let workforce_growth = growth(min_workforce, max_workforce, min_level, max_level);

  let min_production = f64::from(config.min_production);
  let max_production = f64::from(config.max_production);
  let production_growth = growth(min_production, max_production, min_level, max_level);

  let min_capacity = f64::from(config.min_capacity);
  let max_capacity = f64::from(config.max_capacity);
  let capacity_growth = growth(min_capacity, max_capacity, min_level, max_level);

  let min_custom = f64::from(config.min_custom);
  let max_custom = f64::from(config.max_custom);
  let custom_growth = growth(min_custom, max_custom, min_level, max_level);

  let mut cost = f64::from(config.min_cost);
  let mut maintenance = cost * config.maintenance.clamp(0.0, 1.0);
  let mut workforce = f64::from(config.min_workforce);
  let mut production = f64::from(config.min_production);
  let mut capacity = f64::from(config.min_capacity);
  let mut custom = f64::from(config.min_custom);

  let wood = config.wood.clamp(0.0, 1.0);
  let stone = config.stone.clamp(0.0, 1.0);
  let iron = config.iron.clamp(0.0, 1.0);

  for level in config.min_level..=config.max_level {
    stats.push(Stats {
      level,
      wood: (cost * wood).ceil() as u32,
      stone: (cost * stone).ceil() as u32,
      iron: (cost * iron).ceil() as u32,
      cost: cost.ceil() as u32,
      workforce: workforce.ceil() as u32,
      maintenance: maintenance.ceil() as u32,
      production: production.ceil() as u32,
      capacity: capacity.ceil() as u32,
      custom: custom.ceil() as u32,
    });

    cost += (cost * cost_growth).ceil();
    workforce += (workforce * workforce_growth).ceil();
    production += (production * production_growth).ceil();
    capacity += (capacity * capacity_growth).ceil();
    custom += (custom * custom_growth).ceil();

    maintenance = cost * (config.maintenance.clamp(0.0, 1.0));
  }

  stats.sort_by_key(|it| it.level);

  Ok(stats)
}

fn growth(min_value: f64, max_value: f64, min_level: f64, max_level: f64) -> f64 {
  ((max_value / min_value).powf(1.0 / (max_level - min_level))) - 1.0
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Config {
  pub min_level: u32,
  pub max_level: u32,

  pub min_cost: u32,
  pub max_cost: u32,

  pub min_workforce: u32,
  pub max_workforce: u32,

  pub min_production: u32,
  pub max_production: u32,

  pub min_capacity: u32,
  pub max_capacity: u32,

  pub min_custom: u32,
  pub max_custom: u32,

  pub wood: f64,
  pub stone: f64,
  pub iron: f64,
  pub maintenance: f64,
}

#[wasm_bindgen]
impl Config {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      min_level: 1,
      max_level: 30,
      min_cost: 1_000,
      max_cost: 100_000,
      min_workforce: 1,
      max_workforce: 150,
      min_production: 30,
      max_production: 2400,
      min_capacity: 1_000,
      max_capacity: 400_000,
      min_custom: 1,
      max_custom: 1000,
      wood: 0.3,
      stone: 0.4,
      iron: 0.3,
      maintenance: 0.005,
    }
  }
}
