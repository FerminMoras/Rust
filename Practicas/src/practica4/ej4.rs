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
        let mut categorias: HashMap<String, i32> = HashMap::new();

        for venta in &self.vec_ventas {
            for item in &venta.listado {
                let categoria = item.producto.categoria.clone(); 
                let cantidad = item.cant;

                *categorias.entry(categoria).or_insert(0) += cantidad;
            }
        }

        let mut entradas: Vec<_> = categorias.into_iter().collect();
        entradas.sort_by(|a, b| a.0.cmp(&b.0)); 

        for (categoria, cantidad) in entradas {
            reporte.push_str(&format!("Categoria: {} | Cantidad: {} ", categoria, cantidad));
        }

        reporte      
    }

    fn generar_reporte_vendedor(&self) -> String {
        let mut conteo = std::collections::HashMap::new();

        for venta in &self.vec_ventas {
            *conteo.entry(venta.vendedor.legajo).or_insert(0) += 1;
        }

        let mut conteo_vec: Vec<_> = conteo.into_iter().collect();

        conteo_vec.sort_by(|a, b| {
            b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
        });

        let mut reporte = String::new();
        for (legajo, cantidad) in conteo_vec {
            reporte.push_str(&format!(
                "Vendedor {} tiene {} venta(s) ",
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
        let ven = Vendedor::new("Delfina", "Ferrante", "47 767", 1337, 22, 3, 350000.0);
        let cli = Cliente::new("Fermin", "Moras", "La Rioja 668", 41876698, true, Some("feer.moras@gmail.com"));

        let prod = Producto::new("Coca Cola", "Gaseosas".to_string(), 2400.0);
        let items = ItemProductos::new(prod, 3);
        let mut vec_p = Vec::new();
        vec_p.push(items);
        
        let mut v_string = Vec::new();
        v_string.push("Coca cola".to_string());
        v_string.push("Lays".to_string());
        v_string.push("Doritos".to_string());

        let lista_prod = DescuentosProductos::new(v_string, 0.15);

        let venta = Venta::new("16/06/2025", cli, ven, Pagos::TRANSFERENCIA, vec_p.clone());

        let total = venta.precio_final_venta(&lista_prod);
        assert_eq!(total, 6480.0);
    }

    #[test]
    fn test_reportes() {

        let ven1 = Vendedor::new("Carlos", "Moras", "47 767", 1337, 25, 3, 350000.0);
        let ven3 = Vendedor::new("Delfina", "Ferrante", "47 767", 1337, 22, 3, 350000.0);
        let cli = Cliente::new("Fermin", "Moras", "La Rioja 668", 41876698, true, Some("feer.moras@gmail.com"));
        let cli1 = Cliente::new("Fermin", "Moras", "La Rioja 668", 41876698, false, None);
        let cli2 = Cliente::new("Wanchope", "Abila", "Av. Mitre 777", 35672353, true, Some("wanchope@gmail.com"));

        let prod = Producto::new("Coca Cola", "Gaseosas".to_string(), 2400.0);
        let prod1 = Producto::new("Lays", "Snacks".to_string(), 1400.0);
        let prod2 = Producto::new("Fideos", "Pastas".to_string(), 900.0);

        let items = ItemProductos::new(prod, 3);
        let items2 = ItemProductos::new(prod1, 3);
        let items3 = ItemProductos::new(prod2, 3);
        
        let mut vec_p1 = Vec::new();
        vec_p1.push(items);
        vec_p1.push(items2.clone());
        vec_p1.push(items3.clone());

        let mut vec_p2 = Vec::new();
        vec_p2.push(items2);

        let mut vec_p3 = Vec::new();
        vec_p3.push(items3);
        
        let mut v_string = Vec::new();
        v_string.push("Coca cola".to_string());
        v_string.push("Lays".to_string());
        v_string.push("Doritos".to_string());

        let lista_prod = DescuentosProductos::new(v_string, 0.15);

        let venta1 = Venta::new("16/06/2025", cli, ven1.clone(), Pagos::TRANSFERENCIA, vec_p1.clone());
        let venta2 = Venta::new("16/06/2025", cli1, ven1, Pagos::TRANSFERENCIA, vec_p2.clone());
        let venta3 = Venta::new("16/06/2025", cli2, ven3, Pagos::TRANSFERENCIA, vec_p3.clone());
        
        let mut vec_ven = Vec::new();
        vec_ven.push(venta1);
        vec_ven.push(venta2);
        vec_ven.push(venta3);

        let rep = Reporte::new(vec_ven);

        let rep_prod = rep.generar_reporte_producto();
        let rep_ven = rep.generar_reporte_vendedor();
        assert_eq!(rep_ven,"Vendedor 25 tiene 2 venta(s) Vendedor 22 tiene 1 venta(s) ".to_string());
        assert_eq!(rep_prod, "Categoria: Gaseosas | Cantidad: 3 Categoria: Pastas | Cantidad: 6 Categoria: Snacks | Cantidad: 6 ".to_string());
    }
}