use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Stats {
  pub capacity: u32,
  pub iron: u32,
  pub level: u32,
  pub maintenance: u32,
  pub production: u32,
  pub stone: u32,
  pub total_cost: u32,
  pub wood: u32,
  pub workforce: u32,
}

#[wasm_bindgen]
pub fn calc(config: &Config) -> Result<Vec<Stats>, JsError> {
  let capacity = config.max_level.try_into()?;
  let mut stats = Vec::with_capacity(capacity);

  if config.max_level < 1 {
    return Ok(stats);
  }

  let mut cost = f64::from(config.cost);
  let mut maintenance = cost * config.maintenance.clamp(0.0, 1.0);
  let mut workforce = f64::from(config.workforce);
  let mut production = f64::from(config.production);
  let mut capacity = f64::from(config.capacity);

  for level in (1..=config.max_level).rev() {
    stats.push(Stats {
      level,
      wood: (cost * (config.wood.clamp(0.0, 1.0))) as u32,
      stone: (cost * (config.stone.clamp(0.0, 1.0))) as u32,
      iron: (cost * (config.iron.clamp(0.0, 1.0))) as u32,
      capacity: capacity.ceil() as u32,
      maintenance: maintenance.ceil() as u32,
      production: production.ceil() as u32,
      total_cost: cost.ceil() as u32,
      workforce: workforce.ceil() as u32,
    });

    cost -= cost * (config.cost_growth.clamp(0.0, 1.0));
    maintenance -= cost * (config.maintenance.clamp(0.0, 1.0));
    workforce -= workforce * (config.workforce_growth.clamp(0.0, 1.0));
    production -= production * (config.production_growth.clamp(0.0, 1.0));
    capacity -= capacity * (config.capacity_growth.clamp(0.0, 1.0));
  }

  stats.sort_by_key(|it| it.level);

  Ok(stats)
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Config {
  pub capacity: u32,
  pub capacity_growth: f64,
  pub cost: u32,
  pub cost_growth: f64,
  pub iron: f64,
  pub maintenance: f64,
  pub max_level: u32,
  pub production: u32,
  pub production_growth: f64,
  pub stone: f64,
  pub wood: f64,
  pub workforce: u32,
  pub workforce_growth: f64,
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
      max_level: 30,
      wood: 0.3,
      stone: 0.4,
      iron: 0.3,
      maintenance: 0.005,
      cost: 100_000,
      cost_growth: 0.2,
      workforce: 150,
      workforce_growth: 0.2,
      production: 3_600,
      production_growth: 0.2,
      capacity: 400_000,
      capacity_growth: 0.2,
    }
  }
}
