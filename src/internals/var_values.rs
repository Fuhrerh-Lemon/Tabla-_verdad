// modulos de rust, libreria de crates
use indexmap::map::IndexMap;

pub struct VarValues(IndexMap<String, bool>);

// Mapeará los valores
impl VarValues {
    pub fn new(nombres: &[String]) -> Self {
        let mut map = IndexMap::new();
        for nombre in nombres.iter().map(Clone::clone) {
            map.entry(nombre).or_insert(true);
        }
        VarValues(map)
    }
    // Metodos para evaluar el problema del usuario
    pub fn get_value<S: ToString>(&self, nombre: S) -> bool {
        *self.0.get(&nombre.to_string()).unwrap_or_else(||
            panic!("El valor '{}' es una variable inexistente", &nombre.to_string()))
    }

    pub fn names(&self) -> impl Iterator<Item=&String> {
        self.0.keys()
    }

    pub fn values(&self) -> Vec<bool> {
        self.0.values().copied().collect()
    }

    pub fn advance(&mut self) -> bool {
        self.0.values_mut().rev().any(|value| {
            *value = !*value;
            !*value
        })
    }
}