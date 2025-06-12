
struct Concesionario {
    nombre: String,
    direccion: String,
    lista_autos: Vec<Autos>,
    capacidad: u32,
}
#[derive(Debug,Clone,PartialEq)]
struct Autos {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f32,
    color: Color,
}
#[derive(Debug,Clone,PartialEq)]
enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

#[derive(Debug)]
struct AutoPorColor {
    color: Color,
    cant: u32,
}

struct Resumen {
    total_autos: u32,
    promedio: f32,
    mas_caro: String,
    mas_barato: String,
    cantidad_por_color: Vec<AutoPorColor>,
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

    fn que_color_es(&self) -> i32 {
        match self.color {
            Color::ROJO => {
                0
            }
            Color::AZUL => {
                1
            }
            Color::AMARILLO => {
                2
            }
            Color::VERDE => {
                3
            }
            Color::NEGRO => {
                4
            }
            Color::BLANCO => {
                5
            }
            _ => {
                -1
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

    fn auto_por_colores(&self) -> Vec<AutoPorColor> {
        let mut v: Vec<AutoPorColor> = vec![
            AutoPorColor { color: Color::ROJO, cant: 0 },
            AutoPorColor { color: Color::AZUL, cant: 0 },
            AutoPorColor { color: Color::AMARILLO, cant: 0 },
            AutoPorColor { color: Color::VERDE, cant: 0 },
            AutoPorColor { color: Color::NEGRO, cant: 0 },
            AutoPorColor { color: Color::BLANCO, cant: 0 },
        ];
        let vector = &self.lista_autos.clone();
        for i in vector {
            if i.que_color_es() == 0{
                v[0].color = Color::ROJO;
                v[0].cant += 1;
            }
            if i.que_color_es() == 1{
                v[1].color = Color::AZUL;
                v[1].cant += 1;
            }
            if i.que_color_es() == 2{
                v[2].color = Color::AMARILLO;
                v[2].cant += 1;
            }
            if i.que_color_es() == 3{
                v[3].color = Color::VERDE;
                v[3].cant += 1;
            }
            if i.que_color_es() == 4{
                v[4].color = Color::NEGRO;
                v[4].cant += 1;
            }
            if i.que_color_es() == 5{
                v[5].color = Color::BLANCO;
                v[5].cant += 1;
            }  
        }
        return v;
    }

    fn mas_barato(&mut self) -> String {
        let vector = &mut self.lista_autos;
        let mut min = 99999.99;
        let mut marca = "".to_string();
        let mut precio_final = 0.0; 
        for a in vector.iter_mut() {
            if  a.precio_bruto < min {
                min = a.precio_bruto;
                marca = a.marca.clone();
                precio_final = a.calcular_precio().clone();
            }
        }
        return format!("marca {} precio final {}", marca, precio_final);
    }

    fn mas_caro(&mut self) -> String {
        let vector = &mut self.lista_autos;
        let mut marca = "".to_string();
        let mut precio_final: f32 = 0.0;
        let mut max: f32 = -1.0;
        for a in vector.iter_mut() {
            if a.precio_bruto > max {
                max = a.precio_bruto;
                precio_final = a.calcular_precio().clone();
                marca = a.marca.clone();
            }
        }
        return format!("marca {} precio final {}", marca, precio_final);
    }

    fn generar_resumen(&mut self) -> Option<Resumen> {
        if self.lista_autos.is_empty(){
            return None
        }else {    
            let mut cant = 0;
            let mut precio_final:f32 = 0.0;
            let vector = &mut self.lista_autos;
            //calculo cantidad de autos y los precios finales de todos los autos
            for a in vector.iter_mut() {
                cant += 1;
                precio_final += a.calcular_precio();
            }
            let max_caro = self.mas_caro(); 
            let max_barato = self.mas_barato();
            let promedio = precio_final / cant as f32;
            let vec_color_autos: Vec<AutoPorColor> = self.auto_por_colores();
            Some (Resumen {
                total_autos: cant,
                promedio: promedio,
                mas_caro: max_caro,
                mas_barato: max_barato, 
                cantidad_por_color: vec_color_autos, 
            })   
        }
    } 

    fn agregar_auto(&mut self,auto: Autos) -> bool{
        if self.lista_autos.len() < self.capacidad as usize{
            self.lista_autos.push(auto);
            true
        }
        else {
            false
        }
    }

    fn eliminar_auto(&mut self, auto:Autos) {
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
    fn test_crear_concesionario() {
        let vec = Vec::new();
        let mut con = Concesionario::new("Farias-Automotores".to_string(), "Mitre 829".to_string(), vec, 5);

        let mut auto1 = Autos::new("Ford".to_string(), "FOCUS".to_string(), 2013, 15000.0, Color::AZUL);
        let mut auto2 = Autos::new("BMW".to_string(), "M4".to_string(), 2024, 60000.0, Color::NEGRO);
        let auto3 = Autos::new("Dodge".to_string(), "CHARGER".to_string(), 2020, 40000.0, Color::ROJO);
        let auto4 = Autos::new("Chevrolet".to_string(), "CAMARO".to_string(), 2017, 20000.0, Color::AZUL);
        let auto5 = Autos::new("Audi".to_string(), "TT".to_string(), 2005, 30000.0, Color::VERDE);
        let mut auto6 = Autos::new("Nissan".to_string(), "Skyline R34".to_string(), 1999, 60000.0, Color::VERDE);

        assert_eq!(con.agregar_auto(auto1.clone()),true);
        assert_eq!(con.agregar_auto(auto2.clone()),true);
        assert_eq!(con.agregar_auto(auto3),true);
        assert_eq!(con.agregar_auto(auto4),true);
        assert_eq!(con.agregar_auto(auto5.clone()),true);
        assert_eq!(con.agregar_auto(auto6.clone()),false);

        assert_eq!(con.buscar_auto(auto5.clone()), Some(auto5.clone()));
        con.eliminar_auto(auto5.clone());
        assert_eq!(con.buscar_auto(auto5.clone()), None);

        assert_eq!(auto1.calcular_precio(),18750.0);
        assert_eq!(auto2.calcular_precio(),63000.0);
        assert_eq!(auto6.calcular_precio(),51000.0)
    }

    #[test]
    fn test_generar_resumen_bien() {
        let vec = Vec::new();
        let mut con = Concesionario::new("Farias-Automotores".to_string(), "Mitre 829".to_string(), vec, 5);

        let auto1 = Autos::new("Ford".to_string(), "FOCUS".to_string(), 2013, 15000.0, Color::AZUL);
        let auto2 = Autos::new("BMW".to_string(), "M4".to_string(), 2024, 60000.0, Color::NEGRO);
        let auto3 = Autos::new("Dodge".to_string(), "CHARGER".to_string(), 2020, 40000.0, Color::ROJO);
        let auto4 = Autos::new("Chevrolet".to_string(), "CAMARO".to_string(), 2017, 20000.0, Color::AZUL);
        let auto5 = Autos::new("Audi".to_string(), "TT".to_string(), 2005, 30000.0, Color::VERDE);

        con.agregar_auto(auto1);
        con.agregar_auto(auto2);
        con.agregar_auto(auto3);
        con.agregar_auto(auto4);
        con.agregar_auto(auto5);

        if let Some(d) = con.generar_resumen() {
            //Cantidad de autos
            assert_eq!(d.total_autos, 5);
            //Promedio precios finales
            assert_eq!(d.promedio, 36750.0);
            //Auto mas caro y su precio final
            assert_eq!(d.mas_caro, "marca BMW precio final 63000".to_string());
            //Auto mas Barato y su precio final
            assert_eq!(d.mas_barato, "marca Ford precio final 18750".to_string());
            //Autos por color
            //Rojo
            if let Some(cant_rojo) = d.cantidad_por_color.get(0) {
                assert_eq!(cant_rojo.cant,1);
            }else{
                panic!("error");
            }

            //Azul
            if let Some(cant_azul) = d.cantidad_por_color.get(1) {
                assert_eq!(cant_azul.cant,2);
            }else{
                panic!("error");
            }

            //Amarillo
            if let Some(cant_amarillo) = d.cantidad_por_color.get(2) {
                assert_eq!(cant_amarillo.cant,0);
            }else{
                panic!("error");
            }

            //Verde
            if let Some(cant_verde) = d.cantidad_por_color.get(3) {
                assert_eq!(cant_verde.cant,1);
            }else{
                panic!("error");
            }

            //Negro
            if let Some(cant_negro) = d.cantidad_por_color.get(4) {
                assert_eq!(cant_negro.cant,1);
            }else{
                panic!("error");
            }

            //Blanco
            if let Some(cant_blanco) = d.cantidad_por_color.get(5) {
                assert_eq!(cant_blanco.cant,0);
            }else{
                panic!("error");
            }
        }else {
            panic!("no hay autos en la concesionaria");
        }
    }

    #[should_panic]
    #[test]
    fn test_generar_resumen_vacio() {
        let vec = Vec::new();
        let mut con = Concesionario::new("Farias-Automotores".to_string(), "Mitre 829".to_string(), vec, 5);

        if let Some(d) = con.generar_resumen() {
            //Cantidad de autos
            assert_eq!(d.total_autos, 5);
            //Promedio precios finales
            assert_eq!(d.promedio, 36750.0);
            //Auto mas caro y su precio final
            assert_eq!(d.mas_caro, "marca BMW precio final 63000".to_string());
            //Auto mas Barato y su precio final
            assert_eq!(d.mas_barato, "marca Ford precio final 18750".to_string());
            //Autos por color
            //Rojo
            if let Some(cant_rojo) = d.cantidad_por_color.get(0) {
                assert_eq!(cant_rojo.cant,1);
            }else{
                panic!("error");
            }

            //Azul
            if let Some(cant_azul) = d.cantidad_por_color.get(1) {
                assert_eq!(cant_azul.cant,2);
            }else{
                panic!("error");
            }

            //Amarillo
            if let Some(cant_amarillo) = d.cantidad_por_color.get(2) {
                assert_eq!(cant_amarillo.cant,0);
            }else{
                panic!("error");
            }

            //Verde
            if let Some(cant_verde) = d.cantidad_por_color.get(3) {
                assert_eq!(cant_verde.cant,1);
            }else{
                panic!("error");
            }

            //Negro
            if let Some(cant_negro) = d.cantidad_por_color.get(4) {
                assert_eq!(cant_negro.cant,1);
            }else{
                panic!("error");
            }

            //Blanco
            if let Some(cant_blanco) = d.cantidad_por_color.get(5) {
                assert_eq!(cant_blanco.cant,0);
            }else{
                panic!("error");
            }
        }else {
            panic!("no hay autos en la concesionaria");
        }
    }
}    