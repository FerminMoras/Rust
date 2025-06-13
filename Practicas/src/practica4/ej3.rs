#[derive(Debug, PartialEq)]
struct Plataforma {
    usuarios: Vec<Usuario>,
    suscripciones: Vec<Suscripcion>,
}

#[derive(Debug, PartialEq)]
struct Suscripcion {
    costo: f32,
    tipo: Tipos,
    duracion: i32,
    fecha: String,
    pago: Pagos,
}
#[derive(Debug, PartialEq)]
struct Usuario {
    nombre: String,
    suscripcion: Option<Suscripcion>,
}
#[derive(Debug, PartialEq)]
enum Tipos {
    BASIC,
    CLASIC,
    SUPER,
}
#[derive(Debug, PartialEq)]
enum Pagos {
    EFECTIVO,
    MERCADOPAGO,
    CREDITO,
    TRANSFERENCIA,
    CRIPTO,
}

impl Plataforma {
    fn new (usuarios: Vec<Usuario>, suscripciones: Vec<Suscripcion>) -> Plataforma {
        Plataforma {
            usuarios,
            suscripciones,
        }
    }

    fn suscripcion_mas_activa(&self) -> Option<Tipos> {
        let mut basic = 0;
        let mut clasic = 0;
        let mut superr = 0;

        for user in &self.usuarios {
            if let Some(susc) = &user.suscripcion {
                match susc.tipo {
                    Tipos::BASIC => basic += 1,
                    Tipos::CLASIC => clasic += 1,
                    Tipos::SUPER => superr += 1,
                } 
            }
        }

        if basic >= clasic && basic >= superr {
            Some(Tipos::BASIC)
        } else if clasic >= superr {
            Some(Tipos::CLASIC)
        }else {
            Some(Tipos::SUPER)
        }
    }

    /*fn suscripcion_mas_usada(&self) -> Option<Tipos> {
        let mut basic = 0;
        let mut clasic = 0;
        let mut superr = 0;

        for user in &self.usuarios {
                match user.tipo {
                    Tipos::BASIC => basic += 1,
                    Tipos::CLASIC => clasic += 1,
                    Tipos::SUPER => superr += 1,
                } 
        }

        if basic >= clasic && basic >= superr {
            Some(Tipos::BASIC)
        } else if clasic >= superr {
            Some(Tipos::CLASIC)
        }else {
            Some(Tipos::SUPER)
        }
    }*/

    fn pago_mas_activo(&self) -> Option<Pagos> {
        let mut efectivo = 0;
        let mut mercadopago = 0;
        let mut credito = 0;
        let mut transferencia = 0;
        let mut cripto = 0;

        for user in &self.usuarios {
            if let Some(susc) = &user.suscripcion {
                match susc.pago {
                    Pagos::EFECTIVO => efectivo += 1,
                    Pagos::MERCADOPAGO => mercadopago += 1,
                    Pagos::CREDITO => credito += 1,
                    Pagos::TRANSFERENCIA => transferencia += 1,
                    Pagos::CRIPTO => cripto += 1,
                } 
            }
        }

        let datos = vec![(efectivo, Pagos::EFECTIVO), (mercadopago, Pagos::MERCADOPAGO), (credito, Pagos::CREDITO), (transferencia, Pagos::TRANSFERENCIA), (cripto, Pagos::CRIPTO)];
        datos.into_iter().max_by_key(|(count, _)| *count).map(|(_, pago)| pago)
    }

    /*fn pago_mas_usado(&self) -> Option<Pagos> {
        let mut efectivo = 0;
        let mut mercadopago = 0;
        let mut credito = 0;
        let mut transferencia = 0;
        let mut cripto = 0;

        for user in &self.usuarios {
                match user.pago {
                    Pagos::EFECTIVO => efectivo += 1,
                    Pagos::MERCADOPAGO => mercadopago += 1,
                    Pagos::CREDITO => credito += 1,
                    Pagos::TRANSFERENCIA => transferencia += 1,
                    Pagos::CRIPTO => cripto += 1,
                } 
        }
        let datos = vec![(efectivo, Pagos::EFECTIVO), (mercadopago, Pagos::MERCADOPAGO), (credito, Pagos::CREDITO), (transferencia, Pagos::TRANSFERENCIA), (cripto, Pagos::CRIPTO)];
        datos.into_iter().max_by_key(|(count, _)| *count).map(|(_, pago)| pago)
    }*/
}

impl Usuario {
    fn new(nombre: String, suscripcion: Option<Suscripcion>) -> Usuario {
        Usuario {
            nombre,
            suscripcion,
        }
    }
}

trait Gestion {
    fn tiene_suscripcion(&self) -> bool;
    fn cancela_suscripcion(&mut self);
    fn upgrade(&mut self);
    fn downgrade(&mut self);
}

impl Gestion for Usuario {
    fn tiene_suscripcion(&self) -> bool {
        self.suscripcion.is_some()
    }

    fn cancela_suscripcion(&mut self) {
        self.suscripcion = None;
    }

    fn upgrade(&mut self) {
        if let Some(susc) = &mut self.suscripcion {
            if susc.tipo == Tipos::BASIC {
                susc.tipo = Tipos::CLASIC;
            } else if susc.tipo == Tipos::CLASIC {
                susc.tipo = Tipos::SUPER;
            }
        }    
    }

    fn downgrade(&mut self) {
        if let Some(susc) = &mut self.suscripcion {
            if susc.tipo == Tipos::CLASIC {
                susc.tipo = Tipos::BASIC;
            } else if susc.tipo == Tipos::SUPER {
                susc.tipo = Tipos::CLASIC;
            }
        }
    }
}    

impl Suscripcion {
    fn new(costo: f32, tipo: Tipos, duracion: i32, fecha: String, pago: Pagos) -> Suscripcion {
        Suscripcion {
            costo,
            tipo,
            duracion,
            fecha,
            pago,
        }
    }
}