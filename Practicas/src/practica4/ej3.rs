#[derive(Debug, PartialEq,Clone)]
struct Plataforma {
    usuarios: Vec<Usuario>,
}

#[derive(Debug, PartialEq,Clone)]
struct Suscripcion {
    costo: f32,
    tipo: Tipos,
    duracion: i32,
    fecha: String,
    pago: Pagos,
    estado: String,
}
#[derive(Debug, PartialEq,Clone)]
struct Usuario {
    nombre: String,
    suscripcion: Suscripcion,
}
#[derive(Debug, PartialEq,Clone)]
enum Tipos {
    BASIC,
    CLASIC,
    SUPER,
}
#[derive(Debug, PartialEq,Clone)]
enum Pagos {
    EFECTIVO,
    MERCADOPAGO,
    CREDITO,
    TRANSFERENCIA,
    CRIPTO,
}

impl Plataforma {
    fn new (usuarios: Vec<Usuario>) -> Plataforma {
        Plataforma {
            usuarios,
        }
    }

    fn suscripcion_mas_activa(&self) -> Tipos {
        let mut basic = 0;
        let mut clasic = 0;
        let mut superr = 0;

        for user in &self.usuarios {
            if user.tiene_suscripcion_activa() == true {
                match user.suscripcion.tipo {
                    Tipos::BASIC => basic += 1,
                    Tipos::CLASIC => clasic += 1,
                    Tipos::SUPER => superr += 1,
                } 
            }
        }

        if basic >= clasic && basic >= superr {
            Tipos::BASIC
        } else if clasic >= superr {
            Tipos::CLASIC
        }else {
            Tipos::SUPER
        }
    }

    fn suscripcion_mas_usada(&self) -> Tipos {
        let mut basic = 0;
        let mut clasic = 0;
        let mut superr = 0;

        for user in &self.usuarios {
                match user.suscripcion.tipo {
                    Tipos::BASIC => basic += 1,
                    Tipos::CLASIC => clasic += 1,
                    Tipos::SUPER => superr += 1,
                } 
        }

        if basic >= clasic && basic >= superr {
            Tipos::BASIC
        } else if clasic >= superr {
            Tipos::CLASIC
        }else {
            Tipos::SUPER
        }
    }

    fn pago_mas_activo(&self) -> Pagos {
        let mut efectivo = 0;
        let mut mercadopago = 0;
        let mut credito = 0;
        let mut transferencia = 0;
        let mut cripto = 0;

        for user in &self.usuarios {
            if user.tiene_suscripcion_activa() == true {
                match user.suscripcion.pago {
                    Pagos::EFECTIVO => efectivo += 1,
                    Pagos::MERCADOPAGO => mercadopago += 1,
                    Pagos::CREDITO => credito += 1,
                    Pagos::TRANSFERENCIA => transferencia += 1,
                    Pagos::CRIPTO => cripto += 1,
                } 
            }
        }

        let datos = vec![(efectivo, Pagos::EFECTIVO), (mercadopago, Pagos::MERCADOPAGO), (credito, Pagos::CREDITO), (transferencia, Pagos::TRANSFERENCIA), (cripto, Pagos::CRIPTO)];
        datos.into_iter().max_by_key(|(count, _)| *count).map(|(_, pago)| pago).unwrap()
    }

    fn pago_mas_usado(&self) -> Pagos {
        let mut efectivo = 0;
        let mut mercadopago = 0;
        let mut credito = 0;
        let mut transferencia = 0;
        let mut cripto = 0;

        for user in &self.usuarios {
                match user.suscripcion.pago {
                    Pagos::EFECTIVO => efectivo += 1,
                    Pagos::MERCADOPAGO => mercadopago += 1,
                    Pagos::CREDITO => credito += 1,
                    Pagos::TRANSFERENCIA => transferencia += 1,
                    Pagos::CRIPTO => cripto += 1,
                } 
        }
        let datos = vec![(efectivo, Pagos::EFECTIVO), (mercadopago, Pagos::MERCADOPAGO), (credito, Pagos::CREDITO), (transferencia, Pagos::TRANSFERENCIA), (cripto, Pagos::CRIPTO)];
        datos.into_iter().max_by_key(|(count, _)| *count).map(|(_, pago)| pago).unwrap()
    }
}

impl Usuario {
    fn new(nombre: String, suscripcion: Suscripcion) -> Usuario {
        Usuario {
            nombre,
            suscripcion,
        }
    }
}

trait Gestion {
    fn tiene_suscripcion_activa(&self) -> bool;
    fn cancela_suscripcion(&mut self);
    fn upgrade(&mut self);
    fn downgrade(&mut self);
}

impl Gestion for Usuario {
    fn tiene_suscripcion_activa(&self) -> bool {
        self.suscripcion.estado == "activa".to_string()
    }

    fn cancela_suscripcion(&mut self) {
        self.suscripcion.estado = "inactiva".to_string();
    }

    fn upgrade(&mut self) {
        if self.tiene_suscripcion_activa() == true {
            if self.suscripcion.tipo == Tipos::BASIC {
                self.suscripcion.tipo = Tipos::CLASIC;
            } else if self.suscripcion.tipo == Tipos::CLASIC {
                self.suscripcion.tipo = Tipos::SUPER;
            }
        }    
    }

    fn downgrade(&mut self) {
        if self.tiene_suscripcion_activa() == true {
            if self.suscripcion.tipo == Tipos::CLASIC {
                self.suscripcion.tipo = Tipos::BASIC;
            } else if self.suscripcion.tipo == Tipos::SUPER {
                self.suscripcion.tipo = Tipos::CLASIC;
            }
        }
    }
}    

impl Suscripcion {
    fn new(costo: f32, tipo: Tipos, duracion: i32, fecha: String, pago: Pagos, estado: String) -> Suscripcion {
        Suscripcion {
            costo,
            tipo,
            duracion,
            fecha,
            pago,
            estado,
        }
    }
}

#[cfg(test)]
    mod tests {
    use super::*;
        #[test]
        fn test_upgrade_downgrade() {
            let s1 = Suscripcion::new(500.0, Tipos::CLASIC, 6, "13/06/2025".to_string(), Pagos::MERCADOPAGO, "activa".to_string());
            let s2 = Suscripcion::new(300.0, Tipos::BASIC, 6, "13/06/2025".to_string(), Pagos::MERCADOPAGO, "inactiva".to_string());
            let s3 = Suscripcion::new(800.0, Tipos::SUPER, 12, "13/06/2025".to_string(), Pagos::CREDITO, "activa".to_string());

            let mut u1 = Usuario::new("Fermin Moras".to_string(), s1);
            let mut u2 = Usuario::new("Delfina Ferrante".to_string(), s2);
            let mut u3 = Usuario::new("Wanchope Abila".to_string(), s3);

            u1.upgrade();
            u2.cancela_suscripcion();
            u3.downgrade();

            assert_eq!(u1.suscripcion.tipo, Tipos::SUPER);
            assert_eq!(u2.suscripcion.estado, "inactiva".to_string());
            assert_eq!(u3.suscripcion.tipo, Tipos::CLASIC);
        }

        #[test]
        fn test_general() {
            let s1 = Suscripcion::new(500.0, Tipos::CLASIC, 6, "13/06/2025".to_string(), Pagos::MERCADOPAGO, "activa".to_string());
            let s2 = Suscripcion::new(300.0, Tipos::BASIC, 6, "13/06/2025".to_string(), Pagos::TRANSFERENCIA, "inactiva".to_string());
            let s3 = Suscripcion::new(800.0, Tipos::CLASIC, 12, "13/06/2025".to_string(), Pagos::MERCADOPAGO, "activa".to_string());
            let s4 = Suscripcion::new(300.0, Tipos::BASIC, 6, "13/06/2025".to_string(), Pagos::TRANSFERENCIA, "inactiva".to_string());
            let s5 = Suscripcion::new(300.0, Tipos::BASIC, 12, "13/06/2025".to_string(), Pagos::TRANSFERENCIA, "inactiva".to_string());

            let u1 = Usuario::new("Fermin Moras".to_string(), s1);
            let u2 = Usuario::new("Delfina Ferrante".to_string(), s2);
            let u3 = Usuario::new("Wanchope Abila".to_string(), s3);
            let u4 = Usuario::new("Colo Barco".to_string(), s4);
            let u5 = Usuario::new("Equi Fernandez".to_string(), s5);


            let mut vec_usuarios = Vec::new();

            vec_usuarios.push(u1.clone());
            vec_usuarios.push(u2);
            vec_usuarios.push(u3);
            vec_usuarios.push(u4);
            vec_usuarios.push(u5);

            let p = Plataforma::new(vec_usuarios.clone());
            
            //suscripcion mas usada de activos
            assert_eq!(p.suscripcion_mas_activa(), Tipos::CLASIC);

            //suscripciones mas usadas en total
            assert_eq!(p.suscripcion_mas_usada(), Tipos::BASIC);

            //pago mas usado de activos
            assert_eq!(p.pago_mas_activo(), Pagos::MERCADOPAGO);
            //pago mas usado en total
            assert_eq!(p.pago_mas_usado(), Pagos::TRANSFERENCIA);
        }
    }