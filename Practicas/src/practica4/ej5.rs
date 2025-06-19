use std::collections::HashMap;

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
        
    }

}    