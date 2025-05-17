use crate::ej3::Fecha;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola: VecDeque<Mascota>,
    registro: Vec<Atencion>,
}

#[derive(Debug, Clone)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animal,
    dueño: Dueño,
}

#[derive(Debug, Clone)]
struct Dueño {
    nombre: String,
    direccion: String,
    telefono: u32,
}

#[derive(Debug, Clone)]
struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    visita: Option<Fecha>,
}

#[derive(Debug, Clone)]
enum Animal {
    PERRO,
    GATO,
    CABALLO,
    OTROS,
}

impl Mascota {
    fn new(nombre: String, edad: u32, tipo: Animal, dueño: Dueño) -> Mascota {
        Mascota {
            nombre: nombre,
            edad: edad,
            tipo: tipo,
            dueño: dueño,
        }
    }

    fn es_mismo_tipo(&self, tipo:Animal) {
        match (self.tipo,tipo) {
            (Animal::CABALLO, Animal::CABALLO) | (Animal::PERRO, Animal::PERRO) | (Animal::GATO, Animal::GATO) | (Animal::OTROS) => {
                true
            }

            _ => false
        }
    }

    fn es_mismo_tipo(&self, dueño:Dueño) -> bool {
        if self.dueño.nombre == dueño.nombre && self.dueño.direccion == dueño.direccion && self.dueño.telefono == dueño.telefono {
            true
        }
        else {
            false
        }
    }

    fn es_misma_mascota(&self, mascota: Mascota) -> bool {
        if self.nombre == mascota.nombre && self.edad == mascota.edad && self.es_mismo_tipo(&mascota.tipo) && self.es_mismo_dueño(&mascota.dueño) {
            true
        }
        else {
            false
        }
    }
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: u32, cola: VecDeque<Mascota>, registro: Vec<Atencion>) -> Veterinaria {
        Veterinaria {
            nombre: nombre,
            direccion: direccion,
            id: id,
            cola: cola,
            registro: registro,
        }
    }

    fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola.push_back(mascota);
    }

    fn agregar_mascota_siguiente(&mut self, mascota: Mascota) {
        self.cola.push_front(mascota);
    }

    fn atender_mascota(&mut self) -> Option<Mascota> {
        self.cola.pop_front()
    }

    fn elimiar_mascota(&mut self, mascota: Mascota) {
        let mut pos = 0;
        while pos < self.cola.len() {
            if (self.cola[pos].es_misma_mascota(&mascota)) {
                self.cola.remove(pos);
                break;
            }
            pos += 1;
        }
    }

    fn registrar_atencion(&mut self, diagnostico: String, tratamiento: String, visita: Option<Fecha>) {
        if let Some(mascota) = self.cola.pop_front() {
            self.registro.push(Atencion { 
                mascota: mascota,
                diagnostico: diagnostico,
                tratamiento: tratamiento,
                visita: visita
             });
        }
    }

    fn buscar_atencion(&self, nombre_dueño: String, nombre_mascota: String, telefono: u32) -> &Atencion {
        let mut pos = 0;
        while pos < self.registro.len() {
            if self.registro[pos].mascota.dueño.nombre == nombre_dueño && self.registro[pos].mascota.nombre = nombre_mascota && 
            self.registro[pos].mascota.dueño.telefono == telefono {
                return &self.registro[pos];
            } 
            pos += 1;
        }
    }

    fn modificar_diagnostico(&mut self, indice_atencion: usize, nuevo_diagnostico: String) {
        if let Some(atencion) = self.registro.get_mut(indice_atencion) {
            atencion.diagnostico = nuevo_diagnostico; 
        }
    }

    fn modificar_fecha(&mut self, indice_atencion: usize, nueva_fecha: Fecha) {
        if let Some(atencion) = self.registro.get_mut(indice_atencion) {
            atencion.visita = nueva_fecha; 
        }
    }

    fn eliminar_atencion(&mut self, indice_atencion: usize) {
        if self.registro.get(indice_atencion).is_some() {
            self.registro.remove(indice_atencion);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
