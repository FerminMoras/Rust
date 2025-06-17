use std::fmt::format;
use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq)]
struct Producto<'a> {
    nombre: &'a str,
    categoria: String,
    precio_base: f32,
}

#[derive(Debug,Clone,PartialEq)]
struct Vendedor<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    dni: i32,
    legajo: i32,
    antiguedad: i32,
    salario: f32,
}

#[derive(Debug,Clone,PartialEq)]
struct Cliente<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    dni: i32,
    suscripcion: bool,
    email: Option<&'a str>,
}

#[derive(Debug,Clone,PartialEq)]
struct ItemProductos<'a> {
    producto: Producto<'a>,
    cant: i32
}

#[derive(Debug,Clone,PartialEq)]
struct Venta<'a> {
    fecha: &'a str,
    cliente: Cliente<'a>,
    vendedor: Vendedor<'a>,
    pago: Pagos,
    listado: Vec<ItemProductos<'a>>,
}

#[derive(Debug,Clone,PartialEq)]
enum Pagos {
    CREDITO,
    DEBITO,
    TRANSFERENCIA,
    EFECTIVO,
}

#[derive(Debug,Clone,PartialEq)]
struct DescuentosProductos {
    lista_prod: Vec<String>,
    desc: f32,
}
trait Descuentos {
    fn aplicar_descuento(&self, precio: f32, categoria: &String) -> f32;
}

impl Descuentos for DescuentosProductos {
    fn aplicar_descuento(&self, precio: f32, categoria: &String) -> f32 {
        if self.lista_prod.contains(categoria) {
            precio - (precio - self.desc)
        }else {
            precio
        }
    }
}

impl DescuentosProductos {
    fn new(lista_prod: Vec<String>, desc: f32) -> DescuentosProductos {
        DescuentosProductos {
            lista_prod,
            desc
        }
    }
}

impl<'a> ItemProductos<'a> {
    fn new(producto: Producto<'a>, cant: i32) -> ItemProductos<'a> {
        ItemProductos {
            producto,
            cant,
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
struct Reporte<'a> {
    vec_ventas: Vec<Venta<'a>>,
}

impl<'a> Reporte<'a> {
    fn new(vec_ventas: Vec<Venta<'a>>) -> Reporte<'a> {
        Reporte {
            vec_ventas
        }
    }

    fn registrar_venta(&mut self, v: Venta<'a>) {
        self.vec_ventas.push(v);
    }

    fn generar_reporte_producto(&self) -> String {
        let mut reporte = String::new();
        for i in &self.vec_ventas {
            for x in &i.listado {
               reporte.push_str(&format!("Categoria: {} | Cantidad: {} /n", 
                                                x.producto.categoria, x.cant));
            }
        }
        reporte        
    }

    fn generar_reporte_vendedor(&self) -> String {
        let mut conteo = HashMap::new();

        for venta in &self.vec_ventas {
            *conteo.entry(venta.vendedor.legajo).or_insert(0) += 1;
        }

        let mut reporte = String::new();

        for (legajo, cantidad) in conteo {
            reporte.push_str(&format!(
                "Vendedor {} tiene {} venta(s).\n",
                legajo, cantidad
            ));
        }

        reporte
    }
}

impl<'a> Venta<'a> {
    fn new(fecha:&'a str, cliente: Cliente<'a>, vendedor: Vendedor<'a>, pago: Pagos, listado: Vec<ItemProductos<'a>>) -> Venta<'a> {
        Venta {
            fecha,
            cliente,
            vendedor,
            pago,
            listado,
        }
    }

    fn precio_final_venta(&self, descuentos: &DescuentosProductos) -> f32 {
        let mut total = self.listado.iter().map(|item|{
            let mut precio = item.producto.precio_base * item.cant as f32;

            for desc in &descuentos.lista_prod {
                if desc.contains(&item.producto.categoria) {
                    precio -= precio * descuentos.desc;
                }
            }
            precio
        })
        .sum::<f32>();
        if self.cliente.suscripcion == true {
            total = self.cliente.tiene_suscripcion(total)
        }

        total
    }


}

impl<'a> Cliente<'a> {
    fn new(nombre: &'a str, apellido: &'a str, direccion: &'a str, dni: i32, suscripcion: bool, email: Option<&'a str>) -> Cliente<'a> {
        Cliente {
            nombre,
            apellido,
            direccion,
            dni,
            suscripcion,
            email,
        }
    }

    fn tiene_suscripcion(&self, precio: f32) -> f32 {
        if self.suscripcion == true {
            precio - (0.10 * precio)
        }else {
            precio   
        }
    }
}

impl<'a> Vendedor<'a> {
    fn new(nombre: &'a str, apellido: &'a str, direccion: &'a str, dni: i32, legajo: i32, antiguedad: i32, salario: f32) -> Vendedor<'a> {
        Vendedor {
            nombre,
            apellido,
            direccion,
            dni,
            legajo,
            antiguedad,
            salario
        }
    }
}

impl<'a> Producto<'a>  {
    fn new(nombre: &'a str, categoria: String, precio_base: f32) -> Producto<'a> {
        Producto {
            nombre,
            categoria,
            precio_base,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_precio_final() {
        let cli = Cliente::new(nombre, apellido, direccion, dni, suscripcion, email);

        let prod = Producto::new("Coca Cola", "Gaseosas".to_string(), 2400.0);
        let items = ItemProductos::new(prod, 3);

        let mut v_string = Vec::new();
        v_string.push("Coca cola".to_string());
        v_string.push("Lays".to_string());
        v_string.push("Doritos".to_string());

        let lista_prod = DescuentosProductos::new(v_string, 0.15);

        let venta = Venta::new("16/06/2025", cliente, vendedor, pago, listado);
    }
}