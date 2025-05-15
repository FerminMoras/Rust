struct Producto {
    nombre: String,
    precio_bruto: f32,
    id: u32,
}

impl Producto {
    fn new(name: String, price: f32, id: u32) -> Producto{
        if(name == "") || (price < 0.0) || (id == 0) {
            panic!("Datos incorrectos")
        }
        else {
            Producto {
                nombre: name,
                precio_bruto: price,
                id: id,
            }
        }
    }

    fn calcular_impuestos(&self, imp: f32) -> f32 {
        self.precio_bruto * (imp/100.0)
    }

    fn aplicar_descuento(&self, desc: f32) -> f32 {
        self.precio_bruto * (desc/100.0)
    }

    fn calcular_precio_total(&self, imp: f32, desc: f32) -> f32 {
        let impuesto = self.calcular_impuestos(imp);
        let descuento = self.aplicar_descuento(desc);
        
        (self.precio_bruto + impuesto) - descuento
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_producto(){
        let prod = Producto::new("jamon".to_string(), 1360.0, 1);

        assert_eq!(prod.nombre, "jamon".to_string());
        assert_eq!(prod.precio_bruto, 1360.0);
        assert_eq!(prod.id, 1);
    }

    #[should_panic]
    #[test]
    fn test_producto_mal_creado(){
        let _prod1 = Producto::new("".to_string(), 1360.0, 1);
        let _prod2 = Producto::new("jamon".to_string(), -10.5, 1);
        let _prod3 = Producto::new("queso".to_string(), 1360.0, 0);
    }

    #[test]
    fn test_calcular_impuestos() {
        let prod = Producto::new("jamon".to_string(), 1360.0, 1);
        assert_eq!(prod.calcular_impuestos(30.0),408.00003);
    }

    #[test]
    fn test_calcular_descuentos() {
        let prod = Producto::new("jamon".to_string(), 1360.0, 1);
        assert_eq!(prod.aplicar_descuento(15.0),208.00002);
    }

    #[test]
    fn test_calcular_precio_total() {
        let prod = Producto::new("jamon".to_string(), 1360.0, 1);
        assert_eq!(prod.calcular_precio_total(30.0, 15.0), 1564.0);
    }
}