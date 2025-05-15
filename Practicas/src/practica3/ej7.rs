struct Concesionario {
    nombre: String,
    direccion: String,
    lista_autos: Vec<Autos>,
    capacidad: u32
}

struct Autos {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f32,
    color: Color,
}

enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

impl Autos {
    fn new(marca: String, modelo: String, año: u32, precio_bruto: f32, color: Color) -> Autos {
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

    fn calcular_precio(& mut self) {
        if self.color == Color::ROJO || self.color == Color::AZUL || self.color == Color::AMARILLO {
            let recargo = self.precio_bruto * 0.25;
            self.precio_bruto += recargo;
        }
        else {
            let descuento = self.precio_bruto * 0.10;
            self.precio_bruto -= descuento;
        }

        if self.marca == "BMW"{
            let recargo = self.precio_bruto * 0.15;
            self.precio_bruto += recargo;
        }

        if self.año < 2000 {
            let descuento = self.precio_bruto * 0.05;
            self.precio_bruto -= descuento;
        }
    }
}

impl Concesionario {
    fn new(nombre: String, direccion: String, lista_autos: Vec<Autos>, capacidad: u32) -> Concesionario {
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

    fn agregar_auto(auto: Autos){
        
    }
}