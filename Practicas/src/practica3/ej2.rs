#[derive(Debug)]
struct Rectangulo {
    base: f32,
    altura: f32,
}

impl Rectangulo {
    fn new(b: f32, h:f32) -> Rectangulo{
        if (b <= 0.0) || (h <= 0.0){
            panic!("la base y la altura debe ser mayor a 0");
        }

        return Rectangulo {
            base: b,
            altura: h,
        }
    }

    fn calcular_area(&self) -> f32 {
        return self.base * self.altura; 
    }

    fn calcular_perimetro(&self) -> f32 {
        return (self.base*2.0) + (self.altura*2.0);
    }

    fn es_cuadrado(&self) -> bool {
        if self.base == self.altura {
            return true;
        }
        else {
            return false;
        }
    }
}

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_crear_rectangulo(){
            let rectangulo: Rectangulo = Rectangulo::new(5.5,3.5);
            assert_eq!(rectangulo.altura, 3.5);
            assert_eq!(rectangulo.base, 5.5);
        }

        #[should_panic]
        #[test]
        fn test_mal_creado() {
            let _rec= Rectangulo::new(-2.0,0.0);
        }

        #[test]
        fn test_calcular_area(){
            let rec: Rectangulo = Rectangulo::new(5.5,3.5);
            let area = rec.calcular_area();
            assert_eq!(area,19.25);
        }

        #[test]
        fn test_calcular_perimetro(){
            let rec: Rectangulo = Rectangulo::new(5.5,3.5);
            let perimetro = rec.calcular_perimetro();
            assert_eq!(perimetro, 18.0);
        }

        #[test]
        fn test_es_cuadrado_true(){
            let rec: Rectangulo = Rectangulo::new(5.5,3.5);
            let ok = rec.es_cuadrado();
            assert_eq!(ok,false);
        }

        #[test]
        fn test_es_cuadrado_false(){
            let rec: Rectangulo = Rectangulo::new(5.5,5.5);
            let ok = rec.es_cuadrado();
            assert_eq!(ok,true);
        }
    }
