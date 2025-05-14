struct Triangulo{
    lado1: f32,
    lado2: f32,
    lado3: f32,
}

impl Triangulo{
    fn new(l1: f32, l2:f32, l3:f32) -> Triangulo{
        if l1 <= 0 || l2 <= 0 || l3 <= 0{
            panic!("Lados invalidos")
        }
        Triangulo {
            lado1: l1,
            lado2: l2,
            lado3: l3,
        }
    }

    fn determinar_tipo(&self) -> &str {
        if self.lado1 == self.lado2 && self.lado1 == self.lado3{
            "equilatero"
        } else if self.lado1 == self.lado2 || self.lado1 == self.lado3 || self.lado2 == self.lado3 {
            "isoceles"
        }else {
            "escaleno"
        }
    }

    fn calcular_area(&self) -> f32 {
        let tipo = self.determinar_tipo();
        if tipo == "isoceles" {

        }

        if tipo == "escaleno" {

        }

        if tipo == "equilatero" {

        }
    }

    fn calcular_perimetro(&self) -> f32 {
        self.lado1 + self.lado2 + self.lado3
    }
    
}