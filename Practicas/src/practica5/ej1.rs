use core::arch;
use std::fs::File;
use std::{file, path::Display};
use serde::{Serialize,Deserialize};
use std::fmt;
use std::io::prelude::*;
use std::path::Path;

/*
    COMANDOS COVERAGE:
    cargo tarpaulin --target-dir src/coverage --skip-clean
    cargo tarpaulin --target-dir src/coverage --skip-clean --out html
 */

#[derive(Debug,Clone,PartialEq)]
pub struct Concesionario {
    nombre: String,
    direccion: String,
    lista_autos: Vec<Autos>,
    capacidad: u32,
}

#[derive(Debug,Clone,PartialEq,serde::Serialize)]
pub struct Autos {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f32,
    color: Color,
}

#[derive(Debug,Clone,PartialEq,serde::Serialize)]
pub enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorCapacidad;

impl std::fmt::Display for ErrorCapacidad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Se supero el limite de la concesionaria")
    }
}

impl Autos {
    pub fn new(marca: String, modelo: String, año: u32, precio_bruto: f32, color: Color) -> Autos {
       if marca == "" || modelo == "" || año < 1886 || precio_bruto < 0.0 {
        panic!("Dato/s invalidos");
       }
       else{
        Autos {
            marca: marca,
            modelo: modelo,
            año: año,
            precio_bruto: precio_bruto,
            color: color,
        }
       }    
    }

    pub fn calcular_precio(& mut self)-> f32 {
        let mut valor = 0.0;

        match self.color {
            
            Color::ROJO | Color::AZUL | Color::AMARILLO => {
                let recargo = self.precio_bruto * 0.25;
                valor += recargo;
            }
            _ => {
                let descuento = self.precio_bruto * 0.10;
                valor -= descuento;
            }

        }    
        if self.marca == "BMW" {
            let recargo = self.precio_bruto * 0.15;
            valor += recargo;
        }

        if self.año < 2000 {
            let descuento = self.precio_bruto * 0.05;
            valor -= descuento;
        }

        return valor + self.precio_bruto; 
    }
}

impl Concesionario {
    pub fn new(nombre: String, direccion: String, lista_autos: Vec<Autos>, capacidad: u32) -> Concesionario {
        if nombre == "" || direccion == "" {
            panic!("Dato/s invalidos");
        }
        else {
            Concesionario {
                nombre: nombre,
                direccion: direccion,
                lista_autos: lista_autos,
                capacidad: capacidad,
            }
        }
    } 

    pub fn agregar_auto(&mut self,auto: Autos) -> Result<(), ErrorCapacidad>{
        if self.lista_autos.len() < self.capacidad as usize{
            self.lista_autos.push(auto);
            Ok(())
        }
        else {
            Err(ErrorCapacidad)
        }
    }

    pub fn eliminar_auto(&mut self, auto:Autos) {
        let size = self.lista_autos.len();
        let mut ok = false;
        let mut pos = 0;
        while pos < size && !ok {
            if self.lista_autos[pos] == auto {
                self.lista_autos.remove(pos);
                ok = true;
            }
            pos += 1;
        }
    }

    fn buscar_auto(&self, auto: Autos) -> Option<Autos> {
        let size = self.lista_autos.len();
        let mut pos = 0;
        while pos < size {
            if self.lista_autos[pos] == auto {
                return Some(self.lista_autos[pos].clone());
            }
            pos += 1;
        }
        return None;
    }

    fn guardar_en_archivo(&self, path: &str) {
        let archivo = File::create(path);

        let mut archivo = match archivo {
            Ok(file) => file,
            Err(e) => panic!("No se pudo crear el archivo: {}", e),
        };

        let autos_json = match serde_json::to_string(&self.lista_autos) {
            Ok(json) => json,
            Err(e) => panic!("No se pudo serializar el vector de autos: {}", e),
        };

        if let Err(e) = archivo.write_all(autos_json.as_bytes()) {
            panic!("No se pudo escribir en el archivo: {}", e);
        }
    }

    pub fn agregar_y_guardar_datos(&mut self, a: Autos, path: &str) -> Result<(), ErrorCapacidad> {
        match self.agregar_auto(a) {
            Ok(()) => {
                self.guardar_en_archivo(path);
                Ok(())
            },
            Err(e) => Err(e),
        }   
    }


    fn eliminar_y_guardar_datos (&mut self, a: Autos, path: &str) {
        self.eliminar_auto(a);
        self.guardar_en_archivo(path);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_auto() {
        let auto = Autos::new("Ford".to_string(), "FOCUS".to_string(), 2013, 30000.0, Color::AZUL);
        assert_eq!(auto.marca, "Ford".to_string());
    }

    #[should_panic]
    #[test]
    fn test_crear_mal() {
        let _auto = Autos::new("".to_string(), "FOCUS".to_string(), 2013, 30000.0, Color::AZUL);
    }

    #[test]
    fn test_nombre_raro1() {
        let vec = Vec::new();
        let mut con = Concesionario::new("Farias-Automotores".to_string(), "Mitre 829".to_string(), vec, 5);

        let mut auto1 = Autos::new("Ford".to_string(), "FOCUS".to_string(), 2013, 15000.0, Color::AZUL);
        let mut auto2 = Autos::new("BMW".to_string(), "M4".to_string(), 2024, 60000.0, Color::NEGRO);
        let auto3 = Autos::new("Dodge".to_string(), "CHARGER".to_string(), 2020, 40000.0, Color::ROJO);
        let auto4 = Autos::new("Chevrolet".to_string(), "CAMARO".to_string(), 2017, 20000.0, Color::AZUL);
        let auto5 = Autos::new("Audi".to_string(), "TT".to_string(), 2005, 30000.0, Color::VERDE);
        let mut auto6 = Autos::new("Nissan".to_string(), "Skyline R34".to_string(), 1999, 60000.0, Color::VERDE);

        assert_eq!(con.agregar_auto(auto1.clone()),Ok(()));
        assert_eq!(con.agregar_auto(auto2.clone()),Ok(()));
        assert_eq!(con.agregar_auto(auto3),Ok(()));
        assert_eq!(con.agregar_auto(auto4),Ok(()));
        assert_eq!(con.agregar_auto(auto5.clone()),Ok(()));
        assert_eq!(con.agregar_auto(auto6.clone()),Err(ErrorCapacidad));

        assert_eq!(con.buscar_auto(auto5.clone()), Some(auto5.clone()));
        con.eliminar_auto(auto5.clone());
        assert_eq!(con.buscar_auto(auto5.clone()), None);

        assert_eq!(auto1.calcular_precio(),18750.0);
        assert_eq!(auto2.calcular_precio(),63000.0);
        assert_eq!(auto6.calcular_precio(),51000.0);
    }

    #[test]
    fn test_nombre_raro2 () {
        let vec = Vec::new();
        let mut con = Concesionario::new("Farias-Automotores".to_string(), "Mitre 829".to_string(), vec, 5);

        let auto1 = Autos::new("Ford".to_string(), "FOCUS".to_string(), 2013, 15000.0, Color::AZUL);
        let auto2 = Autos::new("BMW".to_string(), "M4".to_string(), 2024, 60000.0, Color::NEGRO);
        let auto3 = Autos::new("Dodge".to_string(), "CHARGER".to_string(), 2020, 40000.0, Color::ROJO);
        let auto4 = Autos::new("Chevrolet".to_string(), "CAMARO".to_string(), 2017, 20000.0, Color::AZUL);
        let auto5 = Autos::new("Audi".to_string(), "TT".to_string(), 2005, 30000.0, Color::VERDE);
        let auto6 = Autos::new("Nissan".to_string(), "Skyline R34".to_string(), 1999, 60000.0, Color::VERDE);

        assert_eq!(con.agregar_y_guardar_datos(auto1,"src/practica5/ArchivosPrac5/archivo_autos.json"),Ok(()));
        assert_eq!(con.agregar_y_guardar_datos(auto2,"src/practica5/ArchivosPrac5/archivo_autos.json"),Ok(()));
        assert_eq!(con.agregar_y_guardar_datos(auto3.clone(),"src/practica5/ArchivosPrac5/archivo_autos.json"),Ok(()));
        assert_eq!(con.agregar_y_guardar_datos(auto4,"src/practica5/ArchivosPrac5/archivo_autos.json"),Ok(()));
        assert_eq!(con.agregar_y_guardar_datos(auto5,"src/practica5/ArchivosPrac5/archivo_autos.json"),Ok(()));
        assert_eq!(con.agregar_y_guardar_datos(auto6,"src/practica5/ArchivosPrac5/archivo_autos.json"),Err(ErrorCapacidad));

        con.eliminar_y_guardar_datos(auto3, "src/practica5/ArchivosPrac5/archivo_autos.json");
    }
}