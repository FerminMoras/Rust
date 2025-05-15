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
            (s * (s - l1) * (s - l2) * (s - b)).sqrt()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_triangulo() {
        let equi = Triangulo::new(10.0,10.0,10.0);
        let iso = Triangulo::new(10.0,10.0,5.0);
        let esca = Triangulo::new(10.0,7.0,5.0);

        assert_eq!(equi.lado1, 10.0);
        assert_eq!(iso.base, 5.0);
        assert_eq!(esca.lado2, 7.0); 
    }

    #[should_panic]
    #[test]
    fn test_triangulo_mal_creado() {
        let _tri = Triangulo::new(-1.0, 2.0, 3.0);
    }

    #[test]
    fn test_determinar_tipo() {
        let equi = Triangulo::new(10.0,10.0,10.0);
        let iso = Triangulo::new(10.0,10.0,5.0);
        let esca = Triangulo::new(10.0,7.0,5.0);

        assert_eq!(equi.determinar_tipo(), "equilatero");
        assert_eq!(iso.determinar_tipo(), "isoceles");
        assert_eq!(esca.determinar_tipo(), "escaleno"); 
    }

    #[test]
    fn test_calcular_area() {
        let equi = Triangulo::new(10.0,10.0,10.0);
        let iso = Triangulo::new(10.0,10.0,5.0);
        let esca = Triangulo::new(10.0,7.0,5.0);

        assert_eq!(equi.calcular_area(), 43.30127);
        assert_eq!(iso.calcular_area(), 25.0);
        assert_eq!(esca.calcular_area(), 16.248077)
    }

    #[test]
    fn test_calcular_perimetro() {
        let equi = Triangulo::new(10.0,10.0,10.0);
        let iso = Triangulo::new(10.0,10.0,5.0);
        let esca = Triangulo::new(10.0,7.0,5.0);

        assert_eq!(equi.calcular_perimetro(), 30.0);
        assert_eq!(iso.calcular_perimetro(), 25.0);
        assert_eq!(esca.calcular_perimetro(), 22.0)
    }
}