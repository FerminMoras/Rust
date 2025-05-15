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
        let mes:u16 = self.mes;
        if (mes >= 1) && (mes <= 12) {
            match mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                    if(self.dia >= 1) && (self.dia <=31) {
                        true
                    } else {
                        false
                    }
                }
                4 | 6 | 9 | 11 => {
                    if(self.dia >= 1) && (self.dia <=30) {
                        true
                    } else {
                        false
                    }
                }
                2 => {
                    if self.es_bisiesto() {
                        if(self.dia >= 1) && (self.dia <= 29) {
                            true
                        } else {
                            false
                        }
                    }else {
                        if(self.dia >= 1) && (self.dia <= 28) {
                            true
                        } else {
                            false
                        }
                    }
                }
                _ => false 
            }
        }else {
            false
        }    
    }

    fn que_mes(&self) -> u16 {
        let mes = self.mes;
        match mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                    31
            }
            4 | 6 | 9 | 11 => {
                    30
            }
            2 => {
                if self.es_bisiesto() {
                    29
                }else {
                    28
                }
            }
            _ => 0   
        }
    }

    fn es_bisiesto(&self) -> bool {
        if self.año % 4 == 0 && self.año % 100 != 0 || self.año % 400 == 0 {
            return true
        }else{
            return false
        }       
    }

    fn sumar_dias(&mut self, mut dias: u16) {
    
        let mut mes = self.que_mes();
        let dias_disp = mes - self.dia;
        if dias <= dias_disp {
            self.dia = self.dia + dias;
        }
        else {
            while dias != 0 {
                if self.dia <= mes {
                    self.dia = self.dia + 1;
                    dias = dias -1;
                }
                else {
                    self.dia = 1;
                    if self.mes == 12 {
                        self.año = self.año + 1;
                        self.mes = 1;
                        mes = self.que_mes();
                    }
                    else{
                        self.mes = self.mes + 1;
                        mes = self.que_mes();
                    }
                }
            }
        }     
    }    

    fn restar_dias(&mut self, mut dias: u16){
        let dias_disp = self.dia;
        if dias < dias_disp {
            self.dia = self.dia - dias;
        }
        else {
            while dias != 0 {
                if self.dia > 1 {
                    self.dia = self.dia - 1;
                    dias = dias -1;
                }
                else {
                    if self.mes == 1 {
                        self.año = self.año - 1;
                        self.mes = 12;
                    }
                    else{
                        self.mes = self.mes - 1;
                    }
                    self.dia = self.que_mes();
                    dias = dias-1;
                }
            }
        }  
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
        let f = Fecha::new(29, 2, 2024);
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
        let mut  f1 = Fecha::new(13, 5, 2020);
        f1.sumar_dias(10);
        assert_eq!(f1.dia,23);
        
        
        let mut  f2 = Fecha::new(13, 5, 2020);
        f2.sumar_dias(20);
        assert_eq!(f2.dia,2);
        assert_eq!(f2.mes,6);

        let mut  f = Fecha::new(13, 12, 2020);
        f.sumar_dias(20);
        assert_eq!(f.dia,2);
        assert_eq!(f.mes,1);
        assert_eq!(f.año,2021);
    }

    #[test]
    fn test_restar_dias() {
        let mut  f1 = Fecha::new(1, 5, 2020);
        f1.restar_dias(1);
        assert_eq!(f1.dia,30);
        assert_eq!(f1.mes,4);
        
        let mut  f2 = Fecha::new(1, 1, 2020);
        f2.restar_dias(1);
        assert_eq!(f2.dia,31);
        assert_eq!(f2.mes,12);
        assert_eq!(f2.año,2019);

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