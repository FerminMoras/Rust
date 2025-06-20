use std::collections::HashMap;
use rand::Rng;

#[derive(Debug,Clone,PartialEq)]
struct Plataforma {
    usuarios: Vec<Usuario>,
    criptomonedas: Vec<Cripto>,
    transacciones: Vec<Transaccion>,
}

#[derive(Debug,Clone,PartialEq)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: i32,
    identidad: Validacion,
    balance: HashMap<String, f32>,
}

#[derive(Debug,Clone,PartialEq)]
enum Validacion {
    VALIDADO,
    INVALIDO,
}

#[derive(Debug,Clone,PartialEq)]
struct Cripto {
    nombre: String,
    prefijo: String,
    cotizacion: f32,
    listado: Vec<Blockchains>,
}

#[derive(Debug,Clone,PartialEq)]
struct Blockchains {
    nombre: String,
    prefijo: String,
}

#[derive(Debug,Clone,PartialEq)]
struct Transaccion {
    fecha: String,
    tipo: Tipos,
    monto: f32,
    dni_usuario: i32,
    cripto: Option<String>,
    blockchain: Option<String>,
    cotizacion: Option<f32>,
    hash: Option<String>,
    medio: Option<Medios>,
}

#[derive(Debug,Clone,PartialEq)]
enum Medios {
    MercadoPago,
    TransferenciaBanc,
}

#[derive(Debug,Clone,PartialEq)]
enum Tipos {
    IngresoFiat,
    CompraCripto,
    VentaCripto,
    RetiroCripto,
    RecepcionCripto,
    RetiroFiat,
}

impl Plataforma {
    fn new(usuarios: Vec<Usuario>, criptomonedas: Vec<Cripto>, transacciones: Vec<Transaccion>) -> Plataforma {
        Plataforma {
            usuarios,
            criptomonedas,
            transacciones,
        }
    }


    fn max_volumen_compra(&self, v: &Vec<Transaccion>) -> String {
    let mut contar = HashMap::new();

    for i in v {
        if let Tipos::CompraCripto = i.tipo {
            if let Some(cripto) = &i.cripto {
                *contar.entry(cripto.clone()).or_insert(0.0) += i.monto;
            }
        }
    }

    contar
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(cripto, volumen)| format!("Cripto con mayor volumen de compra: {}", cripto))
        .unwrap_or("No hay compras registradas.".to_string())
}

fn max_cantidad_compra(&self, v: &Vec<Transaccion>) -> String {
    let mut contar: HashMap<String, u32> = HashMap::new();

    for i in v {
        if let Tipos::CompraCripto = i.tipo {
            if let Some(cripto) = &i.cripto {
                *contar.entry(cripto.clone()).or_insert(0) += 1;
            }
        }
    }

    contar
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(cripto, count)| format!("Cripto con mas compras: {} ({} compras)", cripto, count))
        .unwrap_or("No hay compras registradas.".to_string())
}

fn max_volumen_venta(&self, v: &Vec<Transaccion>) -> String {
    let mut contar = HashMap::new();

    for i in v {
        if let Tipos::VentaCripto = i.tipo {
            if let Some(cripto) = &i.cripto {
                *contar.entry(cripto.clone()).or_insert(0.0) += i.monto;
            }
        }
    }

    contar
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(cripto, volumen)| format!("Cripto con mayor volumen de venta: {}", cripto))
        .unwrap_or("No hay ventas registradas.".to_string())
}

fn max_cantidad_venta(&self, v: &Vec<Transaccion>) -> String {
    let mut contar: HashMap<String, u32> = HashMap::new();

    for i in v {
        if let Tipos::VentaCripto = i.tipo {
            if let Some(cripto) = &i.cripto {
                *contar.entry(cripto.clone()).or_insert(0) += 1;
            }
        }
    }

    contar
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(cripto, count)| format!("Cripto con mas ventas: {} ({} ventas)", cripto, count))
        .unwrap_or("No hay ventas registradas.".to_string())
}


}

trait GestionUsuarios {
    fn ingresar_dinero(&mut self, dni: i32, monto_fiat: f32, fecha: String) -> bool;
    fn comprar_cripto(&mut self, dni: i32, nombre_cripto: String, monto_fiat: f32, fecha: String) -> bool;
    fn vender_cripto(&mut self, dni: i32, nombre_cripto: String, monto_cripto: f32, fecha: String) -> bool;
    fn retirar_cripto(&mut self, dni: i32, nombre_cripto: String, monto_cripto: f32, blockchain: String, fecha: String) -> bool;
    fn recibir_cripto(&mut self, dni: i32, nombre_cripto: String, monto_cripto: f32, blockchain: String, fecha: String) -> bool;
    fn retirar_fiat(&mut self, dni: i32, monto_fiat: f32, pago: Medios, fecha: String) -> bool;
}

impl GestionUsuarios for Plataforma {
    fn ingresar_dinero(&mut self, dni: i32, monto_fiat: f32, fecha: String) -> bool {
        let user = self.usuarios.iter().position(|u| u.dni == dni && u.identidad == Validacion::VALIDADO); //Clousure que busca el usuario y chequea que este verificado

        if let Some(i) = user { 
        
            let usuario = &mut self.usuarios[i]; //guardamos en otra variable la referencia mutable para evitar borrow y lifetime

            let agregar = usuario.balance.entry("fiat".to_string()).or_insert(0.0); //buscamos en el hashmap "fiat" que hace referencia al balance y le sumamos el monto
            *agregar += monto_fiat;

            self.transacciones.push(Transaccion { // creamos la transaccion y la pusheamos en el vector
                fecha,
                tipo: Tipos::IngresoFiat,
                monto: monto_fiat,
                dni_usuario: dni,
                cripto: None,
                blockchain: None,
                cotizacion: None,
                hash: None,
                medio: None,
            });

            true
        } else {
            false
        }
    }

    fn comprar_cripto(&mut self, dni: i32, nombre_cripto: String, monto_fiat: f32, fecha: String) -> bool {
        let user = self.usuarios.iter().position(|u| u.dni == dni && u.identidad == Validacion::VALIDADO); //Clousure que busca el usuario y chequea que este verificado

        if let Some(i) = user {
            let usuario = &mut self.usuarios[i];

            let saldo_fiat = usuario.balance.entry("fiat".to_string()).or_insert(0.0); //buscamos en el hashmap con la palabra "fiat" que hace refencia al monto
            if *saldo_fiat < monto_fiat { //si el saldo es insuficiente retornamos false
                return false;
            }

            if let Some(cripto) = self.criptomonedas.iter().find(|c| c.nombre == nombre_cripto) { //buscamos la cripto, nos guardamos la cotizacion y la cantidad comprada
                let cotizacion = cripto.cotizacion;
                let cantidad_cripto = monto_fiat / cotizacion;
                *saldo_fiat -= monto_fiat; //descontamos del saldo fiat

                let saldo_cripto = usuario.balance.entry(cripto.nombre.clone()).or_insert(0.0); //buscamos en el hashmap del balance y nos guardamos en la variable el balance de la cripto
                *saldo_cripto += cantidad_cripto; //acreditamos el balance de la cripto

                self.transacciones.push(Transaccion {
                    fecha,
                    tipo: Tipos::CompraCripto,
                    monto: monto_fiat,
                    dni_usuario: dni,
                    cripto: Some(nombre_cripto),
                    cotizacion: Some(cotizacion),
                    blockchain: None,
                    hash: None,
                    medio: None,
                });

                return true;
            }else {
                return false;
            } 
        } else{
            return false;
        }    
    }

    fn vender_cripto(&mut self, dni: i32, nombre_cripto: String, monto_cripto: f32, fecha: String) -> bool {
        let user = self.usuarios.iter().position(|u| u.dni == dni && u.identidad == Validacion::VALIDADO); //Clousure que busca el usuario y chequea que este verificado

        if let Some(i) = user {
            let usuario = &mut self.usuarios[i];

            let saldo_cripto = usuario.balance.entry(nombre_cripto.clone()).or_insert(0.0); //buscamos el balance de la cripto y verificamos el monto a vender
            if *saldo_cripto < monto_cripto {
                return false;
            }

            if let Some(cripto) = self.criptomonedas.iter().find(|c| c.nombre == nombre_cripto) { //buscamos la cripto en el balance, nos guardamos la cotizacion y el monto fiat
                let cotizacion = cripto.cotizacion;
                let cantidad_fiat = cotizacion * monto_cripto; 
                *saldo_cripto -= monto_cripto;

                let saldo_fiat = usuario.balance.entry("fiat".to_string()).or_insert(0.0); //buscamos en el balance el saldo fiat para poder incrementar el monto de la venta
                *saldo_fiat += cantidad_fiat;

                self.transacciones.push(Transaccion{
                    fecha,
                    tipo: Tipos::VentaCripto,
                    monto: monto_cripto,
                    dni_usuario: dni,
                    cripto: Some(nombre_cripto),
                    cotizacion: Some(cotizacion),
                    blockchain: None,
                    hash: None,
                    medio: None,
                });

                return true;
            }else {
                return false;
            }
        }else {
            return false;
        }
    }

    fn retirar_cripto(&mut self, dni: i32, nombre_cripto: String, monto_cripto: f32, blockchain: String, fecha: String) -> bool {
        let user = self.usuarios.iter().position(|u| u.dni == dni && u.identidad == Validacion::VALIDADO); //Clousure que busca el usuario y chequea que este verificado

        if let Some(i) = user {
            let usuario = &mut self.usuarios[i];

            let saldo_cripto = usuario.balance.entry(nombre_cripto.clone()).or_insert(0.0); //buscamos el balance de la cripto y verificamos el monto a retirar
            if *saldo_cripto < monto_cripto {
                return false;
            }

            if let Some(cripto) = self.criptomonedas.iter().find(|c| c.nombre == nombre_cripto) { //buscamos la cripto en el balance, nos guardamos la cotizacion
                let cotizacion = cripto.cotizacion;

                let blockchain_valida = cripto.listado.iter().any(|b| b.nombre == blockchain); //buscamos la blockchain para ver si es valida
                if !blockchain_valida {
                    return false;
                }

                *saldo_cripto -= monto_cripto; // descontamos el monto de cripto del saldo

                let random: u32 = rand::thread_rng().gen_range(100000..999999); //generamos el hash
                let hash = format!("{}-{}",blockchain, random);

                self.transacciones.push(Transaccion{
                    fecha,
                    tipo: Tipos::RetiroCripto,
                    monto: monto_cripto,
                    dni_usuario: dni,
                    cripto: Some(nombre_cripto),
                    cotizacion: Some(cotizacion),
                    blockchain: Some(blockchain),
                    hash: Some(hash),
                    medio: None,
                });

                return true;
            }else {
                return false;
            }    
        }else {
            return false;
        }
    }

    fn recibir_cripto(&mut self, dni: i32, nombre_cripto: String, monto_cripto: f32, blockchain: String, fecha: String) -> bool {
        let user = self.usuarios.iter().position(|u| u.dni == dni && u.identidad == Validacion::VALIDADO); //Clousure que busca el usuario y chequea que este verificado

        if let Some(i) = user {
            let usuario = &mut self.usuarios[i];

            if let Some(cripto) = self.criptomonedas.iter().find(|c| c.nombre == nombre_cripto) { //buscamos la cripto en el balance, nos guardamos la cotizacion
                let cotizacion = cripto.cotizacion;

                let blockchain_valida = cripto.listado.iter().any(|b| b.nombre == blockchain); //buscamos la blockchain para ver si es valida
                if !blockchain_valida {
                    return false;
                }

                let saldo_cripto = usuario.balance.entry(nombre_cripto.clone()).or_insert(0.0); //buscamos el balance de la cripto y nos guardamos el saldo
                *saldo_cripto += monto_cripto;

                self.transacciones.push(Transaccion {
                    fecha,
                    tipo: Tipos::RecepcionCripto,
                    monto: monto_cripto,
                    dni_usuario: dni,
                    cripto: Some(nombre_cripto),
                    cotizacion: Some(cotizacion),
                    blockchain: Some(blockchain),
                    hash: None,
                    medio: None,
                });

                return true;
            }else {
                return false;
            }
        }else {
            return false;
        }    
    }

    fn retirar_fiat(&mut self, dni: i32, monto_fiat: f32, pago: Medios, fecha: String) -> bool {
        let user = self.usuarios.iter().position(|u| u.dni == dni && u.identidad == Validacion::VALIDADO); //Clousure que busca el usuario y chequea que este verificado

        if let Some(i) = user {
            let usuario = &mut self.usuarios[i];

            let saldo_fiat = usuario.balance.entry("fiat".to_string()).or_insert(0.0); //buscamos en el hashmap con la palabra "fiat" que hace refencia al monto
                if *saldo_fiat < monto_fiat { //si el saldo a retirar es mayor al saldo actual retornamos false
                    return false;
                }

            *saldo_fiat -= monto_fiat;

             self.transacciones.push(Transaccion {
                fecha,
                tipo: Tipos::RetiroFiat,
                monto: monto_fiat,
                dni_usuario: dni,
                cripto: None,
                cotizacion: None,
                blockchain: None,
                hash: None,
                medio: Some(pago),
             });

             return true;   
        }else {
            return false;
        }
    }

}

impl Usuario {
    fn new (nombre: String, apellido: String, email: String, dni: i32, identidad: Validacion, balance: HashMap<String,f32>) -> Usuario {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            identidad,
            balance,
        }
    }
}

impl Cripto {
    fn new (nombre:String, prefijo: String, cotizacion: f32, listado: Vec<Blockchains>) -> Cripto {
        Cripto {
            nombre,
            prefijo,
            cotizacion,
            listado,
        }
    }
}

impl Blockchains {
    fn new (nombre: String, prefijo: String) -> Blockchains {
        Blockchains {
            nombre,
            prefijo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transacciones_y_maximos() {

        //hashmap para el balance
        let mut h1: HashMap<String,f32> = HashMap::new();
        h1.insert("fiat".to_string(), 10000.0);
        h1.insert("bitcoin".to_string(), 10000.0);
        h1.insert("etherium".to_string(), 10000.0);

        //usuario
        let u1 = Usuario::new("fermin".to_string(), "moras".to_string(), "fermoras@gmail.com".to_string(), 41876698, Validacion::VALIDADO, h1);
        let mut vec_user = Vec::new();
        vec_user.push(u1);

        //blockchain para las cripto y criptos
        let mut vec_block = Vec::new();
        let b1 = Blockchains::new("lemon".to_string(), "lem".to_string());
        let b2 = Blockchains::new("binance".to_string(), "bin".to_string());
        let b3 = Blockchains::new("coinbase".to_string(), "coin".to_string());
        vec_block.push(b1);
        vec_block.push(b2);
        vec_block.push(b3);

        let btc = Cripto::new("bitcoin".to_string(), "btc".to_string(), 100.0, vec_block.clone());
        let eth = Cripto::new("etherium".to_string(), "eth".to_string(), 50.0, vec_block);
        let mut vec_cripto = Vec::new();
        vec_cripto.push(btc);
        vec_cripto.push(eth);

        //plataforma
        let vec_trans: Vec<Transaccion> = Vec::new();
        let mut p = Plataforma::new(vec_user, vec_cripto, vec_trans);

        //TESTEO DE FUNCIONES DE TRANSACCIONES

        //INGRESO DE DINERO
        p.ingresar_dinero(41876698, 5000.0, "20/06/2025".to_string());
        assert_eq!(*p.usuarios[0].balance.get("fiat").unwrap(), 15000.0);

        //COMPRA DE CRIPTO
        p.comprar_cripto(41876698, "bitcoin".to_string(), 5000.0, "20/06/2025".to_string());
        assert_eq!(*p.usuarios[0].balance.get("bitcoin").unwrap(), 10050.0);

        //VENTA DE CRIPTO
        p.vender_cripto(41876698, "etherium".to_string(), 5000.0, "20/06/2025".to_string());
        assert_eq!(*p.usuarios[0].balance.get("etherium").unwrap(), 5000.0);

        //RECIBIR CRIPTO
        p.recibir_cripto(41876698, "bitcoin".to_string(), 5000.0, "binance".to_string(), "20/06/2025".to_string());
        assert_eq!(*p.usuarios[0].balance.get("bitcoin").unwrap(), 15050.0);

        //RETIRAR CRIPTO
        p.retirar_cripto(41876698, "etherium".to_string(), 2000.0,"lemon".to_string(), "20/06/2025".to_string());
        assert_eq!(*p.usuarios[0].balance.get("etherium").unwrap(), 3000.0);

        //RETIRO FIAT
        p.retirar_fiat(41876698, 12000.0, Medios::MercadoPago, "20/06/2025".to_string());
        assert_eq!(*p.usuarios[0].balance.get("fiat").unwrap(), 248000.0);

        //TESTEO DE FUNCIONES DE MAXIMOS

        //CANTIDAD MAXIMA DE CRIPTO QUE SE COMPRO
        assert_eq!(p.max_cantidad_compra(&p.transacciones), "Cripto con mas compras: bitcoin (1 compras)");

        //VOLUMEN MAXIMO DE CRIPTO QUE SE COMPRO
        assert_eq!(p.max_volumen_compra(&p.transacciones), "Cripto con mayor volumen de compra: bitcoin");

        //CANTIDAD MAXIMA DE CRIPTO QUE SE VENDIO
        assert_eq!(p.max_cantidad_venta(&p.transacciones), "Cripto con mas ventas: etherium (1 ventas)");

        //VOLUMEN MAXIMO DE CRIPTO QUE SE VENDIO
        assert_eq!(p.max_volumen_venta(&p.transacciones), "Cripto con mayor volumen de venta: etherium");
    }

}


