use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    x: f32,
    y: f32,
    rotation: f32,
}
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize)]
pub struct Food {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(getter)]
    pub fn get_animals(&self) -> Vec<Animal> {
        self.animals.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn get_foods(&self) -> Vec<Food> {
        self.foods.clone()
    }
}

#[wasm_bindgen]
impl Animal {
    #[wasm_bindgen(getter)]
    pub fn get_x(&self) -> f32 {
        self.x
    }
    #[wasm_bindgen(getter)]
    pub fn get_y(&self) -> f32 {
        self.y
    }
    #[wasm_bindgen(getter)]
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
}
#[wasm_bindgen]
impl Food {
    #[wasm_bindgen(getter)]
    pub fn get_x(&self) -> f32 {
        self.x
    }
    #[wasm_bindgen(getter)]
    pub fn get_y(&self) -> f32 {
        self.y
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.get_world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step();
    }
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world
            .get_animals()
            .iter()
            .map(|e| Animal::from(e))
            .collect();

        let foods = world.get_foods().iter().map(Food::from).collect();

        Self { animals, foods }
    }
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.get_position().x,
            y: animal.get_position().y,
            rotation: animal.get_rotation().angle(),
        }
    }
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.get_position().x,
            y: food.get_position().y,
        }
    }
}
