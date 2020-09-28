mod utils;

use timeseries::TimeSeries;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PrecipitationEvent {
    // from: Time,
    // to: Time,
    // averaged_rainfall: f32,
    // averaged_water_level: f32,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct WetWeatherFlow {
    // rainfall: TimeSeries,
    // gwi: TimeSeries, // Ground Water infiltration
    // bwf: TimeSeries, // Base Water Flow
    // rdii: TimeSeries, // rainfall derived infiltration and inflow (total flow)
}

#[wasm_bindgen]
pub struct Envelope {
    rain: Vec<f32>,
    water: Vec<f32>,
}

#[wasm_bindgen]
pub struct Model {
    rainfall: TimeSeries,
    water_level: TimeSeries,
}

#[wasm_bindgen]
impl Model {

    /// Create new model based on rainfall and water level data
    pub fn new(rain_csv: &str, watercsv: &str) -> Model {
        Model {
            rainfall: TimeSeries::empty(), 
            water_level: TimeSeries::empty(),
        }
    }

    /// Get **count** bigest precipitation events
    pub fn get_events(count: u32) -> *const PrecipitationEvent {
        Vec::default().as_ptr()
    } 

    /// get water flow for the specific date
    pub fn get_wet_weather_flow(from: &str, to:&str) -> WetWeatherFlow {
        WetWeatherFlow {}
    }

    // Calculate I&I Envelope
    pub fn get_envelope() -> Envelope {
        Envelope {
            rain: Vec::default(),
            water: Vec::default(),
        }
    }
}