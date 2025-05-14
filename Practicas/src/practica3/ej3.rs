struct Fecha {
    dia: u16,
    mes: u16,
    año: u16,
}

impl Fecha {
    fn new(day: u16, month:u16, year:u16) -> Fecha {      
        Fecha {
            dia: day,
            mes: month,
            año: year,
        }    
    }
    
    fn es_fecha_valida(&self) -> bool {
        if self.dia >=1 && self.dia <=31 && self.mes >=1 && self.mes <=12 && self.año >= 0 {
            true
        }else {
            false
        }
    }

    fn es_bisiesto(&self) -> bool {
        if self.año % 4 == 0 && self.año % 100 != 0 || self.año % 400 == 0 {
            return true
        }else{
            return false
        }       
    }
    

    fn sumar_dias(&mut self, dias: u16){
        self.dia = self.dia + dias;
    }

    fn restar_dias(&mut self, dias: u16){
        self.dia = self.dia - dias;
    }

    fn es_mayor(&self, una_fecha: Fecha) -> bool {
        if self.año > una_fecha.año {
            return true
        }else if self.año == una_fecha.año && self.mes > una_fecha.mes{
            return true 
        }else if self.año == una_fecha.año && self.mes == una_fecha.mes && self.dia > una_fecha.dia{
            return true
        }else {
            false
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_fecha(){
        let f = Fecha::new(13, 5, 2024);
        assert_eq!(f.dia,13);
        assert_eq!(f.mes,5);
        assert_eq!(f.año,2024);
    }

    #[test]
    fn test_fecha_valida_true() {
        let f = Fecha::new(13, 5, 2024);
        let ok = f.es_fecha_valida();
        assert_eq!(ok,true);
    }

    #[test]
    fn test_fecha_valida_false() {
        let f = Fecha::new(13, 25, 2024);
        let ok = f.es_fecha_valida();
        assert_eq!(ok,false);
    }

    #[test]
    fn test_es_bisiesto() {
        let f = Fecha::new(13, 2, 2020);
        assert_eq!(f.es_bisiesto(),true);
    }

    #[test]
    fn test_sumar_dias() {
        let mut  f = Fecha::new(13, 2, 2020);
        f.sumar_dias(10);
        assert_eq!(f.dia,23);
    }

    #[test]
    fn test_restar_dias() {
        let mut  f = Fecha::new(13, 2, 2020);
        f.restar_dias(10);
        assert_eq!(f.dia,3);
    }

    #[test]
    fn test_fecha_mayor() {
        let f1 = Fecha::new(13, 12, 2020);
        let f2 = Fecha::new(13, 2, 2019);
        let f3 = Fecha::new(13, 10, 2020);
        let f4 = Fecha::new(7, 12, 2020);
        assert_eq!(f1.es_mayor(f2),true);
        assert_eq!(f1.es_mayor(f3),true);
        assert_eq!(f1.es_mayor(f4),true);
    }
}