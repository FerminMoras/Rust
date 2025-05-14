struct Triangulo{
    lado1: f32,
    lado2: f32,
    base: f32,
}

impl Triangulo{
    fn new(l1: f32, l2:f32, base:f32) -> Triangulo{
        if l1 <= 0.0 || l2 <= 0.0 || base <= 0.0{
            panic!("Lados invalidos")
        }
        Triangulo {
            lado1: l1,
            lado2: l2,
            base: base,
        }
    }

    fn determinar_tipo(&self) -> String {
        if self.lado1 == self.lado2 && self.lado1 == self.base{
            "equilatero".to_string()
        } else if self.lado1 == self.lado2 || self.lado1 == self.base || self.lado2 == self.base {
            "isoceles".to_string()
        }else {
            "escaleno".to_string()
        }
    }
    
    fn calcular_area(&self) -> f32 {
        let lado1 = self.lado1;
        let lado2 = self.lado2;
        let base = self.base;
        fn calcular_area_equilatero(l: f32) -> f32 {
            (3.0_f32.sqrt() / 4.0) * l*l
        }

        fn calcular_area_escaleno(l1: f32, l2:f32, b: f32) -> f32 {
            let s = (l1 + l2 + b) / 2.0;
            (s * (s - l1) - (s - l2) - (s - b)).sqrt()
        }

        fn calcular_area_isoceles(b: f32, h: f32) -> f32 {
            (b * h) / 2.0
        }

        let tipo = self.determinar_tipo();
        match tipo.as_str() {
            "equilatero" => {
                calcular_area_equilatero(lado1)
            }
            "isoceles" => {
                calcular_area_isoceles(base,lado1)
            }
            "escaleno" => {
                calcular_area_escaleno(lado1,lado2,base)
            }
            _ => 0.0 
        }
    }
    
    fn calcular_perimetro(&self) -> f32 {
        self.lado1 + self.lado2 + self.base
    }
    
}